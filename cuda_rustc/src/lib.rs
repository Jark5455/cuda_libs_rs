//! `cuda_rustc` library entrypoints.
//!
//! Two binaries (`cuda_rustc` for the rustc wrapper, `cuda_clippy` for the
//! clippy-driver wrapper, `cuda_rustfmt` for the rustfmt wrapper) all share the
//! same CUDA-syntax preprocessor pipeline. The pipeline lives here so each
//! binary entry point can drive it with the right downstream tool.

pub mod driver;
pub mod fatbin;
pub mod preproc;
pub mod ptx;
