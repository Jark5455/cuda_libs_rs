#![allow(non_camel_case_types)]

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct cuDeviceAllocation<T>(pub *mut T);

impl<T> cuDeviceAllocation<T> {
    pub fn as_cuda_slice<'a>(&'a self, len: usize) -> CudaSlice<'a, T> {
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CudaSlice<'a, T>(*mut [T], std::marker::PhantomData<&'a mut T>);

impl<'a, T> CudaSlice<'a, T> {
    pub fn new(alloc: &'a cuDeviceAllocation<T>, len: usize) -> Self {
        Self(core::ptr::slice_from_raw_parts_mut(alloc.0, len), std::marker::PhantomData)
    }

    pub fn len(&self) -> usize {
        unsafe { (&*self.0).len() }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T> std::ops::Deref for CudaSlice<'a, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<'a, T> std::ops::DerefMut for CudaSlice<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T> CudaAsPtr for CudaSlice<'_, T> {
    fn as_const_ptr(&self) -> *const std::ffi::c_void {
        unsafe { (*self.0).as_ptr() as *const _ }
    }

    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        unsafe { (*self.0).as_mut_ptr() as *mut _ }
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
