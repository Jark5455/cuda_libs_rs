//! AST-level preprocessor stage.
//!
//! Runs *after* the chevron rewrite. Parses the rewritten source via `syn`,
//! finds items annotated with `#[global]`, and produces:
//!
//! - A *host* source: each `#[global] fn name(...) { body }` becomes a
//!   stub that panics if called directly, accompanied by a static
//!   `__cuda_kernel_<name>: KernelHandle` that the chevron-rewrite output
//!   refers to.
//! - A *device* source: each kernel function with body intact, wrapped in a
//!   `#[no_std]` nvptx harness with `extern "ptx-kernel"` ABI and `#[no_mangle]`.
//!
//! Items that are *not* `#[global]` are copied verbatim to the host source.
//! Top-level structs/enums/types/consts reachable from a kernel signature or
//! body are also copied into the device source.

use std::collections::{BTreeSet, HashMap};

use proc_macro2::Span;
use quote::format_ident;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{Attribute, Expr, File, Ident, Item, ItemFn, ItemStatic, Meta, Stmt, Type};

#[derive(Debug, Clone)]
pub struct KernelMeta {
    pub name: String,
}

#[derive(Debug)]
pub struct ExtractOutput {
    pub host_src: String,
    /// `Some` only if at least one kernel was found. Single-file render of
    /// the device crate; for multi-file aggregation, prefer the raw
    /// `device_fns` / `reachable_items` fields and call `render_device_aggregate`.
    pub device_src: Option<String>,
    /// Kernel function definitions extracted from this file's `#[global]`
    /// items (with `extern "ptx-kernel"` ABI + `#[no_mangle]` already
    /// applied). Empty if no kernels were found.
    pub device_fns: Vec<ItemFn>,
    /// Top-level structs/enums/unions/types/consts from this file that are
    /// reachable from any of `device_fns`. Copied verbatim into the
    /// aggregated device crate.
    pub reachable_items: Vec<Item>,
    pub kernels: Vec<KernelMeta>,
}

pub fn extract(src: &str) -> Result<ExtractOutput, syn::Error> {
    // Fast path: if the source doesn't textually contain `#[global]`, there
    // are no kernels to extract. Skip the syn round-trip entirely so files
    // unrelated to CUDA don't get reformatted (which would break `mod foo;`
    // resolution since the rewritten file lives in a different directory than
    // the original). Tolerate whitespace / `cfg_attr` wrappers via the
    // looser substring check on `global` only when `#[` precedes it within a
    // few chars; in practice the cheap exact-match catches the common form.
    if !src.contains("#[global]") && !src.contains("global ]") && !src.contains("cfg_attr") {
        return Ok(ExtractOutput {
            host_src: src.to_owned(),
            device_src: None,
            device_fns: Vec::new(),
            reachable_items: Vec::new(),
            kernels: Vec::new(),
        });
    }

    let mut file: File = syn::parse_str(src)?;

    // Snapshot top-level type and const definitions from the original file before
    // any kernel-extraction mutation. These are the items we may need to copy into
    // the device crate (host structs reachable from kernels).
    let device_eligible: HashMap<String, Item> = file.items.iter().filter_map(|i| device_eligible_ident(i).map(|n| (n, i.clone()))).collect();

    let mut kernels = Vec::<KernelMeta>::new();
    let mut device_fns = Vec::<ItemFn>::new();

    let mut visitor = KernelVisitor {
        kernels: &mut kernels,
        device_fns: &mut device_fns,
        seen_names: BTreeSet::new(),
    };
    visitor.visit_file_mut(&mut file);

    if kernels.is_empty() {
        // The string contained `global` but no actual `#[global]`
        // attribute (e.g. a comment or a string). Don't reformat.
        return Ok(ExtractOutput {
            host_src: src.to_owned(),
            device_src: None,
            device_fns: Vec::new(),
            reachable_items: Vec::new(),
            kernels: Vec::new(),
        });
    }

    let host_src = render_host(&file);

    let reachable_items = collect_reachable_items(&device_fns, &device_eligible);
    let device_src = if device_fns.is_empty() { None } else { Some(render_device(&device_fns, &reachable_items)) };

    Ok(ExtractOutput {
        host_src,
        device_src,
        device_fns,
        reachable_items,
        kernels,
    })
}

/// Render an aggregated device crate from `device_fns` + `reachable_items`
/// pulled from multiple host source files. Used by the mod-walker
/// coordinator to produce a single device crate per host crate.
pub fn render_device_aggregate(device_fns: &[ItemFn], reachable: &[Item]) -> String {
    render_device(device_fns, reachable)
}

struct KernelVisitor<'a> {
    kernels: &'a mut Vec<KernelMeta>,
    device_fns: &'a mut Vec<ItemFn>,
    seen_names: BTreeSet<String>,
}

impl VisitMut for KernelVisitor<'_> {
    fn visit_item_mut(&mut self, item: &mut Item) {
        // Detect kernels and replace the entire `Item::Fn(...)` node with an
        // `Item::Static(...)` of the same name. This makes the kernel an
        // ordinary importable item — `use crate::kernels::print_arr;` brings
        // the `KernelHandle` into scope, and `print_arr<<<g, b>>>(args)`
        // (whose chevron rewrite emits `&print_arr`) resolves to that handle.
        // The kernel body lives only in the device crate.
        if let Item::Fn(item_fn) = item
            && let Some(extracted) = take_if_kernel(item_fn)
        {
            let name_ident = item_fn.sig.ident.clone();
            let name_str = name_ident.to_string();
            if !self.seen_names.insert(name_str.clone()) {
                // Duplicate kernel name — `cuModuleGetFunction` would only
                // resolve the first; skip the duplicate. TODO: catch this
                // earlier with a top-level `compile_error!`.
            }
            self.kernels.push(KernelMeta { name: name_str.clone() });
            self.device_fns.push(extracted);

            let vis = item_fn.vis.clone();
            let name_lit = syn::LitStr::new(&name_str, name_ident.span());
            let mut static_item: ItemStatic = syn::parse_quote! {
                #[allow(non_upper_case_globals, dead_code)]
                #vis static #name_ident: ::cuda_rustc_runtime::KernelHandle =
                    ::cuda_rustc_runtime::KernelHandle::new(#name_lit);
            };
            strip_marker_attrs(&mut static_item.attrs);
            *item = Item::Static(static_item);
            return;
        }
        syn::visit_mut::visit_item_mut(self, item);
    }
}

/// If `item` carries `#[global]`, return a clone of `item` *with* the
/// attribute removed and ABI/mangle set up for nvptx. The original `item` is
/// left intact for the caller to mutate into a host stub.
///
/// Also walks the cloned body and rewrites any `#[shared] static mut` items
/// into a form whose PTX symbol carries a recognizable `__cuda_shared_` prefix
/// so the PTX post-processor can switch the storage class to `.shared`.
fn take_if_kernel(item: &ItemFn) -> Option<ItemFn> {
    if !has_global(&item.attrs) {
        return None;
    }
    let mut device_fn = item.clone();
    strip_marker_attrs(&mut device_fn.attrs);
    device_fn.attrs.push(syn::parse_quote!(#[unsafe(no_mangle)]));
    device_fn.sig.abi = Some(syn::parse_quote!(extern "ptx-kernel"));
    device_fn.vis = syn::Visibility::Public(syn::parse_quote!(pub));

    let kernel_name = item.sig.ident.to_string();
    let mut shared_rewriter = SharedStaticRewriter { kernel_name: &kernel_name, dyn_bindings: Vec::new() };
    syn::visit_mut::visit_item_fn_mut(&mut shared_rewriter, &mut device_fn);

    if !shared_rewriter.dyn_bindings.is_empty() {
        prepend_dyn_bindings(&mut device_fn, &shared_rewriter.dyn_bindings, &kernel_name);
    }

    Some(device_fn)
}

/// Stash for a `#[shared] static mut DYN: &mut? [T] = ...;` whose declaration
/// was rewritten to a zero-sized marker. The rewriter records each one and the
/// caller injects a `let DYN: &mut? [T] = unsafe { dynamic_shared::<T>(...) };`
/// at the top of the kernel body so user references to `DYN` keep type-checking.
struct DynSharedBinding {
    /// User's original identifier (e.g. `DYN`).
    user_ident: Ident,
    /// Element type `T`.
    elem_type: Type,
    /// Whether the original was `&mut [T]` (true) or `&[T]` (false).
    mutable: bool,
}

/// Walks a kernel body and rewrites every `#[shared] static mut NAME: TY = INIT;`
/// into a form that pins its PTX symbol to `__cuda_shared_<kernel>__<name>` via
/// `#[unsafe(export_name = …)]`. The user-visible identifier is preserved so
/// the kernel body's references to the static keep compiling.
///
/// Why an explicit `export_name` rather than just renaming the identifier? If
/// we renamed, every reference inside the body would need to be updated too —
/// solvable with another visitor pass but more invasive than necessary. Pinning
/// the symbol is simpler and lets the PTX post-processor reliably find the
/// `.global` decl that needs to become `.shared`.
struct SharedStaticRewriter<'a> {
    kernel_name: &'a str,
    dyn_bindings: Vec<DynSharedBinding>,
}

impl VisitMut for SharedStaticRewriter<'_> {
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        if let Stmt::Item(Item::Static(s)) = stmt
            && has_shared(&s.attrs)
        {
            self.process_shared(s);
        }
        syn::visit_mut::visit_stmt_mut(self, stmt);
    }

    fn visit_item_mut(&mut self, item: &mut Item) {
        if let Item::Static(s) = item
            && has_shared(&s.attrs)
        {
            self.process_shared(s);
        }
        syn::visit_mut::visit_item_mut(self, item);
    }
}

impl SharedStaticRewriter<'_> {
    fn process_shared(&mut self, s: &mut ItemStatic) {
        // First check whether the static's type is `&[T]` / `&mut [T]` —
        // dynamic shared memory. If so, rewrite to a zero-size marker and
        // record a let-binding to inject at the top of the kernel.
        if let Some(binding) = try_take_dyn_shared(s, self.kernel_name) {
            self.dyn_bindings.push(binding);
            return;
        }
        // Static-shared fallback path: pin symbol via `export_name` so the PTX
        // post-processor can find it.
        rewrite_shared_static(s, self.kernel_name);
    }
}

fn has_shared(attrs: &[Attribute]) -> bool {
    attrs.iter().any(is_shared_attr)
}

fn is_shared_attr(attr: &Attribute) -> bool {
    let path = match &attr.meta {
        Meta::Path(p) => p,
        Meta::List(ml) => &ml.path,
        Meta::NameValue(nv) => &nv.path,
    };
    path.is_ident("shared")
}

fn rewrite_shared_static(s: &mut ItemStatic, kernel_name: &str) {
    strip_marker_attrs(&mut s.attrs);
    let symbol = format!("__cuda_shared_{kernel_name}__{}", s.ident);
    let lit = syn::LitStr::new(&symbol, Span::call_site());
    s.attrs.push(syn::parse_quote!(#[unsafe(export_name = #lit)]));
}

/// If `s` declares a `#[shared] static mut? IDENT: &mut? [T] = ...;`, rewrite
/// it in place to a zero-sized marker pinned to a `__cuda_shared_dyn_*` symbol
/// and return a `DynSharedBinding` describing the user-facing slice that the
/// caller should re-introduce as a `let` at the top of the kernel body.
///
/// The marker symbol is what the PTX post-processor recognizes to emit an
/// `.extern .shared` declaration. The let-binding restores `IDENT: &mut? [T]`
/// in the user's scope so existing references type-check.
fn try_take_dyn_shared(s: &mut ItemStatic, kernel_name: &str) -> Option<DynSharedBinding> {
    let (elem_type, mutable) = match &*s.ty {
        Type::Reference(r) => {
            if let Type::Slice(slice) = &*r.elem {
                (Type::clone(&slice.elem), r.mutability.is_some())
            } else {
                return None;
            }
        }
        _ => return None,
    };

    let user_ident = s.ident.clone();
    let new_name = format!("__cuda_shared_dyn_{kernel_name}__{user_ident}");
    let new_ident = format_ident!("{}", new_name);

    // Rewrite to: `static mut __cuda_shared_dyn_K_X: [T; 0] = [];`
    let new_ty: Type = syn::parse_quote!([#elem_type; 0]);
    let new_init: Expr = syn::parse_quote!([]);

    s.ident = new_ident;
    s.ty = Box::new(new_ty);
    s.expr = Box::new(new_init);
    strip_marker_attrs(&mut s.attrs);

    let lit = syn::LitStr::new(&new_name, Span::call_site());
    s.attrs.push(syn::parse_quote!(#[unsafe(export_name = #lit)]));
    s.attrs.push(syn::parse_quote!(#[allow(non_upper_case_globals)]));

    Some(DynSharedBinding { user_ident, elem_type, mutable })
}

/// Inject `let DYN: &mut? [T] = unsafe { dynamic_shared::<T>(&raw mut MARKER as *mut T) };`
/// at the top of the kernel body for each recorded dynamic-shared binding.
/// Insertion order matches recording order so the user's first declaration
/// stays first.
fn prepend_dyn_bindings(device_fn: &mut ItemFn, bindings: &[DynSharedBinding], kernel_name: &str) {
    let mut prepended: Vec<Stmt> = Vec::with_capacity(bindings.len());
    for b in bindings {
        let user = &b.user_ident;
        let elem = &b.elem_type;
        let marker = format_ident!("__cuda_shared_dyn_{}__{}", kernel_name, user);

        let stmt: Stmt = if b.mutable {
            syn::parse_quote!(
                #[allow(unused_mut)]
                let mut #user: &mut [#elem] = unsafe {
                    ::cuda_rustc_runtime::device::dynamic_shared::<#elem>(
                        &raw mut #marker as *mut #elem,
                    )
                };
            )
        } else {
            syn::parse_quote!(
                let #user: &[#elem] = unsafe {
                    &*::cuda_rustc_runtime::device::dynamic_shared::<#elem>(
                        &raw mut #marker as *mut #elem,
                    )
                };
            )
        };
        prepended.push(stmt);
    }

    let mut combined = prepended;
    combined.append(&mut device_fn.block.stmts);
    device_fn.block.stmts = combined;
}

fn has_global(attrs: &[Attribute]) -> bool {
    attrs.iter().any(is_global)
}

fn is_global(attr: &Attribute) -> bool {
    let path = match &attr.meta {
        Meta::Path(p) => p,
        Meta::List(ml) => &ml.path,
        Meta::NameValue(nv) => &nv.path,
    };
    path.is_ident("global")
}

/// Remove any `#[global]` or `#[shared]` attribute from `attrs`. Called on
/// every item the preprocessor touches before emit so the cached output never
/// carries the markers — rustc compiles the cache files with no
/// `cuda_libs_macros` import in scope.
fn strip_marker_attrs(attrs: &mut Vec<Attribute>) {
    attrs.retain(|a| !is_global(a) && !is_shared_attr(a));
}

fn render_host(file: &File) -> String {
    // Final sweep: strip `#[global]` / `#[shared]` from any item that escaped
    // the kernel/shared rewriters (e.g. attribute on a non-`Item::Fn` or
    // nested inside a body the visitors didn't recurse through).
    let mut cleaned = file.clone();
    let mut stripper = MarkerAttrStripper;
    stripper.visit_file_mut(&mut cleaned);
    let mut out = quote::quote!(#cleaned).to_string();
    out.push('\n');
    out
}

/// Walks an entire `syn` AST and removes `#[global]` / `#[shared]` from any
/// item's attribute list. Used as a defensive sweep on both the host and
/// device renders so the cached source never carries the markers — rustc
/// compiles those files with no `cuda_libs_macros` import in scope.
struct MarkerAttrStripper;

impl VisitMut for MarkerAttrStripper {
    fn visit_attributes_mut(&mut self, attrs: &mut Vec<Attribute>) {
        strip_marker_attrs(attrs);
        // No further recursion needed — attributes don't carry nested attrs.
    }
}

fn render_device(device_fns: &[ItemFn], reachable: &[Item]) -> String {
    let mut s = String::new();
    s.push_str("#![no_std]\n");
    s.push_str("#![feature(abi_ptx, stdarch_nvptx, asm_experimental_arch)]\n");
    s.push_str("#![allow(internal_features)]\n");
    s.push_str("#![feature(link_llvm_intrinsics)]\n");
    s.push_str("#![allow(improper_ctypes_definitions)]\n");
    s.push_str("#![allow(unused_features)]\n");
    s.push_str("#![crate_type = \"cdylib\"]\n\n");
    s.push_str("#[panic_handler]\n");
    s.push_str("fn panic(_info: &core::panic::PanicInfo) -> ! { unsafe { ::core::hint::unreachable_unchecked() } }\n\n");

    let mut stripper = MarkerAttrStripper;

    // Copies of top-level types and consts reachable from kernel sigs+bodies.
    //
    // No `#[repr(C)]` requirement is enforced. Both the host and device crates are
    // compiled by the same rustc invocation chain (same toolchain version), so
    // Rust's per-version-stable-but-unspecified default layout (`#[repr(Rust)]`)
    // produces identical field offsets / alignments on both sides.
    for item in reachable {
        let mut cleaned = item.clone();
        stripper.visit_item_mut(&mut cleaned);
        s.push_str(&quote::quote!(#cleaned).to_string());
        s.push_str("\n\n");
    }

    for f in device_fns {
        let mut cleaned = f.clone();
        stripper.visit_item_fn_mut(&mut cleaned);
        s.push_str(&quote::quote!(#cleaned).to_string());
        s.push_str("\n\n");
    }
    s
}

/// Predicate: items eligible to be copied into the device crate. Covers
/// type-shaped items (`struct`, `enum`, `union`, `type` aliases) and `const`s.
/// Helper functions are not eligible — they would need their own `#[device]` opt-in.
fn device_eligible_ident(item: &Item) -> Option<String> {
    match item {
        Item::Struct(s) => Some(s.ident.to_string()),
        Item::Enum(e) => Some(e.ident.to_string()),
        Item::Union(u) => Some(u.ident.to_string()),
        Item::Type(t) => Some(t.ident.to_string()),
        Item::Const(c) => Some(c.ident.to_string()),
        _ => None,
    }
}

/// Walk every kernel function (signature + body) and every item reached from
/// them, collecting the set of top-level idents that name items in `eligible`.
/// Returns the items in their original file order.
fn collect_reachable_items(kernels: &[ItemFn], eligible: &HashMap<String, Item>) -> Vec<Item> {
    let mut reached: BTreeSet<String> = BTreeSet::new();
    let mut worklist: Vec<String> = Vec::new();

    for k in kernels {
        let mut col = IdentCollector { found: BTreeSet::new() };
        col.visit_item_fn(k);
        for name in col.found {
            if eligible.contains_key(&name) && reached.insert(name.clone()) {
                worklist.push(name);
            }
        }
    }

    while let Some(name) = worklist.pop() {
        let Some(item) = eligible.get(&name) else { continue };
        let mut col = IdentCollector { found: BTreeSet::new() };
        col.visit_item(item);
        for n in col.found {
            if eligible.contains_key(&n) && reached.insert(n.clone()) {
                worklist.push(n);
            }
        }
    }

    eligible.iter().filter(|(name, _)| reached.contains(*name)).map(|(_, item)| item.clone()).collect()
}

/// Visit a syn AST node and collect the identifiers of every `Path` it walks.
/// We over-approximate: any first-segment ident is added. The reachability
/// caller filters against the eligible-items map, so foreign idents (`Vec`,
/// `core`, etc.) drop out naturally.
struct IdentCollector {
    found: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for IdentCollector {
    fn visit_path(&mut self, p: &'ast syn::Path) {
        if let Some(seg) = p.segments.first() {
            self.found.insert(seg.ident.to_string());
        }
        syn::visit::visit_path(self, p);
    }

    fn visit_ident(&mut self, i: &'ast Ident) {
        self.found.insert(i.to_string());
    }
}

/// Append the fatbin embedding + registration ctor to a host source string.
///
/// Emits four `static`s in user-visible link sections:
/// - `.nv_fatbin`        — the NVIDIA fatbinary blob (magic `0xBA55ED50`)
///                         produced by the `fatbinary` tool from per-arch
///                         cubins.
/// - `.nvFatBinSegment`  — a `__fatBinC_Wrapper_t` referencing the blob.
/// - `.__nv_module_id`   — the unique module identifier string.
/// - `.init_array`       — function pointer to a registration ctor that runs
///                         once at process startup and registers the wrapper
///                         with the runtime.
///
/// Layout matches what nvcc emits, so `cuobjdump` and other CUDA tools that
/// scan ELF sections recognize the blob and can dump per-arch cubins / SASS.
pub fn fatbin_embed_block(fatbin_path: &std::path::Path, module_id: &str) -> String {
    let mid_bytes = sanitize_module_id(module_id);
    let mid_lit = bytes_as_byte_string(&mid_bytes);
    let mid_len = mid_bytes.len();

    let ctor_section_attrs = r#"#[cfg_attr(any(target_os = "linux", target_os = "android", target_os = "freebsd"), unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]"#;

    let path_str = escape_for_string_literal(&fatbin_path.to_string_lossy());

    format!(
        r#"
// --- cuda_rustc generated fatbin embedding ---

#[allow(non_upper_case_globals)]
#[unsafe(link_section = ".nv_fatbin")]
#[used]
static __CUDA_RUSTC_FATBIN_BLOB: [u8; include_bytes!("{path_str}").len()] = *include_bytes!("{path_str}");

#[allow(non_upper_case_globals)]
#[unsafe(link_section = ".nvFatBinSegment")]
#[used]
static __CUDA_RUSTC_FATBIN_WRAPPER: ::cuda_rustc_runtime::registration::FatBinWrapper =
    ::cuda_rustc_runtime::registration::FatBinWrapper {{
        magic: 0x466243B1,
        version: 1,
        data: __CUDA_RUSTC_FATBIN_BLOB.as_ptr(),
        filename: ::core::ptr::null(),
    }};

#[allow(non_upper_case_globals)]
#[unsafe(link_section = ".__nv_module_id")]
#[used]
static __CUDA_RUSTC_MODULE_ID: [u8; {mid_len}] = *b"{mid_lit}";

extern "C" fn __cuda_rustc_register() {{
    ::cuda_rustc_runtime::registration::register_fatbin(&__CUDA_RUSTC_FATBIN_WRAPPER);
}}

{ctor_section_attrs}
#[allow(non_upper_case_globals)]
#[used]
static __CUDA_RUSTC_REGISTER_FN: extern "C" fn() = __cuda_rustc_register;
"#,
        path_str = path_str,
        mid_len = mid_len,
        mid_lit = mid_lit,
        ctor_section_attrs = ctor_section_attrs,
    )
}

fn escape_for_string_literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn sanitize_module_id(id: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = id.bytes().map(|b| if b.is_ascii_alphanumeric() || b == b'_' { b } else { b'_' }).collect();
    bytes.push(0);
    bytes
}

fn bytes_as_byte_string(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        match b {
            b'"' => out.push_str("\\\""),
            b'\\' => out.push_str("\\\\"),
            0 => out.push_str("\\0"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            b if (b as u32) < 0x20 || b >= 0x7f => out.push_str(&format!("\\x{:02x}", b)),
            b => out.push(b as char),
        }
    }
    out
}

/// Convenience: run the extraction. Kept for backward compat with existing
/// callers and tests; kernel handles are now emitted inline at each kernel's
/// original location (as `pub static <name>: KernelHandle`), so no separate
/// finalize step is needed.
pub fn extract_and_finalize(src: &str) -> Result<ExtractOutput, syn::Error> {
    extract(src)
}
