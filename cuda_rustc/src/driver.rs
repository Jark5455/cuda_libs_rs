//! Wrapper-driver entrypoint.
//!
//! Cargo invokes us as `cuda_rustc <RUSTC> <ARGS...>`. We parse the args just enough
//! to find the input `.rs` file path, run it through the preprocessor, swap the
//! input arg with the preprocessed host file, and exec the underlying rustc. If
//! the preprocessor produced a device-side source (because the file contained
//! `#[global]` items), we additionally build a derived nvptx crate to PTX.

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::preproc;
use crate::preproc::{PreprocOutput, ast};
use crate::ptx;

pub type Result<T> = std::result::Result<T, String>;

pub fn run() -> Result<u8> {
    let mut args: Vec<OsString> = env::args_os().collect();
    if args.len() < 2 {
        return Err("expected at least one argument (the underlying rustc path)".into());
    }

    args.remove(0);
    let rustc = args.remove(0);

    if is_probe_invocation(&args) {
        return exec_rustc(&rustc, &args);
    }

    let cuda_enabled = is_cuda_enabled();

    if cuda_enabled && let Some(input_idx) = find_input_index(&args) {
        let input_path = PathBuf::from(&args[input_idx]);
        let crate_name = extract_crate_name(&args);
        let cache_root = preproc_cache_root(&args)?;

        match preproc::process_file(&input_path, &cache_root, crate_name.as_deref()) {
            Ok(output) => {
                if output.host_path != input_path {
                    args[input_idx] = output.host_path.clone().into_os_string();
                }
                if output.device_path.is_some() {
                    if let Err(e) = build_device(&rustc, &cache_root, crate_name.as_deref(), &output, &input_path) {
                        eprintln!("cuda_rustc: device build failed: {e}");
                    }
                }
            }
            Err(e) => {
                eprintln!("cuda_rustc: preprocessor failed on {}: {e}; falling back to original source", input_path.display());
            }
        }
    }

    exec_rustc(&rustc, &args)
}

fn is_cuda_enabled() -> bool {
    matches!(env::var("CUDA_RUSTC_ENABLE").ok().as_deref(), Some("1") | Some("true"))
}

fn is_probe_invocation(args: &[OsString]) -> bool {
    args.iter().any(|a| {
        let s = a.to_string_lossy();
        s == "-vV" || s == "--version" || s == "-V"
    }) || !args.iter().any(|a| a.to_string_lossy().ends_with(".rs"))
}

fn find_input_index(args: &[OsString]) -> Option<usize> {
    let mut found = None;
    for (i, a) in args.iter().enumerate() {
        let s = a.to_string_lossy();
        if !s.starts_with('-') && s.ends_with(".rs") {
            found = Some(i);
        }
    }
    found
}

fn extract_crate_name(args: &[OsString]) -> Option<String> {
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        let s = a.to_string_lossy();
        if s == "--crate-name" {
            return iter.next().map(|n| n.to_string_lossy().into_owned());
        }
        if let Some(rest) = s.strip_prefix("--crate-name=") {
            return Some(rest.to_owned());
        }
    }
    None
}

fn preproc_cache_root(args: &[OsString]) -> Result<PathBuf> {
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        let s = a.to_string_lossy();
        if s == "--out-dir"
            && let Some(p) = iter.next()
        {
            let cache = PathBuf::from(p).join("cuda_rustc_preproc");
            std::fs::create_dir_all(&cache).map_err(|e| format!("create_dir_all({}) failed: {e}", cache.display()))?;
            return Ok(cache);
        }
    }
    let tmp = env::temp_dir().join("cuda_rustc_preproc");
    std::fs::create_dir_all(&tmp).map_err(|e| format!("create_dir_all({}) failed: {e}", tmp.display()))?;
    Ok(tmp)
}

fn exec_rustc(rustc: &OsString, args: &[OsString]) -> Result<u8> {
    let status = Command::new(rustc).args(args).status().map_err(|e| format!("failed to spawn rustc ({}): {e}", Path::new(rustc).display()))?;

    Ok(status.code().unwrap_or(1) as u8)
}

/// Build the device-side derived crate to PTX, once per requested compute
/// capability. Each PTX entry is post-processed (shared-memory state-space
/// rewrite), linked against `libcudadevrt.a` into a per-arch cubin, then the
/// `fatbinary` tool packages all per-arch cubins into a single NVIDIA
/// fatbinary blob (magic `0xBA55ED50`) which is embedded in `.nv_fatbin` and
/// wrapped by a `__fatBinC_Wrapper_t` in `.nvFatBinSegment`.
fn build_device(rustc: &OsString, cache_root: &Path, crate_name: Option<&str>, output: &PreprocOutput, host_input: &Path) -> Result<()> {
    let device_src = output.device_path.as_ref().ok_or_else(|| "build_device called without a device source".to_string())?;

    let crate_label = crate_name.unwrap_or("anon");
    let device_crate_dir = cache_root.join(format!("{crate_label}__device"));
    let src_dir = device_crate_dir.join("src");
    std::fs::create_dir_all(&src_dir).map_err(|e| format!("mkdir {}: {e}", src_dir.display()))?;

    // Sync the device source into the derived crate. We copy rather than symlink
    // because cargo on Windows is symlink-shy.
    let lib_rs = src_dir.join("lib.rs");
    let device_text = std::fs::read_to_string(device_src).map_err(|e| format!("read {}: {e}", device_src.display()))?;
    if std::fs::read_to_string(&lib_rs).ok().as_deref() != Some(&device_text) {
        std::fs::write(&lib_rs, &device_text).map_err(|e| format!("write {}: {e}", lib_rs.display()))?;
    }

    // The device crate may need `::cuda_rustc_runtime::device::dynamic_shared`
    // for kernels that use dynamic shared memory, and `::cuda_libs_cudart::sys::*`
    // for cudadevrt-resolved cudart calls. Forward whatever cuda_libs_* path-deps
    // the host manifest declares (each with default-features off so host-only
    // FFI/runtime-link feature glue stays out of the nvptx build).
    let device_dep_lines = detect_device_deps(host_input);
    let device_deps_block = if device_dep_lines.is_empty() { String::from("# no cuda_libs path deps detected") } else { device_dep_lines.join("\n") };

    let cargo_toml = format!(
        r#"[package]
name = "{label}_device"
version = "0.0.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
{device_deps_block}

[workspace]
"#,
        label = sanitize_crate_name(crate_label)
    );
    let cargo_toml_path = device_crate_dir.join("Cargo.toml");
    if std::fs::read_to_string(&cargo_toml_path).ok().as_deref() != Some(&cargo_toml) {
        std::fs::write(&cargo_toml_path, &cargo_toml).map_err(|e| format!("write {}: {e}", cargo_toml_path.display()))?;
    }

    let archs = detect_cuda_archs();
    if archs.is_empty() {
        return Err("CUDA_ARCH was set but parsed to zero archs".into());
    }

    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let host_rustc = rustc.clone();

    // Locate libcudadevrt.a once at build time. Every kernel gets cudadevrt
    // statically linked into its cubin via `nvlink`, so the runtime never
    // needs libcudadevrt.a — only the build host does. nvlink is a static
    // archive-aware linker: it pulls in only the cudadevrt object files
    // whose symbols are actually referenced, so kernels with no
    // dynamic-parallelism / device-cudart calls embed nothing extra.
    let libcudadevrt = locate_libcudadevrt().ok_or_else(|| {
        "libcudadevrt.a was not found; install the CUDA toolkit or set CUDA_HOME. \
         cuda_rustc statically links cudadevrt into every cubin at build time \
         so the runtime has no libcudadevrt.a dependency."
            .to_string()
    })?;

    // Build one device payload per requested arch. Each invocation gets its
    // own `--target-dir` so cargo's per-arch output doesn't clobber the
    // previous. After the device build, each arch's PTX runs through
    // `ptxas -c` (relocatable cubin) → `nvlink` (link in cudadevrt) → final
    // self-contained cubin. The runtime loads the cubin via plain
    // `cuModuleLoadData`; no JIT or runtime linking.
    let mut arch_payload_paths: Vec<ArchPayload> = Vec::with_capacity(archs.len());

    for arch_str in &archs {
        let arch_num = arch_string_to_num(arch_str).ok_or_else(|| format!("unrecognized arch `{arch_str}` (expected `sm_<digits>`)"))?;

        let target_dir = device_crate_dir.join(format!("target-{arch_str}"));

        let status = Command::new(&cargo)
            .current_dir(&device_crate_dir)
            .arg("build")
            .arg("--release")
            .arg("--target")
            .arg("nvptx64-nvidia-cuda")
            .arg("-Z")
            .arg("build-std=core")
            .arg("--target-dir")
            .arg(&target_dir)
            .env("RUSTFLAGS", format!("-C target-cpu={arch_str} --emit=asm"))
            .env("RUSTC", &host_rustc)
            .env_remove("CARGO")
            .env_remove("RUSTC_WRAPPER")
            .env_remove("CARGO_MANIFEST_DIR")
            .status()
            .map_err(|e| format!("spawn cargo for device build (arch={arch_str}): {e}"))?;

        if !status.success() {
            return Err(format!("cargo failed for device crate at {} (arch={arch_str})", device_crate_dir.display()));
        }

        let ptx = locate_ptx(&target_dir, &sanitize_crate_name(crate_label))?;
        let raw_ptx_text = std::fs::read_to_string(&ptx).map_err(|e| format!("read raw ptx {}: {e}", ptx.display()))?;
        let rewritten_ptx_text = ptx::rewrite_shared(&raw_ptx_text);

        let ptx_stash = cache_root.join(format!("{crate_label}.{arch_str}.ptx"));
        if std::fs::read_to_string(&ptx_stash).ok().as_deref() != Some(rewritten_ptx_text.as_str()) {
            std::fs::write(&ptx_stash, &rewritten_ptx_text).map_err(|e| format!("write rewritten ptx {}: {e}", ptx_stash.display()))?;
        }
        if env::var_os("CUDA_RUSTC_VERBOSE").is_some() {
            eprintln!("cuda_rustc: produced PTX for {arch_str} at {}", ptx_stash.display());
        }

        // Compile-time link: ptxas produces a relocatable cubin (-c), then
        // nvlink merges it with libcudadevrt.a into a final self-contained
        // cubin. The cubin is what the runtime loads via cuModuleLoadData.
        let _ = rewritten_ptx_text; // silence unused-binding; PTX is on disk now.
        let cubin_path = cache_root.join(format!("{crate_label}.{arch_str}.cubin"));
        link_to_cubin(&ptx_stash, &cubin_path, arch_str, &libcudadevrt)?;
        if env::var_os("CUDA_RUSTC_VERBOSE").is_some() {
            eprintln!("cuda_rustc: linked cubin (cudadevrt resolved) for {arch_str} at {}", cubin_path.display());
        }
        let payload = ArchPayload {
            arch: arch_num,
            kind: PayloadKindGen::Cubin,
            path: cubin_path,
        };

        arch_payload_paths.push(payload);
    }

    // Run `fatbinary` to bundle every per-arch cubin into a single NVIDIA
    // fatbinary blob. `cuobjdump` recognizes the magic-prefixed blob, and
    // `cuModuleLoadData` accepts it directly — the CUDA driver picks the
    // right arch entry against the running device's compute capability.
    let fatbin_path = cache_root.join(format!("{crate_label}.fatbin"));
    package_fatbinary(&arch_payload_paths, &fatbin_path)?;

    let module_id = format!("cuda_rustc__{}", sanitize_crate_name(crate_label));
    let augmented_host = build_augmented_host_src(&output.host_path, &fatbin_path, &module_id)?;
    std::fs::write(&output.host_path, &augmented_host).map_err(|e| format!("write augmented host source {}: {e}", output.host_path.display()))?;

    if env::var_os("CUDA_RUSTC_VERBOSE").is_some() {
        eprintln!(
            "cuda_rustc: packaged fatbinary at {} with {} arch image(s): {}",
            fatbin_path.display(),
            arch_payload_paths.len(),
            arch_payload_paths.iter().map(|p| format!("sm_{}({:?})", p.arch, p.kind)).collect::<Vec<_>>().join(",")
        );
    }

    Ok(())
}

/// Bundle the per-arch cubins into a single NVIDIA fatbinary blob via the
/// `fatbinary` toolkit binary. Uses `--image3=kind=elf,sm=XX,file=…` (the
/// "image with kind+sm" form supported by every CUDA 11+ toolkit). The
/// resulting file's first four bytes are `0x50 0xED 0x55 0xBA` (little-endian
/// `0xBA55ED50`); `cuobjdump --dump-elf-symbols` reads it directly.
fn package_fatbinary(payloads: &[ArchPayload], out_path: &Path) -> Result<()> {
    if payloads.is_empty() {
        return Err("package_fatbinary called with empty payload list".into());
    }
    let fatbinary_bin = locate_toolkit_bin("fatbinary").ok_or_else(|| {
        "`fatbinary` was not found; install the CUDA toolkit or set CUDA_HOME. \
         cuda_rustc invokes fatbinary to produce a real NVIDIA fatbin blob in \
         the host binary's .nv_fatbin section."
            .to_string()
    })?;

    let mut cmd = Command::new(&fatbinary_bin);
    cmd.arg("--64");
    cmd.arg(format!("--create={}", out_path.display()));
    for payload in payloads {
        let kind = match payload.kind {
            PayloadKindGen::Ptx => "ptx",
            PayloadKindGen::Cubin => "elf",
        };
        cmd.arg(format!("--image3=kind={kind},sm={arch},file={file}", arch = payload.arch, file = payload.path.display(),));
    }
    let out = cmd.output().map_err(|e| format!("spawn {}: {e}", fatbinary_bin.display()))?;
    if !out.status.success() {
        eprint!("{}", String::from_utf8_lossy(&out.stderr));
        eprint!("{}", String::from_utf8_lossy(&out.stdout));
        return Err(format!("fatbinary failed (status={}): see stderr above", out.status.code().unwrap_or(-1)));
    }

    // Sanity-check the magic so the host binary never embeds a non-fatbin
    // blob. nvcc's runtime uses the same `0xBA55ED50` little-endian magic
    // as its fatbin format id; `cuobjdump` and `cuModuleLoadData`'s
    // fatbin-detection path both key off this prefix.
    let head = std::fs::read(out_path).map_err(|e| format!("read produced fatbin {}: {e}", out_path.display()))?;
    if head.len() < 4 || head[..4] != [0x50, 0xED, 0x55, 0xBA] {
        return Err(format!("fatbinary produced a blob with unexpected magic at {} (got {:02x?}, want 50 ED 55 BA)", out_path.display(), head.get(..4).unwrap_or(&head)));
    }
    Ok(())
}

/// Per-arch payload kind: which `--image3=kind=…` form `fatbinary` should
/// receive. The driver always produces `Cubin` today (PTX is linked offline
/// via `ptxas -c` + `nvlink`); `Ptx` is reserved for future raw-PTX bake.
#[derive(Copy, Clone, Debug)]
pub enum PayloadKindGen {
    Ptx,
    Cubin,
}

pub struct ArchPayload {
    pub arch: u32,
    pub kind: PayloadKindGen,
    pub path: PathBuf,
}

/// Run `ptxas -arch=sm_XX -o output.cubin input.ptx`. The PTX path must be
/// the post-processed version (shared-memory rewrites already applied). The
/// resulting cubin is opaque ELF bytes that `cuModuleLoadData` accepts directly
/// without JIT.
/// Compile-time link: PTX → relocatable cubin (`ptxas -c`) → final cubin
/// (`nvlink` with libcudadevrt.a). Produces a self-contained cubin the
/// runtime can load via plain `cuModuleLoadData`. `nvlink` is a static
/// archive-aware linker, so cudadevrt object files are pulled in only for
/// symbols actually referenced — no overhead for kernels that don't use
/// dynamic parallelism or device-side cudart.
fn link_to_cubin(ptx_path: &Path, cubin_path: &Path, arch_str: &str, libcudadevrt: &Path) -> Result<()> {
    let ptxas_bin = locate_toolkit_bin("ptxas").ok_or_else(|| "`ptxas` was not found; install the CUDA toolkit or set CUDA_HOME".to_string())?;
    let nvlink_bin = locate_toolkit_bin("nvlink").ok_or_else(|| "`nvlink` was not found; install the CUDA toolkit or set CUDA_HOME".to_string())?;

    // ptxas -c: emit a relocatable cubin so nvlink can resolve externs against
    // the cudadevrt archive. Without -c, ptxas refuses any unresolved extern
    // and produces a non-relocatable cubin only.
    let reloc_path = cubin_path.with_extension("reloc.cubin");
    let ptxas_out = Command::new(&ptxas_bin).arg("-c").arg(format!("-arch={arch_str}")).arg("-o").arg(&reloc_path).arg(ptx_path).output().map_err(|e| format!("spawn {}: {e}", ptxas_bin.display()))?;
    if !ptxas_out.status.success() {
        eprint!("{}", String::from_utf8_lossy(&ptxas_out.stderr));
        return Err(format!("ptxas -c failed (arch={arch_str}, status={}): see stderr above", ptxas_out.status.code().unwrap_or(-1)));
    }

    let nvlink_out = Command::new(&nvlink_bin)
        .arg(format!("-arch={arch_str}"))
        .arg("-o")
        .arg(cubin_path)
        .arg(&reloc_path)
        .arg(libcudadevrt)
        .output()
        .map_err(|e| format!("spawn {}: {e}", nvlink_bin.display()))?;
    if !nvlink_out.status.success() {
        eprint!("{}", String::from_utf8_lossy(&nvlink_out.stderr));
        return Err(format!("nvlink failed (arch={arch_str}, status={}): see stderr above", nvlink_out.status.code().unwrap_or(-1)));
    }

    Ok(())
}

fn locate_toolkit_bin(name: &str) -> Option<PathBuf> {
    // Prefer CUDA_HOME, then defaults, then PATH.
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(home) = env::var("CUDA_HOME") {
        candidates.push(PathBuf::from(&home).join("bin").join(name));
    }
    candidates.push(PathBuf::from(format!("/usr/local/cuda/bin/{name}")));
    candidates.push(PathBuf::from(format!("/opt/cuda/bin/{name}")));
    if let Some(p) = candidates.into_iter().find(|p| p.exists()) {
        return Some(p);
    }
    // Fall through to whatever `which` finds on PATH.
    let out = Command::new("which").arg(name).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let line = String::from_utf8_lossy(&out.stdout).lines().next()?.trim().to_owned();
    if line.is_empty() { None } else { Some(PathBuf::from(line)) }
}

fn locate_libcudadevrt() -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(home) = env::var("CUDA_HOME") {
        candidates.push(PathBuf::from(&home).join("lib64").join("libcudadevrt.a"));
        candidates.push(PathBuf::from(&home).join("lib").join("libcudadevrt.a"));
    }
    candidates.push(PathBuf::from("/usr/local/cuda/lib64/libcudadevrt.a"));
    candidates.push(PathBuf::from("/opt/cuda/lib64/libcudadevrt.a"));
    candidates.push(PathBuf::from("/usr/lib/x86_64-linux-gnu/libcudadevrt.a"));
    candidates.into_iter().find(|p| p.exists())
}

fn build_augmented_host_src(host_path: &Path, fatbin_path: &Path, module_id: &str) -> Result<String> {
    let existing = std::fs::read_to_string(host_path).map_err(|e| format!("read host source {}: {e}", host_path.display()))?;

    let marker = "// --- cuda_rustc generated fatbin embedding ---";
    let base = existing.split(marker).next().unwrap_or(&existing);

    let mut out = base.to_owned();
    out.push_str(&ast::fatbin_embed_block(fatbin_path, module_id));
    Ok(out)
}

/// Convert `sm_86` / `sm_90a` → `86` / `90`. Returns `None` if the prefix or
/// digit body is missing.
fn arch_string_to_num(arch: &str) -> Option<u32> {
    let rest = arch.strip_prefix("sm_")?;
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

fn sanitize_crate_name(name: &str) -> String {
    name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).collect()
}

fn locate_ptx(target_root: &Path, crate_label: &str) -> Result<PathBuf> {
    let target_dir = target_root.join("nvptx64-nvidia-cuda").join("release");
    let candidates = [target_dir.join(format!("{crate_label}_device.ptx")), target_dir.join(format!("{crate_label}_device.s"))];
    for c in &candidates {
        if c.exists() {
            return Ok(c.clone());
        }
    }
    // Fall back to scanning the directory for any .ptx/.s file.
    if let Ok(read) = std::fs::read_dir(&target_dir) {
        for entry in read.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension()
                && (ext == "ptx" || ext == "s")
            {
                return Ok(path);
            }
        }
    }
    Err(format!("could not find PTX output under {} (expected one of {})", target_dir.display(), candidates.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")))
}

/// Walk up from the host source path looking for the nearest `Cargo.toml` that
/// declares cuda_libs path-deps usable on the device side, and emit a
/// manifest-ready dependency line per matching crate.
///
/// Forwards `cuda_rustc_runtime` (needed for `device::dynamic_shared` and Dim3)
/// plus any `cuda_libs_*` crate the host depends on (so device kernels can call
/// the bindgen-emitted `extern "C"` declarations — at JIT/link time those
/// symbols resolve against libcudadevrt for the cudart subset cudadevrt
/// implements). Each dep is emitted with `default-features = false` so the
/// host-only `runtime-link`/`dynamic-link` glue (which pulls in libloading,
/// libc, std::sync, etc.) stays out of the nvptx build.
fn detect_device_deps(host_src_path: &Path) -> Vec<String> {
    // Keep this list to crates whose `sys.rs` we have actually made
    // nvptx-compatible (no_std, ::core paths, no host-only types). Adding a
    // crate here that still references std/libloading/std::sync at module scope
    // will break the device build. The Driver/cuBLAS/cuFFT etc. crates are
    // host-only for now; only cudart has a cudadevrt-resolved subset.
    const FORWARD_CRATES: &[&str] = &["cuda_rustc_runtime", "cuda_libs_cudart"];

    let Some(mut dir) = host_src_path.parent().map(Path::to_path_buf) else {
        return Vec::new();
    };
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists()
            && let Ok(text) = std::fs::read_to_string(&manifest)
        {
            let mut out = Vec::new();
            for crate_name in FORWARD_CRATES {
                if let Some(rel_path) = parse_path_dep(&text, crate_name) {
                    let abs = dir.join(&rel_path);
                    let abs_canonical = abs.canonicalize().unwrap_or(abs);
                    out.push(format!("{crate_name} = {{ path = \"{}\", default-features = false }}", abs_canonical.display()));
                }
            }
            if !out.is_empty() {
                return out;
            }
        }
        if !dir.pop() {
            return Vec::new();
        }
    }
}

fn parse_path_dep(manifest: &str, crate_name: &str) -> Option<String> {
    for line in manifest.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with(crate_name) {
            continue;
        }
        let after_name = &trimmed[crate_name.len()..];
        if !after_name.starts_with(|c: char| c.is_whitespace() || c == '=') {
            continue;
        }
        let path_marker = "path = \"";
        let idx = line.find(path_marker)?;
        let after = &line[idx + path_marker.len()..];
        let end = after.find('"')?;
        return Some(after[..end].to_owned());
    }
    None
}

/// Resolve the list of compute capabilities to build PTX for. Honors
/// `CUDA_ARCH=sm_80,sm_86,sm_90` (comma-separated; whitespace tolerated) for
/// multi-arch fatbins. Falls back to the local GPU's compute capability via
/// `nvidia-smi`, then to `sm_80` if neither source resolves.
fn detect_cuda_archs() -> Vec<String> {
    if let Ok(env_val) = env::var("CUDA_ARCH") {
        let parts: Vec<String> = env_val.split(',').map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()).collect();
        if !parts.is_empty() {
            return parts;
        }
    }
    let out = Command::new("nvidia-smi").args(["--query-gpu=compute_cap", "--format=csv,noheader,nounits"]).output();
    if let Ok(out) = out
        && out.status.success()
        && let Some(cap) = String::from_utf8_lossy(&out.stdout).lines().next()
    {
        let cap = cap.trim();
        if !cap.is_empty() {
            return vec![format!("sm_{}", cap.replace('.', ""))];
        }
    }
    vec!["sm_80".to_owned()]
}
