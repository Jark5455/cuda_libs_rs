#![allow(unused)]

use std::collections::HashMap;
use std::ffi::{CString, c_char};
use std::sync::atomic::AtomicPtr;
use std::sync::{Arc, LazyLock, RwLock};

use crate::safe::{cuCtxCreate_v4, cuModuleGetFunction, cuModuleLoad, cuModuleLoadData, cuModuleUnload};
use crate::sys::CUctx_flags::{CU_CTX_MAP_HOST, CU_CTX_SCHED_AUTO};
use crate::sys::{CUcontext, CUfunction, CUmodule, CUresult};

pub static DEFAULT_DEVICE: LazyLock<CudaDevice> = LazyLock::new(|| CudaDevice::default().expect("Failed to create cuda ctx"));

pub struct CudaDevice {
    device_id: i32,
    context: CUcontext,

    modules: RwLock<HashMap<String, CUmodule>>,
    functions: RwLock<HashMap<String, CUfunction>>,
}

impl CudaDevice {
    pub fn default() -> Result<Self, CUresult> {
        let ctx = unsafe { cuCtxCreate_v4(std::ptr::null_mut(), CU_CTX_SCHED_AUTO as u32, 0)? };

        Ok(Self {
            device_id: 0,
            context: ctx,
            modules: RwLock::new(HashMap::new()),
            functions: RwLock::new(HashMap::new()),
        })
    }

    pub fn new(device_id: i32, ctx: CUcontext) -> Self {
        Self {
            device_id,
            context: ctx,
            modules: RwLock::new(HashMap::new()),
            functions: RwLock::new(HashMap::new()),
        }
    }

    pub fn try_get_function(&self, name: &str, ptx: *const c_char) -> Result<CUfunction, CUresult> {
        if let Some(f) = self.functions.read().unwrap().get(name) {
            return Ok(*f); // skip loading if already loaded
        }

        unsafe {
            let fname = CString::new(name).unwrap();

            let module = cuModuleLoadData(ptx as *const _)?;
            let funct = cuModuleGetFunction(module, fname.as_ptr())?;

            self.modules.write().unwrap().insert(name.to_owned(), module);
            self.functions.write().unwrap().insert(name.to_owned(), funct);

            Ok(*self.functions.read().unwrap().get(name).unwrap())
        }
    }
}

impl Drop for CudaDevice {
    fn drop(&mut self) {
        for (name, module) in self.modules.write().unwrap().drain() {
            unsafe {
                cuModuleUnload(module).expect(format!("Failed to unload module {}", name).as_str());
            };
        }
    }
}

// CUDA handles are thread-safe; the RwLock wrappers guard mutable access.
unsafe impl Send for CudaDevice {}
unsafe impl Sync for CudaDevice {}
