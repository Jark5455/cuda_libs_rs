//! `cuda_clippy` — clippy-driver wrapper.
//!
//! Used like `RUSTC_WORKSPACE_WRAPPER=cuda_clippy cargo clippy`. Cargo passes
//! `<clippy-driver> <args...>` exactly the way it would pass `<rustc> <args...>`
//! to the regular rustc wrapper, so the same `cuda_rustc::driver::run` entry
//! point handles both — it just execs whatever binary is passed as the first
//! argument after the preprocessing pass.

use std::process::ExitCode;

fn main() -> ExitCode {
    match cuda_rustc::driver::run() {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("cuda_clippy: {err}");
            ExitCode::from(1)
        }
    }
}
