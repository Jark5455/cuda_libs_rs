#![cfg_attr(target_arch = "nvptx64", no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unnecessary_transmutes)]

extern crate self as cuda_libs_cudart;

#[cfg(not(target_arch = "nvptx64"))]
pub mod safe;
pub mod sys;
#[cfg(not(target_arch = "nvptx64"))]
pub mod types;
