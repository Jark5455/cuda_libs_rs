pub mod lib_generator;

use lib_generator::{HandleConfig, LibraryConfig, generate_library};
use std::fmt::format;

fn main() {
    let cuda_home = env!("CUDA_HOME");

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_driver",
        out_dir: "./cuda_libs_driver/src",
        headers: vec![&format!("{}/include/cuda.h", cuda_home)],
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
        const_overrides: std::collections::HashMap::new(),
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cudart",
        out_dir: "./cuda_libs_cudart/src",
        headers: vec![&format!("{}/include/cuda_runtime.h", cuda_home)],
        allowlist_functions: "cuda.*",
        allowlist_types: "cuda.*|cu.*|libraryPropertyType.*|libraryPropertyType_t|float2|double2|__BindgenBitfieldUnit",
        allowlist_vars: "cuda.*|CUDA.*|cu.*|CU.*|LIBRARY_.*",
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
        const_overrides: std::collections::HashMap::new(),
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cublas_lt",
        out_dir: "./cuda_libs_cublas_lt/src",
        headers: vec![&format!("{}/include/cublasLt.h", cuda_home)],
        allowlist_functions: "cublasLt.*",
        allowlist_types: "cublasLt.*",
        allowlist_vars: "CUBLASLT.*",
        blocklist_types: vec![".*cuda.*", "libraryPropertyType_t", "libraryPropertyType", "CUstream_st", "float2", "double2", "cuComplex", "cuDoubleComplex", "cuFloatComplex"],
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
        const_overrides: std::collections::HashMap::new(),
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cublas",
        out_dir: "./cuda_libs_cublas/src",
        headers: vec![&format!("{}/include/cublas_api.h", cuda_home)],
        allowlist_functions: "cublas.*",
        allowlist_types: "cublas.*",
        allowlist_vars: "CUBLAS.*",
        blocklist_types: vec![".*cuda.*", "libraryPropertyType_t", "libraryPropertyType", "CUstream_st", "float2", "double2", "cuComplex", "cuDoubleComplex", "cuFloatComplex"],
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
        const_overrides: std::collections::HashMap::new(),
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cusolver",
        out_dir: "./cuda_libs_cusolver/src",
        headers: vec![&format!("{}/include/cusolverDn.h", cuda_home), &format!("{}/include/cusolverSp.h", cuda_home)],
        allowlist_functions: "cusolver.*",
        allowlist_types: "cusolver.*",
        allowlist_vars: "CUSOLVER.*",
        blocklist_types: vec![
            ".*cuda.*",
            ".*cublas.*",
            ".*cusparse.*",
            "libraryPropertyType_t",
            "libraryPropertyType",
            "CUstream_st",
            "float2",
            "double2",
            "cuComplex",
            "cuDoubleComplex",
            "cuFloatComplex",
            "__BindgenBitfieldUnit",
            "FILE",
            "_IO_FILE",
            "_IO_codecvt",
            "_IO_lock_t",
            "_IO_marker",
            "_IO_wide_data",
        ],
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
        extra_imports: vec!["cuda_libs_cudart", "cuda_libs_cublas", "cuda_libs_cusparse"],
        extra_safe_code: "",
        use_cuda_as_ptr: true,
        const_overrides: std::collections::HashMap::from([("cusolverDnDSgesv", vec![3, 6]), ("cusolverDnSSgesv", vec![3, 6]), ("cusolverDnDDgesv", vec![3, 6]), ("cusolverDnCCgesv", vec![3, 6]), ("cusolverDnZZgesv", vec![3, 6])]),
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cufft",
        out_dir: "./cuda_libs_cufft/src",
        headers: vec![&format!("{}/include/cufft.h", cuda_home)],
        allowlist_functions: "cufft.*",
        allowlist_types: "cufft.*",
        allowlist_vars: "CUFFT.*",
        blocklist_types: vec![".*cuda.*", "libraryPropertyType_t", "libraryPropertyType", "CUstream_st", "float2", "double2", "cuComplex", "cuDoubleComplex", "cuFloatComplex"],
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
        const_overrides: std::collections::HashMap::new(),
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_curand",
        out_dir: "./cuda_libs_curand/src",
        headers: vec![&format!("{}/include/curand.h", cuda_home)],
        allowlist_functions: "curand.*",
        allowlist_types: "curand.*",
        allowlist_vars: "CURAND.*",
        blocklist_types: vec![".*cuda.*", "libraryPropertyType_t", "libraryPropertyType", "CUstream_st", "float2", "double2", "cuComplex", "cuDoubleComplex", "cuFloatComplex"],
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
        const_overrides: std::collections::HashMap::new(),
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cusparse",
        out_dir: "./cuda_libs_cusparse/src",
        headers: vec![&format!("{}/include/cusparse.h", cuda_home)],
        allowlist_functions: "cusparse.*",
        allowlist_types: "cusparse.*",
        allowlist_vars: "CUSPARSE.*",
        blocklist_types: vec![".*cuda.*", "libraryPropertyType_t", "libraryPropertyType", "CUstream_st", "float2", "double2", "cuComplex", "cuDoubleComplex", "cuFloatComplex"],
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
        const_overrides: std::collections::HashMap::new(),
    });

    generate_library(&LibraryConfig {
        lib_name: "cuda_libs_cudnn",
        out_dir: "./cuda_libs_cudnn/src",
        headers: vec![&format!("{}/include/cudnn.h", cuda_home)],
        allowlist_functions: "cudnn.*",
        allowlist_types: "cudnn.*",
        allowlist_vars: "CUDNN.*",
        blocklist_types: vec![".*cuda.*", "libraryPropertyType_t", "libraryPropertyType", "CUstream_st", "float2", "double2", "cuComplex", "cuDoubleComplex", "cuFloatComplex"],
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
        const_overrides: std::collections::HashMap::new(),
    });
}
