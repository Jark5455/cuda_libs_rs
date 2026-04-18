#![allow(non_camel_case_types)]

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct cuDeviceAllocation<T>(pub *mut T);

impl<T> cuDeviceAllocation<T> {
    pub fn as_cuda_slice(self, len: usize) -> CudaSlice<T> {
        CudaSlice::new(self, len)
    }
}

pub trait CudaAsPtr {
    fn as_const_ptr(&self) -> *const std::ffi::c_void;
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void;
}

impl<T> CudaAsPtr for *mut T {
    fn as_const_ptr(&self) -> *const std::ffi::c_void {
        *self as *const _
    }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        *self as *mut _
    }
}

impl<T> CudaAsPtr for *const T {
    fn as_const_ptr(&self) -> *const std::ffi::c_void {
        *self as *const _
    }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        panic!("Cannot mutate a const pointer.")
    }
}

impl<T> CudaAsPtr for cuDeviceAllocation<T> {
    fn as_const_ptr(&self) -> *const std::ffi::c_void {
        self.0 as *const _
    }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        self.0 as *mut _
    }
}

impl<T> CudaAsPtr for &cuDeviceAllocation<T> {
    fn as_const_ptr(&self) -> *const std::ffi::c_void {
        self.0 as *const _
    }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        panic!("Cannot mutate via shared ref to dev alloc.")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CudaSlice<T> {
    pub alloc: cuDeviceAllocation<T>,
    pub len: usize,
}

impl<T> CudaSlice<T> {
    pub fn new(alloc: cuDeviceAllocation<T>, len: usize) -> Self {
        Self { alloc, len }
    }

    pub unsafe fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.alloc.0, self.len) }
    }

    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.alloc.0, self.len) }
    }
}

impl<T> CudaAsPtr for CudaSlice<T> {
    fn as_const_ptr(&self) -> *const std::ffi::c_void {
        self.alloc.0 as *const _
    }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        self.alloc.0 as *mut _
    }
}

impl<T> CudaAsPtr for &mut cuDeviceAllocation<T> {
    fn as_const_ptr(&self) -> *const std::ffi::c_void {
        self.0 as *const _
    }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        self.0 as *mut _
    }
}

impl<T> CudaAsPtr for &[T] {
    fn as_const_ptr(&self) -> *const std::ffi::c_void {
        <[T]>::as_ptr(self) as *const _
    }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        panic!("Cannot mutate via shared slice.")
    }
}

impl<T> CudaAsPtr for &mut [T] {
    fn as_const_ptr(&self) -> *const std::ffi::c_void {
        <[T]>::as_ptr(self) as *const _
    }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        <[T]>::as_mut_ptr(self) as *mut _
    }
}
