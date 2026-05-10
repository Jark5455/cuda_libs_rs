// `#[shared]` kernel locals appear as `static mut` in the user-visible source.
// rust-analyzer (and rustc 2024) flag bare reads/writes against them, but the
// cuda_rustc preprocessor rewrites those decls into `let mut` bindings (dynamic
// shared) or `export_name`-pinned statics (static shared) before rustc parses
// the file, so the lint never reaches the actual compile. Suppress here for IDE
// tooling that sees the raw text.
#![allow(static_mut_refs)]

//! End-to-end smoke test for `cuda_rustc`.
//!
//! Defines kernels at module scope with `#[global]` and launches them via
//! native CUDA chevron syntax (`print_arr<<<1, 1>>>(d_data_slice)`). The
//! `cuda_rustc` wrapper preprocesses each source file before rustc sees it:
//! chevron calls become `cuda_rustc_runtime::launch_kernel` calls, kernel
//! bodies are extracted into a separately-compiled nvptx crate, and the
//! resulting PTX/cubin is embedded in the host binary's `.nv_fatbin` section.

use cuda_libs::prelude::*;

mod kernels;

// Inline mod containing an external mod — exercises the deep mod-walker
// path: `nested` is inline, `leaf` is external resolved to
// `src/nested/leaf.rs`. The walker must descend into `nested`'s body with
// `mod_dir = src/nested/` and resolve `leaf` as if it were declared at
// that level.
mod nested {
    pub mod leaf;
}

// Cross-file import: `print_arr` is defined as `#[global] pub fn print_arr(...)`
// in `kernels.rs`. The preprocessor's mod-walker descends into that file,
// replaces the kernel fn with `pub static print_arr: KernelHandle`, and
// this `use` brings that handle into scope so the chevron call below
// resolves correctly.
use crate::kernels::print_arr;
use crate::nested::leaf::nested_kernel;

/// 2D point passed by-reference into a kernel. The struct is defined in host
/// code; cuda_rustc copies it into the device crate via reachability analysis.
/// No `#[repr(C)]` is needed: both host and device are compiled by the same
/// rustc invocation chain, so the default `#[repr(Rust)]` layout matches on
/// both sides.
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Translate a slice of host-defined `Point`s by `(dx, dy)`. Per-thread.
#[global]
fn translate_points(points: &mut [Point], dx: f32, dy: f32) {
    use core::arch::nvptx::*;
    let tid = unsafe { _block_idx_x() as usize * _block_dim_x() as usize + _thread_idx_x() as usize };
    if tid < points.len() {
        points[tid].x += dx;
        points[tid].y += dy;
    }
}

/// Sum-reduction with **dynamic** shared memory. The size of the per-block
/// SDATA tile is set at launch time by the third chevron argument, so the same
/// kernel works for any block size up to the device's smem cap. The
/// `static mut SDATA: &mut [f32];` form has no right-hand side — cuda_rustc
/// auto-injects it and emits an `.extern .shared` declaration in PTX.
#[global]
fn dyn_reduce(input: &[f32], output: &mut [f32]) {
    use core::arch::nvptx::*;

    #[shared]
    static mut sdata: &mut [f32];

    let tid = unsafe { _thread_idx_x() } as usize;
    let bdx = unsafe { _block_dim_x() } as usize;
    let bix = unsafe { _block_idx_x() } as usize;
    let n = input.len();

    let idx = bix * bdx + tid;
    sdata[tid] = if idx < n { input[idx] } else { 0.0 };
    unsafe { _syncthreads() };

    let mut s = bdx / 2;
    while s > 0 {
        if tid < s {
            sdata[tid] += sdata[tid + s];
        }
        unsafe { _syncthreads() };
        s /= 2;
    }

    if tid == 0 {
        output[bix] = sdata[0];
    }
}

/// Dynamic-parallelism child kernel. Launched from `parent_launch` via native
/// chevron syntax. The cuda_rustc preprocessor cfg-gates the chevron rewrite —
/// the device arm dispatches via
/// `cuda_rustc_runtime::launch::launch_kernel_device` → `cudaLaunchDevice` —
/// and the build-time nvlink pass resolves the `cudaLaunchDevice` extern symbol
/// against `libcudadevrt.a` so the final cubin is self-contained.
#[global]
fn child_print() {
    use core::arch::nvptx::*;
    unsafe extern "C" {
        fn vprintf(format: *const core::ffi::c_char, valist: *const core::ffi::c_void) -> i32;
    }
    let tid = unsafe { _thread_idx_x() };
    unsafe {
        vprintf(
            "child tid=%d\n\0".as_ptr() as *const core::ffi::c_char,
            &tid as *const _ as *const core::ffi::c_void,
        );
    }
}

/// Device-side cudart smoke kernel. Reaches into `cuda_libs_cudart::sys` for
/// `cudaGetDevice` / `cudaGetLastError` — bindgen emits these as plain
/// `unsafe extern "C"` declarations (no host-only state), so on the nvptx
/// target they land in PTX as `.extern .func` symbols. At module-load time the
/// runtime's cuLink chain JIT-links them against `libcudadevrt.a`. If no
/// kernel actually calls into cudart, the symbols are stripped by ptxas /
/// rustc's PTX backend and the cuLink path is skipped.
///
/// Note: `cudaDeviceSynchronize` was removed from the device-side cudart API
/// in CUDA 12 for sm_90+ (see CUDA Programming Guide §C.4); use
/// `cudaGetLastError` to flush errors instead.
#[global]
fn cudart_device_probe(out_dev: &mut [i32], out_err: &mut [i32]) {
    use cuda_libs_cudart::sys::{cudaGetDevice, cudaGetLastError};
    use core::arch::nvptx::*;

    let tid = unsafe { _thread_idx_x() } as usize;
    if tid != 0 {
        return;
    }

    let mut dev: i32 = -1;
    let s1 = unsafe { cudaGetDevice(&mut dev as *mut i32) };
    let last = unsafe { cudaGetLastError() };

    out_dev[0] = dev;
    // Fold both statuses into a single bitfield so callers can verify each
    // call succeeded (cudaSuccess == 0, so OR of two zeros == 0).
    out_err[0] = (s1 as i32) | (last as i32);
}

#[global]
fn parent_launch() {
    child_print<<<1, 4>>>();
}

/// 2D matrix transpose kernel. Tile-based with a `#[shared]` block-shared
/// staging buffer; tile width is 17 to avoid bank conflicts when reading the
/// transposed columns. Demonstrates 2D grid/block launches via tuple chevron
/// args (`transpose_2d<<<(grid_x, grid_y), (16, 16)>>>(…)`).
#[global]
unsafe fn transpose_2d(input: &[f32], output: &mut [f32], rows: u32, cols: u32) {
    use core::arch::nvptx::*;

    #[shared]
    static mut TILE: [[f32; 17]; 16] = [[0.0; 17]; 16];

    let tx = unsafe { _thread_idx_x() };
    let ty = unsafe { _thread_idx_y() };
    let bx = unsafe { _block_idx_x() };
    let by = unsafe { _block_idx_y() };

    let src_col = bx * 16 + tx;
    let src_row = by * 16 + ty;

    if src_row < rows && src_col < cols {
        unsafe {
            TILE[ty as usize][tx as usize] = input[(src_row * cols + src_col) as usize];
        }
    }

    unsafe { _syncthreads() };

    let dst_col = by * 16 + tx;
    let dst_row = bx * 16 + ty;

    if dst_row < cols && dst_col < rows {
        unsafe {
            output[(dst_row * rows + dst_col) as usize] = TILE[tx as usize][ty as usize];
        }
    }
}

/// Tree-style sum-reduction kernel. Each block reduces a `THREADS_PER_BLOCK*2`
/// chunk of `input` and writes the partial sum to `output[blockIdx.x]`. Uses a
/// `#[shared]` tile to coalesce across the warp.
#[global]
unsafe fn reduce_add(input: &[f32], output: &mut [f32]) {
    use core::arch::nvptx::*;

    #[shared]
    static mut SDATA: [f32; 256] = [0.0; 256];

    let tid = unsafe { _thread_idx_x() } as usize;
    let bdx = unsafe { _block_dim_x() } as usize;
    let bix = unsafe { _block_idx_x() } as usize;
    let n = input.len();

    let mut i = bix * (bdx * 2) + tid;
    let stride = bdx * 2 * unsafe { _grid_dim_x() } as usize;
    let mut acc = 0.0f32;
    while i < n {
        let a = input[i];
        let b = if i + bdx < n { input[i + bdx] } else { 0.0 };
        acc += a + b;
        i += stride;
    }
    unsafe {
        SDATA[tid] = acc;
        _syncthreads();
    }

    let mut s = bdx / 2;
    while s > 0 {
        if tid < s {
            unsafe {
                SDATA[tid] += SDATA[tid + s];
            }
        }
        unsafe { _syncthreads() };
        s /= 2;
    }

    if tid == 0 {
        output[bix] = unsafe { SDATA[0] };
    }
}

fn main() {
    cuda_libs::runtime_link_load();
    let device_count = unsafe { cudaGetDeviceCount().expect("Failed to get device count") };
    println!("Cuda device count: {}", device_count);

    let data = vec![1.0f32, 2.0f32, 3.0f32, 4.0f32];
    unsafe {
        let mut d_data = cudaMalloc::<f32>(size_of::<f32>() * data.len()).unwrap();
        cudaMemcpy(
            &mut d_data,
            data.as_ptr(),
            size_of::<f32>() * data.len(),
            cudaMemcpyKind::cudaMemcpyHostToDevice,
        )
        .unwrap();

        let d_data_slice = d_data.as_cuda_slice(data.len());

        // Native CUDA launch syntax. The cuda_rustc preprocessor rewrites this
        // line into a runtime call before rustc parses the file.
        print_arr<<<1, 1>>>(d_data_slice);

        cudaDeviceSynchronize().unwrap();
        cudaFree(d_data).unwrap();
    }

    println!("--- reduce_add ---");
    reduce_add_example();

    println!("--- translate_points (host struct on device) ---");
    translate_points_example();

    println!("--- dyn_reduce (dynamic shared memory) ---");
    dyn_reduce_example();

    println!("--- parent_launch (dynamic parallelism, kernel-from-kernel) ---");
    parent_launch_example();

    println!("--- transpose_2d (2D grid + tile shared) ---");
    transpose_2d_example();

    println!("--- cudart_device_probe (device-side cuda_libs_cudart calls) ---");
    cudart_device_probe_example();

    println!("--- nested_kernel (inline mod -> external mod) ---");
    nested_kernel_example();
}

fn nested_kernel_example() {
    unsafe {
        let buf = cudaMalloc::<i32>(size_of::<i32>()).unwrap();
        let mut slice = buf.as_cuda_slice(1);
        nested_kernel<<<1, 1>>>(&mut slice[..]);
        cudaDeviceSynchronize().unwrap();

        let mut out = [0i32; 1];
        cudaMemcpy(out.as_mut_ptr(), slice, size_of::<i32>(), cudaMemcpyKind::cudaMemcpyDeviceToHost).unwrap();
        println!("nested_kernel: out[0] = {} ({})", out[0], if out[0] == 42 { "CORRECT" } else { "MISMATCH" });
        cudaFree(buf).unwrap();
    }
}

fn cudart_device_probe_example() {
    unsafe {
        let dev_buf = cudaMalloc::<i32>(size_of::<i32>()).unwrap();
        let err_buf = cudaMalloc::<i32>(size_of::<i32>()).unwrap();
        let mut dev_slice = dev_buf.as_cuda_slice(1);
        let mut err_slice = err_buf.as_cuda_slice(1);

        cudart_device_probe<<<1, 1>>>(&mut dev_slice[..], &mut err_slice[..]);
        cudaDeviceSynchronize().unwrap();

        let mut dev_out = [0i32; 1];
        let mut err_out = [0i32; 1];
        cudaMemcpy(
            dev_out.as_mut_ptr(),
            dev_slice,
            size_of::<i32>(),
            cudaMemcpyKind::cudaMemcpyDeviceToHost,
        )
        .unwrap();
        cudaMemcpy(
            err_out.as_mut_ptr(),
            err_slice,
            size_of::<i32>(),
            cudaMemcpyKind::cudaMemcpyDeviceToHost,
        )
        .unwrap();

        println!("cudart_device_probe: device id = {}, status bits = {}", dev_out[0], err_out[0]);
        println!("cudart_device_probe: {}", if err_out[0] == 0 && dev_out[0] >= 0 { "CORRECT" } else { "MISMATCH" });

        cudaFree(dev_buf).unwrap();
        cudaFree(err_buf).unwrap();
    }
}

fn transpose_2d_example() {
    const ROWS: usize = 64;
    const COLS: usize = 48;

    let input: Vec<f32> = (0..(ROWS * COLS)).map(|i| i as f32).collect();
    let mut output = vec![0.0f32; COLS * ROWS];

    unsafe {
        let mut d_in = cudaMalloc::<f32>(size_of::<f32>() * input.len()).unwrap();
        let d_out = cudaMalloc::<f32>(size_of::<f32>() * output.len()).unwrap();

        cudaMemcpy(
            &mut d_in,
            input.as_ptr(),
            size_of::<f32>() * input.len(),
            cudaMemcpyKind::cudaMemcpyHostToDevice,
        )
        .unwrap();

        let grid_x = COLS.div_ceil(16) as u32;
        let grid_y = ROWS.div_ceil(16) as u32;

        let d_in_slice = d_in.as_cuda_slice(ROWS * COLS);
        let d_out_slice = d_out.as_cuda_slice(COLS * ROWS);

        transpose_2d<<<(grid_x, grid_y), (16u32, 16u32)>>>(d_in_slice, d_out_slice, ROWS as u32, COLS as u32);
        cudaDeviceSynchronize().unwrap();

        cudaMemcpy(
            output.as_mut_ptr(),
            &d_out,
            size_of::<f32>() * output.len(),
            cudaMemcpyKind::cudaMemcpyDeviceToHost,
        )
        .unwrap();

        cudaFree(d_in).unwrap();
        cudaFree(d_out).unwrap();
    }

    let mut ok = true;
    'outer: for r in 0..ROWS {
        for c in 0..COLS {
            let expected = input[r * COLS + c];
            let got = output[c * ROWS + r];
            if (expected - got).abs() > 1e-5 {
                println!("transpose_2d MISMATCH at ({r},{c}): expected {expected}, got {got}");
                ok = false;
                break 'outer;
            }
        }
    }
    println!("transpose_2d ({ROWS}x{COLS}): {}", if ok { "CORRECT" } else { "MISMATCH" });
}

fn parent_launch_example() {
    unsafe {
        parent_launch<<<1, 1>>>();
        cudaDeviceSynchronize().unwrap();
    }
    println!("parent_launch: OK");
}

fn dyn_reduce_example() {
    const BLOCK: usize = 128;
    let n: usize = 1 << 16;
    let blocks = n.div_ceil(BLOCK);
    let shmem_bytes = (BLOCK * size_of::<f32>()) as u32;

    let input = vec![1.0f32; n];

    unsafe {
        let d_in = cudaMalloc::<f32>(size_of::<f32>() * n).unwrap();
        let d_partials = cudaMalloc::<f32>(size_of::<f32>() * blocks).unwrap();

        let mut d_in_slice = d_in.as_cuda_slice(n);
        let mut d_partials_slice = d_partials.as_cuda_slice(blocks);

        cudaMemcpy(
            d_in_slice.as_mut_ptr(),
            input.as_ptr(),
            size_of::<f32>() * n,
            cudaMemcpyKind::cudaMemcpyHostToDevice,
        )
        .unwrap();

        let in_view = &d_in_slice[..n];
        let out_view = &mut d_partials_slice[..blocks];

        // Third chevron arg = dynamic shared bytes. The kernel reads
        // `%dynamic_smem_size` to recover the per-block tile length.
        dyn_reduce<<<blocks as u32, BLOCK as u32, shmem_bytes>>>(in_view, out_view);
        cudaDeviceSynchronize().unwrap();

        let mut partials = vec![0.0f32; blocks];
        cudaMemcpy(
            partials.as_mut_ptr(),
            d_partials_slice,
            size_of::<f32>() * blocks,
            cudaMemcpyKind::cudaMemcpyDeviceToHost,
        )
        .unwrap();

        let total: f32 = partials.iter().sum();
        println!("dyn_reduce total = {} (expected {})", total, n as f32);

        cudaFree(d_in).unwrap();
        cudaFree(d_partials).unwrap();
    }
}

fn translate_points_example() {
    let mut points: Vec<Point> = (0..8).map(|i| Point { x: i as f32, y: -(i as f32) }).collect();
    println!("before: {:?}", &points[..3]);

    unsafe {
        let mut d_points = cudaMalloc::<Point>(size_of::<Point>() * points.len()).unwrap();
        cudaMemcpy(
            &mut d_points,
            points.as_ptr(),
            size_of::<Point>() * points.len(),
            cudaMemcpyKind::cudaMemcpyHostToDevice,
        )
        .unwrap();

        let mut d_slice = d_points.as_cuda_slice(points.len());
        let view = &mut d_slice[..];

        translate_points<<<1, 8>>>(view, 1.5f32, 2.5f32);
        cudaDeviceSynchronize().unwrap();

        cudaMemcpy(
            points.as_mut_ptr(),
            d_slice,
            size_of::<Point>() * points.len(),
            cudaMemcpyKind::cudaMemcpyDeviceToHost,
        )
        .unwrap();

        cudaFree(d_points).unwrap();
    }

    println!("after:  {:?}", &points[..3]);
    let ok = points.iter().enumerate().all(|(i, p)| (p.x - (i as f32 + 1.5)).abs() < 1e-5 && (p.y - (-(i as f32) + 2.5)).abs() < 1e-5);
    println!("translate_points: {}", if ok { "CORRECT" } else { "MISMATCH" });
}

fn reduce_add_example() {
    const THREADS_PER_BLOCK: usize = 256;
    let n: usize = 1 << 20;
    let blocks = n.div_ceil(THREADS_PER_BLOCK * 2);

    let input = vec![1.0f32; n];

    unsafe {
        let d_in = cudaMalloc::<f32>(size_of::<f32>() * n).unwrap();
        let d_partials = cudaMalloc::<f32>(size_of::<f32>() * blocks).unwrap();

        let mut d_in_slice = d_in.as_cuda_slice(n);
        let mut d_partials_slice = d_partials.as_cuda_slice(blocks);

        cudaMemcpy(
            d_in_slice.as_mut_ptr(),
            input.as_ptr(),
            size_of::<f32>() * n,
            cudaMemcpyKind::cudaMemcpyHostToDevice,
        )
        .unwrap();

        let in_view = &d_in_slice[0..n];
        let out_view = &mut d_partials_slice[0..blocks];

        reduce_add<<<blocks as u32, THREADS_PER_BLOCK as u32>>>(in_view, out_view);
        cudaDeviceSynchronize().unwrap();

        let mut partials = vec![0.0f32; blocks];
        cudaMemcpy(
            partials.as_mut_ptr(),
            d_partials_slice,
            size_of::<f32>() * blocks,
            cudaMemcpyKind::cudaMemcpyDeviceToHost,
        )
        .unwrap();

        let total: f32 = partials.iter().sum();
        println!("reduce_add total = {} (expected {})", total, n as f32);

        cudaFree(d_in).unwrap();
        cudaFree(d_partials).unwrap();
    }
}
