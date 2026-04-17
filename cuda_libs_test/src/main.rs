use cuda_libs::cublas::sys::cublasOperation_t;
use cuda_libs::cudart::sys::cudaMemcpyKind::{cudaMemcpyDeviceToHost, cudaMemcpyHostToDevice};
use cuda_libs::cudart::sys::cudaStreamNonBlocking;
use cuda_libs::cufft::sys::cufftType::CUFFT_C2C;
use cuda_libs::cufft::sys::{CUFFT_FORWARD, CUFFT_INVERSE};
use cuda_libs::prelude::*;

#[cuda_libs::cuda_load]
fn main() {
    let device_count = unsafe { cudaGetDeviceCount().expect("Failed to get device count") };
    println!("Cuda device count: {}", device_count);

    cublas_dgemm_example();
    cufft_1dc2c_example();
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

        let d_A = cudaMalloc::<f64>(size_of::<f64>() * a.len()).unwrap();
        let d_B = cudaMalloc::<f64>(size_of::<f64>() * b.len()).unwrap();
        let d_C = cudaMalloc::<f64>(size_of::<f64>() * c.len()).unwrap();

        cudaMemcpyAsync(d_A, a.as_ptr(), size_of::<f64>() * a.len(), cudaMemcpyHostToDevice, stream).unwrap();
        cudaMemcpyAsync(d_B, b.as_ptr(), size_of::<f64>() * b.len(), cudaMemcpyHostToDevice, stream).unwrap();

        cublasDgemm_v2(cublas_h, transa, transb, M, N, K, &ALPHA as *const f64, d_A, LDA, d_B, LDB, &BETA as *const f64, d_C, LDC).unwrap();

        cudaMemcpyAsync(c.as_mut_ptr(), d_C, size_of::<f64>() * c.len(), cudaMemcpyDeviceToHost, stream).unwrap();

        cudaStreamSynchronize(stream).unwrap();

        print!("c\n");
        println!("{} {}", c[0], c[2]);
        println!("{} {}", c[1], c[3]);
        print!("=====\n");

        /*
         *   c = 23.0 31.0
         *       34.0 46.0
         */

        cudaFree(d_A).unwrap();
        cudaFree(d_B).unwrap();
        cudaFree(d_C).unwrap();

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
        for (i, d) in data.iter().enumerate() {
            print!("{} + {}i\n", d.re, d.im);
        }
        print!("=====\n");

        let plan = cufftPlan1d(fft_size, CUFFT_C2C, batch_size).unwrap();
        let stream = cudaStreamCreateWithFlags(cudaStreamNonBlocking).unwrap();
        cufftSetStream(plan, stream).unwrap();

        let d_data = cudaMalloc::<Complex32>(size_of::<Complex32>() * data.len()).unwrap();
        cudaMemcpyAsync(d_data, data.as_ptr(), size_of::<Complex32>() * data.len(), cudaMemcpyHostToDevice, stream).unwrap();

        cufftExecC2C(plan, d_data, d_data, CUFFT_FORWARD).unwrap();
        let alpha = 1.0 / fft_size as f32;

        // we dont have a scaling kernel so we have to use cublas
        {
            let cublas_handle = cublasCreate_v2().unwrap();
            cublasCsscal_v2(cublas_handle, element_count, &alpha as *const f32, d_data, 1).unwrap();
            cublasDestroy_v2(cublas_handle).unwrap();
        }

        cufftExecC2C(plan, d_data, d_data, CUFFT_INVERSE as i32).unwrap();

        cudaMemcpyAsync(data.as_mut_ptr(), d_data, size_of::<Complex32>() * data.len(), cudaMemcpyDeviceToHost, stream).unwrap();
        cudaStreamSynchronize(stream).unwrap();

        print!("Output array after Forward FFT, Normalization, and Inverse FFT :\n");
        for (i, d) in data.iter().enumerate() {
            print!("{} + {}i\n", d.re, d.im);
        }
        print!("=====\n");

        cudaFree(d_data).unwrap();
        cufftDestroy(plan).unwrap();
        cudaStreamDestroy(stream).unwrap();
    }
}
