//! Kernel launch entry point used by code emitted by the cuda_rustc preprocessor.
//!
//! The preprocessor rewrites `kernel<<<grid, block, shmem, stream>>>(args...)` into a
//! call to `launch_kernel`. On the host this dispatches via the CUDA driver API.
//! On `nvptx64` (device-side, dynamic parallelism) this dispatches via `cudaLaunchDevice`.

#[cfg(not(target_arch = "nvptx64"))]
use core::ffi::c_void;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Dim3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Dim3 {
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}

impl From<u32> for Dim3 {
    fn from(x: u32) -> Self {
        Self { x, y: 1, z: 1 }
    }
}

impl From<usize> for Dim3 {
    fn from(x: usize) -> Self {
        Self { x: x as u32, y: 1, z: 1 }
    }
}

impl From<i32> for Dim3 {
    fn from(x: i32) -> Self {
        Self { x: x as u32, y: 1, z: 1 }
    }
}

impl From<(u32, u32)> for Dim3 {
    fn from(t: (u32, u32)) -> Self {
        Self { x: t.0, y: t.1, z: 1 }
    }
}

impl From<(u32, u32, u32)> for Dim3 {
    fn from(t: (u32, u32, u32)) -> Self {
        Self { x: t.0, y: t.1, z: t.2 }
    }
}

impl From<(usize, usize)> for Dim3 {
    fn from(t: (usize, usize)) -> Self {
        Self { x: t.0 as u32, y: t.1 as u32, z: 1 }
    }
}

impl From<(usize, usize, usize)> for Dim3 {
    fn from(t: (usize, usize, usize)) -> Self {
        Self { x: t.0 as u32, y: t.1 as u32, z: t.2 as u32 }
    }
}

#[cfg(not(target_arch = "nvptx64"))]
pub type CUstream = *mut core::ffi::c_void;

#[cfg(target_arch = "nvptx64")]
pub type CUstream = *mut core::ffi::c_void;

#[cfg(not(target_arch = "nvptx64"))]
pub fn launch_kernel(handle: &crate::handle::KernelHandle, grid: Dim3, block: Dim3, shared_mem_bytes: u32, stream: CUstream, args: &[*const c_void]) {
    use cuda_libs_driver::safe::cuLaunchKernel;

    let func = handle.resolve().unwrap_or_else(|e| panic!("cuda_rustc: failed to resolve kernel `{}`: {:?}", handle.name(), e));

    let mut params: Vec<*mut c_void> = args.iter().map(|p| *p as *mut c_void).collect();

    unsafe {
        cuLaunchKernel(func, grid.x, grid.y, grid.z, block.x, block.y, block.z, shared_mem_bytes, stream as _, params.as_mut_ptr(), core::ptr::null_mut()).unwrap_or_else(|e| panic!("cuda_rustc: cuLaunchKernel failed for `{}`: {:?}", handle.name(), e));
    }
}

#[cfg(target_arch = "nvptx64")]
unsafe extern "C" {
    fn cudaLaunchDevice(func: *mut core::ffi::c_void, param_buffer: *mut core::ffi::c_void, grid_dim: Dim3, block_dim: Dim3, shared_mem_size: u32, stream: CUstream) -> i32;

    fn cudaGetParameterBuffer(alignment: usize, size: usize) -> *mut core::ffi::c_void;
}

#[cfg(target_arch = "nvptx64")]
pub unsafe fn launch_kernel_device(func_ptr: *mut core::ffi::c_void, grid: Dim3, block: Dim3, shared_mem_bytes: u32, stream: CUstream, arg_bytes: &[u8]) -> i32 {
    unsafe {
        let buf = cudaGetParameterBuffer(64, arg_bytes.len());
        if buf.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(arg_bytes.as_ptr(), buf as *mut u8, arg_bytes.len());
        cudaLaunchDevice(func_ptr, buf, grid, block, shared_mem_bytes, stream)
    }
}
