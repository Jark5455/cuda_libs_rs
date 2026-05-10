#![cfg_attr(target_arch = "nvptx64", no_std)]
#![cfg_attr(target_arch = "nvptx64", feature(asm_experimental_arch))]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod launch;

#[cfg(target_arch = "nvptx64")]
pub mod device;

#[cfg(not(target_arch = "nvptx64"))]
pub mod handle;

#[cfg(not(target_arch = "nvptx64"))]
pub mod registration;

#[cfg(not(target_arch = "nvptx64"))]
pub use handle::KernelHandle;

pub use launch::Dim3;

pub mod prelude {
    #[cfg(not(target_arch = "nvptx64"))]
    pub use crate::handle::KernelHandle;
    pub use crate::launch::Dim3;
    #[cfg(not(target_arch = "nvptx64"))]
    pub use crate::launch::launch_kernel;
}
