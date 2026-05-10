//! Cross-file kernel module — exercises the cuda_rustc preprocessor's
//! mod-walker. `main.rs` declares `mod kernels;` and uses `print_arr` via
//! `use crate::kernels::print_arr;`. The preprocessor descends into this
//! file via the mod-walker, extracts the `#[global]` body into the merged
//! device crate, and replaces this declaration with a `pub static
//! print_arr: KernelHandle` that `main.rs` can import.

#[global]
pub fn print_arr(arr: &[f32]) {
    unsafe {
        unsafe extern "C" {
            fn vprintf(format: *const core::ffi::c_char, valist: *const core::ffi::c_void) -> i32;
        }

        for val in arr {
            let v = (*val) as f64;
            vprintf(
                "%f \0".as_ptr() as *const core::ffi::c_char,
                &v as *const f64 as *const core::ffi::c_void,
            );
        }

        vprintf(
            "\n\0".as_ptr() as *const core::ffi::c_char,
            core::ptr::null() as *const core::ffi::c_void,
        );
    }
}
