//! Runtime fatbin registry.
//!
//! The cuda_rustc driver injects, into the host crate, a `#[ctor]`-style entry
//! that calls `register_fatbin(&WRAPPER)` exactly once at process startup. The
//! wrapper points at a real NVIDIA fatbinary blob (magic `0xBA55ED50`) that
//! packages one or more per-arch cubins. The CUDA driver picks the right arch
//! itself when `cuModuleLoadData` is called against a fatbinary blob — no
//! manual compute-capability query needed.

use std::collections::HashMap;
use std::ffi::{CString, c_void};
use std::sync::{Mutex, OnceLock};

use cuda_libs_driver::safe::{cuModuleGetFunction, cuModuleLoadData};
use cuda_libs_driver::sys::{CUfunction, CUmodule, CUresult};

/// nvcc's `__fatBinC_Wrapper_t` shape. The cuda_rustc driver emits one of
/// these per host crate, in the `.nvFatBinSegment` link section, pointing at
/// the fatbinary blob in `.nv_fatbin`.
#[repr(C)]
pub struct FatBinWrapper {
    pub magic: i32,
    pub version: i32,
    pub data: *const u8,
    pub filename: *const u8,
}

unsafe impl Sync for FatBinWrapper {}

struct Registry {
    fatbins: Vec<&'static FatBinWrapper>,
    modules: Vec<CUmoduleSend>,
    functions: HashMap<String, CUfunctionSend>,
    initialized: bool,
}

#[derive(Copy, Clone)]
struct CUmoduleSend(CUmodule);
#[derive(Copy, Clone)]
struct CUfunctionSend(CUfunction);

unsafe impl Send for CUmoduleSend {}
unsafe impl Sync for CUmoduleSend {}
unsafe impl Send for CUfunctionSend {}
unsafe impl Sync for CUfunctionSend {}

static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();

fn registry() -> &'static Mutex<Registry> {
    REGISTRY.get_or_init(|| {
        Mutex::new(Registry {
            fatbins: Vec::new(),
            modules: Vec::new(),
            functions: HashMap::new(),
            initialized: false,
        })
    })
}

/// Called once per fatbin from a process-init ctor that the cuda_rustc driver
/// injects into the host crate.
pub fn register_fatbin(wrapper: &'static FatBinWrapper) {
    if wrapper.data.is_null() {
        eprintln!("cuda_rustc: register_fatbin called with null data; skipping");
        return;
    }
    let mut reg = registry().lock().unwrap();
    reg.fatbins.push(wrapper);
}

fn ensure_loaded(reg: &mut Registry) -> Result<(), CUresult> {
    if reg.initialized {
        return Ok(());
    }

    // Touch DEFAULT_DEVICE so that a CUDA context exists before module load.
    let _ = &*cuda_libs_driver::types::DEFAULT_DEVICE;

    for wrapper in &reg.fatbins {
        let module = unsafe { load_fatbin(wrapper)? };
        reg.modules.push(CUmoduleSend(module));
    }
    reg.initialized = true;
    Ok(())
}

/// Load a fatbinary blob into a CUDA module. The driver accepts a fatbinary
/// blob, ELF cubin, or PTX text via `cuModuleLoadData`; the magic-byte sniff
/// at the start of the blob selects which path it takes internally and picks
/// the right arch entry for the current device.
unsafe fn load_fatbin(wrapper: &'static FatBinWrapper) -> Result<CUmodule, CUresult> {
    unsafe { cuModuleLoadData(wrapper.data as *const c_void) }
}

pub fn resolve_function(name: &str) -> Result<CUfunction, CUresult> {
    let mut reg = registry().lock().unwrap();

    if let Some(f) = reg.functions.get(name) {
        return Ok(f.0);
    }

    ensure_loaded(&mut reg)?;

    let cname = CString::new(name).expect("kernel name contained an interior NUL");

    for module in &reg.modules.clone() {
        unsafe {
            match cuModuleGetFunction(module.0, cname.as_ptr()) {
                Ok(f) => {
                    reg.functions.insert(name.to_owned(), CUfunctionSend(f));
                    return Ok(f);
                }
                Err(CUresult::CUDA_ERROR_NOT_FOUND) => continue,
                Err(e) => return Err(e),
            }
        }
    }

    Err(CUresult::CUDA_ERROR_NOT_FOUND)
}
