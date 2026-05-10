//! `cuda_rustc` — drop-in rustc wrapper that adds a CUDA preprocessing pass.
//!
//! Invoked by Cargo via `RUSTC_WRAPPER=cuda_rustc`. With that env var set, Cargo
//! calls `cuda_rustc <real_rustc> <flags...>` instead of `<real_rustc> <flags...>`.
//!
//! Pipeline:
//!   1. Identify the rust source file inputs in the rustc invocation.
//!   2. Run them through the source preprocessor (rewrites `kernel<<<...>>>(args)`
//!      into a runtime call, extracts a derived nvptx device crate, compiles it
//!      to PTX, packages a fatbin, and injects host-side registration glue).
//!   3. Substitute the rewritten paths back into the rustc invocation.
//!   4. Exec the underlying rustc.

use std::process::ExitCode;

fn main() -> ExitCode {
    match cuda_rustc::driver::run() {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("cuda_rustc: {err}");
            ExitCode::from(1)
        }
    }
}
