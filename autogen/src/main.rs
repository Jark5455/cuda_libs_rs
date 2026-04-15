pub mod lib_generator;

use lib_generator::{generate_library, LibraryConfig, HandleConfig};

fn main() {
    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_rt",
        out_dir: "../cuda_libs_rt/src",
        headers: vec!["/opt/cuda/include/cuda_runtime.h"],
        allowlist_functions: "cuda.*",
        allowlist_types: "cuda.*",
        allowlist_vars: "CUDA.*",
        blocklist_types: vec![],
        status_type: "cudaError",
        success_variant: "cudaSuccess",
        handles: vec![HandleConfig { wrapper_name: "CudaExecutionContext", handle_type: "cudaExecutionContext_t" }],
        handle_types_regex: vec!["Context", "Stream_t", "Handle", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec![],
        extra_safe_code: "
            #[allow(non_upper_case_globals)]
            pub use crate::sys::cudaError as CudaStatusEnum;

            impl CudaExecutionContext {
                pub fn cudaGreenCtxCreate(desc: crate::sys::cudaDevResourceDesc_t, device: std::os::raw::c_int, flags: std::os::raw::c_uint) -> Result<Self, crate::sys::cudaError> {
                    unsafe {
                        let mut handle = std::ptr::null_mut();
                        let status = crate::sys::cudaGreenCtxCreate(&mut handle, desc, device, flags);
                        if status == crate::sys::cudaError::cudaSuccess {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }

                pub fn cudaDeviceGetExecutionCtx(device: std::os::raw::c_int) -> Result<Self, crate::sys::cudaError> {
                    unsafe {
                        let mut handle = std::ptr::null_mut();
                        let status = crate::sys::cudaDeviceGetExecutionCtx(&mut handle, device);
                        if status == crate::sys::cudaError::cudaSuccess {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }
            }

            impl Drop for CudaExecutionContext {
                fn drop(&mut self) {
                    unsafe {
                        let _ = crate::sys::cudaExecutionCtxDestroy(self.handle);
                    }
                }
            }
        ",
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cublas_lt",
        out_dir: "../cuda_libs_cublas_lt/src",
        headers: vec!["/opt/cuda/include/cublasLt.h"],
        allowlist_functions: "cublasLt.*",
        allowlist_types: "cublasLt.*",
        allowlist_vars: "CUBLASLT.*",
        blocklist_types: vec![".*cuda.*"],
        status_type: "cublasStatus_t",
        success_variant: "CUBLAS_STATUS_SUCCESS",
        handles: vec![HandleConfig { wrapper_name: "CublasLtHandle", handle_type: "cublasLtHandle_t" }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec![],
        extra_safe_code: "
            impl CublasLtHandle {
                pub fn new() -> Result<Self, crate::sys::cublasStatus_t> {
                    unsafe {
                        let mut handle = std::ptr::null_mut();
                        let status = crate::sys::cublasLtCreate(&mut handle);
                        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }
            }
            impl Drop for CublasLtHandle {
                fn drop(&mut self) {
                    unsafe { crate::sys::cublasLtDestroy(self.handle); }
                }
            }
        ",
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cublas",
        out_dir: "../cuda_libs_cublas/src",
        headers: vec!["/opt/cuda/include/cublas_api.h"],
        allowlist_functions: "cublas.*",
        allowlist_types: "cublas.*",
        allowlist_vars: "CUBLAS.*",
        blocklist_types: vec![".*cuda.*"],
        status_type: "cublasStatus_t",
        success_variant: "CUBLAS_STATUS_SUCCESS",
        handles: vec![HandleConfig { wrapper_name: "CublasHandle", handle_type: "cublasHandle_t" }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec![],
        extra_safe_code: "
            impl CublasHandle {
                pub fn new() -> Result<Self, crate::sys::cublasStatus_t> {
                    unsafe {
                        let mut handle = std::ptr::null_mut();
                        let status = crate::sys::cublasCreate_v2(&mut handle);
                        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }
            }
            impl Drop for CublasHandle {
                fn drop(&mut self) {
                    unsafe { crate::sys::cublasDestroy_v2(self.handle); }
                }
            }
        ",
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cusolver",
        out_dir: "../cuda_libs_cusolver/src",
        headers: vec!["/opt/cuda/include/cusolverDn.h", "/opt/cuda/include/cusolverSp.h"],
        allowlist_functions: "cusolver.*",
        allowlist_types: "cusolver.*",
        allowlist_vars: "CUSOLVER.*",
        blocklist_types: vec![".*cuda.*", "CUstream_st"],
        status_type: "cusolverStatus_t",
        success_variant: "CUSOLVER_STATUS_SUCCESS",
        handles: vec![
            HandleConfig { wrapper_name: "CusolverDnHandle", handle_type: "cusolverDnHandle_t" },
            HandleConfig { wrapper_name: "CusolverSpHandle", handle_type: "cusolverSpHandle_t" },
        ],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec![],
        extra_safe_code: "
            impl CusolverDnHandle {
                pub fn new() -> Result<Self, crate::sys::cusolverStatus_t> {
                    unsafe {
                        let mut handle = std::ptr::null_mut();
                        let status = crate::sys::cusolverDnCreate(&mut handle);
                        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }
            }
            impl Drop for CusolverDnHandle {
                fn drop(&mut self) {
                    unsafe { crate::sys::cusolverDnDestroy(self.handle); }
                }
            }

            impl CusolverSpHandle {
                pub fn new() -> Result<Self, crate::sys::cusolverStatus_t> {
                    unsafe {
                        let mut handle = std::ptr::null_mut();
                        let status = crate::sys::cusolverSpCreate(&mut handle);
                        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }
            }
            impl Drop for CusolverSpHandle {
                fn drop(&mut self) {
                    unsafe { crate::sys::cusolverSpDestroy(self.handle); }
                }
            }
        ",
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cufft",
        out_dir: "../cuda_libs_cufft/src",
        headers: vec!["/opt/cuda/include/cufft.h"],
        allowlist_functions: "cufft.*",
        allowlist_types: "cufft.*",
        allowlist_vars: "CUFFT.*",
        blocklist_types: vec![".*cuda.*"],
        status_type: "cufftResult",
        success_variant: "CUFFT_SUCCESS",
        handles: vec![HandleConfig { wrapper_name: "CufftHandle", handle_type: "cufftHandle" }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec![],
        extra_safe_code: "
            impl CufftHandle {
                pub fn new() -> Result<Self, crate::sys::cufftResult> {
                    unsafe {
                        let mut handle: crate::sys::cufftHandle = 0;
                        let status = crate::sys::cufftCreate(&mut handle);
                        if status == crate::sys::cufftResult_t::CUFFT_SUCCESS {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }
            }
            impl Drop for CufftHandle {
                fn drop(&mut self) {
                    unsafe { crate::sys::cufftDestroy(self.handle); }
                }
            }
        ",
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_curand",
        out_dir: "../cuda_libs_curand/src",
        headers: vec!["/opt/cuda/include/curand.h"],
        allowlist_functions: "curand.*",
        allowlist_types: "curand.*",
        allowlist_vars: "CURAND.*",
        blocklist_types: vec![".*cuda.*"],
        status_type: "curandStatus_t",
        success_variant: "CURAND_STATUS_SUCCESS",
        handles: vec![HandleConfig { wrapper_name: "CurandGenerator", handle_type: "curandGenerator_t" }],
        handle_types_regex: vec!["Generator", "Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec![],
        extra_safe_code: "
            impl CurandGenerator {
                pub fn new(rng_type: crate::sys::curandRngType_t) -> Result<Self, crate::sys::curandStatus_t> {
                    unsafe {
                        let mut handle = std::ptr::null_mut();
                        let status = crate::sys::curandCreateGenerator(&mut handle, rng_type);
                        if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }
            }
            impl Drop for CurandGenerator {
                fn drop(&mut self) {
                    unsafe { crate::sys::curandDestroyGenerator(self.handle); }
                }
            }
        ",
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cusparse",
        out_dir: "../cuda_libs_cusparse/src",
        headers: vec!["/opt/cuda/include/cusparse.h"],
        allowlist_functions: "cusparse.*",
        allowlist_types: "cusparse.*",
        allowlist_vars: "CUSPARSE.*",
        blocklist_types: vec![".*cuda.*"],
        status_type: "cusparseStatus_t",
        success_variant: "CUSPARSE_STATUS_SUCCESS",
        handles: vec![HandleConfig { wrapper_name: "CusparseHandle", handle_type: "cusparseHandle_t" }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec![],
        extra_safe_code: "
            impl CusparseHandle {
                pub fn new() -> Result<Self, crate::sys::cusparseStatus_t> {
                    unsafe {
                        let mut handle = std::ptr::null_mut();
                        let status = crate::sys::cusparseCreate(&mut handle);
                        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }
            }
            impl Drop for CusparseHandle {
                fn drop(&mut self) {
                    unsafe { crate::sys::cusparseDestroy(self.handle); }
                }
            }
        ",
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cudnn",
        out_dir: "../cuda_libs_cudnn/src",
        headers: vec!["/usr/include/cudnn.h"],
        allowlist_functions: "cudnn.*",
        allowlist_types: "cudnn.*",
        allowlist_vars: "CUDNN.*",
        blocklist_types: vec![".*cuda.*"],
        status_type: "cudnnStatus_t",
        success_variant: "CUDNN_STATUS_SUCCESS",
        handles: vec![HandleConfig { wrapper_name: "CudnnHandle", handle_type: "cudnnHandle_t" }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec![],
        extra_safe_code: "
            impl CudnnHandle {
                pub fn new() -> Result<Self, crate::sys::cudnnStatus_t> {
                    unsafe {
                        let mut handle = std::ptr::null_mut();
                        let status = crate::sys::cudnnCreate(&mut handle);
                        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
                            Ok(Self { handle })
                        } else {
                            Err(status)
                        }
                    }
                }
            }
            impl Drop for CudnnHandle {
                fn drop(&mut self) {
                    unsafe { crate::sys::cudnnDestroy(self.handle); }
                }
            }
        ",
    });
}
