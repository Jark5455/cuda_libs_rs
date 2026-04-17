use cuda_libs::cublas::sys::cublasOperation_t;
use cuda_libs::cublas_lt::sys::cublasComputeType_t::CUBLAS_COMPUTE_32F;
use cuda_libs::cublas_lt::sys::cublasLtMatmulDesc_t;
use cuda_libs::cublas_lt::sys::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_TRANSA;
use cuda_libs::cudart::sys::cudaDataType::CUDA_R_32F;
use cuda_libs::cudart::sys::cudaMemcpyKind::{cudaMemcpyDeviceToHost, cudaMemcpyHostToDevice};
use cuda_libs::cudart::sys::cudaStreamNonBlocking;
use cuda_libs::prelude::*;
use num_complex::Complex32;
use cuda_libs::cufft::sys::{CUFFT_FORWARD, CUFFT_INVERSE};
use cuda_libs::cufft::sys::cufftType::CUFFT_C2C;

#[cuda_libs::cuda_load]
fn main() {
    let device_count = unsafe { cudaGetDeviceCount().expect("Failed to get device count") };
    println!("Cuda device count: {}", device_count);

    cublas_dgemm_example();
    cufft_1dc2c_example();
}

fn cublas_dgemm_example() {
    /// https://github.com/NVIDIA/CUDALibrarySamples/blob/main/cuBLAS/Level-3/gemm/cublas_gemm_example.cu

    const m: i32 = 2;
    const n: i32 = 2;
    const k: i32 = 2;
    const lda: i32 = 2;
    const ldb: i32 = 2;
    const ldc: i32 = 2;

    let A = vec![1.0, 3.0, 2.0, 4.0];
    let B = vec![5.0, 7.0, 6.0, 8.0];

    let mut C = vec![0.0; (m * n) as usize];
    const alpha: f64 = 1.0;
    const beta: f64 = 0.0;

    // the cublas repo is wrong and storing in row major instead of column major
    let transa = cublasOperation_t::CUBLAS_OP_T;
    let transb = cublasOperation_t::CUBLAS_OP_T;

    print!("A\n");
    println!("{} {}", A[0], A[2]);
    println!("{} {}", A[1], A[3]);
    print!("=====\n");

    print!("B\n");
    println!("{} {}", B[0], B[2]);
    println!("{} {}", B[1], B[3]);
    print!("=====\n");

    unsafe {
        let cublasH = cublasCreate_v2().unwrap();
        let mut stream = cudaStreamCreateWithFlags(cudaStreamNonBlocking).unwrap();

        cublasSetStream_v2(cublasH, stream).unwrap();

        let d_A = cudaMalloc::<f64>(size_of::<f64>() * A.len()).unwrap();
        let d_B = cudaMalloc::<f64>(size_of::<f64>() * B.len()).unwrap();
        let d_C = cudaMalloc::<f64>(size_of::<f64>() * C.len()).unwrap();

        cudaMemcpyAsync(d_A, A.as_ptr(), size_of::<f64>() * A.len(), cudaMemcpyHostToDevice, stream).unwrap();
        cudaMemcpyAsync(d_B, B.as_ptr(), size_of::<f64>() * B.len(), cudaMemcpyHostToDevice, stream).unwrap();

        cublasDgemm_v2(cublasH, transa, transb, m, n, k, &alpha as *const f64, d_A, lda, d_B, ldb, &beta as *const f64, d_C, ldc).unwrap();

        cudaMemcpyAsync(C.as_mut_ptr(), d_C, size_of::<f64>() * C.len(), cudaMemcpyDeviceToHost, stream).unwrap();

        cudaStreamSynchronize(stream).unwrap();

        print!("C\n");
        println!("{} {}", C[0], C[2]);
        println!("{} {}", C[1], C[3]);
        print!("=====\n");

        /*
         *   C = 23.0 31.0
         *       34.0 46.0
         */

        cudaFree(d_A).unwrap();
        cudaFree(d_B).unwrap();
        cudaFree(d_C).unwrap();

        cublasDestroy_v2(cublasH).unwrap();
        cudaStreamDestroy(stream).unwrap();
        cudaDeviceReset().unwrap();
    }
}

fn cufft_1dc2c_example() {
    use num_complex::Complex32;

    unsafe {
        let fft_size = 8;
        let batch_size = 2;
        let element_count = batch_size * fft_size;

        let mut data = (0..element_count)
            .map(|i| Complex32::new(i as f32, -i as f32))
            .collect::<Vec<_>>();

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
        cudaDeviceReset().unwrap();
    }
}
