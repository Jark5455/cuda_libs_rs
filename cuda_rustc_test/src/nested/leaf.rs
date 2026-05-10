//! Stress test: kernel reached through `mod nested { mod leaf; }` (inline
//! mod containing an external mod). The mod walker must descend the inline
//! `nested` mod with `mod_dir = src/nested/` and resolve `leaf` to
//! `src/nested/leaf.rs`. The kernel below ends up in the merged device
//! crate; the host side gets a `pub static nested_kernel: KernelHandle`
//! that `main.rs` reaches as `crate::nested::leaf::nested_kernel`.

#[global]
pub fn nested_kernel(out: &mut [i32]) {
    use core::arch::nvptx::*;
    let tid = unsafe { _thread_idx_x() } as usize;
    if tid == 0 {
        out[0] = 42;
    }
}
