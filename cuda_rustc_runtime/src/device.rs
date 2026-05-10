//! Device-side runtime helpers. Only compiled when `target_arch = "nvptx64"`.
//!
//! The cuda_rustc preprocessor injects calls to these helpers into kernel
//! bodies; user code does not call them directly.

#![cfg(target_arch = "nvptx64")]

/// Reconstruct the dynamic-shared-memory slice for the current kernel launch.
///
/// `base` must be a raw pointer to the start of the dynamic shared region —
/// in practice, the address of a zero-sized marker static placed in shared
/// memory by the cuda_rustc PTX post-processor (`.extern .shared` declaration).
/// The length is read from the `%dynamic_smem_size` PTX special register
/// (available on sm_50 and later) and divided by `size_of::<T>()`.
///
/// # Safety
///
/// - Caller must hold exclusive access (the cuda_rustc preprocessor enforces
///   this by emitting at most one such let-binding per kernel).
/// - `base` must be the marker static's address; passing any other pointer
///   produces a slice over arbitrary memory.
/// - The returned slice is valid only for the duration of the kernel launch.
///   We label the lifetime `'static` because there is no shorter lifetime that
///   the borrow checker can express here, and the slice never escapes the
///   kernel function's frame.
pub unsafe fn dynamic_shared<T>(base: *mut T) -> &'static mut [T] {
    let bytes: u32;
    unsafe {
        core::arch::asm!(
            "mov.u32 {0}, %dynamic_smem_size;",
            out(reg32) bytes,
            options(nomem, nostack, preserves_flags),
        );
    }
    let len = (bytes as usize) / core::mem::size_of::<T>();
    unsafe { core::slice::from_raw_parts_mut(base, len) }
}
