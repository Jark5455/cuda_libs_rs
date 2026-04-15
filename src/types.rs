#![allow(non_camel_case_types)]

use std::ffi::c_void;
use std::os::raw::c_int;

pub enum CUstream_st {}
pub type cudaStream_t = *mut CUstream_st;

pub enum CUctx_st {}
pub type cudaCtx_t = *mut CUctx_st;

pub type cudaDevice_t = c_int;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct cuDeviceAllocation<T>(pub *mut T);

pub trait CudaAsPtr {
    fn as_const_ptr(&self) -> *const std::ffi::c_void;
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void;
}

impl<T> CudaAsPtr for *mut T {
    fn as_const_ptr(&self) -> *const std::ffi::c_void { *self as *const _ }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void { *self as *mut _ }
}

impl<T> CudaAsPtr for *const T {
    fn as_const_ptr(&self) -> *const std::ffi::c_void { *self as *const _ }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void { panic!("Cannot mutate a const pointer.") }
}

impl<T> CudaAsPtr for cuDeviceAllocation<T> {
    fn as_const_ptr(&self) -> *const std::ffi::c_void { self.0 as *const _ }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void { self.0 as *mut _ }
}

impl<T> CudaAsPtr for &cuDeviceAllocation<T> {
    fn as_const_ptr(&self) -> *const std::ffi::c_void { self.0 as *const _ }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void { panic!("Cannot mutate via shared ref to dev alloc.") }
}

impl<T> CudaAsPtr for &mut cuDeviceAllocation<T> {
    fn as_const_ptr(&self) -> *const std::ffi::c_void { self.0 as *const _ }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void { self.0 as *mut _ }
}

impl<T> CudaAsPtr for &[T] {
    fn as_const_ptr(&self) -> *const std::ffi::c_void { <[T]>::as_ptr(self) as *const _ }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void { panic!("Cannot mutate via shared slice.") }
}

impl<T> CudaAsPtr for &mut [T] {
    fn as_const_ptr(&self) -> *const std::ffi::c_void { <[T]>::as_ptr(self) as *const _ }
    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void { <[T]>::as_mut_ptr(self) as *mut _ }
}


