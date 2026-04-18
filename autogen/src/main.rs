pub mod lib_generator;

use lib_generator::{HandleConfig, LibraryConfig, generate_library};

fn main() {
    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_driver",
        out_dir: "./cuda_libs_driver/src",
        headers: vec!["/opt/cuda/include/cuda.h"],
        allowlist_functions: "cu.*|CU.*",
        // cudaError_enum is the underlying C enum that CUresult aliases; allow it explicitly
        allowlist_types: "cu.*|CU.*|cudaError_enum",
        allowlist_vars: "CU.*",
        // Block all cuda runtime types EXCEPT cudaError_enum (which is the CUresult backing enum)
        blocklist_types: vec!["^cuda[^E].*", "^cudaE[^r].*", "^cudaEr[^r].*"],
        blocklist_functions: vec![],
        status_type: "CUresult",
        success_variant: "CUDA_SUCCESS",
        handles: vec![HandleConfig { wrapper_name: "DriverContext", handle_type: "CUctx" }],
        handle_types_regex: vec!["CUctx"],
        extra_imports: vec![],
        extra_safe_code: "",
        use_cuda_as_ptr: false,
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cudart",
        out_dir: "./cuda_libs_cudart/src",
        headers: vec!["/opt/cuda/include/cuda_runtime.h"],
        allowlist_functions: "cuda.*",
        allowlist_types: "cuda.*",
        allowlist_vars: "cuda.*|CUDA.*",
        blocklist_types: vec![],
        blocklist_functions: vec![],
        status_type: "cudaError",
        success_variant: "cudaSuccess",
        handles: vec![HandleConfig {
            wrapper_name: "CudaExecutionContext",
            handle_type: "cudaExecutionContext_t",
        }],
        handle_types_regex: vec!["Context", "Stream_t", "Handle", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec![],
        extra_safe_code: "
            #[allow(non_upper_case_globals)]
            pub use crate::sys::cudaError as CudaStatusEnum;
        ",
        use_cuda_as_ptr: true,
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cublas_lt",
        out_dir: "./cuda_libs_cublas_lt/src",
        headers: vec!["/opt/cuda/include/cublasLt.h"],
        allowlist_functions: "cublasLt.*",
        allowlist_types: "cublasLt.*",
        allowlist_vars: "CUBLASLT.*",
        blocklist_types: vec![".*cuda.*"],
        blocklist_functions: vec![],
        status_type: "cublasStatus_t",
        success_variant: "CUBLAS_STATUS_SUCCESS",
        handles: vec![HandleConfig {
            wrapper_name: "CublasLtHandle",
            handle_type: "cublasLtHandle_t",
        }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec!["cuda_libs_cudart"],
        extra_safe_code: "",
        use_cuda_as_ptr: true,
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cublas",
        out_dir: "./cuda_libs_cublas/src",
        headers: vec!["/opt/cuda/include/cublas_api.h"],
        allowlist_functions: "cublas.*",
        allowlist_types: "cublas.*",
        allowlist_vars: "CUBLAS.*",
        blocklist_types: vec![".*cuda.*"],
        blocklist_functions: vec![],
        status_type: "cublasStatus_t",
        success_variant: "CUBLAS_STATUS_SUCCESS",
        handles: vec![HandleConfig {
            wrapper_name: "CublasHandle",
            handle_type: "cublasHandle_t",
        }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec!["cuda_libs_cudart"],
        extra_safe_code: "",
        use_cuda_as_ptr: true,
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cusolver",
        out_dir: "./cuda_libs_cusolver/src",
        headers: vec!["/opt/cuda/include/cusolverDn.h", "/opt/cuda/include/cusolverSp.h"],
        allowlist_functions: "cusolver.*",
        allowlist_types: "cusolver.*",
        allowlist_vars: "CUSOLVER.*",
        blocklist_types: vec![".*cuda.*", "CUstream_st"],
        blocklist_functions: vec![],
        status_type: "cusolverStatus_t",
        success_variant: "CUSOLVER_STATUS_SUCCESS",
        handles: vec![
            HandleConfig {
                wrapper_name: "CusolverDnHandle",
                handle_type: "cusolverDnHandle_t",
            },
            HandleConfig {
                wrapper_name: "CusolverSpHandle",
                handle_type: "cusolverSpHandle_t",
            },
        ],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec!["cuda_libs_cudart"],
        extra_safe_code: "",
        use_cuda_as_ptr: true,
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cufft",
        out_dir: "./cuda_libs_cufft/src",
        headers: vec!["/opt/cuda/include/cufft.h"],
        allowlist_functions: "cufft.*",
        allowlist_types: "cufft.*",
        allowlist_vars: "CUFFT.*",
        blocklist_types: vec![".*cuda.*"],
        blocklist_functions: vec![],
        status_type: "cufftResult",
        success_variant: "CUFFT_SUCCESS",
        handles: vec![HandleConfig {
            wrapper_name: "CufftHandle",
            handle_type: "cufftHandle",
        }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec!["cuda_libs_cudart"],
        extra_safe_code: "",
        use_cuda_as_ptr: true,
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_curand",
        out_dir: "./cuda_libs_curand/src",
        headers: vec!["/opt/cuda/include/curand.h"],
        allowlist_functions: "curand.*",
        allowlist_types: "curand.*",
        allowlist_vars: "CURAND.*",
        blocklist_types: vec![".*cuda.*"],
        blocklist_functions: vec![],
        status_type: "curandStatus_t",
        success_variant: "CURAND_STATUS_SUCCESS",
        handles: vec![HandleConfig {
            wrapper_name: "CurandGenerator",
            handle_type: "curandGenerator_t",
        }],
        handle_types_regex: vec!["Generator", "Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec!["cuda_libs_cudart"],
        extra_safe_code: "",
        use_cuda_as_ptr: true,
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cusparse",
        out_dir: "./cuda_libs_cusparse/src",
        headers: vec!["/opt/cuda/include/cusparse.h"],
        allowlist_functions: "cusparse.*",
        allowlist_types: "cusparse.*",
        allowlist_vars: "CUSPARSE.*",
        blocklist_types: vec![".*cuda.*"],
        blocklist_functions: vec![],
        status_type: "cusparseStatus_t",
        success_variant: "CUSPARSE_STATUS_SUCCESS",
        handles: vec![HandleConfig {
            wrapper_name: "CusparseHandle",
            handle_type: "cusparseHandle_t",
        }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec!["cuda_libs_cudart"],
        extra_safe_code: "",
        use_cuda_as_ptr: true,
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cudnn",
        out_dir: "./cuda_libs_cudnn/src",
        headers: vec!["/usr/include/cudnn.h"],
        allowlist_functions: "cudnn.*",
        allowlist_types: "cudnn.*",
        allowlist_vars: "CUDNN.*",
        blocklist_types: vec![".*cuda.*"],
        blocklist_functions: vec![],
        status_type: "cudnnStatus_t",
        success_variant: "CUDNN_STATUS_SUCCESS",
        handles: vec![HandleConfig {
            wrapper_name: "CudnnHandle",
            handle_type: "cudnnHandle_t",
        }],
        handle_types_regex: vec!["Context", "Stream_t", "Stream", "ctx", "Device", "CUstream_st"],
        extra_imports: vec!["cuda_libs_cudart"],
        extra_safe_code: "",
        use_cuda_as_ptr: true,
    });
}
