//! Source-text preprocessor.
//!
//! Two stages, run in order on every `.rs` input:
//!
//! 1. **Chevron rewrite** (`rewrite.rs`): `kernel<<<...>>>(args)` → runtime call.
//!    Pure text-level. Skips comments/strings via `lexer.rs`.
//! 2. **AST extraction** (`ast.rs`): parse the rewritten source via syn, find
//!    `#[global]` items, split into host (with stubs + handle statics) and
//!    device (nvptx no_std harness) sources.
//!
//! `process_file` returns paths to the host and device cache files plus the kernel
//! metadata. The driver hands the host path to rustc as the input source and, if
//! a device source was produced, kicks off a separate device compilation.

pub mod ast;
pub mod lexer;
pub mod rewrite;
pub mod shared_decl;

use std::io;
use std::path::{Path, PathBuf};

pub use ast::KernelMeta;

#[derive(Debug)]
pub struct PreprocOutput {
    /// Path that should be handed to rustc as the input source.
    pub host_path: PathBuf,
    /// Path of the generated device-side `.rs` file, if any kernels were found.
    pub device_path: Option<PathBuf>,
    /// Kernels discovered in the source.
    pub kernels: Vec<KernelMeta>,
}

pub fn process_file(input: &Path, cache_root: &Path, crate_name: Option<&str>) -> io::Result<PreprocOutput> {
    // Bypass third-party dependencies entirely. `RUSTC_WRAPPER=cuda_rustc`
    // intercepts every rustc invocation cargo makes, including transitive
    // crates pulled from the registry / git cache. Those crates never
    // contain `#[global]` kernels, but they do contain macro-heavy code
    // (e.g. libc's `prelude!()`/`cfg_if!`) that doesn't always survive a
    // syn round-trip cleanly. Skip them so cuda_rustc only ever rewrites
    // the user's own source.
    if is_third_party_source(input) {
        return Ok(PreprocOutput {
            host_path: input.to_path_buf(),
            device_path: None,
            kernels: Vec::new(),
        });
    }

    // Walk the mod tree starting at `input`, preprocessing each `mod foo;`
    // referenced file. Aggregated device fns + reachable items are merged
    // into a single device source.
    let mut acc = DeviceAccumulator::default();
    let mut all_kernels: Vec<KernelMeta> = Vec::new();

    // Crate root's submod search dir: parent of the entry file. Per
    // Rust's module system, both `src/main.rs` and `src/lib.rs` resolve
    // their child mods relative to `src/`.
    let initial_mod_dir = input.parent().map(Path::to_path_buf).unwrap_or_else(|| PathBuf::from("."));

    let host_path = match process_recursive(input, &initial_mod_dir, cache_root, crate_name, &mut acc, &mut all_kernels) {
        Ok(p) => p,
        Err(e) => {
            // If preprocessing the root file fails outright (read error,
            // syn parse failure that even the chevron-only fallback can't
            // recover from), fall through with the original input so
            // rustc surfaces the real diagnostic.
            eprintln!("cuda_rustc: process_recursive failed on {}: {e}; falling back to original source", input.display());
            return Ok(PreprocOutput {
                host_path: input.to_path_buf(),
                device_path: None,
                kernels: Vec::new(),
            });
        }
    };

    let device_path = if acc.device_fns.is_empty() {
        None
    } else {
        let device_src = ast::render_device_aggregate(&acc.device_fns, &acc.reachable_items);
        Some(write_cache(input, cache_root, crate_name, "device", &device_src)?)
    };

    Ok(PreprocOutput { host_path, device_path, kernels: all_kernels })
}

/// Per-crate accumulator of device-side artifacts. Each preprocessed file
/// pushes its `#[global]` fns and reachable host structs into here; the
/// coordinator emits one merged device crate at the end.
#[derive(Default)]
struct DeviceAccumulator {
    device_fns: Vec<syn::ItemFn>,
    reachable_items: Vec<syn::Item>,
}

/// Preprocess `input`, recursively descend into any external `mod foo;`
/// declarations it contains, inject `#[path = "<cache>"]` overrides on
/// those declarations so rustc reads our rewritten copies, and append the
/// file's kernels + reachable items to `acc`. Returns the cache-file path
/// rustc should use for `input`.
fn process_recursive(input: &Path, mod_dir: &Path, cache_root: &Path, crate_name: Option<&str>, acc: &mut DeviceAccumulator, kernels_out: &mut Vec<KernelMeta>) -> io::Result<PathBuf> {
    let source = std::fs::read_to_string(input)?;

    // Stage 0: synthesize missing `= EXPR` initializer for `#[shared]` slice
    // statics that omit one. Runs before the chevron rewrite so subsequent
    // stages always see syntactically valid Rust.
    let rhs_injected = shared_decl::inject_missing_rhs(&source);

    // Stage 1: chevron rewrite.
    let rewritten = rewrite::rewrite(&rhs_injected);

    // Stage 2: AST extraction. On parse failure, fall through with chevron-only
    // text so the user gets rustc's real diagnostic instead of a syn error.
    let extract = match ast::extract(&rewritten) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("cuda_rustc: AST extraction failed for {}: {e}; emitting chevron-rewrite only", input.display());
            return if rewritten == source { Ok(input.to_path_buf()) } else { write_cache(input, cache_root, crate_name, "host", &rewritten) };
        }
    };

    // Stage 3: aggregate this file's device contributions.
    acc.device_fns.extend(extract.device_fns.iter().cloned());
    acc.reachable_items.extend(extract.reachable_items.iter().cloned());
    kernels_out.extend(extract.kernels.iter().cloned());

    // Stage 4: descend into the file's mod tree. The walker visits every
    // `Item::Mod` — both external (`mod foo;`) and inline (`mod foo { ... }`)
    // — recursing into inline-mod bodies with `mod_dir/<name>` (Rust's
    // rule: each inline mod nests one directory level for sub-external
    // resolution) and recursing into external mods by reading the resolved
    // file from disk. External mods get `#[path = "<cache>"]` injected so
    // rustc reads our preprocessed copy.
    let mut file: syn::File = match syn::parse_str(&extract.host_src) {
        Ok(f) => f,
        Err(_) => {
            // host_src didn't round-trip through syn (extremely rare since
            // `extract` already parsed it). Fall back to writing chevron-
            // rewrite output without mod-walking.
            return if extract.host_src == source { Ok(input.to_path_buf()) } else { write_cache(input, cache_root, crate_name, "host", &extract.host_src) };
        }
    };
    let mut changed = walk_mods(&mut file.items, mod_dir, cache_root, crate_name, acc, kernels_out)?;
    changed |= extract.host_src != source;

    // Stage 5: write to cache only if content changed.
    if !changed {
        Ok(input.to_path_buf())
    } else {
        let host_src = quote::quote!(#file).to_string();
        write_cache(input, cache_root, crate_name, "host", &host_src)
    }
}

/// Walk `items` looking for `Item::Mod` nodes. Per Rust's module system:
///   - **Inline mod** `mod foo { ... }`: descend into `m.content.items`
///     with `mod_dir = current_mod_dir/<foo>`. The inline mod itself does
///     not correspond to a file on disk, but its sub-`mod bar;` decls
///     resolve as if they were external mods inside a file at
///     `<mod_dir>/foo/`. This rule applies uniformly regardless of how
///     deeply the inline mod is nested.
///   - **External mod** `mod foo;` (or `#[path = "x"] mod foo;`): resolve
///     to a source file (default search or user-supplied `#[path]`),
///     recursively preprocess it, then inject `#[path = "<cache>"]` on
///     this decl so rustc reads our preprocessed copy. Submod search
///     dir for the recursed file is `<current_mod_dir>/<foo>` regardless
///     of whether the resolved source was `foo.rs` or `foo/mod.rs`.
///
/// Returns `true` if any mod was rewritten (so the caller knows to
/// re-serialize the file).
fn walk_mods(items: &mut Vec<syn::Item>, mod_dir: &Path, cache_root: &Path, crate_name: Option<&str>, acc: &mut DeviceAccumulator, kernels_out: &mut Vec<KernelMeta>) -> io::Result<bool> {
    let mut changed = false;
    for item in items.iter_mut() {
        let syn::Item::Mod(m) = item else { continue };
        let name = m.ident.to_string();

        if let Some((_, inline_items)) = &mut m.content {
            // Inline mod — descend with the directory level appended but
            // do not touch this file's bytes (the inline body's bytes are
            // already part of `host_src`; we only mutate child external
            // mod decls).
            let sub_dir = mod_dir.join(&name);
            if walk_mods(inline_items, &sub_dir, cache_root, crate_name, acc, kernels_out)? {
                changed = true;
            }
            continue;
        }

        // External mod. Honor a user-supplied `#[path = "..."]` attribute
        // if present; otherwise fall back to Rust's default search order.
        let resolved = user_supplied_path(&m.attrs, mod_dir).or_else(|| resolve_submod(mod_dir, &name));
        let Some(sub_file) = resolved else {
            // Neither `<mod_dir>/<name>.rs` nor `<mod_dir>/<name>/mod.rs`
            // exists. Leave the decl alone — rustc will surface its own
            // `file not found for module` error pointing at the user's
            // source. (This is the right outcome: we shouldn't fail the
            // wrapper just because of a missing module file.)
            continue;
        };

        let sub_mod_dir = mod_dir.join(&name);
        let sub_cache = process_recursive(&sub_file, &sub_mod_dir, cache_root, crate_name, acc, kernels_out)?;

        // Replace any pre-existing `#[path]` attribute (we've consumed it
        // already to find `sub_file`) with our cache-pointing one.
        m.attrs.retain(|a| !is_path_attr(a));
        let lit = syn::LitStr::new(&sub_cache.display().to_string(), proc_macro2::Span::call_site());
        m.attrs.push(syn::parse_quote!(#[path = #lit]));
        changed = true;
    }
    Ok(changed)
}

/// Detect cargo registry / git cache paths so the preprocessor can bail
/// out on third-party deps. `RUSTC_WRAPPER` intercepts every rustc
/// invocation cargo issues, so without this gate we'd re-parse every
/// transitive crate's source through `syn`+`quote::quote!` round-trip.
/// Macro-heavy crates (libc's `prelude!`/`cfg_if!`, etc.) don't always
/// survive that cleanly. Those crates never carry `#[global]` kernels,
/// so skipping them is safe.
fn is_third_party_source(p: &Path) -> bool {
    let s = p.to_string_lossy();
    s.contains("/.cargo/registry/") || s.contains("/.cargo/git/") || s.contains("/rustlib/") || s.contains("/.rustup/")
}

/// Resolve `mod <name>;` to a source file via Rust's default search:
/// `<mod_dir>/<name>.rs` (file form) takes precedence, then
/// `<mod_dir>/<name>/mod.rs` (mod.rs form).
fn resolve_submod(mod_dir: &Path, name: &str) -> Option<PathBuf> {
    let file_form = mod_dir.join(format!("{name}.rs"));
    if file_form.exists() {
        return Some(file_form);
    }
    let mod_rs_form = mod_dir.join(name).join("mod.rs");
    if mod_rs_form.exists() {
        return Some(mod_rs_form);
    }
    None
}

/// If the mod decl carries `#[path = "..."]`, resolve it relative to
/// `mod_dir` (rustc resolves user-supplied paths relative to the directory
/// containing the file in which the `mod` decl appears, which is the same
/// as our current `mod_dir` traversal state).
fn user_supplied_path(attrs: &[syn::Attribute], mod_dir: &Path) -> Option<PathBuf> {
    for attr in attrs {
        if !is_path_attr(attr) {
            continue;
        }
        if let syn::Meta::NameValue(nv) = &attr.meta
            && let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value
        {
            let raw = s.value();
            let candidate = if Path::new(&raw).is_absolute() { PathBuf::from(raw) } else { mod_dir.join(raw) };
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }
    None
}

fn is_path_attr(attr: &syn::Attribute) -> bool {
    let path = match &attr.meta {
        syn::Meta::Path(p) => p,
        syn::Meta::List(ml) => &ml.path,
        syn::Meta::NameValue(nv) => &nv.path,
    };
    path.is_ident("path")
}

fn write_cache(input: &Path, cache_root: &Path, crate_name: Option<&str>, label: &str, content: &str) -> io::Result<PathBuf> {
    let stem = input.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "input.rs".to_string());

    let key = file_cache_key(input, crate_name);
    let out = cache_root.join(format!("{key}-{label}-{stem}"));

    if let Ok(prev) = std::fs::read_to_string(&out)
        && prev == content
    {
        return Ok(out);
    }

    std::fs::write(&out, content)?;
    Ok(out)
}

fn file_cache_key(input: &Path, crate_name: Option<&str>) -> String {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    input.hash(&mut h);
    if let Some(n) = crate_name {
        n.hash(&mut h);
    }
    format!("{:016x}", h.finish())
}
