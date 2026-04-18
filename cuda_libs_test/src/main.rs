use cuda_libs::cublas::sys::cublasOperation_t;
use cuda_libs::cudart::sys::cudaMemcpyKind::{cudaMemcpyDeviceToHost, cudaMemcpyHostToDevice};
use cuda_libs::cudart::sys::cudaStreamNonBlocking;
use cuda_libs::cufft::sys::cufftType::CUFFT_C2C;
use cuda_libs::cufft::sys::{CUFFT_FORWARD, CUFFT_INVERSE};
use cuda_libs::prelude::*;
use cuda_libs::types::CudaAsPtr;

#[cuda_libs::cuda_load]
fn main() {
    let device_count = unsafe { cudaGetDeviceCount().expect("Failed to get device count") };
    println!("Cuda device count: {}", device_count);

    cublas_dgemm_example();
    print!("=====\n");
    cufft_1dc2c_example();
    print!("=====\n");
    cuda_reduce_add_example();
    print!("=====\n");
    cuda_matrix_transpose_2d_example();
}

fn cublas_dgemm_example() {
    /// https://github.com/NVIDIA/CUDALibrarySamples/blob/main/cuBLAS/Level-3/gemm/cublas_gemm_example.cu

    const M: i32 = 2;
    const N: i32 = 2;
    const K: i32 = 2;
    const LDA: i32 = 2;
    const LDB: i32 = 2;
    const LDC: i32 = 2;

    let a = vec![1.0, 3.0, 2.0, 4.0];
    let b = vec![5.0, 7.0, 6.0, 8.0];

    let mut c = vec![0.0; (M * N) as usize];
    const ALPHA: f64 = 1.0;
    const BETA: f64 = 0.0;

    // the cublas repo is wrong and storing in row major instead of column major
    let transa = cublasOperation_t::CUBLAS_OP_T;
    let transb = cublasOperation_t::CUBLAS_OP_T;

    print!("a\n");
    println!("{} {}", a[0], a[2]);
    println!("{} {}", a[1], a[3]);
    print!("=====\n");

    print!("b\n");
    println!("{} {}", b[0], b[2]);
    println!("{} {}", b[1], b[3]);
    print!("=====\n");

    unsafe {
        let cublas_h = cublasCreate_v2().unwrap();
        let stream = cudaStreamCreateWithFlags(cudaStreamNonBlocking).unwrap();

        cublasSetStream_v2(cublas_h, stream).unwrap();

        let mut d_a = cudaMalloc::<f64>(size_of::<f64>() * a.len()).unwrap();
        let mut d_b = cudaMalloc::<f64>(size_of::<f64>() * b.len()).unwrap();
        let mut d_c = cudaMalloc::<f64>(size_of::<f64>() * c.len()).unwrap();

        cudaMemcpyAsync(&mut d_a, a.as_ptr(), size_of::<f64>() * a.len(), cudaMemcpyHostToDevice, stream).unwrap();
        cudaMemcpyAsync(&mut d_b, b.as_ptr(), size_of::<f64>() * b.len(), cudaMemcpyHostToDevice, stream).unwrap();

        cublasDgemm_v2(cublas_h, transa, transb, M, N, K, &ALPHA as *const f64, &d_a, LDA, &d_b, LDB, &BETA as *const f64, &mut d_c, LDC).unwrap();

        cudaMemcpyAsync(c.as_mut_ptr(), &d_c, size_of::<f64>() * c.len(), cudaMemcpyDeviceToHost, stream).unwrap();

        cudaStreamSynchronize(stream).unwrap();

        print!("c\n");
        println!("{} {}", c[0], c[2]);
        println!("{} {}", c[1], c[3]);

        /*
         *   c = 23.0 31.0
         *       34.0 46.0
         */

        cudaFree(d_a).unwrap();
        cudaFree(d_b).unwrap();
        cudaFree(d_c).unwrap();

        cublasDestroy_v2(cublas_h).unwrap();
        cudaStreamDestroy(stream).unwrap();
    }
}

fn cufft_1dc2c_example() {
    use num_complex::Complex32;

    unsafe {
        let fft_size = 8;
        let batch_size = 2;
        let element_count = batch_size * fft_size;

        let mut data = (0..element_count).map(|i| Complex32::new(i as f32, -i as f32)).collect::<Vec<_>>();

        print!("Input array:\n");
        for d in &data {
            print!("{} + {}i\n", d.re, d.im);
        }
        print!("=====\n");

        let plan = cufftPlan1d(fft_size, CUFFT_C2C, batch_size).unwrap();
        let stream = cudaStreamCreateWithFlags(cudaStreamNonBlocking).unwrap();
        cufftSetStream(plan, stream).unwrap();

        let mut d_data = cudaMalloc::<Complex32>(size_of::<Complex32>() * data.len()).unwrap();
        cudaMemcpyAsync(&mut d_data, data.as_ptr(), size_of::<Complex32>() * data.len(), cudaMemcpyHostToDevice, stream).unwrap();

        // Use raw pointers for in-place FFT to satisfy borrowing rules for device memory
        cufftExecC2C(plan, d_data.0, d_data.0, CUFFT_FORWARD).unwrap();
        let alpha = 1.0 / fft_size as f32;

        // we dont have a scaling kernel so we have to use cublas
        {
            let cublas_handle = cublasCreate_v2().unwrap();
            cublasCsscal_v2(cublas_handle, element_count, &alpha as *const f32, &mut d_data, 1).unwrap();
            cublasDestroy_v2(cublas_handle).unwrap();
        }

        cufftExecC2C(plan, d_data.0, d_data.0, CUFFT_INVERSE as i32).unwrap();

        cudaMemcpyAsync(data.as_mut_ptr(), &d_data, size_of::<Complex32>() * data.len(), cudaMemcpyDeviceToHost, stream).unwrap();
        cudaStreamSynchronize(stream).unwrap();

        print!("Output array after Forward FFT, Normalization, and Inverse FFT :\n");
        for d in &data {
            print!("{} + {}i\n", d.re, d.im);
        }

        cudaFree(d_data).unwrap();
        cufftDestroy(plan).unwrap();
        cudaStreamDestroy(stream).unwrap();
    }
}

fn cuda_reduce_add_example() {
    // Reduction #6 from "Optimizing Parallel Reduction in CUDA" by Mark Harris.

    #[rustfmt::skip]
    #[cuda_libs::global]
    pub unsafe fn reduce_add(input: &[f32], output: &mut [f32]) {
        use core::arch::nvptx::*;

        unsafe fn warp_reduce(sdata: &mut [f32], tid: u32) {
            if _block_dim_x() >= 64 { sdata[tid as usize] += sdata[tid as usize + 32] };
            if _block_dim_x() >= 32 { sdata[tid as usize] += sdata[tid as usize + 16] };
            if _block_dim_x() >= 16 { sdata[tid as usize] += sdata[tid as usize + 8] };
            if _block_dim_x() >= 8 { sdata[tid as usize] += sdata[tid as usize + 4] };
            if _block_dim_x() >= 4 { sdata[tid as usize] += sdata[tid as usize + 2] };
            if _block_dim_x() >= 2 { sdata[tid as usize] += sdata[tid as usize + 1] };
        }

        #[link_section = ".shared"]
        static mut sdata: [f32; 256] = [0.0; 256];

        let tid = _thread_idx_x();
        let mut i = _block_idx_x() * (_block_dim_x() * 2) + tid;
        let grid_size = _block_dim_x() * 2 * _grid_dim_x();

        let n = input.len() as u32;
        sdata[tid as usize] = 0.0;

        while i < n {
            sdata[tid as usize] += input[i as usize] + input[i as usize + _block_dim_x() as usize];
            i += grid_size;
        }

        _syncthreads();

        if _block_dim_x() >= 512 { if tid < 256 { sdata[tid as usize] += sdata[tid as usize + 256]; } _syncthreads(); }
        if _block_dim_x() >= 256 { if tid < 128 { sdata[tid as usize] += sdata[tid as usize + 128]; } _syncthreads(); }
        if _block_dim_x() >= 128 { if tid < 64 { sdata[tid as usize] += sdata[tid as usize + 64]; } _syncthreads(); }

        if tid < 32 { warp_reduce(sdata, tid) };
        if tid == 0 { output[_block_idx_x() as usize] = sdata[0] };
    }

    const THREADS_PER_BLOCK: usize = 256;

    unsafe {
        let input = vec![1.0f32; 1 << 22];
        let n = input.len();

        let d_in_alloc = cudaMalloc::<f32>(size_of::<f32>() * n).unwrap();
        let d_tmp_alloc = cudaMalloc::<f32>(size_of::<f32>() * n).unwrap();

        let mut d_in = d_in_alloc.as_cuda_slice(n);
        let d_tmp = d_tmp_alloc.as_cuda_slice(n);

        cudaMemcpy(d_in.as_mut_ptr(), input.as_ptr(), size_of::<f32>() * n, cudaMemcpyHostToDevice).unwrap();

        let mut cur_in = d_in;
        let mut cur_out = d_tmp;
        let mut cur_n = n;

        loop {
            if cur_n <= (THREADS_PER_BLOCK * 8) {
                break;
            }

            let blocks = cur_n.div_ceil(THREADS_PER_BLOCK * 2).max(1);

            let in_view = &cur_in[0..cur_n];
            let out_view = &mut cur_out[0..blocks];

            reduce_add! { <<< blocks, THREADS_PER_BLOCK >>>(in_view, out_view) }
            cudaDeviceSynchronize().unwrap();

            cur_in = cur_out;
            cur_out = if cur_out.as_const_ptr() == d_tmp.as_const_ptr() { d_in } else { d_tmp };
            cur_n = blocks;
        }

        let mut final_partials = vec![0.0f32; cur_n];
        cudaMemcpy(final_partials.as_mut_ptr(), cur_in, size_of::<f32>() * cur_n, cudaMemcpyDeviceToHost).unwrap();

        let result: f32 = final_partials.iter().sum();
        println!("Reduce-add result: {} (expected {})", result, n as f32);

        cudaFree(d_in_alloc).unwrap();
        cudaFree(d_tmp_alloc).unwrap();
    }
}

fn cuda_matrix_transpose_2d_example() {
    #[rustfmt::skip]
    #[cuda_libs::global]
    pub unsafe fn transpose_2d(input: &[f32], output: &mut [f32], rows: u32, cols: u32) {
        use core::arch::nvptx::*;
    
        #[link_section = ".shared"]
        static mut tile: [[f32; 17]; 16] = [[0.0; 17]; 16];

        let tx = _thread_idx_x();
        let ty = _thread_idx_y();
        let bx = _block_idx_x();
        let by = _block_idx_y();

        let src_col = bx * 16 + tx;
        let src_row = by * 16 + ty;

        if src_row < rows && src_col < cols {
            tile[ty as usize][tx as usize] = input[(src_row * cols + src_col) as usize];
        }

        _syncthreads();

        let dst_col = by * 16 + tx;
        let dst_row = bx * 16 + ty;

        if dst_row < cols && dst_col < rows {
            output[(dst_row * rows + dst_col) as usize] = tile[tx as usize][ty as usize];
        }
    }

    const ROWS: usize = 64;
    const COLS: usize = 48;

    // Build a simple test matrix: element [r][c] = (r * COLS + c) as f32
    let input: Vec<f32> = (0..(ROWS * COLS)).map(|i| i as f32).collect();
    let mut output = vec![0.0f32; COLS * ROWS];

    println!("Input matrix (top-left 4x4):");
    for r in 0..4 {
        for c in 0..4 {
            print!("{:>6.1} ", input[r * COLS + c]);
        }
        println!();
    }

    unsafe {
        use cuda_libs::cudart::sys::cudaMemcpyKind::{cudaMemcpyDeviceToHost, cudaMemcpyHostToDevice};

        let mut d_in = cudaMalloc::<f32>(size_of::<f32>() * input.len()).unwrap();
        let d_out = cudaMalloc::<f32>(size_of::<f32>() * output.len()).unwrap();

        cudaMemcpy(&mut d_in, input.as_ptr(), size_of::<f32>() * input.len(), cudaMemcpyHostToDevice).unwrap();

        let grid_x = COLS.div_ceil(16) as u32;
        let grid_y = ROWS.div_ceil(16) as u32;

        let d_in_slice = d_in.as_cuda_slice(ROWS * COLS);
        let d_out_slice = d_out.as_cuda_slice(COLS * ROWS);

        transpose_2d! { <<< (grid_x, grid_y), (16u32, 16u32) >>> (d_in_slice, d_out_slice, ROWS as u32, COLS as u32) }
        cudaDeviceSynchronize().unwrap();

        cudaMemcpy(output.as_mut_ptr(), &d_out, size_of::<f32>() * output.len(), cudaMemcpyDeviceToHost).unwrap();

        cudaFree(d_in).unwrap();
        cudaFree(d_out).unwrap();
    }

    println!("Output matrix (top-left 4x4):");
    for r in 0..4 {
        for c in 0..4 {
            print!("{:>6.1} ", output[r * ROWS + c]);
        }
        println!();
    }

    // Verify: output[c][r] should equal input[r][c]
    let mut ok = true;
    'outer: for r in 0..ROWS {
        for c in 0..COLS {
            let expected = input[r * COLS + c];
            let got = output[c * ROWS + r];
            if (expected - got).abs() > 1e-5 {
                println!("Transpose MISMATCH at ({r},{c}): expected {expected}, got {got}");
                ok = false;
                break 'outer;
            }
        }
    }

    if ok {
        println!("2D matrix transpose ({ROWS}x{COLS}): CORRECT ✓");
    }
}
