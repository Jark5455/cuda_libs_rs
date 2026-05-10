//! Kernel handle: lazy resolver from a kernel symbol name to a `CUfunction`.
//!
//! Each `#[global]` kernel emits a `static __cuda_kernel_<name>: KernelHandle` that
//! resolves on first launch via the global fatbin registry populated by
//! `crate::registration` (which in turn was populated by the host-side ctor that
//! the cuda_rustc driver injected).

use std::sync::OnceLock;

use cuda_libs_driver::sys::{CUfunction, CUresult};

pub struct KernelHandle {
    name: &'static str,
    func: OnceLock<CUfunctionPtr>,
}

#[derive(Copy, Clone)]
struct CUfunctionPtr(CUfunction);

unsafe impl Send for CUfunctionPtr {}
unsafe impl Sync for CUfunctionPtr {}

impl KernelHandle {
    pub const fn new(name: &'static str) -> Self {
        Self { name, func: OnceLock::new() }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn resolve(&self) -> Result<CUfunction, CUresult> {
        if let Some(f) = self.func.get() {
            return Ok(f.0);
        }

        let f = crate::registration::resolve_function(self.name)?;
        let _ = self.func.set(CUfunctionPtr(f));
        Ok(f)
    }

    pub fn cached(&self) -> Option<CUfunction> {
        self.func.get().map(|p| p.0)
    }
}
