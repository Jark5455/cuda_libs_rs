use cuda_libs::cublas::sys::cublasOperation_t;
use cuda_libs::cublas_lt::sys::cublasLtMatmulHeuristicResult_t;
use cuda_libs::cudart::sys::cudaStreamCaptureStatus::cudaStreamCaptureStatusNone;
use cuda_libs::cudart::sys::{cudaStream_t, cudaStreamCreateWithFlags, cudaStreamNonBlocking, cudaStreamDestroy};
use cuda_libs::cudart::sys::cudaMemcpyKind::{cudaMemcpyDeviceToHost, cudaMemcpyHostToDevice};
use cuda_libs::prelude::*;

#[cuda_libs::cuda_load]
fn main() {
    let device_count = unsafe { cudaGetDeviceCount().expect("Failed to get device count") };
    println!("Cuda device count: {}", device_count);

    cublas_dgemm_example();
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

    /* step 1: create cublas handle, bind a stream */
    let cublasH = CublasHandle::new().unwrap();
    let mut stream = cudaStream_t::default();

    unsafe {
        cudaStreamCreateWithFlags(&mut stream, cudaStreamNonBlocking);
        cublasH.cublasSetStream_v2(stream).unwrap();

        let d_A = cudaMalloc::<f64>(size_of::<f64>() * A.len()).unwrap();
        let d_B = cudaMalloc::<f64>(size_of::<f64>() * B.len()).unwrap();
        let d_C = cudaMalloc::<f64>(size_of::<f64>() * C.len()).unwrap();

        cudaMemcpyAsync(d_A, A.as_ptr(), size_of::<f64>() * A.len(), cudaMemcpyHostToDevice, stream).unwrap();
        cudaMemcpyAsync(d_B, B.as_ptr(), size_of::<f64>() * B.len(), cudaMemcpyHostToDevice, stream).unwrap();

        cublasH.cublasDgemm_v2(transa, transb, m, n, k, &alpha as *const f64, d_A, lda, d_B, ldb, &beta as *const f64, d_C, ldc).unwrap();

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

        cudaFree(d_A);
        cudaFree(d_B);
        cudaFree(d_C);

        drop(cublasH);
        cudaStreamDestroy(stream);
        cudaDeviceReset().unwrap();
    }
}
