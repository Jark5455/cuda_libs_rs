//! Build a self-contained "cuda" rustup-toolchain directory.
//!
//! Output layout matches what `rustup` produces, so IDEs that accept a folder
//! as a Rust toolchain (RustRover, IntelliJ Rust, vscode rust-analyzer custom
//! toolchain, …) can point directly at the generated directory.
//!
//! ```
//! <out>/
//! ├── bin/
//! │   ├── rustc           — nightly rustc (RUSTC_WRAPPER is set externally)
//! │   ├── cargo           — shell wrapper: sets RUSTC_WRAPPER=cuda_rustc + CUDA_RUSTC_ENABLE=1
//! │   ├── cargo-fmt       — shell wrapper: exec nightly cargo-fmt
//! │   ├── cargo-clippy    — shell wrapper: sets RUSTC_WRAPPER=cuda_clippy
//! │   ├── rustfmt         — cuda_rustfmt symlink
//! │   ├── clippy-driver   — cuda_clippy symlink
//! │   ├── rust-analyzer   — cuda_rust_analyzer symlink
//! │   └── rustdoc, rust-gdb, … (nightly symlinks)
//! ├── lib/   — symlink to nightly lib   (rustlib, libstd*.so, libLLVM*.so, …)
//! ├── share/ — symlink to nightly share (man pages, doc, zsh completion)
//! └── etc/   — symlink to nightly etc   (when present)
//! ```
//!
//! The toolchain is *not* installed via `rustup link`; the directory is the
//! deliverable. To use from the CLI: `cargo +<absolute/path/to/dir> build`
//! requires a rustup link, but RustRover-style "custom toolchain folder"
//! pickers consume the directory directly.
//!
//! Invocation:
//! ```text
//! cargo run --bin cuda_toolchain_build -- [out_dir] [--profile release|debug]
//! ```
//! Defaults: `out_dir = <workspace>/target/cuda-toolchain`, `profile = debug`.

use std::env;
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut profile = "debug".to_string();
    if let Some(idx) = args.iter().position(|a| a == "--profile") {
        if idx + 1 >= args.len() {
            die("--profile requires a value (debug|release)");
        }
        profile = args.remove(idx + 1);
        args.remove(idx);
    }

    let workspace_root = locate_workspace_root();
    let out_dir = match args.first() {
        Some(p) => PathBuf::from(p),
        None => workspace_root.join("target").join("cuda-toolchain"),
    };
    let target_dir = workspace_root.join("target").join(&profile);

    // Verify the cuda_rustc binaries exist before we start laying out the
    // toolchain. Running `cargo build` first is the user's responsibility.
    let required = ["cuda_rustc", "cuda_clippy", "cuda_rustfmt", "cuda_rust_analyzer"];
    for bin in &required {
        let p = target_dir.join(bin);
        if !p.exists() {
            die(&format!("{} not found at {}. Run `cargo build{}` first to produce the cuda binaries.", bin, p.display(), if profile == "release" { " --release" } else { "" }));
        }
    }

    let nightly_root = locate_nightly_root();

    let bin_dir = out_dir.join("bin");
    fs::create_dir_all(&bin_dir).unwrap_or_else(|e| die(&format!("mkdir {}: {e}", bin_dir.display())));

    // Symlink top-level dirs from nightly. lib/ has rustlib (host stdlib +
    // nvptx target if installed via rustup target add). share/ has docs and
    // shell completions. etc/ exists on some installs and contains shell
    // completion bash sources.
    for d in ["lib", "share", "etc"] {
        let src = nightly_root.join(d);
        if !src.exists() {
            continue;
        }
        let dst = out_dir.join(d);
        replace_symlink(&src, &dst);
    }

    // bin/rustc — nightly's rustc, used directly. RUSTC_WRAPPER is set by the
    // cargo wrapper below, which is what triggers cuda_rustc preprocessing.
    replace_symlink(&nightly_root.join("bin").join("rustc"), &bin_dir.join("rustc"));

    // bin/cargo — shell wrapper that sets the cuda env vars and exec's
    // nightly's cargo. We can't symlink cargo because cargo introspects its
    // own argv[0] to find the toolchain prefix.
    let cuda_rustc = target_dir.join("cuda_rustc");
    let nightly_cargo = nightly_root.join("bin").join("cargo");
    write_cargo_wrapper(&bin_dir.join("cargo"), &nightly_cargo, &cuda_rustc);

    // bin/cargo-fmt — straight passthrough to nightly's cargo-fmt; rustfmt
    // (the actual formatter) is overridden by bin/rustfmt below.
    let nightly_cargo_fmt = nightly_root.join("bin").join("cargo-fmt");
    if nightly_cargo_fmt.exists() {
        write_passthrough(&bin_dir.join("cargo-fmt"), &nightly_cargo_fmt);
    }

    // bin/cargo-clippy — wrapper sets RUSTC_WRAPPER=cuda_clippy so the
    // chevron-rewriting preprocessor runs before clippy-driver lints. nightly
    // ships this as `cargo-clippy`.
    let nightly_cargo_clippy = nightly_root.join("bin").join("cargo-clippy");
    if nightly_cargo_clippy.exists() {
        let cuda_clippy = target_dir.join("cuda_clippy");
        write_cargo_clippy_wrapper(&bin_dir.join("cargo-clippy"), &nightly_cargo_clippy, &cuda_clippy);
    }

    // bin/rustfmt — cuda_rustfmt directly. It substitutes chevron syntax
    // before forwarding to the real rustfmt (resolved via
    // CUDA_RUSTFMT_BACKEND or `rustfmt` on PATH).
    replace_symlink(&target_dir.join("cuda_rustfmt"), &bin_dir.join("rustfmt"));

    // bin/clippy-driver — clippy-driver is the rustc-shaped binary that
    // actually runs lints. cuda_clippy delegates to cuda_rustc::driver::run
    // which speaks rustc's arg shape.
    replace_symlink(&target_dir.join("cuda_clippy"), &bin_dir.join("clippy-driver"));

    // bin/rust-analyzer — LSP proxy that rewrites chevron syntax in
    // textDocument/didOpen + didChange before forwarding to the real
    // rust-analyzer (resolved via RUST_ANALYZER_BACKEND or `rust-analyzer`
    // on PATH).
    replace_symlink(&target_dir.join("cuda_rust_analyzer"), &bin_dir.join("rust-analyzer"));

    // Remaining nightly binaries — rustdoc, rust-gdb, rust-lldb, cargo-miri,
    // miri, rls, rustup-init mimics, etc. Symlink each so the toolchain feels
    // complete to consumers that probe these.
    if let Ok(entries) = fs::read_dir(nightly_root.join("bin")) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            // Already provided above:
            if matches!(name_str.as_ref(), "rustc" | "cargo" | "cargo-fmt" | "cargo-clippy" | "rustfmt" | "clippy-driver" | "rust-analyzer") {
                continue;
            }
            let src = entry.path();
            let dst = bin_dir.join(&name);
            replace_symlink(&src, &dst);
        }
    }

    println!("cuda toolchain assembled at {}", out_dir.display());
    println!("RustRover: point the toolchain folder picker at the directory above.");
    println!(
        "CLI: `cargo +{}` won't work without `rustup toolchain link cuda {}`; \
         the directory is intended for IDE consumption.",
        out_dir.display(),
        out_dir.display()
    );
}

fn die(msg: &str) -> ! {
    eprintln!("cuda_toolchain_build: {msg}");
    std::process::exit(1);
}

fn replace_symlink(src: &Path, dst: &Path) {
    if dst.exists() || dst.is_symlink() {
        let _ = fs::remove_file(dst).or_else(|_| fs::remove_dir_all(dst));
    }
    unix_fs::symlink(src, dst).unwrap_or_else(|e| die(&format!("symlink {} -> {}: {e}", dst.display(), src.display())));
}

fn write_cargo_wrapper(path: &Path, real_cargo: &Path, cuda_rustc: &Path) {
    // The wrapper:
    //   1. Prepends the toolchain's `bin/` to PATH so that when cargo passes
    //      bare `rustc` (or any other tool name) to RUSTC_WRAPPER, lookup
    //      hits our nightly-symlinked rustc instead of the rustup proxy
    //      pinned to a different channel.
    //   2. Exports RUSTC_WRAPPER=cuda_rustc and CUDA_RUSTC_ENABLE=1 so
    //      every rustc invocation goes through the chevron-rewriting
    //      preprocessor.
    //   3. Exports RUSTC pointing at our nightly rustc symlink so build
    //      scripts and tooling that read $RUSTC directly also pick the
    //      right compiler.
    let bin_dir = path.parent().expect("cargo wrapper path has a parent");
    let script = format!(
        "#!/bin/sh\n\
         BIN_DIR='{bin_dir}'\n\
         case \":$PATH:\" in\n\
         \t*\":$BIN_DIR:\"*) ;;\n\
         \t*) PATH=\"$BIN_DIR:$PATH\";;\n\
         esac\n\
         export PATH\n\
         export RUSTC='{bin_dir}/rustc'\n\
         export RUSTC_WRAPPER='{wrapper}'\n\
         export CUDA_RUSTC_ENABLE=1\n\
         exec '{cargo}' \"$@\"\n",
        bin_dir = bin_dir.display(),
        wrapper = cuda_rustc.display(),
        cargo = real_cargo.display(),
    );
    write_executable(path, &script);
}

fn write_cargo_clippy_wrapper(path: &Path, real_cargo_clippy: &Path, cuda_clippy: &Path) {
    let bin_dir = path.parent().expect("cargo-clippy wrapper path has a parent");
    let script = format!(
        "#!/bin/sh\n\
         BIN_DIR='{bin_dir}'\n\
         case \":$PATH:\" in\n\
         \t*\":$BIN_DIR:\"*) ;;\n\
         \t*) PATH=\"$BIN_DIR:$PATH\";;\n\
         esac\n\
         export PATH\n\
         export RUSTC='{bin_dir}/rustc'\n\
         export RUSTC_WRAPPER='{wrapper}'\n\
         export CUDA_RUSTC_ENABLE=1\n\
         exec '{cargo_clippy}' \"$@\"\n",
        bin_dir = bin_dir.display(),
        wrapper = cuda_clippy.display(),
        cargo_clippy = real_cargo_clippy.display(),
    );
    write_executable(path, &script);
}

fn write_passthrough(path: &Path, real: &Path) {
    let script = format!("#!/bin/sh\nexec '{real}' \"$@\"\n", real = real.display(),);
    write_executable(path, &script);
}

fn write_executable(path: &Path, contents: &str) {
    use std::os::unix::fs::PermissionsExt;
    if path.exists() || path.is_symlink() {
        let _ = fs::remove_file(path);
    }
    fs::write(path, contents).unwrap_or_else(|e| die(&format!("write {}: {e}", path.display())));
    let mut perm = fs::metadata(path).unwrap().permissions();
    perm.set_mode(0o755);
    fs::set_permissions(path, perm).unwrap();
}

/// Walk up from CARGO_MANIFEST_DIR (or the current directory) looking for the
/// workspace root — the first ancestor whose Cargo.toml contains
/// `[workspace]`. Falls back to current directory if no workspace marker is
/// found within a few levels.
fn locate_workspace_root() -> PathBuf {
    let start = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| env::current_dir().expect("cwd"));
    let mut dir = start.clone();
    for _ in 0..8 {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists() {
            if let Ok(text) = fs::read_to_string(&manifest)
                && text.contains("[workspace]")
            {
                return dir;
            }
        }
        if !dir.pop() {
            break;
        }
    }
    start
}

/// Resolve nightly's toolchain root via `rustup which --toolchain nightly rustc`.
/// rustup prints e.g. `/home/u/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc`;
/// the toolchain root is two levels up.
fn locate_nightly_root() -> PathBuf {
    let out = Command::new("rustup").args(["which", "--toolchain", "nightly", "rustc"]).output().unwrap_or_else(|e| die(&format!("spawn rustup: {e}")));
    if !out.status.success() {
        die(&format!("`rustup which --toolchain nightly rustc` failed (status={}):\n{}", out.status.code().unwrap_or(-1), String::from_utf8_lossy(&out.stderr)));
    }
    let line = String::from_utf8_lossy(&out.stdout).lines().next().unwrap_or_default().trim().to_owned();
    let rustc_path = PathBuf::from(line);
    rustc_path.parent().and_then(|p| p.parent()).map(Path::to_path_buf).unwrap_or_else(|| die("could not derive nightly toolchain root from rustc path"))
}
