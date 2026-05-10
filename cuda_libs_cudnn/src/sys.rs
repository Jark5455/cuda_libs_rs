#![allow(unsafe_op_in_unsafe_fn)]
use cuda_libs_cudart::sys::*;
pub const CUDNN_MAJOR: u32 = 9;
pub const CUDNN_MINOR: u32 = 22;
pub const CUDNN_PATCHLEVEL: u32 = 0;
pub const CUDNN_VERSION: u32 = 92200;
pub const CUDNN_MAX_SM_MAJOR_NUMBER: u32 = 12;
pub const CUDNN_MAX_SM_MINOR_NUMBER: u32 = 1;
pub const CUDNN_MAX_DEVICE_VERSION: u32 = 1210;
pub const CUDNN_GRAPH_MAJOR: u32 = 9;
pub const CUDNN_GRAPH_MINOR: u32 = 22;
pub const CUDNN_GRAPH_PATCH: u32 = 0;
pub const CUDNN_DIM_MAX: u32 = 8;
pub const CUDNN_OPS_MAJOR: u32 = 9;
pub const CUDNN_OPS_MINOR: u32 = 22;
pub const CUDNN_OPS_PATCH: u32 = 0;
pub const CUDNN_LRN_MIN_N: u32 = 1;
pub const CUDNN_LRN_MAX_N: u32 = 16;
pub const CUDNN_LRN_MIN_K: f64 = 0.00001;
pub const CUDNN_LRN_MIN_BETA: f64 = 0.01;
pub const CUDNN_BN_MIN_EPSILON: f64 = 0.0;
pub const CUDNN_ADV_MAJOR: u32 = 9;
pub const CUDNN_ADV_MINOR: u32 = 22;
pub const CUDNN_ADV_PATCH: u32 = 0;
pub const CUDNN_RNN_PADDED_IO_DISABLED: u32 = 0;
pub const CUDNN_RNN_PADDED_IO_ENABLED: u32 = 1;
pub const CUDNN_SEQDATA_DIM_COUNT: u32 = 4;
pub const CUDNN_ATTN_QUERYMAP_ALL_TO_ONE: u32 = 0;
pub const CUDNN_ATTN_QUERYMAP_ONE_TO_ONE: u32 = 1;
pub const CUDNN_ATTN_DISABLE_PROJ_BIASES: u32 = 0;
pub const CUDNN_ATTN_ENABLE_PROJ_BIASES: u32 = 2;
pub const CUDNN_ATTN_WKIND_COUNT: u32 = 8;
pub const CUDNN_CNN_MAJOR: u32 = 9;
pub const CUDNN_CNN_MINOR: u32 = 22;
pub const CUDNN_CNN_PATCH: u32 = 0;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraph_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnContext {
    _unused: [u8; 0],
}
#[doc = "Opaque pointer to cuDNN library context.\n> **Since** cuDNN 9.0.0"]
pub type cudnnHandle_t = *mut cudnnContext;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns cuDNN library version (MAJOR*10000 + MINOR*100 + PATCH).\n\n# Returns\n\nThe cuDNN version as an encoded integer.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetVersion() -> usize;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns max supported GPU compute capability.\n\n# Returns\n\nThe maximum supported device version.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetMaxDeviceVersion() -> usize;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns CUDA Runtime version linked against cuDNN.\n\n# Returns\n\nThe CUDA Runtime version.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetCudartVersion() -> usize;
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_ALLOC_FAILED: cudnnStatus_t = cudnnStatus_t::CUDNN_STATUS_INTERNAL_ERROR_HOST_ALLOCATION_FAILED;
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_ARCH_MISMATCH: cudnnStatus_t = cudnnStatus_t::CUDNN_STATUS_NOT_SUPPORTED_ARCH_MISMATCH;
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_MAPPING_ERROR: cudnnStatus_t = cudnnStatus_t::CUDNN_STATUS_INTERNAL_ERROR_TEXTURE_CREATION_FAILED;
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_RUNTIME_PREREQUISITE_MISSING: cudnnStatus_t = cudnnStatus_t::CUDNN_STATUS_NOT_SUPPORTED_RUNTIME_PREREQUISITE_MISSING;
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_VERSION_MISMATCH: cudnnStatus_t = cudnnStatus_t::CUDNN_STATUS_SUBLIBRARY_VERSION_MISMATCH;
}
#[repr(u32)]
#[doc = "Return status codes for cuDNN API calls.\nStatus codes are grouped by category: 0=success, 1xxx=initialization/version,\n2xxx=bad parameter, 3xxx=not supported, 4xxx=internal error, 5xxx=execution failed.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnStatus_t {
    #[doc = "< Operation completed successfully. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_SUCCESS = 0,
    #[doc = "< cuDNN library not initialized. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_INITIALIZED = 1001,
    #[doc = "< Sub-library version mismatch. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_SUBLIBRARY_VERSION_MISMATCH = 1002,
    #[doc = "< Serialization version mismatch. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_SERIALIZATION_VERSION_MISMATCH = 1003,
    #[doc = "< Deprecated feature was used. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_DEPRECATED = 1004,
    #[doc = "< License error. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_LICENSE_ERROR = 1005,
    #[doc = "< Runtime operation in progress. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_RUNTIME_IN_PROGRESS = 1006,
    #[doc = "< Floating-point overflow at runtime. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_RUNTIME_FP_OVERFLOW = 1007,
    #[doc = "< Sub-library loading failed. > **Since** cuDNN 9.2.0"]
    CUDNN_STATUS_SUBLIBRARY_LOADING_FAILED = 1008,
    #[doc = "< Invalid parameter value. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM = 2000,
    #[doc = "< Null pointer passed as parameter. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM_NULL_POINTER = 2002,
    #[doc = "< Misaligned pointer passed. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM_MISALIGNED_POINTER = 2003,
    #[doc = "< Descriptor not finalized. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM_NOT_FINALIZED = 2004,
    #[doc = "< Parameter out of bounds. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM_OUT_OF_BOUND = 2005,
    #[doc = "< Insufficient buffer size. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM_SIZE_INSUFFICIENT = 2006,
    #[doc = "< CUDA stream mismatch. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM_STREAM_MISMATCH = 2007,
    #[doc = "< Tensor shape mismatch. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM_SHAPE_MISMATCH = 2008,
    #[doc = "< Duplicated entries detected. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM_DUPLICATED_ENTRIES = 2009,
    #[doc = "< Wrong attribute type. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_BAD_PARAM_ATTRIBUTE_TYPE = 2010,
    #[doc = "< CUDA graph mismatch. > **Since** cuDNN 9.5.0"]
    CUDNN_STATUS_BAD_PARAM_CUDA_GRAPH_MISMATCH = 2011,
    #[doc = "< Wrong descriptor type. > **Since** cuDNN 9.6.0"]
    CUDNN_STATUS_BAD_PARAM_DESCRIPTOR_TYPE = 2012,
    #[doc = "< Operation not supported. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED = 3000,
    #[doc = "< Graph pattern not supported. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_GRAPH_PATTERN = 3001,
    #[doc = "< Shape not supported. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_SHAPE = 3002,
    #[doc = "< Data type not supported. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_DATA_TYPE = 3003,
    #[doc = "< Layout not supported. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_LAYOUT = 3004,
    #[doc = "< Incompatible CUDA driver. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_INCOMPATIBLE_CUDA_DRIVER = 3005,
    #[doc = "< Incompatible CUDA runtime. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_INCOMPATIBLE_CUDART = 3006,
    #[doc = "< GPU architecture mismatch. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_ARCH_MISMATCH = 3007,
    #[doc = "< Runtime prerequisite missing. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_RUNTIME_PREREQUISITE_MISSING = 3008,
    #[doc = "< Sub-library unavailable. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_SUBLIBRARY_UNAVAILABLE = 3009,
    #[doc = "< Insufficient shared memory. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_SHARED_MEMORY_INSUFFICIENT = 3010,
    #[doc = "< Padding mode not supported. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_PADDING = 3011,
    #[doc = "< Bad launch parameters. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_NOT_SUPPORTED_BAD_LAUNCH_PARAM = 3012,
    #[doc = "< CUDA graph native API not supported. > **Since** cuDNN 9.5.0"]
    CUDNN_STATUS_NOT_SUPPORTED_CUDA_GRAPH_NATIVE_API = 3013,
    #[doc = "< Invalid dynamic shape. > **Since** cuDNN 9.18.0"]
    CUDNN_STATUS_NOT_SUPPORTED_INVALID_DYNAMIC_SHAPE = 3014,
    #[doc = "< Internal error. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_INTERNAL_ERROR = 4000,
    #[doc = "< Kernel compilation failed. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_INTERNAL_ERROR_COMPILATION_FAILED = 4001,
    #[doc = "< Unexpected internal value. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_INTERNAL_ERROR_UNEXPECTED_VALUE = 4002,
    #[doc = "< Host memory allocation failed. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_INTERNAL_ERROR_HOST_ALLOCATION_FAILED = 4003,
    #[doc = "< Device memory allocation failed. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_INTERNAL_ERROR_DEVICE_ALLOCATION_FAILED = 4004,
    #[doc = "< Bad internal launch parameters. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_INTERNAL_ERROR_BAD_LAUNCH_PARAM = 4005,
    #[doc = "< Texture creation failed. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_INTERNAL_ERROR_TEXTURE_CREATION_FAILED = 4006,
    #[doc = "< Execution failed. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_EXECUTION_FAILED = 5000,
    #[doc = "< CUDA driver execution failure. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_EXECUTION_FAILED_CUDA_DRIVER = 5001,
    #[doc = "< cuBLAS execution failure. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_EXECUTION_FAILED_CUBLAS = 5002,
    #[doc = "< CUDA runtime execution failure. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_EXECUTION_FAILED_CUDART = 5003,
    #[doc = "< cuRAND execution failure. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_EXECUTION_FAILED_CURAND = 5004,
    #[doc = "< > **Deprecated** Use CUDNN_STATUS_BAD_PARAM. > **Since** cuDNN 9.0.0"]
    CUDNN_STATUS_INVALID_VALUE = 2001,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Converts status code to human-readable string.\n\n# Arguments\n\n* `status` [in]  - The cuDNN status code to convert.\n\n# Returns\n\nPointer to a static string describing the status code.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetErrorString(status: cudnnStatus_t) -> *const ::core::ffi::c_char;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves most recent error message. Thread-safe.\n\n# Arguments\n\n* `message` [out]  -   Buffer to receive the error message string.\n* `max_size` [in]  -  Maximum number of bytes to write into `message.`\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetLastErrorString(message: *mut ::core::ffi::c_char, max_size: usize);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnRuntimeTag_t {
    _unused: [u8; 0],
}
#[repr(u32)]
#[doc = "Error query modes for cudnnQueryRuntimeError.\n> **Deprecated** > **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnErrQueryMode_t {
    #[doc = "< Return raw error code. > **Since** cuDNN 9.0.0"]
    CUDNN_ERRQUERY_RAWCODE = 0,
    #[doc = "< Non-blocking error query. > **Since** cuDNN 9.0.0"]
    CUDNN_ERRQUERY_NONBLOCKING = 1,
    #[doc = "< Blocking error query. > **Since** cuDNN 9.0.0"]
    CUDNN_ERRQUERY_BLOCKING = 2,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Queries remote kernel error state.\n> **Deprecated** Use cudnnGetLastErrorString instead.\n\n# Arguments\n\n* `handle` [in]  -   cuDNN handle.\n* `rstatus` [out]  -  Pointer to receive the runtime status.\n* `mode` [in]  -     Error query mode.\n* `tag` [out]  -      Runtime tag (unused, may be NULL).\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnQueryRuntimeError(handle: cudnnHandle_t, rstatus: *mut cudnnStatus_t, mode: cudnnErrQueryMode_t, tag: *mut cudnnRuntimeTag_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Queries library property (major, minor, or patch version).\n\n# Arguments\n\n* `type` [in]  -   The property type to query (MAJOR_VERSION, MINOR_VERSION, or PATCH_LEVEL).\n* `value` [out]  -  Pointer to receive the property value.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetProperty(type_: libraryPropertyType, value: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates cuDNN context. Must precede all other cuDNN library calls.\n\n# Arguments\n\n* `handle` [out]  -  Pointer to receive the newly created cuDNN handle.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_NOT_INITIALIZED\n@retval CUDNN_STATUS_NOT_SUPPORTED_ARCH_MISMATCH\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCreate(handle: *mut cudnnHandle_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys cuDNN context. Calls cudaDeviceSynchronize.\n\n# Arguments\n\n* `handle` [in]  -  The cuDNN handle to destroy.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnDestroy(handle: cudnnHandle_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Associates CUDA stream with cuDNN handle.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN handle.\n* `streamId` [in]  -  CUDA stream to associate.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetStream(handle: cudnnHandle_t, streamId: cudaStream_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves CUDA stream from cuDNN handle.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN handle.\n* `streamId` [out]  -  Pointer to receive the associated CUDA stream.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetStream(handle: cudnnHandle_t, streamId: *mut cudaStream_t) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Supported data types for cuDNN tensors and operations.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnDataType_t {
    #[doc = "< 32-bit IEEE floating point. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_FLOAT = 0,
    #[doc = "< 64-bit IEEE floating point. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_DOUBLE = 1,
    #[doc = "< 16-bit IEEE floating point (FP16). > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_HALF = 2,
    #[doc = "< 8-bit signed integer. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_INT8 = 3,
    #[doc = "< 32-bit signed integer. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_INT32 = 4,
    #[doc = "< > **Deprecated** Vectorized 4x INT8. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_INT8x4 = 5,
    #[doc = "< 8-bit unsigned integer. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_UINT8 = 6,
    #[doc = "< > **Deprecated** Vectorized 4x UINT8. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_UINT8x4 = 7,
    #[doc = "< > **Deprecated** Vectorized 32x INT8. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_INT8x32 = 8,
    #[doc = "< Brain floating point (BF16). > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_BFLOAT16 = 9,
    #[doc = "< 64-bit signed integer. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_INT64 = 10,
    #[doc = "< Boolean type. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_BOOLEAN = 11,
    #[doc = "< FP8 with 4-bit exponent, 3-bit mantissa. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_FP8_E4M3 = 12,
    #[doc = "< FP8 with 5-bit exponent, 2-bit mantissa. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_FP8_E5M2 = 13,
    #[doc = "< Fast float accumulator type for FP8 compute paths. > **Since** cuDNN 9.0.0"]
    CUDNN_DATA_FAST_FLOAT_FOR_FP8 = 14,
    #[doc = "< Pure-exponent scale format (8-bit exponent, 0-bit mantissa) for block scaling. > **Since** cuDNN 9.7.0"]
    CUDNN_DATA_FP8_E8M0 = 15,
    #[doc = "< FP4 with 2-bit exponent, 1-bit mantissa. > **Since** cuDNN 9.7.0"]
    CUDNN_DATA_FP4_E2M1 = 16,
    #[doc = "< 4-bit signed integer. > **Since** cuDNN 9.11.0"]
    CUDNN_DATA_INT4 = 17,
    #[doc = "< 4-bit unsigned integer. > **Since** cuDNN 9.11.0"]
    CUDNN_DATA_UINT4 = 18,
    #[doc = "< 32-bit unsigned integer. > **Since** cuDNN 9.11.0"]
    CUDNN_DATA_UINT32 = 19,
    #[doc = "< Complex 32-bit floating point. > **Since** cuDNN 9.14.0"]
    CUDNN_DATA_COMPLEX_FP32 = 20,
    #[doc = "< Complex 64-bit floating point. > **Since** cuDNN 9.14.0"]
    CUDNN_DATA_COMPLEX_FP64 = 21,
}
#[repr(u32)]
#[doc = "Math precision modes for cuDNN operations.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnMathType_t {
    #[doc = "< Default math mode. > **Since** cuDNN 9.0.0"]
    CUDNN_DEFAULT_MATH = 0,
    #[doc = "< Tensor Core math. > **Since** cuDNN 9.0.0"]
    CUDNN_TENSOR_OP_MATH = 1,
    #[doc = "< Tensor Core math with type conversion. > **Since** cuDNN 9.0.0"]
    CUDNN_TENSOR_OP_MATH_ALLOW_CONVERSION = 2,
    #[doc = "< FMA (fused multiply-add) math only. > **Since** cuDNN 9.0.0"]
    CUDNN_FMA_MATH = 3,
}
#[repr(u32)]
#[doc = "NaN propagation modes.\n> **Deprecated** > **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnNanPropagation_t {
    #[doc = "< > **Deprecated** Do not propagate NaN. > **Since** cuDNN 9.0.0"]
    CUDNN_NOT_PROPAGATE_NAN = 0,
    #[doc = "< > **Deprecated** Propagate NaN values. > **Since** cuDNN 9.0.0"]
    CUDNN_PROPAGATE_NAN = 1,
}
#[repr(u32)]
#[doc = "CTC gradient modes controlling behavior for out-of-bounds (OOB) samples.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnCTCGradMode_t {
    #[doc = "< Zero the gradient for OOB samples (guarantees finite). > **Since** cuDNN 9.0.0"]
    CUDNN_CTC_ZERO_OOB_GRADIENTS = 0,
    #[doc = "< Skip writing gradient for OOB samples. > **Since** cuDNN 9.0.0"]
    CUDNN_CTC_SKIP_OOB_GRADIENTS = 1,
}
#[repr(u32)]
#[doc = "Tensor memory layout formats.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnTensorFormat_t {
    #[doc = "< Row major layout (wStride = 1, hStride = w). > **Since** cuDNN 9.0.0"]
    CUDNN_TENSOR_NCHW = 0,
    #[doc = "< Feature maps interleaved (cStride = 1). > **Since** cuDNN 9.0.0"]
    CUDNN_TENSOR_NHWC = 1,
    #[doc = "< Vectorized channel layout, vector length in data type. > **Since** cuDNN 9.0.0"]
    CUDNN_TENSOR_NCHW_VECT_C = 2,
}
#[repr(u32)]
#[doc = "Reduction operations for tensor reduction.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnReduceTensorOp_t {
    #[doc = "< Sum reduction. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_ADD = 0,
    #[doc = "< Product reduction. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_MUL = 1,
    #[doc = "< Minimum value reduction. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_MIN = 2,
    #[doc = "< Maximum value reduction. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_MAX = 3,
    #[doc = "< Maximum absolute value reduction. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_AMAX = 4,
    #[doc = "< Average reduction. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_AVG = 5,
    #[doc = "< L1 norm reduction. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_NORM1 = 6,
    #[doc = "< L2 norm reduction. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_NORM2 = 7,
    #[doc = "< Product reduction ignoring zeros. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_MUL_NO_ZEROS = 8,
}
#[repr(u32)]
#[doc = "Activation function modes.\n> **Deprecated** Use pointwise operations instead.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnActivationMode_t {
    #[doc = "< > **Deprecated** Sigmoid activation. > **Since** cuDNN 9.0.0"]
    CUDNN_ACTIVATION_SIGMOID = 0,
    #[doc = "< > **Deprecated** ReLU activation. > **Since** cuDNN 9.0.0"]
    CUDNN_ACTIVATION_RELU = 1,
    #[doc = "< > **Deprecated** Tanh activation. > **Since** cuDNN 9.0.0"]
    CUDNN_ACTIVATION_TANH = 2,
    #[doc = "< > **Deprecated** Clipped ReLU activation. > **Since** cuDNN 9.0.0"]
    CUDNN_ACTIVATION_CLIPPED_RELU = 3,
    #[doc = "< > **Deprecated** ELU activation. > **Since** cuDNN 9.0.0"]
    CUDNN_ACTIVATION_ELU = 4,
    #[doc = "< > **Deprecated** Identity (pass-through). > **Since** cuDNN 9.0.0"]
    CUDNN_ACTIVATION_IDENTITY = 5,
    #[doc = "< > **Deprecated** Swish activation. > **Since** cuDNN 9.0.0"]
    CUDNN_ACTIVATION_SWISH = 6,
}
#[repr(u32)]
#[doc = "Debug severity levels for cuDNN callback messages.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnSeverity_t {
    #[doc = "< Fatal error severity. > **Since** cuDNN 9.0.0"]
    CUDNN_SEV_FATAL = 0,
    #[doc = "< Error severity. > **Since** cuDNN 9.0.0"]
    CUDNN_SEV_ERROR = 1,
    #[doc = "< Warning severity. > **Since** cuDNN 9.0.0"]
    CUDNN_SEV_WARNING = 2,
    #[doc = "< Informational severity. > **Since** cuDNN 9.0.0"]
    CUDNN_SEV_INFO = 3,
}
#[doc = "Debug callback metadata containing version, status, timestamps, handle, stream, PID, TID, and device ID.\n> **Since** cuDNN 9.0.0"]
#[repr(C)]
pub struct cudnnDebugStruct {
    #[doc = "< cuDNN library version."]
    pub cudnn_version: ::core::ffi::c_uint,
    #[doc = "< Status code for this API call."]
    pub cudnnStatus: cudnnStatus_t,
    #[doc = "< Epoch time in seconds."]
    pub time_sec: ::core::ffi::c_uint,
    #[doc = "< Microseconds part of epoch time."]
    pub time_usec: ::core::ffi::c_uint,
    #[doc = "< Time since start in seconds."]
    pub time_delta: ::core::ffi::c_uint,
    #[doc = "< cuDNN handle."]
    pub handle: cudnnHandle_t,
    #[doc = "< CUDA stream ID."]
    pub stream: cudaStream_t,
    #[doc = "< Process ID."]
    pub pid: ::core::ffi::c_ulonglong,
    #[doc = "< Thread ID."]
    pub tid: ::core::ffi::c_ulonglong,
    #[doc = "< CUDA device ID."]
    pub cudaDeviceId: ::core::ffi::c_int,
    #[doc = "< Reserved for future use."]
    pub reserved: [::core::ffi::c_int; 15usize],
}
impl Default for cudnnDebugStruct {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "Debug callback metadata containing version, status, timestamps, handle, stream, PID, TID, and device ID.\n> **Since** cuDNN 9.0.0"]
pub type cudnnDebug_t = cudnnDebugStruct;
#[doc = "Callback function type for cuDNN debug messages.\n> **Since** cuDNN 9.0.0"]
pub type cudnnCallback_t = ::core::option::Option<unsafe extern "C" fn(sev: cudnnSeverity_t, udata: *mut ::core::ffi::c_void, dbg: *const cudnnDebug_t, msg: *const ::core::ffi::c_char)>;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Registers debug callback with message mask.\n\n# Arguments\n\n* `mask` [in]  -   Bitmask of severity levels to enable (see CUDNN_SEV_*_EN).\n* `udata` [in]  -  User data pointer passed to callback.\n* `fptr` [in]  -   Callback function pointer.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetCallback(mask: ::core::ffi::c_uint, udata: *mut ::core::ffi::c_void, fptr: cudnnCallback_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves registered debug callback and its configuration.\n\n# Arguments\n\n* `mask` [out]  -   Pointer to receive the current severity mask.\n* `udata` [out]  -  Pointer to receive the user data pointer.\n* `fptr` [out]  -   Pointer to receive the callback function pointer.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetCallback(mask: *mut ::core::ffi::c_uint, udata: *mut *mut ::core::ffi::c_void, fptr: *mut cudnnCallback_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Cross-library version checker.\nThis function is implemented differently in each sub-library. Each sublib\nchecks whether its own version matches that of its dependencies.\n@retval CUDNN_STATUS_SUCCESS if the version check passes.\n@retval CUDNN_STATUS_SUBLIBRARY_VERSION_MISMATCH if the versions are inconsistent.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGraphVersionCheck() -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Convolution operation modes.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnConvolutionMode_t {
    #[doc = "< Standard convolution. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION = 0,
    #[doc = "< Cross-correlation. > **Since** cuDNN 9.0.0"]
    CUDNN_CROSS_CORRELATION = 1,
}
#[repr(u32)]
#[doc = "Tensor reorder type.\n> **Deprecated** > **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnReorderType_t {
    #[doc = "< > **Deprecated** Default reordering behavior. > **Since** cuDNN 9.0.0"]
    CUDNN_DEFAULT_REORDER = 0,
    #[doc = "< > **Deprecated** No reordering. > **Since** cuDNN 9.0.0"]
    CUDNN_NO_REORDER = 1,
}
#[doc = "Opaque pointer to a cuDNN backend descriptor.\n> **Since** cuDNN 9.0.0"]
pub type cudnnBackendDescriptor_t = *mut ::core::ffi::c_void;
#[doc = "Integer fraction with numerator and denominator.\n> **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct cudnnFractionStruct {
    #[doc = "< Fraction numerator."]
    pub numerator: i64,
    #[doc = "< Fraction denominator."]
    pub denominator: i64,
}
#[doc = "Integer fraction with numerator and denominator.\n> **Since** cuDNN 9.0.0"]
pub type cudnnFraction_t = cudnnFractionStruct;
#[repr(u32)]
#[doc = "Pointwise operation modes including binary, unary, activation forward/backward,\ncomparison, logical, and special operations.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnPointwiseMode_t {
    #[doc = "< Element-wise addition. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_ADD = 0,
    #[doc = "< Element-wise add-and-square. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_ADD_SQUARE = 5,
    #[doc = "< Element-wise division. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_DIV = 6,
    #[doc = "< Element-wise maximum. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_MAX = 3,
    #[doc = "< Element-wise minimum. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_MIN = 2,
    #[doc = "< Element-wise modulo. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_MOD = 7,
    #[doc = "< Element-wise multiplication. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_MUL = 1,
    #[doc = "< Element-wise power. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_POW = 8,
    #[doc = "< Element-wise subtraction. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_SUB = 9,
    #[doc = "< Absolute value. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_ABS = 10,
    #[doc = "< Ceiling. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_CEIL = 11,
    #[doc = "< Cosine. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_COS = 12,
    #[doc = "< Exponential. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_EXP = 13,
    #[doc = "< Floor. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_FLOOR = 14,
    #[doc = "< Natural logarithm. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_LOG = 15,
    #[doc = "< Negation. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_NEG = 16,
    #[doc = "< Reciprocal square root. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_RSQRT = 17,
    #[doc = "< Sine. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_SIN = 18,
    #[doc = "< Square root. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_SQRT = 4,
    #[doc = "< Tangent. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_TAN = 19,
    #[doc = "< Error function. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_ERF = 20,
    #[doc = "< Identity (no-op); enables implicit data type conversion between tensors. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_IDENTITY = 21,
    #[doc = "< Reciprocal. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_RECIPROCAL = 22,
    #[doc = "< Two-argument arctangent. > **Since** cuDNN 9.1.0"]
    CUDNN_POINTWISE_ATAN2 = 23,
    #[doc = "< ReLU forward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_RELU_FWD = 100,
    #[doc = "< Tanh forward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_TANH_FWD = 101,
    #[doc = "< Sigmoid forward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_SIGMOID_FWD = 102,
    #[doc = "< ELU forward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_ELU_FWD = 103,
    #[doc = "< GELU forward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_GELU_FWD = 104,
    #[doc = "< Softplus forward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_SOFTPLUS_FWD = 105,
    #[doc = "< Swish forward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_SWISH_FWD = 106,
    #[doc = "< GELU forward using tanh approximation: 0.5*x*(1+tanh[sqrt(2/pi)*(x+0.044715*x^3)]). > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_GELU_APPROX_TANH_FWD = 107,
    #[doc = "< ReLU backward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_RELU_BWD = 200,
    #[doc = "< Tanh backward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_TANH_BWD = 201,
    #[doc = "< Sigmoid backward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_SIGMOID_BWD = 202,
    #[doc = "< ELU backward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_ELU_BWD = 203,
    #[doc = "< GELU backward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_GELU_BWD = 204,
    #[doc = "< Softplus backward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_SOFTPLUS_BWD = 205,
    #[doc = "< Swish backward. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_SWISH_BWD = 206,
    #[doc = "< GELU backward using tanh approximation. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_GELU_APPROX_TANH_BWD = 207,
    #[doc = "< Equal comparison. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_CMP_EQ = 300,
    #[doc = "< Not-equal comparison. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_CMP_NEQ = 301,
    #[doc = "< Greater-than comparison. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_CMP_GT = 302,
    #[doc = "< Greater-or-equal comparison. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_CMP_GE = 303,
    #[doc = "< Less-than comparison. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_CMP_LT = 304,
    #[doc = "< Less-or-equal comparison. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_CMP_LE = 305,
    #[doc = "< Logical AND. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_LOGICAL_AND = 400,
    #[doc = "< Logical OR. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_LOGICAL_OR = 401,
    #[doc = "< Logical NOT. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_LOGICAL_NOT = 402,
    #[doc = "< Generates a tensor of index values along a given axis. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_GEN_INDEX = 501,
    #[doc = "< Ternary select: y = predicate ? x : b, using three input tensors. > **Since** cuDNN 9.0.0"]
    CUDNN_POINTWISE_BINARY_SELECT = 601,
}
impl cudnnResampleMode_t {
    pub const CUDNN_RESAMPLE_AVGPOOL_INCLUDE_PADDING: cudnnResampleMode_t = cudnnResampleMode_t::CUDNN_RESAMPLE_AVGPOOL;
}
#[repr(u32)]
#[doc = "Resampling modes for pooling and interpolation.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnResampleMode_t {
    #[doc = "< Nearest-neighbor interpolation. > **Since** cuDNN 9.0.0"]
    CUDNN_RESAMPLE_NEAREST = 0,
    #[doc = "< Bilinear interpolation. > **Since** cuDNN 9.0.0"]
    CUDNN_RESAMPLE_BILINEAR = 1,
    #[doc = "< Average pooling (include padding). > **Since** cuDNN 9.0.0"]
    CUDNN_RESAMPLE_AVGPOOL = 2,
    #[doc = "< Average pooling excluding padding from divisor. > **Since** cuDNN 9.0.0"]
    CUDNN_RESAMPLE_AVGPOOL_EXCLUDE_PADDING = 4,
    #[doc = "< Max pooling. > **Since** cuDNN 9.0.0"]
    CUDNN_RESAMPLE_MAXPOOL = 3,
}
#[repr(u32)]
#[doc = "Signal synchronization modes.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnSignalMode_t {
    #[doc = "< Set signal. > **Since** cuDNN 9.0.0"]
    CUDNN_SIGNAL_SET = 0,
    #[doc = "< Wait on signal. > **Since** cuDNN 9.0.0"]
    CUDNN_SIGNAL_WAIT = 1,
}
#[repr(u32)]
#[doc = "Statistics generation mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnGenStatsMode_t {
    #[doc = "< Generate sum and sum-of-squares statistics. > **Since** cuDNN 9.0.0"]
    CUDNN_GENSTATS_SUM_SQSUM = 0,
}
#[repr(u32)]
#[doc = "Batch normalization finalize statistics mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBnFinalizeStatsMode_t {
    #[doc = "< Training mode finalization. > **Since** cuDNN 9.0.0"]
    CUDNN_BN_FINALIZE_STATISTICS_TRAINING = 0,
    #[doc = "< Inference mode finalization. > **Since** cuDNN 9.0.0"]
    CUDNN_BN_FINALIZE_STATISTICS_INFERENCE = 1,
}
#[repr(u32)]
#[doc = "Random number generator distribution types.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnRngDistribution_t {
    #[doc = "< Bernoulli distribution. > **Since** cuDNN 9.0.0"]
    CUDNN_RNG_DISTRIBUTION_BERNOULLI = 0,
    #[doc = "< Uniform distribution. > **Since** cuDNN 9.0.0"]
    CUDNN_RNG_DISTRIBUTION_UNIFORM = 1,
    #[doc = "< Normal (Gaussian) distribution. > **Since** cuDNN 9.0.0"]
    CUDNN_RNG_DISTRIBUTION_NORMAL = 2,
}
#[repr(u32)]
#[doc = "Mixture-of-Experts grouped matmul modes.\n> **Since** cuDNN 9.15.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnMoeGroupedMatmulMode_t {
    #[doc = "< No gather/scatter. > **Since** cuDNN 9.15.0"]
    CUDNN_MOE_GROUPED_MATMUL_MODE_NONE = 0,
    #[doc = "< Gather mode. > **Since** cuDNN 9.15.0"]
    CUDNN_MOE_GROUPED_MATMUL_MODE_GATHER = 1,
    #[doc = "< Scatter mode. > **Since** cuDNN 9.15.0"]
    CUDNN_MOE_GROUPED_MATMUL_MODE_SCATTER = 2,
}
#[repr(u32)]
#[doc = "Backend attribute names for configuring and querying backend descriptors.\nAttribute names are grouped by descriptor type and numeric range:\n- 0-9: Pointwise attributes\n- 100-106: Convolution attributes\n- 200-204: Engine heuristic attributes\n- 300-304: Engine config attributes\n- 400-407: Execution plan attributes\n- 500-503: Intermediate info attributes\n- 600-601: Knob choice attributes\n- 700-717: Operation convolution attributes\n- 750-758: Operation pointwise attributes\n- 770-796: Operation gen-stats and BN finalize attributes\n- 800-804: Operation graph attributes\n- 900-913: Tensor attributes\n- 1000-1012: Variant pack attributes\n- 1100+: Layout info, knob info, engine, matmul, reduction, resample, etc.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendAttributeName_t {
    #[doc = "< Pointwise operation type. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_MODE = 0,
    #[doc = "< Computation precision for pointwise ops. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_MATH_PREC = 1,
    #[doc = "< NaN handling behavior. > **Deprecated** > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_NAN_PROPAGATION = 2,
    #[doc = "< Lower clipping threshold for ReLU. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_RELU_LOWER_CLIP = 3,
    #[doc = "< Upper clipping threshold for ReLU. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_RELU_UPPER_CLIP = 4,
    #[doc = "< Slope below lower clip for leaky ReLU. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_RELU_LOWER_CLIP_SLOPE = 5,
    #[doc = "< Alpha parameter for ELU activation. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_ELU_ALPHA = 6,
    #[doc = "< Beta parameter for softplus function. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_SOFTPLUS_BETA = 7,
    #[doc = "< Beta parameter for swish activation. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_SWISH_BETA = 8,
    #[doc = "< Axis for axis-dependent pointwise operations. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_POINTWISE_AXIS = 9,
    #[doc = "< Computation data type for convolution. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_CONVOLUTION_COMP_TYPE = 100,
    #[doc = "< Convolution vs cross-correlation mode. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_CONVOLUTION_CONV_MODE = 101,
    #[doc = "< Dilation factors per spatial dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_CONVOLUTION_DILATIONS = 102,
    #[doc = "< Filter stride values per spatial dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_CONVOLUTION_FILTER_STRIDES = 103,
    #[doc = "< Post-paddings per spatial dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_CONVOLUTION_POST_PADDINGS = 104,
    #[doc = "< Pre-paddings per spatial dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_CONVOLUTION_PRE_PADDINGS = 105,
    #[doc = "< Number of spatial dimensions. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_CONVOLUTION_SPATIAL_DIMS = 106,
    #[doc = "< Heuristic algorithm selection mode. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINEHEUR_MODE = 200,
    #[doc = "< Associated operation graph descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINEHEUR_OPERATION_GRAPH = 201,
    #[doc = "< Array of resulting engine configurations. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINEHEUR_RESULTS = 202,
    #[doc = "< Target streaming multiprocessor count. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINEHEUR_SM_COUNT_TARGET = 203,
    #[doc = "< Device properties for heuristic query. > **Since** cuDNN 9.8.0"]
    CUDNN_ATTR_ENGINEHEUR_DEVICEPROP = 204,
    #[doc = "< Selected engine from heuristic results. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINECFG_ENGINE = 300,
    #[doc = "< Intermediate tensor information. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINECFG_INTERMEDIATE_INFO = 301,
    #[doc = "< Performance tuning knob selections. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINECFG_KNOB_CHOICES = 302,
    #[doc = "< Required workspace memory size. > **Since** cuDNN 9.2.0"]
    CUDNN_ATTR_ENGINECFG_WORKSPACE_SIZE = 303,
    #[doc = "< Shared memory used by engine. > **Since** cuDNN 9.2.0"]
    CUDNN_ATTR_ENGINECFG_SHARED_MEMORY_USED = 304,
    #[doc = "< Associated cuDNN handle. > **Deprecated** > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_EXECUTION_PLAN_HANDLE = 400,
    #[doc = "< Engine configuration to execute. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_EXECUTION_PLAN_ENGINE_CONFIG = 401,
    #[doc = "< Total workspace size requirement. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_EXECUTION_PLAN_WORKSPACE_SIZE = 402,
    #[doc = "< UIDs of computed intermediates. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_EXECUTION_PLAN_COMPUTED_INTERMEDIATE_UIDS = 403,
    #[doc = "< Run-only intermediate UIDs. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_EXECUTION_PLAN_RUN_ONLY_INTERMEDIATE_UIDS = 404,
    #[doc = "< Human-readable execution plan JSON. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_EXECUTION_PLAN_JSON_REPRESENTATION = 405,
    #[doc = "< Compiled kernel cache descriptor. > **Since** cuDNN 9.4.0"]
    CUDNN_ATTR_EXECUTION_PLAN_KERNEL_CACHE = 406,
    #[doc = "< Device properties for execution plan. > **Since** cuDNN 9.8.0"]
    CUDNN_ATTR_EXECUTION_PLAN_DEVICEPROP = 407,
    #[doc = "< Unique identifier for intermediate tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_INTERMEDIATE_INFO_UNIQUE_ID = 500,
    #[doc = "< Memory size requirement in bytes. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_INTERMEDIATE_INFO_SIZE = 501,
    #[doc = "< Data dependency UIDs. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_INTERMEDIATE_INFO_DEPENDENT_DATA_UIDS = 502,
    #[doc = "< Attribute dependencies. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_INTERMEDIATE_INFO_DEPENDENT_ATTRIBUTES = 503,
    #[doc = "< Type of performance tuning knob. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_KNOB_CHOICE_KNOB_TYPE = 600,
    #[doc = "< Selected value for the knob. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_KNOB_CHOICE_KNOB_VALUE = 601,
    #[doc = "< Forward convolution scaling factor alpha. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_ALPHA = 700,
    #[doc = "< Forward convolution scaling factor beta. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_BETA = 701,
    #[doc = "< Forward convolution descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_CONV_DESC = 702,
    #[doc = "< Forward convolution filter weight tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_W = 703,
    #[doc = "< Forward convolution input tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_X = 704,
    #[doc = "< Forward convolution output tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_Y = 705,
    #[doc = "< Backward data scaling factor alpha. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_ALPHA = 706,
    #[doc = "< Backward data scaling factor beta. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_BETA = 707,
    #[doc = "< Backward data convolution descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_CONV_DESC = 708,
    #[doc = "< Backward data filter weights. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_W = 709,
    #[doc = "< Backward data input gradient output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DX = 710,
    #[doc = "< Backward data output gradient input. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DY = 711,
    #[doc = "< Backward filter scaling factor alpha. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_ALPHA = 712,
    #[doc = "< Backward filter scaling factor beta. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_BETA = 713,
    #[doc = "< Backward filter convolution descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_CONV_DESC = 714,
    #[doc = "< Backward filter gradient output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DW = 715,
    #[doc = "< Backward filter input feature maps. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_X = 716,
    #[doc = "< Backward filter output gradients. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DY = 717,
    #[doc = "< Pointwise descriptor reference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_POINTWISE_PW_DESCRIPTOR = 750,
    #[doc = "< First input tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_POINTWISE_XDESC = 751,
    #[doc = "< Bias or second operand descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_POINTWISE_BDESC = 752,
    #[doc = "< Output tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_POINTWISE_YDESC = 753,
    #[doc = "< First scaling constant. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_POINTWISE_ALPHA1 = 754,
    #[doc = "< Second scaling constant. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_POINTWISE_ALPHA2 = 755,
    #[doc = "< Input gradient descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_POINTWISE_DXDESC = 756,
    #[doc = "< Output gradient descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_POINTWISE_DYDESC = 757,
    #[doc = "< Intermediate tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_POINTWISE_TDESC = 758,
    #[doc = "< Statistics computation mode. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_GENSTATS_MODE = 770,
    #[doc = "< Computation precision for statistics. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_GENSTATS_MATH_PREC = 771,
    #[doc = "< Input data tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_GENSTATS_XDESC = 772,
    #[doc = "< Sum output descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_GENSTATS_SUMDESC = 773,
    #[doc = "< Sum of squares output descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_GENSTATS_SQSUMDESC = 774,
    #[doc = "< Training vs inference mode. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_STATS_MODE = 780,
    #[doc = "< Computation precision. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_MATH_PREC = 781,
    #[doc = "< Sum of batch norm outputs. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_Y_SUM_DESC = 782,
    #[doc = "< Sum of squared outputs. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_Y_SQ_SUM_DESC = 783,
    #[doc = "< Batch norm scale parameter. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_SCALE_DESC = 784,
    #[doc = "< Batch norm bias parameter. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_BIAS_DESC = 785,
    #[doc = "< Previous running mean. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_PREV_RUNNING_MEAN_DESC = 786,
    #[doc = "< Previous running variance. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_PREV_RUNNING_VAR_DESC = 787,
    #[doc = "< Updated running mean output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_UPDATED_RUNNING_MEAN_DESC = 788,
    #[doc = "< Updated running variance output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_UPDATED_RUNNING_VAR_DESC = 789,
    #[doc = "< Cached mean for backward pass. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_SAVED_MEAN_DESC = 790,
    #[doc = "< Cached inverse std dev for backward. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_SAVED_INV_STD_DESC = 791,
    #[doc = "< Equivalent scale for fused inference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_EQ_SCALE_DESC = 792,
    #[doc = "< Equivalent bias for fused inference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_EQ_BIAS_DESC = 793,
    #[doc = "< Accumulation sample counter. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_ACCUM_COUNT_DESC = 794,
    #[doc = "< Numerical stability epsilon. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_EPSILON_DESC = 795,
    #[doc = "< Exponential averaging factor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_FINALIZE_EXP_AVERATE_FACTOR_DESC = 796,
    #[doc = "< Associated cuDNN handle. > **Deprecated** > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATIONGRAPH_HANDLE = 800,
    #[doc = "< Array of operation descriptors in the graph. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATIONGRAPH_OPS = 801,
    #[doc = "< Total number of engines available. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATIONGRAPH_ENGINE_GLOBAL_COUNT = 802,
    #[doc = "< Dynamic shape support flag. > **Since** cuDNN 9.4.0"]
    CUDNN_ATTR_OPERATIONGRAPH_IS_DYNAMIC_SHAPE_ENABLED = 803,
    #[doc = "< Same topology reuse flag. > **Since** cuDNN 9.6.0"]
    CUDNN_ATTR_OPERATIONGRAPH_IS_SAME_TOPOLOGY = 804,
    #[doc = "< Dynamic shape support with execute time override > **Since** cuDNN 9.21.0"]
    CUDNN_ATTR_OPERATIONGRAPH_IS_OVERRIDE_SHAPE_ENABLED = 805,
    #[doc = "< Memory alignment requirement in bytes. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_BYTE_ALIGNMENT = 900,
    #[doc = "< Element data type. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_DATA_TYPE = 901,
    #[doc = "< Dimension sizes array. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_DIMENSIONS = 902,
    #[doc = "< Memory strides per dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_STRIDES = 903,
    #[doc = "< Vectorization element count. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_VECTOR_COUNT = 904,
    #[doc = "< Which dimension is vectorized. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_VECTORIZED_DIMENSION = 905,
    #[doc = "< Unique identifier for graph connectivity. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_UNIQUE_ID = 906,
    #[doc = "< Virtual (intermediate) vs I/O tensor flag. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_IS_VIRTUAL = 907,
    #[doc = "< Constant scalar vs device pointer flag. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_IS_BY_VALUE = 908,
    #[doc = "< Memory layout transformation type. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_REORDERING_MODE = 909,
    #[doc = "< Compile-time constant value for by-value tensors. Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_TENSOR_CONSTANT_VALUE = 910,
    #[doc = "< Ragged tensor offset descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_TENSOR_RAGGED_OFFSET_DESC = 913,
    #[doc = "< Tensor UIDs in this variant pack. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_VARIANT_PACK_UNIQUE_IDS = 1000,
    #[doc = "< GPU memory data pointers array. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_VARIANT_PACK_DATA_POINTERS = 1001,
    #[doc = "< Intermediate tensor pointers. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_VARIANT_PACK_INTERMEDIATES = 1002,
    #[doc = "< Workspace memory pointer. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_VARIANT_PACK_WORKSPACE = 1003,
    #[doc = "< Override tensor UIDs for dynamic shapes. > **Since** cuDNN 9.18.0"]
    CUDNN_ATTR_VARIANT_PACK_OVERRIDE_UNIQUE_IDS = 1010,
    #[doc = "< Override shapes for dynamic shapes. > **Since** cuDNN 9.18.0"]
    CUDNN_ATTR_VARIANT_PACK_OVERRIDE_SHAPES = 1011,
    #[doc = "< Override strides for dynamic shapes. > **Since** cuDNN 9.18.0"]
    CUDNN_ATTR_VARIANT_PACK_OVERRIDE_STRIDES = 1012,
    #[doc = "< Associated tensor identifier. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_LAYOUT_INFO_TENSOR_UID = 1100,
    #[doc = "< Available memory layout types. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_LAYOUT_INFO_TYPES = 1101,
    #[doc = "< Knob type being described. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_KNOB_INFO_TYPE = 1200,
    #[doc = "< Upper bound for knob value. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_KNOB_INFO_MAXIMUM_VALUE = 1201,
    #[doc = "< Lower bound for knob value. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_KNOB_INFO_MINIMUM_VALUE = 1202,
    #[doc = "< Valid increment between knob values. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_KNOB_INFO_STRIDE = 1203,
    #[doc = "< Operation graph this engine processes. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINE_OPERATION_GRAPH = 1300,
    #[doc = "< Engine index in the global engine list. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINE_GLOBAL_INDEX = 1301,
    #[doc = "< Available knob configuration options. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINE_KNOB_INFO = 1302,
    #[doc = "< Numerical properties (tensor cores, precision). > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINE_NUMERICAL_NOTE = 1303,
    #[doc = "< Preferred tensor memory layouts. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINE_LAYOUT_INFO = 1304,
    #[doc = "< Runtime behavior characteristics. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINE_BEHAVIOR_NOTE = 1305,
    #[doc = "< Streaming multiprocessor target count. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_ENGINE_SM_COUNT_TARGET = 1306,
    #[doc = "< Device properties descriptor. > **Since** cuDNN 9.8.0"]
    CUDNN_ATTR_ENGINE_DEVICEPROP = 1307,
    #[doc = "< Disable cluster cooperative kernels. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_ENGINE_DISABLE_CLUSTER_COOPERATIVE = 1308,
    #[doc = "< Computation precision type for matmul. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_MATMUL_COMP_TYPE = 1500,
    #[doc = "< Padding value for incomplete blocks. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_MATMUL_PADDING_VALUE = 1503,
    #[doc = "< First input matrix (A) tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_MATMUL_ADESC = 1520,
    #[doc = "< Second input matrix (B) tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_MATMUL_BDESC = 1521,
    #[doc = "< Output matrix (C) tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_MATMUL_CDESC = 1522,
    #[doc = "< MatMul operation configuration descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_MATMUL_DESC = 1523,
    #[doc = "< Irregular batch count. > **Deprecated** > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_MATMUL_IRREGULARLY_STRIDED_BATCH_COUNT = 1524,
    #[doc = "< Override for output rows (M). > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_MATMUL_GEMM_M_OVERRIDE_DESC = 1525,
    #[doc = "< Override for output columns (N). > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_MATMUL_GEMM_N_OVERRIDE_DESC = 1526,
    #[doc = "< Override for contraction dim (K). > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_MATMUL_GEMM_K_OVERRIDE_DESC = 1527,
    #[doc = "< Reduction operation type (ADD, MUL, MIN, etc.). > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_REDUCTION_OPERATOR = 1600,
    #[doc = "< Computation data type for reduction. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_REDUCTION_COMP_TYPE = 1601,
    #[doc = "< Whether reduction must be deterministic. > **Since** cuDNN 9.11.0"]
    CUDNN_ATTR_REDUCTION_IS_DETERMINISTIC = 1602,
    #[doc = "< Reduction input tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_REDUCTION_XDESC = 1610,
    #[doc = "< Reduction output tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_REDUCTION_YDESC = 1611,
    #[doc = "< Reduction descriptor reference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_REDUCTION_DESC = 1612,
    #[doc = "< Computation precision. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_MATH_PREC = 1620,
    #[doc = "< Cached batch mean from forward. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_MEAN_DESC = 1621,
    #[doc = "< Cached inverse std dev from forward. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_INVSTD_DESC = 1622,
    #[doc = "< Batch norm scale parameter. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_BN_SCALE_DESC = 1623,
    #[doc = "< Forward input tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_X_DESC = 1624,
    #[doc = "< Output gradient tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DY_DESC = 1625,
    #[doc = "< Scale gradient output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DBN_SCALE_DESC = 1626,
    #[doc = "< Bias gradient output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DBN_BIAS_DESC = 1627,
    #[doc = "< Equivalent output gradient scale. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_DY_SCALE_DESC = 1628,
    #[doc = "< Equivalent input scale. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_X_SCALE_DESC = 1629,
    #[doc = "< Equivalent bias value. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_BIAS = 1630,
    #[doc = "< Resampling interpolation method. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RESAMPLE_MODE = 1700,
    #[doc = "< Computation precision for resampling. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RESAMPLE_COMP_TYPE = 1701,
    #[doc = "< Number of spatial dimensions. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RESAMPLE_SPATIAL_DIMS = 1702,
    #[doc = "< Post-paddings per spatial dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RESAMPLE_POST_PADDINGS = 1703,
    #[doc = "< Pre-paddings per spatial dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RESAMPLE_PRE_PADDINGS = 1704,
    #[doc = "< Stride factors per spatial dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RESAMPLE_STRIDES = 1705,
    #[doc = "< Filter window sizes per dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RESAMPLE_WINDOW_DIMS = 1706,
    #[doc = "< NaN handling behavior. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RESAMPLE_NAN_PROPAGATION = 1707,
    #[doc = "< Padding strategy (zero, neg_inf, edge). > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RESAMPLE_PADDING_MODE = 1708,
    #[doc = "< Forward resample input tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_FWD_XDESC = 1710,
    #[doc = "< Forward resample output tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_FWD_YDESC = 1711,
    #[doc = "< Max pooling index tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_FWD_IDXDESC = 1712,
    #[doc = "< Output scaling factor. > **Deprecated** > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_FWD_ALPHA = 1713,
    #[doc = "< Accumulation scaling factor. > **Deprecated** > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_FWD_BETA = 1714,
    #[doc = "< Resample descriptor reference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_FWD_DESC = 1716,
    #[doc = "< Backward resample input gradient. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DXDESC = 1720,
    #[doc = "< Backward resample output gradient. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DYDESC = 1721,
    #[doc = "< Index tensor from forward pass. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_BWD_IDXDESC = 1722,
    #[doc = "< Gradient scaling factor. > **Deprecated** > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_BWD_ALPHA = 1723,
    #[doc = "< Accumulation scaling. > **Deprecated** > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_BWD_BETA = 1724,
    #[doc = "< Resample descriptor reference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DESC = 1725,
    #[doc = "< Forward input reference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_BWD_XDESC = 1726,
    #[doc = "< Forward output reference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESAMPLE_BWD_YDESC = 1727,
    #[doc = "< Concatenation axis dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONCAT_AXIS = 1800,
    #[doc = "< Array of input tensor descriptors. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONCAT_INPUT_DESCS = 1801,
    #[doc = "< In-place output tensor selection index. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONCAT_INPLACE_INDEX = 1802,
    #[doc = "< Concatenated output descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_CONCAT_OUTPUT_DESC = 1803,
    #[doc = "< Signal set vs wait mode. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_SIGNAL_MODE = 1900,
    #[doc = "< Flag variable tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_SIGNAL_FLAGDESC = 1901,
    #[doc = "< Signal value for comparison. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_SIGNAL_VALUE = 1902,
    #[doc = "< Input tensor for signal set. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_SIGNAL_XDESC = 1903,
    #[doc = "< Output tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_SIGNAL_YDESC = 1904,
    #[doc = "< Cache container descriptor. > **Since** cuDNN 9.4.0"]
    CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_CONTAINER_DESC = 1950,
    #[doc = "< Output tensor descriptor. > **Since** cuDNN 9.4.0"]
    CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_YDESC = 1951,
    #[doc = "< Load sequence specification. > **Since** cuDNN 9.4.0"]
    CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_SEQUENCE_DESC = 1952,
    #[doc = "< Page table mapping descriptor. > **Since** cuDNN 9.4.0"]
    CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_PAGE_TABLE_DESC = 1953,
    #[doc = "< Normalization algorithm type. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_MODE = 2000,
    #[doc = "< Training vs inference phase. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_PHASE = 2001,
    #[doc = "< Input tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_XDESC = 2002,
    #[doc = "< Computed or cached mean tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_MEAN_DESC = 2003,
    #[doc = "< Computed or cached inverse variance. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_INV_VARIANCE_DESC = 2004,
    #[doc = "< Learnable scale parameter (gamma). > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_SCALE_DESC = 2005,
    #[doc = "< Learnable bias parameter (beta). > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_BIAS_DESC = 2006,
    #[doc = "< Numerical stability constant. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_EPSILON_DESC = 2007,
    #[doc = "< Momentum for running statistics. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_EXP_AVG_FACTOR_DESC = 2008,
    #[doc = "< Input running mean. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_INPUT_RUNNING_MEAN_DESC = 2009,
    #[doc = "< Input running variance. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_INPUT_RUNNING_VAR_DESC = 2010,
    #[doc = "< Updated running mean output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_OUTPUT_RUNNING_MEAN_DESC = 2011,
    #[doc = "< Updated running variance output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_OUTPUT_RUNNING_VAR_DESC = 2012,
    #[doc = "< Output tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_YDESC = 2013,
    #[doc = "< Peer statistics for multi-GPU sync. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_FWD_PEER_STAT_DESCS = 2014,
    #[doc = "< Normalization algorithm type. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_MODE = 2100,
    #[doc = "< Forward input tensor reference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_XDESC = 2101,
    #[doc = "< Cached mean from forward pass. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_MEAN_DESC = 2102,
    #[doc = "< Cached inverse std dev. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_INV_VARIANCE_DESC = 2103,
    #[doc = "< Output gradient tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_DYDESC = 2104,
    #[doc = "< Forward scale parameter. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_SCALE_DESC = 2105,
    #[doc = "< Numerical stability epsilon. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_EPSILON_DESC = 2106,
    #[doc = "< Scale gradient output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_DSCALE_DESC = 2107,
    #[doc = "< Bias gradient output. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_DBIAS_DESC = 2108,
    #[doc = "< Input gradient output tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_DXDESC = 2109,
    #[doc = "< Peer gradient statistics for multi-GPU. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_NORM_BWD_PEER_STAT_DESCS = 2110,
    #[doc = "< Reshape input tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESHAPE_XDESC = 2200,
    #[doc = "< Reshape output tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RESHAPE_YDESC = 2201,
    #[doc = "< Reshape mode (view-only or logical). Logical mode needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_RESHAPE_MODE = 2202,
    #[doc = "< Transpose input tensor descriptor. Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_TRANSPOSE_XDESC = 3200,
    #[doc = "< Transpose output tensor descriptor. Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_TRANSPOSE_YDESC = 3201,
    #[doc = "< Transpose permutation array. Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_TRANSPOSE_PERMUTATION = 3202,
    #[doc = "< Slice input tensor descriptor. Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_SLICE_XDESC = 3300,
    #[doc = "< Slice output tensor descriptor. Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_SLICE_YDESC = 3301,
    #[doc = "< Slice start indices. Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_SLICE_START_INDICES = 3302,
    #[doc = "< Slice limit indices. Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_SLICE_LIMIT_INDICES = 3303,
    #[doc = "< Slice strides. Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_SLICE_STRIDES = 3304,
    #[doc = "< Band matrix input tensor. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_XDESC = 2250,
    #[doc = "< Expanded output tensor. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_YDESC = 2251,
    #[doc = "< Lower bandwidth of the band. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_LOWER_BANDWIDTH = 2252,
    #[doc = "< Upper bandwidth of the band. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_UPPER_BANDWIDTH = 2253,
    #[doc = "< Axis along which to expand. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_AXIS = 2254,
    #[doc = "< Padding value outside the band. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_PAD_VALUE = 2255,
    #[doc = "< KV token offset descriptor. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_KV_TOKEN_OFFSET_DESC = 2256,
    #[doc = "< Speculative decoding mask. > **Since** cuDNN 9.13.0"]
    CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_SPECULATIVE_MASK_DESC = 2257,
    #[doc = "< Full matrix input tensor. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_XDESC = 2270,
    #[doc = "< Contracted band output tensor. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_YDESC = 2271,
    #[doc = "< Lower bandwidth. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_LOWER_BANDWIDTH = 2272,
    #[doc = "< Upper bandwidth. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_UPPER_BANDWIDTH = 2273,
    #[doc = "< Axis along which to contract. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_AXIS = 2274,
    #[doc = "< Padding value. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_PAD_VALUE = 2275,
    #[doc = "< Maximum token value for contraction. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_OPERATION_CONTRACT_BAND_MAX_TOKEN_VALUE = 2276,
    #[doc = "< Random distribution type selection. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RNG_DISTRIBUTION = 2300,
    #[doc = "< Normal distribution mean parameter. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RNG_NORMAL_DIST_MEAN = 2301,
    #[doc = "< Normal distribution std deviation. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RNG_NORMAL_DIST_STANDARD_DEVIATION = 2302,
    #[doc = "< Uniform distribution upper bound. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RNG_UNIFORM_DIST_MAXIMUM = 2303,
    #[doc = "< Uniform distribution lower bound. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RNG_UNIFORM_DIST_MINIMUM = 2304,
    #[doc = "< Bernoulli probability of 1. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_RNG_BERNOULLI_DIST_PROBABILITY = 2305,
    #[doc = "< RNG output tensor descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RNG_YDESC = 2310,
    #[doc = "< RNG seed value. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RNG_SEED = 2311,
    #[doc = "< RNG descriptor reference. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RNG_DESC = 2312,
    #[doc = "< RNG offset/state descriptor. > **Since** cuDNN 9.0.0"]
    CUDNN_ATTR_OPERATION_RNG_OFFSET_DESC = 2313,
    #[doc = "< Operation graph for kernel cache. > **Since** cuDNN 9.5.0"]
    CUDNN_ATTR_KERNEL_CACHE_OPERATION_GRAPH = 2400,
    #[doc = "< Whether kernel is cached. > **Since** cuDNN 9.4.0"]
    CUDNN_ATTR_KERNEL_CACHE_IS_ENGINECFG_KERNEL_CACHED = 2401,
    #[doc = "< Kernel cache JSON serialization. > **Since** cuDNN 9.10.0"]
    CUDNN_ATTR_KERNEL_CACHE_JSON_REPRESENTATION = 2402,
    #[doc = "< Input float tensor to quantize. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_XDESC = 2500,
    #[doc = "< Quantized output tensor. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_YDESC = 2501,
    #[doc = "< Per-block scaling factors output. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_SCALE_DESC = 2502,
    #[doc = "< Computation precision. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_MATH_PREC = 2503,
    #[doc = "< Quantization block size. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_BLOCK_SIZE = 2504,
    #[doc = "< Quantized input tensor. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_XDESC = 2600,
    #[doc = "< Per-block scale factors. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_SCALE_DESC = 2601,
    #[doc = "< Dequantized output tensor. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_YDESC = 2602,
    #[doc = "< Computation precision. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_MATH_PREC = 2603,
    #[doc = "< Dequantization block size. > **Since** cuDNN 9.7.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_BLOCK_SIZE = 2604,
    #[doc = "< Negative scale handling. > **Since** cuDNN 9.13.0"]
    CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_NEG_SCALE = 2605,
    #[doc = "< CUDA device identifier. > **Since** cuDNN 9.8.0"]
    CUDNN_ATTR_DEVICEPROP_DEVICE_ID = 2700,
    #[doc = "< Associated cuDNN handle. > **Since** cuDNN 9.8.0"]
    CUDNN_ATTR_DEVICEPROP_HANDLE = 2701,
    #[doc = "< Device properties JSON. > **Since** cuDNN 9.8.0"]
    CUDNN_ATTR_DEVICEPROP_JSON_REPRESENTATION = 2702,
    #[doc = "< Query tensor descriptor. > **Since** cuDNN 9.13.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_QDESC = 2800,
    #[doc = "< Key tensor descriptor. > **Since** cuDNN 9.13.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_KDESC = 2801,
    #[doc = "< Value tensor descriptor. > **Since** cuDNN 9.13.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_VDESC = 2802,
    #[doc = "< Output tensor descriptor. > **Since** cuDNN 9.13.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_ODESC = 2803,
    #[doc = "< Statistics output descriptor. > **Since** cuDNN 9.13.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_STATSDESC = 2804,
    #[doc = "< Attention scaling factor descriptor. > **Since** cuDNN 9.13.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_SCALEDESC = 2805,
    #[doc = "< Block-sparse attention mask. > **Since** cuDNN 9.14.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_BLOCK_MASK_DESC = 2806,
    #[doc = "< Paged attention key page table. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_PAGE_TABLE_KDESC = 2807,
    #[doc = "< Paged attention value page table. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_PAGE_TABLE_VDESC = 2808,
    #[doc = "< Query sequence length tensor. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_SEQ_LEN_QDESC = 2809,
    #[doc = "< Key-value sequence length tensor. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_SEQ_LEN_KVDESC = 2810,
    #[doc = "< Forward SDPA subgraph. > **Since** cuDNN 9.21.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_SUBGRAPH = 2811,
    #[doc = "< Subgraph input tensor UID. > **Since** cuDNN 9.21.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_SUBGRAPH_INPUT_UID = 2812,
    #[doc = "< Subgraph output tensor UID. > **Since** cuDNN 9.21.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_SUBGRAPH_OUTPUT_UID = 2813,
    #[doc = "< Softmax descriptor. > **Since** cuDNN 9.21.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_SOFTMAX_DESC = 2814,
    #[doc = "< Dropout seed tensor descriptor. > **Since** cuDNN 9.21.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_DROPOUT_SEED_DESC = 2815,
    #[doc = "< Dropout offset tensor descriptor. > **Since** cuDNN 9.21.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_DROPOUT_OFFSET_DESC = 2816,
    #[doc = "< Dropout RNG dump tensor descriptor. > **Since** cuDNN 9.21.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_DROPOUT_RNG_DUMP_DESC = 2817,
    #[doc = "< Dropout probability. > **Since** cuDNN 9.21.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_DROPOUT_PROBABILITY = 2818,
    #[doc = "< Unfuse FMA in softmax for SM100. > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_SDPA_FWD_UNFUSE_FMA = 2819,
    #[doc = "< Query tensor descriptor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_QDESC = 2851,
    #[doc = "< Key tensor descriptor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_KDESC = 2852,
    #[doc = "< Value tensor descriptor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_VDESC = 2853,
    #[doc = "< Forward output tensor descriptor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_ODESC = 2854,
    #[doc = "< Forward statistics descriptor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_STATSDESC = 2855,
    #[doc = "< Attention scaling factor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_SCALEDESC = 2856,
    #[doc = "< Query sequence length tensor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_SEQ_LEN_QDESC = 2857,
    #[doc = "< Key-value sequence length tensor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_SEQ_LEN_KVDESC = 2858,
    #[doc = "< Query gradient output tensor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_DQDESC = 2859,
    #[doc = "< Key gradient output tensor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_DKDESC = 2860,
    #[doc = "< Value gradient output tensor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_DVDESC = 2861,
    #[doc = "< Output gradient input tensor. > **Since** cuDNN 9.17.0"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_DODDESC = 2862,
    #[doc = "< Backward sink descriptor. > **Since** UNPUBLISHED"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_SINK_DESC = 2863,
    #[doc = "< Backward sink gradient descriptor. > **Since** UNPUBLISHED"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_DSINK_DESC = 2864,
    #[doc = "< Max total query sequence length. > **Since** UNPUBLISHED"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_MAX_TOTAL_SEQ_LEN_Q = 2865,
    #[doc = "< Max total KV sequence length. > **Since** UNPUBLISHED"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_MAX_TOTAL_SEQ_LEN_KV = 2866,
    #[doc = "< Backward SDPA subgraph. > **Since** UNPUBLISHED"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_SUBGRAPH = 2867,
    #[doc = "< Subgraph input tensor UID. > **Since** UNPUBLISHED"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_SUBGRAPH_INPUT_UID = 2868,
    #[doc = "< Subgraph output tensor UID. > **Since** UNPUBLISHED"]
    CUDNN_ATTR_OPERATION_SDPA_BWD_SUBGRAPH_OUTPUT_UID = 2869,
    #[doc = "< Gather/scatter mode for MoE. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_MODE = 2900,
    #[doc = "< Computation precision. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_MATH_PREC = 2901,
    #[doc = "< Token tensor descriptor. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_TOKEN_DESC = 2902,
    #[doc = "< Expert weight tensor descriptor. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_WEIGHT_DESC = 2903,
    #[doc = "< First token offset per expert. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_FIRST_TOKEN_OFFSET_DESC = 2904,
    #[doc = "< Output tensor descriptor. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_OUTPUT_DESC = 2905,
    #[doc = "< Token routing index descriptor. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_TOKEN_INDEX_DESC = 2906,
    #[doc = "< Token routing weights descriptor. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_TOKEN_KS_DESC = 2907,
    #[doc = "< Top-K experts per token. > **Since** cuDNN 9.15.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_TOP_K = 2908,
    #[doc = "< Backward math precision for MoE grouped matmul. > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_BWD_MATH_PREC = 2951,
    #[doc = "< Backward token descriptor for MoE grouped matmul. > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_BWD_TOKEN_DESC = 2952,
    #[doc = "< Backward weight descriptor for MoE grouped matmul. > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_BWD_DWEIGHT_DESC = 2953,
    #[doc = "< Backward first token offset descriptor for MoE grouped matmul. > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_BWD_FIRST_TOKEN_OFFSET_DESC = 2954,
    #[doc = "< Backward output descriptor for MoE grouped matmul. > **Since** cuDNN 9.22.0"]
    CUDNN_ATTR_OPERATION_MOE_GROUPED_MATMUL_BWD_DOUTPUT_DESC = 2955,
    #[doc = "< Input tensor descriptor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_DIAGONAL_BAND_MASK_XDESC = 3000,
    #[doc = "< KV sequence length tensor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_DIAGONAL_BAND_MASK_SEQ_LEN_KVDESC = 3001,
    #[doc = "< Query sequence length tensor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_DIAGONAL_BAND_MASK_SEQ_LEN_QDESC = 3002,
    #[doc = "< Left bound offset descriptor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_DIAGONAL_BAND_MASK_LEFT_BOUND_DESC = 3003,
    #[doc = "< Right bound shift descriptor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_DIAGONAL_BAND_MASK_SHIFT_RIGHT_BOUND_DESC = 3004,
    #[doc = "< Band descriptor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_DIAGONAL_BAND_MASK_BDESC = 3005,
    #[doc = "< Output mask tensor descriptor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_DIAGONAL_BAND_MASK_YDESC = 3006,
    #[doc = "< Mask comparison mode. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_DIAGONAL_BAND_MASK_COMPARISON_MODE = 3007,
    #[doc = "< Softmax input tensor descriptor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_SOFTMAX_XDESC = 3100,
    #[doc = "< Softmax output tensor descriptor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_SOFTMAX_YDESC = 3101,
    #[doc = "< Softmax statistics output. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_SOFTMAX_STATS_DESC = 3102,
    #[doc = "< Row-wise max values descriptor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_SOFTMAX_MAX_DESC = 3103,
    #[doc = "< Row-wise sum of exponentials. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_SOFTMAX_SUM_EXP_DESC = 3104,
    #[doc = "< Softmax sink descriptor. > **Since** cuDNN 9.20.0"]
    CUDNN_ATTR_OPERATION_SOFTMAX_SINK_DESC = 3105,
}
#[repr(u32)]
#[doc = "Attribute data types used by the cuDNN backend API for get/set operations.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendAttributeType_t {
    #[doc = "< cudnnHandle_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_HANDLE = 0,
    #[doc = "< cudnnDataType_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_DATA_TYPE = 1,
    #[doc = "< Boolean value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_BOOLEAN = 2,
    #[doc = "< 64-bit signed integer value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_INT64 = 3,
    #[doc = "< 32-bit float value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_FLOAT = 4,
    #[doc = "< 64-bit double value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_DOUBLE = 5,
    #[doc = "< Void pointer value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_VOID_PTR = 6,
    #[doc = "< cudnnConvolutionMode_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_CONVOLUTION_MODE = 7,
    #[doc = "< cudnnBackendHeurMode_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_HEUR_MODE = 8,
    #[doc = "< cudnnBackendKnobType_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_KNOB_TYPE = 9,
    #[doc = "< cudnnNanPropagation_t value. > **Deprecated** > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_NAN_PROPOGATION = 10,
    #[doc = "< cudnnBackendNumericalNote_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_NUMERICAL_NOTE = 11,
    #[doc = "< cudnnBackendLayoutType_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_LAYOUT_TYPE = 12,
    #[doc = "< cudnnBackendAttributeName_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_ATTRIB_NAME = 13,
    #[doc = "< cudnnPointwiseMode_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_POINTWISE_MODE = 14,
    #[doc = "< cudnnBackendDescriptor_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_BACKEND_DESCRIPTOR = 15,
    #[doc = "< cudnnGenStatsMode_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_GENSTATS_MODE = 16,
    #[doc = "< cudnnBnFinalizeStatsMode_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_BN_FINALIZE_STATS_MODE = 17,
    #[doc = "< cudnnReduceTensorOp_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_REDUCTION_OPERATOR_TYPE = 18,
    #[doc = "< cudnnBackendBehaviorNote_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_BEHAVIOR_NOTE = 19,
    #[doc = "< cudnnBackendTensorReordering_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_TENSOR_REORDERING_MODE = 20,
    #[doc = "< cudnnResampleMode_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_RESAMPLE_MODE = 21,
    #[doc = "< cudnnPaddingMode_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_PADDING_MODE = 22,
    #[doc = "< 32-bit signed integer value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_INT32 = 23,
    #[doc = "< Character value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_CHAR = 24,
    #[doc = "< cudnnSignalMode_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_SIGNAL_MODE = 25,
    #[doc = "< cudnnFraction_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_FRACTION = 26,
    #[doc = "< cudnnBackendNormMode_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_NORM_MODE = 27,
    #[doc = "< cudnnBackendNormFwdPhase_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_NORM_FWD_PHASE = 28,
    #[doc = "< cudnnRngDistribution_t value. > **Since** cuDNN 9.0.0"]
    CUDNN_TYPE_RNG_DISTRIBUTION = 29,
    #[doc = "< cudnnMoeGroupedMatmulMode_t value. > **Since** cuDNN 9.15.0"]
    CUDNN_TYPE_MOE_GROUPED_MATMUL_MODE = 30,
    #[doc = "< cudnnBackendReshapeMode_t value. > **Since** cuDNN 9.22.0"]
    CUDNN_TYPE_RESHAPE_MODE = 31,
}
#[repr(u32)]
#[doc = "Backend descriptor types identifying the kind of descriptor to create.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendDescriptorType_t {
    #[doc = "< Pointwise op config: mode, math precision, activation params. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_POINTWISE_DESCRIPTOR = 0,
    #[doc = "< Convolution config: compute type, mode, dilation, stride, padding. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_CONVOLUTION_DESCRIPTOR = 1,
    #[doc = "< Engine (kernel grouping) to compute an operation graph. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_ENGINE_DESCRIPTOR = 2,
    #[doc = "< Engine configuration: engine descriptor plus knob choices. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_ENGINECFG_DESCRIPTOR = 3,
    #[doc = "< Engine configurations ranked by performance via cuDNN heuristics. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_ENGINEHEUR_DESCRIPTOR = 4,
    #[doc = "< Finalized execution plan: engine config, workspace size, optional kernel cache. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_EXECUTION_PLAN_DESCRIPTOR = 5,
    #[doc = "< Read-only info about a reusable execution intermediate. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_INTERMEDIATE_INFO_DESCRIPTOR = 6,
    #[doc = "< Type and value of an engine performance tuning knob. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_KNOB_CHOICE_DESCRIPTOR = 7,
    #[doc = "< Read-only info about an engine knob: type, min/max, and stride. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_KNOB_INFO_DESCRIPTOR = 8,
    #[doc = "< Read-only info on the preferred memory layout for a tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_LAYOUT_INFO_DESCRIPTOR = 9,
    #[doc = "< Forward convolution: y = alpha * conv(w, x) + beta * y. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_CONVOLUTION_FORWARD_DESCRIPTOR = 10,
    #[doc = "< Backward filter convolution: computes dw from x and dy. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_FILTER_DESCRIPTOR = 11,
    #[doc = "< Backward data convolution: computes dx from w and dy. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_DATA_DESCRIPTOR = 12,
    #[doc = "< Pointwise operation: Y = op(alpha1*X) or Y = op(alpha1*X, alpha2*B). > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_POINTWISE_DESCRIPTOR = 13,
    #[doc = "< Generates per-channel statistics (sum and sum-of-squares). > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_GEN_STATS_DESCRIPTOR = 14,
    #[doc = "< Operation graph: a DAG of operations connected by virtual tensors. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATIONGRAPH_DESCRIPTOR = 15,
    #[doc = "< Binds device pointers to non-virtual tensors, workspace, and intermediates. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_VARIANT_PACK_DESCRIPTOR = 16,
    #[doc = "< Tensor: data type, dimensions, strides, alignment, unique ID, virtual flag. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_TENSOR_DESCRIPTOR = 17,
    #[doc = "< Matrix multiply config: compute type and padding value. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_MATMUL_DESCRIPTOR = 18,
    #[doc = "< Matrix multiplication operation: C = A * B with optional overrides. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_MATMUL_DESCRIPTOR = 19,
    #[doc = "< BN finalize: computes running stats, saved mean/invstd, eq scale/bias. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_BN_FINALIZE_STATISTICS_DESCRIPTOR = 20,
    #[doc = "< Reduction config: operator type (ADD/MUL/MIN/MAX/etc.) and compute type. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_REDUCTION_DESCRIPTOR = 21,
    #[doc = "< Reduces input tensor values along one or more dimensions. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_REDUCTION_DESCRIPTOR = 22,
    #[doc = "< BN backward weights: computes dScale, dBias, and equivalent gradients. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_BN_BWD_WEIGHTS_DESCRIPTOR = 23,
    #[doc = "< Resample config: mode, spatial dims, padding, strides, window dims. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_RESAMPLE_DESCRIPTOR = 24,
    #[doc = "< Forward resampling (pooling/interpolation) with alpha/beta scaling. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_RESAMPLE_FWD_DESCRIPTOR = 25,
    #[doc = "< Backward resampling: computes dx from dy with alpha/beta scaling. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_RESAMPLE_BWD_DESCRIPTOR = 26,
    #[doc = "< Concatenates multiple tensors along a given axis. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_CONCAT_DESCRIPTOR = 27,
    #[doc = "< Updates or waits on a flag variable for inter-graph signaling. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_SIGNAL_DESCRIPTOR = 28,
    #[doc = "< Forward normalization (layer/instance/batch/RMS) with optional running stats. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_NORM_FORWARD_DESCRIPTOR = 29,
    #[doc = "< Backward normalization: computes dX, dScale, dBias from dY. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_NORM_BACKWARD_DESCRIPTOR = 30,
    #[doc = "< Reshapes a tensor from one layout to another. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_RESHAPE_DESCRIPTOR = 31,
    #[doc = "< RNG config: distribution type (Bernoulli/uniform/normal) and parameters. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_RNG_DESCRIPTOR = 32,
    #[doc = "< Generates a tensor of random numbers from a specified distribution. > **Since** cuDNN 9.0.0"]
    CUDNN_BACKEND_OPERATION_RNG_DESCRIPTOR = 33,
    #[doc = "< Caches compiled kernels to speed up plan finalization for dynamic-shape graphs. > **Since** cuDNN 9.4.0"]
    CUDNN_BACKEND_KERNEL_CACHE_DESCRIPTOR = 34,
    #[doc = "< Reconstructs K/V-cache pages in fused flash attention forward graphs. > **Since** cuDNN 9.4.0"]
    CUDNN_BACKEND_OPERATION_PAGED_CACHE_LOAD_DESCRIPTOR = 35,
    #[doc = "< Block-scale quantization: converts float tensors to block-scaled format. > **Since** cuDNN 9.7.0"]
    CUDNN_BACKEND_OPERATION_BLOCK_SCALE_QUANTIZE_DESCRIPTOR = 36,
    #[doc = "< Block-scale dequantization: converts block-scaled tensors back to float. > **Since** cuDNN 9.7.0"]
    CUDNN_BACKEND_OPERATION_BLOCK_SCALE_DEQUANTIZE_DESCRIPTOR = 37,
    #[doc = "< CUDA device properties: device ID, handle, JSON representation. > **Since** cuDNN 9.8.0"]
    CUDNN_BACKEND_DEVICEPROP_DESCRIPTOR = 38,
    #[doc = "< Expands a band matrix into a full matrix representation. > **Since** cuDNN 9.10.0"]
    CUDNN_BACKEND_OPERATION_EXPAND_BAND_MATRIX_DESCRIPTOR = 39,
    #[doc = "< Contracts a full matrix into a band matrix representation. > **Since** cuDNN 9.10.0"]
    CUDNN_BACKEND_OPERATION_CONTRACT_BAND_MATRIX_DESCRIPTOR = 40,
    #[doc = "< Scaled dot-product attention forward (fused flash attention). > **Since** cuDNN 9.13.0"]
    CUDNN_BACKEND_OPERATION_SDPA_FWD_DESCRIPTOR = 41,
    #[doc = "< Mixture-of-Experts grouped matmul with token routing. > **Since** cuDNN 9.15.0"]
    CUDNN_BACKEND_OPERATION_MOE_GROUPED_MATMUL_DESCRIPTOR = 42,
    #[doc = "< Scaled dot-product attention backward: computes dQ, dK, dV. > **Since** cuDNN 9.17.0"]
    CUDNN_BACKEND_OPERATION_SDPA_BWD_DESCRIPTOR = 43,
    #[doc = "< Generates a diagonal band attention mask. > **Since** cuDNN 9.20.0"]
    CUDNN_BACKEND_OPERATION_DIAGONAL_BAND_MASK_DESCRIPTOR = 44,
    #[doc = "< Softmax operation with optional statistics output. > **Since** cuDNN 9.20.0"]
    CUDNN_BACKEND_OPERATION_SOFTMAX_DESCRIPTOR = 45,
    #[doc = "< Transpose operation: permutes tensor dimensions (transpose). Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_BACKEND_OPERATION_TRANSPOSE_DESCRIPTOR = 46,
    #[doc = "< Slice operation: extracts a strided subtensor (slice). Needs CUDA TK > 13.1 > **Since** cuDNN 9.22.0"]
    CUDNN_BACKEND_OPERATION_SLICE_DESCRIPTOR = 47,
    #[doc = "< Backward MoE grouped matmul. > **Since** cuDNN 9.22.0"]
    CUDNN_BACKEND_OPERATION_MOE_GROUPED_MATMUL_BWD_DESCRIPTOR = 48,
}
#[repr(u32)]
#[doc = "Numerical behavior notes describing properties of engine implementations.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendNumericalNote_t {
    #[doc = "< Engine uses Tensor Core hardware acceleration. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_TENSOR_CORE = 0,
    #[doc = "< Engine down-converts inputs to lower precision for compute. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_DOWN_CONVERT_INPUTS = 1,
    #[doc = "< Engine uses reduced-precision accumulation in reductions. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_REDUCED_PRECISION_REDUCTION = 2,
    #[doc = "< Engine uses FFT-based computation. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_FFT = 3,
    #[doc = "< Engine may produce non-deterministic results across runs. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_NONDETERMINISTIC = 4,
    #[doc = "< Engine uses Winograd transform. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_WINOGRAD = 5,
    #[doc = "< Engine uses Winograd with 4x4 output tiles. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_4x4 = 6,
    #[doc = "< Engine uses Winograd with 6x6 output tiles. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_6x6 = 7,
    #[doc = "< Engine uses Winograd with 13x13 output tiles. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_13x13 = 8,
    #[doc = "< Engine strictly propagates NaN values. > **Since** cuDNN 9.1.0"]
    CUDNN_NUMERICAL_NOTE_STRICT_NAN_PROP = 9,
    #[doc = "< Number of numerical note types. > **Since** cuDNN 9.0.0"]
    CUDNN_NUMERICAL_NOTE_TYPE_COUNT = 10,
}
#[repr(u32)]
#[doc = "Engine behavior notes describing runtime requirements and capabilities.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendBehaviorNote_t {
    #[doc = "< Engine requires runtime compilation (NVRTC). > **Since** cuDNN 9.0.0"]
    CUDNN_BEHAVIOR_NOTE_RUNTIME_COMPILATION = 0,
    #[doc = "< Engine requires INT8x32-reordered filter tensors. > **Since** cuDNN 9.0.0"]
    CUDNN_BEHAVIOR_NOTE_REQUIRES_FILTER_INT8x32_REORDER = 1,
    #[doc = "< Engine requires INT8x32-reordered bias tensors. > **Since** cuDNN 9.0.0"]
    CUDNN_BEHAVIOR_NOTE_REQUIRES_BIAS_INT8x32_REORDER = 2,
    #[doc = "< Engine supports native CUDA graph capture. > **Since** cuDNN 9.5.0"]
    CUDNN_BEHAVIOR_NOTE_SUPPORTS_CUDA_GRAPH_NATIVE_API = 3,
    #[doc = "< Engine depends on cuBLASLt library. > **Since** cuDNN 9.15.0"]
    CUDNN_BEHAVIOR_NOTE_CUBLASLT_DEPENDENCY = 4,
    #[doc = "< Number of behavior note types. > **Since** cuDNN 9.0.0"]
    CUDNN_BEHAVIOR_NOTE_TYPE_COUNT = 5,
}
#[repr(u32)]
#[doc = "Engine tuning knob types for fine-grained engine configuration.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendKnobType_t {
    #[doc = "< > **Deprecated** Split-K factor. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SPLIT_K = 0,
    #[doc = "< Memory access swizzle pattern for coalescing. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SWIZZLE = 1,
    #[doc = "< Thread block tile size for computation. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_TILE_SIZE = 2,
    #[doc = "< > **Deprecated** Use texture memory. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_USE_TEX = 3,
    #[doc = "< Edge/boundary handling mode. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_EDGE = 4,
    #[doc = "< > **Deprecated** K-block size. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_KBLOCK = 5,
    #[doc = "< > **Deprecated** Load granularity A. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_LDGA = 6,
    #[doc = "< > **Deprecated** Load granularity B. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_LDGB = 7,
    #[doc = "< > **Deprecated** Chunk-K size. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_CHUNK_K = 8,
    #[doc = "< > **Deprecated** Split-H factor. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SPLIT_H = 9,
    #[doc = "< > **Deprecated** Winograd tile size. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_WINO_TILE = 10,
    #[doc = "< Multiplication factor for tiling. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_MULTIPLY = 11,
    #[doc = "< Split-K with separate output buffers. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SPLIT_K_BUF = 12,
    #[doc = "< Tile size along K (contraction) dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_TILEK = 13,
    #[doc = "< Number of pipeline stages. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_STAGES = 14,
    #[doc = "< Cross-thread/cross-block reduction strategy. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_REDUCTION_MODE = 15,
    #[doc = "< > **Deprecated** CTA split-K mode. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_CTA_SPLIT_K_MODE = 16,
    #[doc = "< Split-K slice count. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SPLIT_K_SLC = 17,
    #[doc = "< Index computation mode. > **Since** cuDNN 9.7.0"]
    CUDNN_KNOB_TYPE_IDX_MODE = 18,
    #[doc = "< > **Deprecated** Sliced mode. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SLICED = 19,
    #[doc = "< > **Deprecated** Split-RS factor. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SPLIT_RS = 20,
    #[doc = "< > **Deprecated** Single buffer mode. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SINGLEBUFFER = 21,
    #[doc = "< > **Deprecated** Load granularity C. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_LDGC = 22,
    #[doc = "< Specialized filter processing mode. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SPECFILT = 23,
    #[doc = "< Kernel configuration selector. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_KERNEL_CFG = 24,
    #[doc = "< Workspace size preference. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_WORKSPACE = 25,
    #[doc = "< > **Deprecated** CGA tile size. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_TILE_CGA = 26,
    #[doc = "< CGA cluster tile size along M dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_TILE_CGA_M = 27,
    #[doc = "< CGA cluster tile size along N dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_TILE_CGA_N = 28,
    #[doc = "< Thread block size (threads per block). > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_BLOCK_SIZE = 29,
    #[doc = "< Target occupancy level. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_OCCUPANCY = 30,
    #[doc = "< Register array size per thread. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_ARRAY_SIZE_PER_THREAD = 31,
    #[doc = "< > **Deprecated** Channels per block. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_NUM_C_PER_BLOCK = 32,
    #[doc = "< Column split factor for parallelism. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_SPLIT_COLS = 33,
    #[doc = "< Tile size along row dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_TILE_ROWS = 34,
    #[doc = "< Tile size along column dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_TILE_COLS = 35,
    #[doc = "< Memory load granularity. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_LOAD_SIZE = 36,
    #[doc = "< Number of CTAs (thread blocks) to launch. > **Since** cuDNN 9.7.0"]
    CUDNN_KNOB_TYPE_CTA_COUNT = 37,
    #[doc = "< Stream-K work distribution mode. > **Since** cuDNN 9.7.0"]
    CUDNN_KNOB_TYPE_STREAM_K = 38,
    #[doc = "< Split-P slice count. > **Since** cuDNN 9.7.0"]
    CUDNN_KNOB_TYPE_SPLIT_P_SLC = 39,
    #[doc = "< Tile size along M (output rows) dimension. > **Since** cuDNN 9.7.0"]
    CUDNN_KNOB_TYPE_TILE_M = 40,
    #[doc = "< Tile size along N (output cols) dimension. > **Since** cuDNN 9.7.0"]
    CUDNN_KNOB_TYPE_TILE_N = 41,
    #[doc = "< Warp specialization configuration. > **Since** cuDNN 9.7.0"]
    CUDNN_KNOB_TYPE_WARP_SPEC_CFG = 42,
    #[doc = "< Swap A and B operands in matmul. > **Since** cuDNN 9.18.0"]
    CUDNN_KNOB_TYPE_SWAP_AB = 43,
    #[doc = "< Enable TMA for input tensors. > **Since** cuDNN 9.22.0"]
    CUDNN_KNOB_TYPE_INPUT_TMA_ENABLE = 44,
    #[doc = "< Enable TMA for output tensors. > **Since** cuDNN 9.22.0"]
    CUDNN_KNOB_TYPE_OUTPUT_TMA_ENABLE = 45,
    #[doc = "< Number of knob types. > **Since** cuDNN 9.0.0"]
    CUDNN_KNOB_TYPE_COUNTS = 46,
}
#[repr(u32)]
#[doc = "Preferred tensor layout types reported by engines.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendLayoutType_t {
    #[doc = "< Prefers NCHW layout. > **Since** cuDNN 9.0.0"]
    CUDNN_LAYOUT_TYPE_PREFERRED_NCHW = 0,
    #[doc = "< Prefers NHWC layout. > **Since** cuDNN 9.0.0"]
    CUDNN_LAYOUT_TYPE_PREFERRED_NHWC = 1,
    #[doc = "< Prefers padded 4CK layout. > **Since** cuDNN 9.0.0"]
    CUDNN_LAYOUT_TYPE_PREFERRED_PAD4CK = 2,
    #[doc = "< Prefers padded 8CK layout. > **Since** cuDNN 9.0.0"]
    CUDNN_LAYOUT_TYPE_PREFERRED_PAD8CK = 3,
    #[doc = "< Number of layout types. > **Since** cuDNN 9.0.0"]
    CUDNN_LAYOUT_TYPE_COUNT = 4,
}
#[repr(u32)]
#[doc = "Heuristic modes for engine selection.\nINSTANT provides fast heuristic lookup, B uses neural-net-based heuristics,\nFALLBACK provides functional (non-optimized) results, A is an alias for INSTANT.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendHeurMode_t {
    #[doc = "< Fast decision-tree heuristic with minimal CPU overhead. > **Since** cuDNN 9.0.0"]
    CUDNN_HEUR_MODE_INSTANT = 0,
    #[doc = "< Neural-net heuristic; 10-100x CPU cost vs INSTANT, better GPU perf. No 3D/grouped/dilated conv. > **Since** cuDNN 9.0.0"]
    CUDNN_HEUR_MODE_B = 1,
    #[doc = "< Functional fallback engines with no GPU performance guarantee. > **Since** cuDNN 9.0.0"]
    CUDNN_HEUR_MODE_FALLBACK = 2,
    #[doc = "< Decision-tree heuristic (preferred over INSTANT). > **Since** cuDNN 9.0.0"]
    CUDNN_HEUR_MODE_A = 3,
    #[doc = "< Number of heuristic modes. > **Since** cuDNN 9.0.0"]
    CUDNN_HEUR_MODES_COUNT = 4,
}
#[repr(u32)]
#[doc = "Tensor reordering modes for specialized memory layouts.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendTensorReordering_t {
    #[doc = "< No tensor reordering applied. > **Since** cuDNN 9.0.0"]
    CUDNN_TENSOR_REORDERING_NONE = 0,
    #[doc = "< INT8 data reordered into 32-element vectors. > **Since** cuDNN 9.0.0"]
    CUDNN_TENSOR_REORDERING_INT8x32 = 1,
    #[doc = "< FP16 data reordered into 16-element vectors. > **Since** cuDNN 9.0.0"]
    CUDNN_TENSOR_REORDERING_F16x16 = 2,
    #[doc = "< FP8 data reordered into 128x4 blocks. > **Since** cuDNN 9.7.0"]
    CUDNN_TENSOR_REORDERING_F8_128x4 = 3,
}
#[repr(u32)]
#[doc = "Padding modes for convolution and pooling operations.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnPaddingMode_t {
    #[doc = "< Pads with zeros. > **Since** cuDNN 9.0.0"]
    CUDNN_ZERO_PAD = 0,
    #[doc = "< Pads with negative infinity (for max pooling). > **Since** cuDNN 9.0.0"]
    CUDNN_NEG_INF_PAD = 1,
    #[doc = "< Pads by replicating edge values. > **Since** cuDNN 9.0.0"]
    CUDNN_EDGE_VAL_PAD = 2,
}
#[repr(u32)]
#[doc = "Normalization modes supported by norm forward/backward operations.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendNormMode_t {
    #[doc = "< Layer normalization: normalizes over feature dims per sample. > **Since** cuDNN 9.0.0"]
    CUDNN_LAYER_NORM = 0,
    #[doc = "< Instance normalization: normalizes per instance per channel. > **Since** cuDNN 9.0.0"]
    CUDNN_INSTANCE_NORM = 1,
    #[doc = "< Batch normalization: normalizes across the batch dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_BATCH_NORM = 2,
    #[doc = "< Group normalization (unsupported; returns CUDNN_STATUS_INTERNAL_ERROR). > **Since** cuDNN 9.0.0"]
    CUDNN_GROUP_NORM = 3,
    #[doc = "< Root mean square normalization: normalizes by RMS of activations. > **Since** cuDNN 9.0.0"]
    CUDNN_RMS_NORM = 4,
    #[doc = "< Adaptive layer normalization with learned affine parameters. > **Since** cuDNN 9.7.0"]
    CUDNN_ADA_LAYER_NORM = 5,
}
#[repr(u32)]
#[doc = "Normalization forward phase (inference or training).\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendNormFwdPhase_t {
    #[doc = "< Inference phase: uses pre-computed running statistics. > **Since** cuDNN 9.0.0"]
    CUDNN_NORM_FWD_INFERENCE = 0,
    #[doc = "< Training phase: computes batch statistics and updates running stats. > **Since** cuDNN 9.0.0"]
    CUDNN_NORM_FWD_TRAINING = 1,
}
#[repr(u32)]
#[doc = "Reshape operation mode.\n> **Since** cuDNN 9.22.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBackendReshapeMode_t {
    #[doc = "< View-only reshape (no data movement). > **Since** cuDNN 9.22.0"]
    CUDNN_RESHAPE_VIEW_ONLY = 0,
    #[doc = "< Logical reshape (may involve data movement). > **Since** cuDNN 9.22.0"]
    CUDNN_RESHAPE_LOGICAL = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Allocates memory for a backend descriptor of the specified type.\n\n# Arguments\n\n* `descriptorType` [in]  -  The type of descriptor to create.\n* `descriptor` [out]  -      Pointer to receive the newly created descriptor.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_NOT_SUPPORTED\n@retval CUDNN_STATUS_ALLOC_FAILED\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnBackendCreateDescriptor(descriptorType: cudnnBackendDescriptorType_t, descriptor: *mut cudnnBackendDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Deallocates a backend descriptor and frees associated memory.\n\n# Arguments\n\n* `descriptor` [in]  -  The descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_ALLOC_FAILED\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnBackendDestroyDescriptor(descriptor: cudnnBackendDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Repurposes pre-allocated memory for a backend descriptor.\n> **Deprecated** Since cuDNN 9.2. Use cudnnBackendCreateDescriptor instead.\n\n# Arguments\n\n* `descriptor` [in]  -  The descriptor to initialize.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnBackendInitialize(descriptor: cudnnBackendDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Validates and finalizes a descriptor. After finalization, attributes become read-only.\n\n# Arguments\n\n* `descriptor` [in]  -  The descriptor to finalize.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_NOT_SUPPORTED\n@retval CUDNN_STATUS_INTERNAL_ERROR\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnBackendFinalize(descriptor: cudnnBackendDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Sets an attribute on an unfinalized backend descriptor.\n\n# Arguments\n\n* `descriptor` [in]  -       The target descriptor (must not be finalized).\n* `attributeName` [in]  -    The attribute to set.\n* `attributeType` [in]  -    The data type of the attribute values.\n* `elementCount` [in]  -     Number of elements in `arrayOfElements.`\n* `arrayOfElements` [in]  -  Pointer to the attribute value(s).\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_NOT_INITIALIZED\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_NOT_SUPPORTED\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnBackendSetAttribute(descriptor: cudnnBackendDescriptor_t, attributeName: cudnnBackendAttributeName_t, attributeType: cudnnBackendAttributeType_t, elementCount: i64, arrayOfElements: *const ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves an attribute from a finalized backend descriptor.\n\n# Arguments\n\n* `descriptor` [in]  -             The source descriptor (must be finalized).\n* `attributeName` [in]  -          The attribute to query.\n* `attributeType` [in]  -          The expected data type of the attribute.\n* `requestedElementCount` [in]  -  Maximum number of elements to retrieve.\n* `elementCount` [out]  -           Pointer to receive the actual element count.\n* `arrayOfElements` [out]  -        Buffer to receive the attribute value(s).\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_NOT_INITIALIZED\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnBackendGetAttribute(descriptor: cudnnBackendDescriptor_t, attributeName: cudnnBackendAttributeName_t, attributeType: cudnnBackendAttributeType_t, requestedElementCount: i64, elementCount: *mut i64, arrayOfElements: *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Runs an execution plan with the given variant pack containing data pointers.\n\n# Arguments\n\n* `handle` [in]  -         cuDNN handle.\n* `executionPlan` [in]  -  Finalized execution plan descriptor.\n* `variantPack` [in]  -    Finalized variant pack descriptor with data pointers.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_INTERNAL_ERROR\n@retval CUDNN_STATUS_EXECUTION_FAILED\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnBackendExecute(handle: cudnnHandle_t, executionPlan: cudnnBackendDescriptor_t, variantPack: cudnnBackendDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Populates a CUDA graph with nodes from an execution plan.\n\n# Arguments\n\n* `handle` [in]  -         cuDNN handle.\n* `executionPlan` [in]  -  Finalized execution plan descriptor.\n* `variantPack` [in]  -    Finalized variant pack descriptor.\n* `graph` [inout]  -          CUDA graph to populate.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_INTERNAL_ERROR\n@retval CUDNN_STATUS_EXECUTION_FAILED\n@retval CUDNN_STATUS_NOT_SUPPORTED\n> **Since** cuDNN 9.5.0"]
    pub fn cudnnBackendPopulateCudaGraph(handle: cudnnHandle_t, executionPlan: cudnnBackendDescriptor_t, variantPack: cudnnBackendDescriptor_t, graph: cudaGraph_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Updates an existing CUDA graph with new data pointers from a variant pack.\n\n# Arguments\n\n* `handle` [in]  -         cuDNN handle.\n* `executionPlan` [in]  -  Finalized execution plan descriptor.\n* `variantPack` [in]  -    Finalized variant pack with updated data pointers.\n* `graph` [inout]  -          CUDA graph to update.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_INTERNAL_ERROR\n@retval CUDNN_STATUS_EXECUTION_FAILED\n@retval CUDNN_STATUS_NOT_SUPPORTED\n> **Since** cuDNN 9.5.0"]
    pub fn cudnnBackendUpdateCudaGraph(handle: cudnnHandle_t, executionPlan: cudnnBackendDescriptor_t, variantPack: cudnnBackendDescriptor_t, graph: cudaGraph_t) -> cudnnStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnTensorStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for a tensor. > **Since** cuDNN 9.0.0"]
pub type cudnnTensorDescriptor_t = *mut cudnnTensorStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnPoolingStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for a pooling operation. > **Since** cuDNN 9.0.0 > **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub type cudnnPoolingDescriptor_t = *mut cudnnPoolingStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnFilterStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for a filter (convolution kernel). > **Since** cuDNN 9.0.0 > **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub type cudnnFilterDescriptor_t = *mut cudnnFilterStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnLRNStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for Local Response Normalization (LRN). > **Since** cuDNN 9.0.0"]
pub type cudnnLRNDescriptor_t = *mut cudnnLRNStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnActivationStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for an activation function. > **Since** cuDNN 9.0.0 > **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub type cudnnActivationDescriptor_t = *mut cudnnActivationStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnSpatialTransformerStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for a spatial transformer network. > **Since** cuDNN 9.0.0"]
pub type cudnnSpatialTransformerDescriptor_t = *mut cudnnSpatialTransformerStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnOpTensorStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for an element-wise tensor operation. > **Since** cuDNN 9.0.0 > **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub type cudnnOpTensorDescriptor_t = *mut cudnnOpTensorStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnReduceTensorStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for a tensor reduction operation. > **Since** cuDNN 9.0.0 > **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub type cudnnReduceTensorDescriptor_t = *mut cudnnReduceTensorStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnCTCLossStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for a CTC loss function. > **Since** cuDNN 9.0.0"]
pub type cudnnCTCLossDescriptor_t = *mut cudnnCTCLossStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnTensorTransformStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for tensor transform operations. > **Since** cuDNN 9.0.0 > **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub type cudnnTensorTransformDescriptor_t = *mut cudnnTensorTransformStruct;
#[repr(u32)]
#[doc = "Indicates whether results are guaranteed to be reproducible across runs.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnDeterminism_t {
    #[doc = "< Results may vary across runs. > **Since** cuDNN 9.0.0"]
    CUDNN_NON_DETERMINISTIC = 0,
    #[doc = "< Results are guaranteed to be reproducible. > **Since** cuDNN 9.0.0"]
    CUDNN_DETERMINISTIC = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a tensor descriptor.\nAllocates and initializes a new tensor descriptor object.\n\n# Arguments\n\n* `tensorDesc` [out]  -  Pointer to the newly created tensor descriptor.\n@retval CUDNN_STATUS_SUCCESS           The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED      Memory allocation failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroyTensorDescriptor,`] cudnnSetTensor4dDescriptor"]
    pub fn cudnnCreateTensorDescriptor(tensorDesc: *mut cudnnTensorDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Sets a 4D tensor descriptor.\nInitializes a previously created tensor descriptor with the specified format,\ndata type, and dimensions. Strides are computed automatically based on the format.\n\n# Arguments\n\n* `tensorDesc` [in,out]  -  Tensor descriptor to initialize.\n* `format` [in]  -      Memory layout format (e.g., NCHW or NHWC).\n* `dataType` [in]  -    Data type of the tensor elements.\n* `n` [in]  -           Number of images (batch size).\n* `c` [in]  -           Number of feature maps (channels).\n* `h` [in]  -           Height of each feature map.\n* `w` [in]  -           Width of each feature map.\n@retval CUDNN_STATUS_SUCCESS           The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM         An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensor4dDescriptorEx,`] cudnnGetTensor4dDescriptor"]
    pub fn cudnnSetTensor4dDescriptor(tensorDesc: cudnnTensorDescriptor_t, format: cudnnTensorFormat_t, dataType: cudnnDataType_t, n: ::core::ffi::c_int, c: ::core::ffi::c_int, h: ::core::ffi::c_int, w: ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Sets a 4D tensor descriptor with explicit strides.\nInitializes a previously created tensor descriptor with the specified data type,\ndimensions, and explicit stride values for each dimension.\n\n# Arguments\n\n* `tensorDesc` [in,out]  -  Tensor descriptor to initialize.\n* `dataType` [in]  -    Data type of the tensor elements.\n* `n` [in]  -           Number of images (batch size).\n* `c` [in]  -           Number of feature maps (channels).\n* `h` [in]  -           Height of each feature map.\n* `w` [in]  -           Width of each feature map.\n* `nStride` [in]  -     Stride between images.\n* `cStride` [in]  -     Stride between feature maps.\n* `hStride` [in]  -     Stride between rows.\n* `wStride` [in]  -     Stride between columns.\n@retval CUDNN_STATUS_SUCCESS           The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM         An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensor4dDescriptor,`] cudnnGetTensor4dDescriptor"]
    pub fn cudnnSetTensor4dDescriptorEx(
        tensorDesc: cudnnTensorDescriptor_t,
        dataType: cudnnDataType_t,
        n: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        nStride: ::core::ffi::c_int,
        cStride: ::core::ffi::c_int,
        hStride: ::core::ffi::c_int,
        wStride: ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of a previously initialized 4D tensor descriptor.\n\n# Arguments\n\n* `tensorDesc` [in]  -  Tensor descriptor to query.\n* `dataType` [out]  -    Data type of the tensor.\n* `n` [out]  -           Number of images (batch size).\n* `c` [out]  -           Number of feature maps (channels).\n* `h` [out]  -           Height of each feature map.\n* `w` [out]  -           Width of each feature map.\n* `nStride` [out]  -     Stride between images.\n* `cStride` [out]  -     Stride between feature maps.\n* `hStride` [out]  -     Stride between rows.\n* `wStride` [out]  -     Stride between columns.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensor4dDescriptor`]"]
    pub fn cudnnGetTensor4dDescriptor(
        tensorDesc: cudnnTensorDescriptor_t,
        dataType: *mut cudnnDataType_t,
        n: *mut ::core::ffi::c_int,
        c: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
        w: *mut ::core::ffi::c_int,
        nStride: *mut ::core::ffi::c_int,
        cStride: *mut ::core::ffi::c_int,
        hStride: *mut ::core::ffi::c_int,
        wStride: *mut ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Sets an N-dimensional tensor descriptor.\nInitializes a tensor descriptor with arbitrary dimensionality, data type, dimensions, and strides.\n\n# Arguments\n\n* `tensorDesc` [in,out]  -  Tensor descriptor to initialize.\n* `dataType` [in]  -    Data type of the tensor elements.\n* `nbDims` [in]  -      Number of dimensions.\n* `dimA` [in]  -        Array of dimension sizes (length nbDims).\n* `strideA` [in]  -     Array of strides (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnGetTensorNdDescriptor`]"]
    pub fn cudnnSetTensorNdDescriptor(tensorDesc: cudnnTensorDescriptor_t, dataType: cudnnDataType_t, nbDims: ::core::ffi::c_int, dimA: *const ::core::ffi::c_int, strideA: *const ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Sets an N-dimensional tensor descriptor with automatic stride computation.\nInitializes a tensor descriptor with the specified format; strides are computed\nautomatically from the format and dimensions.\n\n# Arguments\n\n* `tensorDesc` [in,out]  -  Tensor descriptor to initialize.\n* `format` [in]  -      Memory layout format.\n* `dataType` [in]  -    Data type of the tensor elements.\n* `nbDims` [in]  -      Number of dimensions.\n* `dimA` [in]  -        Array of dimension sizes (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensorNdDescriptor`]"]
    pub fn cudnnSetTensorNdDescriptorEx(tensorDesc: cudnnTensorDescriptor_t, format: cudnnTensorFormat_t, dataType: cudnnDataType_t, nbDims: ::core::ffi::c_int, dimA: *const ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of a previously initialized N-dimensional tensor descriptor.\n\n# Arguments\n\n* `tensorDesc` [in]  -      Tensor descriptor to query.\n* `nbDimsRequested` [in]  - Number of dimensions to retrieve.\n* `dataType` [out]  -        Data type of the tensor.\n* `nbDims` [out]  -          Actual number of dimensions in the descriptor.\n* `dimA` [out]  -            Array to receive dimension sizes (length nbDimsRequested).\n* `strideA` [out]  -         Array to receive strides (length nbDimsRequested).\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensorNdDescriptor`]"]
    pub fn cudnnGetTensorNdDescriptor(tensorDesc: cudnnTensorDescriptor_t, nbDimsRequested: ::core::ffi::c_int, dataType: *mut cudnnDataType_t, nbDims: *mut ::core::ffi::c_int, dimA: *mut ::core::ffi::c_int, strideA: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the memory size in bytes required by a tensor.\n\n# Arguments\n\n* `tensorDesc` [in]  -  Tensor descriptor to query.\n* `size` [out]  -        Memory size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetTensorSizeInBytes(tensorDesc: cudnnTensorDescriptor_t, size: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a tensor descriptor.\nReleases the resources associated with a tensor descriptor object.\n\n# Arguments\n\n* `tensorDesc` [in]  -  Tensor descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateTensorDescriptor`]"]
    pub fn cudnnDestroyTensorDescriptor(tensorDesc: cudnnTensorDescriptor_t) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Specifies the direction for tensor transform folding operations.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnFoldingDirection_t {
    #[doc = "< Fold the tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_TRANSFORM_FOLD = 0,
    #[doc = "< Unfold the tensor. > **Since** cuDNN 9.0.0"]
    CUDNN_TRANSFORM_UNFOLD = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Initializes the destination tensor descriptor for a tensor transform.\nComputes the destination tensor dimensions and size based on the transform and source descriptors.\n\n# Arguments\n\n* `transformDesc` [in]  -   Transform descriptor specifying the operation.\n* `srcDesc` [in]  -         Source tensor descriptor.\n* `destDesc` [in,out]  -        Destination tensor descriptor to be initialized.\n* `destSizeInBytes` [out]  - Memory size in bytes of the destination tensor.\n@retval CUDNN_STATUS_SUCCESS     The destination descriptor was initialized successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnTransformTensorEx`]"]
    pub fn cudnnInitTransformDest(transformDesc: cudnnTensorTransformDescriptor_t, srcDesc: cudnnTensorDescriptor_t, destDesc: cudnnTensorDescriptor_t, destSizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a tensor transform descriptor.\nAllocates and initializes a new tensor transform descriptor object.\n\n# Arguments\n\n* `transformDesc` [out]  -  Pointer to the newly created transform descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyTensorTransformDescriptor,`] cudnnSetTensorTransformDescriptor"]
    pub fn cudnnCreateTensorTransformDescriptor(transformDesc: *mut cudnnTensorTransformDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures a tensor transform descriptor.\nSets the parameters of a previously created tensor transform descriptor including\npadding, folding, and destination format.\n\n# Arguments\n\n* `transformDesc` [in,out]  -  Transform descriptor to configure.\n* `nbDims` [in]  -         Number of dimensions.\n* `destFormat` [in]  -     Destination tensor format.\n* `padBeforeA` [in]  -     Array of padding values before each dimension.\n* `padAfterA` [in]  -      Array of padding values after each dimension.\n* `foldA` [in]  -          Array of fold parameters per dimension.\n* `direction` [in]  -      Folding direction (fold or unfold).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was configured successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetTensorTransformDescriptor`]"]
    pub fn cudnnSetTensorTransformDescriptor(transformDesc: cudnnTensorTransformDescriptor_t, nbDims: u32, destFormat: cudnnTensorFormat_t, padBeforeA: *const i32, padAfterA: *const i32, foldA: *const u32, direction: cudnnFoldingDirection_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of a previously initialized tensor transform descriptor.\n\n# Arguments\n\n* `transformDesc` [in]  -   Transform descriptor to query.\n* `nbDimsRequested` [in]  - Number of dimensions to retrieve.\n* `destFormat` [out]  -      Destination tensor format.\n* `padBeforeA` [out]  -      Array to receive pre-padding values.\n* `padAfterA` [out]  -       Array to receive post-padding values.\n* `foldA` [out]  -           Array to receive fold parameters.\n* `direction` [out]  -       Folding direction.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetTensorTransformDescriptor`]"]
    pub fn cudnnGetTensorTransformDescriptor(transformDesc: cudnnTensorTransformDescriptor_t, nbDimsRequested: u32, destFormat: *mut cudnnTensorFormat_t, padBeforeA: *mut i32, padAfterA: *mut i32, foldA: *mut u32, direction: *mut cudnnFoldingDirection_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a tensor transform descriptor.\nReleases the resources associated with a tensor transform descriptor.\n\n# Arguments\n\n* `transformDesc` [in]  -  Transform descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateTensorTransformDescriptor`]"]
    pub fn cudnnDestroyTensorTransformDescriptor(transformDesc: cudnnTensorTransformDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Copies and converts tensor data between layouts with alpha/beta blending.\nPerforms y = alpha * x + beta * y, converting between tensor formats as needed.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `alpha` [in]  -   Scaling factor for the source tensor.\n* `xDesc` [in]  -   Source tensor descriptor.\n* `x` [in]  -       Pointer to source tensor data.\n* `beta` [in]  -    Scaling factor for the destination tensor.\n* `yDesc` [in]  -   Destination tensor descriptor.\n* `y` [in,out]  -       Pointer to destination tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnTransformTensorEx`]"]
    pub fn cudnnTransformTensor(handle: cudnnHandle_t, alpha: *const ::core::ffi::c_void, xDesc: cudnnTensorDescriptor_t, x: *const ::core::ffi::c_void, beta: *const ::core::ffi::c_void, yDesc: cudnnTensorDescriptor_t, y: *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Extended tensor transform with folding/padding support.\nPerforms dest = alpha * transform(src) + beta * dest, using the specified\ntransform descriptor for padding and folding configuration.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `transDesc` [in]  - Transform descriptor specifying the operation.\n* `alpha` [in]  -     Scaling factor for the source tensor.\n* `srcDesc` [in]  -   Source tensor descriptor.\n* `srcData` [in]  -   Pointer to source tensor data.\n* `beta` [in]  -      Scaling factor for the destination tensor.\n* `destDesc` [in]  -  Destination tensor descriptor.\n* `destData` [in,out]  -  Pointer to destination tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnTransformTensor,`] cudnnSetTensorTransformDescriptor"]
    pub fn cudnnTransformTensorEx(
        handle: cudnnHandle_t,
        transDesc: cudnnTensorTransformDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        srcDesc: cudnnTensorDescriptor_t,
        srcData: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        destDesc: cudnnTensorDescriptor_t,
        destData: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Adds a scaled bias tensor to a destination tensor with broadcasting.\nPerforms C = alpha * A + beta * C, where A is broadcast to match C dimensions.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `alpha` [in]  -   Scaling factor for the bias tensor A.\n* `aDesc` [in]  -   Bias tensor descriptor.\n* `A` [in]  -       Pointer to bias tensor data.\n* `beta` [in]  -    Scaling factor for the destination tensor C.\n* `cDesc` [in]  -   Destination tensor descriptor.\n* `C` [in,out]  -       Pointer to destination tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
    pub fn cudnnAddTensor(handle: cudnnHandle_t, alpha: *const ::core::ffi::c_void, aDesc: cudnnTensorDescriptor_t, A: *const ::core::ffi::c_void, beta: *const ::core::ffi::c_void, cDesc: cudnnTensorDescriptor_t, C: *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Enumerates the element-wise tensor operations supported by cudnnOpTensor.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnOpTensorOp_t {
    #[doc = "< Element-wise addition. > **Since** cuDNN 9.0.0"]
    CUDNN_OP_TENSOR_ADD = 0,
    #[doc = "< Element-wise multiplication. > **Since** cuDNN 9.0.0"]
    CUDNN_OP_TENSOR_MUL = 1,
    #[doc = "< Element-wise minimum. > **Since** cuDNN 9.0.0"]
    CUDNN_OP_TENSOR_MIN = 2,
    #[doc = "< Element-wise maximum. > **Since** cuDNN 9.0.0"]
    CUDNN_OP_TENSOR_MAX = 3,
    #[doc = "< Element-wise square root (unary, B tensor ignored). > **Since** cuDNN 9.0.0"]
    CUDNN_OP_TENSOR_SQRT = 4,
    #[doc = "< Element-wise logical NOT (unary, B tensor ignored). > **Since** cuDNN 9.0.0"]
    CUDNN_OP_TENSOR_NOT = 5,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates an op tensor descriptor.\n\n# Arguments\n\n* `opTensorDesc` [out]  -  Pointer to the newly created op tensor descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyOpTensorDescriptor`]"]
    pub fn cudnnCreateOpTensorDescriptor(opTensorDesc: *mut cudnnOpTensorDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures an op tensor descriptor.\n\n# Arguments\n\n* `opTensorDesc` [in,out]  -     Op tensor descriptor to configure.\n* `opTensorOp` [in]  -       Tensor operation to perform.\n* `opTensorCompType` [in]  - Computation data type.\n* `opTensorNanOpt` [in]  -   NaN propagation policy.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetOpTensorDescriptor`]"]
    pub fn cudnnSetOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t, opTensorOp: cudnnOpTensorOp_t, opTensorCompType: cudnnDataType_t, opTensorNanOpt: cudnnNanPropagation_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of an op tensor descriptor.\n\n# Arguments\n\n* `opTensorDesc` [in]  -     Op tensor descriptor to query.\n* `opTensorOp` [out]  -       Tensor operation type.\n* `opTensorCompType` [out]  - Computation data type.\n* `opTensorNanOpt` [out]  -   NaN propagation policy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetOpTensorDescriptor`]"]
    pub fn cudnnGetOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t, opTensorOp: *mut cudnnOpTensorOp_t, opTensorCompType: *mut cudnnDataType_t, opTensorNanOpt: *mut cudnnNanPropagation_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys an op tensor descriptor.\n\n# Arguments\n\n* `opTensorDesc` [in]  -  Op tensor descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateOpTensorDescriptor`]"]
    pub fn cudnnDestroyOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs element-wise tensor operations.\nComputes C = op(alpha1 * A, alpha2 * B) + beta * C. The B tensor is ignored\nfor CUDNN_OP_TENSOR_SQRT and CUDNN_OP_TENSOR_NOT (unary operations).\n\n# Arguments\n\n* `handle` [in]  -       cuDNN library handle.\n* `opTensorDesc` [in]  - Op tensor descriptor specifying the operation.\n* `alpha1` [in]  -       Scaling factor for tensor A.\n* `aDesc` [in]  -        Descriptor for tensor A.\n* `A` [in]  -            Pointer to tensor A data.\n* `alpha2` [in]  -       Scaling factor for tensor B.\n* `bDesc` [in]  -        Descriptor for tensor B.\n* `B` [in]  -            Pointer to tensor B data.\n* `beta` [in]  -         Scaling factor for tensor C.\n* `cDesc` [in]  -        Descriptor for tensor C.\n* `C` [in,out]  -            Pointer to tensor C data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetOpTensorDescriptor`]"]
    pub fn cudnnOpTensor(
        handle: cudnnHandle_t,
        opTensorDesc: cudnnOpTensorDescriptor_t,
        alpha1: *const ::core::ffi::c_void,
        aDesc: cudnnTensorDescriptor_t,
        A: *const ::core::ffi::c_void,
        alpha2: *const ::core::ffi::c_void,
        bDesc: cudnnTensorDescriptor_t,
        B: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        cDesc: cudnnTensorDescriptor_t,
        C: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Specifies whether indices are computed during a reduction operation.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnReduceTensorIndices_t {
    #[doc = "< Do not compute indices. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_NO_INDICES = 0,
    #[doc = "< Compute flattened indices of min/max values. > **Since** cuDNN 9.0.0"]
    CUDNN_REDUCE_TENSOR_FLATTENED_INDICES = 1,
}
#[repr(u32)]
#[doc = "Data type used for reduction indices (all unsigned).\nCurrently only 32-bit unsigned is fully supported; other sizes are reserved.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnIndicesType_t {
    #[doc = "< 32-bit unsigned indices. > **Since** cuDNN 9.0.0"]
    CUDNN_32BIT_INDICES = 0,
    #[doc = "< 64-bit unsigned indices. > **Since** cuDNN 9.0.0"]
    CUDNN_64BIT_INDICES = 1,
    #[doc = "< 16-bit unsigned indices. > **Since** cuDNN 9.0.0"]
    CUDNN_16BIT_INDICES = 2,
    #[doc = "< 8-bit unsigned indices. > **Since** cuDNN 9.0.0"]
    CUDNN_8BIT_INDICES = 3,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a reduce tensor descriptor.\n\n# Arguments\n\n* `reduceTensorDesc` [out]  -  Pointer to the newly created reduce tensor descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyReduceTensorDescriptor`]"]
    pub fn cudnnCreateReduceTensorDescriptor(reduceTensorDesc: *mut cudnnReduceTensorDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures a reduce tensor descriptor.\n\n# Arguments\n\n* `reduceTensorDesc` [in,out]  -        Reduce tensor descriptor to configure.\n* `reduceTensorOp` [in]  -          Reduction operation to perform.\n* `reduceTensorCompType` [in]  -    Computation data type.\n* `reduceTensorNanOpt` [in]  -      NaN propagation policy (applies to min/max only).\n* `reduceTensorIndices` [in]  -     Whether to compute indices.\n* `reduceTensorIndicesType` [in]  - Data type for computed indices.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetReduceTensorDescriptor`]"]
    pub fn cudnnSetReduceTensorDescriptor(
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        reduceTensorOp: cudnnReduceTensorOp_t,
        reduceTensorCompType: cudnnDataType_t,
        reduceTensorNanOpt: cudnnNanPropagation_t,
        reduceTensorIndices: cudnnReduceTensorIndices_t,
        reduceTensorIndicesType: cudnnIndicesType_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of a reduce tensor descriptor.\n\n# Arguments\n\n* `reduceTensorDesc` [in]  -        Reduce tensor descriptor to query.\n* `reduceTensorOp` [out]  -          Reduction operation type.\n* `reduceTensorCompType` [out]  -    Computation data type.\n* `reduceTensorNanOpt` [out]  -      NaN propagation policy.\n* `reduceTensorIndices` [out]  -     Whether indices are computed.\n* `reduceTensorIndicesType` [out]  - Data type for computed indices.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetReduceTensorDescriptor`]"]
    pub fn cudnnGetReduceTensorDescriptor(
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        reduceTensorOp: *mut cudnnReduceTensorOp_t,
        reduceTensorCompType: *mut cudnnDataType_t,
        reduceTensorNanOpt: *mut cudnnNanPropagation_t,
        reduceTensorIndices: *mut cudnnReduceTensorIndices_t,
        reduceTensorIndicesType: *mut cudnnIndicesType_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a reduce tensor descriptor.\n\n# Arguments\n\n* `reduceTensorDesc` [in]  -  Reduce tensor descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateReduceTensorDescriptor`]"]
    pub fn cudnnDestroyReduceTensorDescriptor(reduceTensorDesc: cudnnReduceTensorDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the minimum size of the index space for a reduction operation.\n\n# Arguments\n\n* `handle` [in]  -           cuDNN library handle.\n* `reduceTensorDesc` [in]  - Reduce tensor descriptor.\n* `aDesc` [in]  -            Input tensor descriptor.\n* `cDesc` [in]  -            Output tensor descriptor.\n* `sizeInBytes` [out]  -      Minimum index space size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnReduceTensor`]"]
    pub fn cudnnGetReductionIndicesSize(handle: cudnnHandle_t, reduceTensorDesc: cudnnReduceTensorDescriptor_t, aDesc: cudnnTensorDescriptor_t, cDesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the minimum workspace size required for a reduction operation.\n\n# Arguments\n\n* `handle` [in]  -           cuDNN library handle.\n* `reduceTensorDesc` [in]  - Reduce tensor descriptor.\n* `aDesc` [in]  -            Input tensor descriptor.\n* `cDesc` [in]  -            Output tensor descriptor.\n* `sizeInBytes` [out]  -      Minimum workspace size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnReduceTensor`]"]
    pub fn cudnnGetReductionWorkspaceSize(handle: cudnnHandle_t, reduceTensorDesc: cudnnReduceTensorDescriptor_t, aDesc: cudnnTensorDescriptor_t, cDesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs a tensor reduction operation.\nComputes C = reduce_op(alpha * A) + beta * C. NaN propagation applies only\nto min and max operations. The indices space is ignored for operations other\nthan min or max.\n\n# Arguments\n\n* `handle` [in]  -              cuDNN library handle.\n* `reduceTensorDesc` [in]  -    Reduce tensor descriptor.\n* `indices` [out]  -             Pointer to index space (for min/max ops).\n* `indicesSizeInBytes` [in]  -  Size of the index space in bytes.\n* `workspace` [out]  -           Pointer to workspace memory.\n* `workspaceSizeInBytes` [in]  - Size of the workspace in bytes.\n* `alpha` [in]  -               Scaling factor for the input tensor.\n* `aDesc` [in]  -               Input tensor descriptor.\n* `A` [in]  -                   Pointer to input tensor data.\n* `beta` [in]  -                Scaling factor for the output tensor.\n* `cDesc` [in]  -               Output tensor descriptor.\n* `C` [in,out]  -                   Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetReductionWorkspaceSize,`] cudnnGetReductionIndicesSize"]
    pub fn cudnnReduceTensor(
        handle: cudnnHandle_t,
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        indices: *mut ::core::ffi::c_void,
        indicesSizeInBytes: usize,
        workspace: *mut ::core::ffi::c_void,
        workspaceSizeInBytes: usize,
        alpha: *const ::core::ffi::c_void,
        aDesc: cudnnTensorDescriptor_t,
        A: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        cDesc: cudnnTensorDescriptor_t,
        C: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Fills a tensor with a constant value.\nSets every element of the tensor to the specified value: y[i] = value[0].\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `yDesc` [in]  -     Tensor descriptor.\n* `y` [out]  -         Pointer to tensor data.\n* `valuePtr` [in]  -  Pointer to the fill value (type matches tensor data type).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnScaleTensor`]"]
    pub fn cudnnSetTensor(handle: cudnnHandle_t, yDesc: cudnnTensorDescriptor_t, y: *mut ::core::ffi::c_void, valuePtr: *const ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Scales all elements of a tensor by a constant factor.\nPerforms y[i] = alpha * y[i] for every element.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `yDesc` [in]  -   Tensor descriptor.\n* `y` [in,out]  -       Pointer to tensor data.\n* `alpha` [in]  -   Scaling factor (type matches tensor computation type).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetTensor`]"]
    pub fn cudnnScaleTensor(handle: cudnnHandle_t, yDesc: cudnnTensorDescriptor_t, y: *mut ::core::ffi::c_void, alpha: *const ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a filter descriptor.\nAllocates and initializes a new filter (convolution kernel) descriptor.\n\n# Arguments\n\n* `filterDesc` [out]  -  Pointer to the newly created filter descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyFilterDescriptor`]"]
    pub fn cudnnCreateFilterDescriptor(filterDesc: *mut cudnnFilterDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Sets a 4D filter descriptor.\nInitializes a filter descriptor with the specified data type, format, and dimensions.\n\n# Arguments\n\n* `filterDesc` [in,out]  -  Filter descriptor to initialize.\n* `dataType` [in]  -    Data type of the filter elements.\n* `format` [in]  -      Memory layout format.\n* `k` [in]  -           Number of output feature maps.\n* `c` [in]  -           Number of input feature maps.\n* `h` [in]  -           Height of each filter.\n* `w` [in]  -           Width of each filter.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetFilter4dDescriptor`]"]
    pub fn cudnnSetFilter4dDescriptor(filterDesc: cudnnFilterDescriptor_t, dataType: cudnnDataType_t, format: cudnnTensorFormat_t, k: ::core::ffi::c_int, c: ::core::ffi::c_int, h: ::core::ffi::c_int, w: ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of a 4D filter descriptor.\n\n# Arguments\n\n* `filterDesc` [in]  -  Filter descriptor to query.\n* `dataType` [out]  -    Data type of the filter.\n* `format` [out]  -      Memory layout format.\n* `k` [out]  -           Number of output feature maps.\n* `c` [out]  -           Number of input feature maps.\n* `h` [out]  -           Height of each filter.\n* `w` [out]  -           Width of each filter.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetFilter4dDescriptor`]"]
    pub fn cudnnGetFilter4dDescriptor(filterDesc: cudnnFilterDescriptor_t, dataType: *mut cudnnDataType_t, format: *mut cudnnTensorFormat_t, k: *mut ::core::ffi::c_int, c: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int, w: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Sets an N-dimensional filter descriptor.\n\n# Arguments\n\n* `filterDesc` [in,out]  -  Filter descriptor to initialize.\n* `dataType` [in]  -    Data type of the filter elements.\n* `format` [in]  -      Memory layout format.\n* `nbDims` [in]  -      Number of dimensions.\n* `filterDimA` [in]  -  Array of filter dimension sizes (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetFilterNdDescriptor`]"]
    pub fn cudnnSetFilterNdDescriptor(filterDesc: cudnnFilterDescriptor_t, dataType: cudnnDataType_t, format: cudnnTensorFormat_t, nbDims: ::core::ffi::c_int, filterDimA: *const ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of an N-dimensional filter descriptor.\n\n# Arguments\n\n* `filterDesc` [in]  -      Filter descriptor to query.\n* `nbDimsRequested` [in]  - Number of dimensions to retrieve.\n* `dataType` [out]  -        Data type of the filter.\n* `format` [out]  -          Memory layout format.\n* `nbDims` [out]  -          Actual number of dimensions.\n* `filterDimA` [out]  -      Array to receive dimension sizes.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetFilterNdDescriptor`]"]
    pub fn cudnnGetFilterNdDescriptor(filterDesc: cudnnFilterDescriptor_t, nbDimsRequested: ::core::ffi::c_int, dataType: *mut cudnnDataType_t, format: *mut cudnnTensorFormat_t, nbDims: *mut ::core::ffi::c_int, filterDimA: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the memory size in bytes required by a filter.\n\n# Arguments\n\n* `filterDesc` [in]  -  Filter descriptor to query.\n* `size` [out]  -        Memory size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
    pub fn cudnnGetFilterSizeInBytes(filterDesc: cudnnFilterDescriptor_t, size: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Transforms filter data between layouts.\nConverts filter data from one format to another using the specified transform descriptor.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `transDesc` [in]  - Transform descriptor specifying the operation.\n* `alpha` [in]  -     Scaling factor for the source filter.\n* `srcDesc` [in]  -   Source filter descriptor.\n* `srcData` [in]  -   Pointer to source filter data.\n* `beta` [in]  -      Scaling factor for the destination filter.\n* `destDesc` [in]  -  Destination filter descriptor.\n* `destData` [in,out]  -  Pointer to destination filter data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnTransformTensorEx`]"]
    pub fn cudnnTransformFilter(
        handle: cudnnHandle_t,
        transDesc: cudnnTensorTransformDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        srcDesc: cudnnFilterDescriptor_t,
        srcData: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        destDesc: cudnnFilterDescriptor_t,
        destData: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a filter descriptor.\n\n# Arguments\n\n* `filterDesc` [in]  -  Filter descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateFilterDescriptor`]"]
    pub fn cudnnDestroyFilterDescriptor(filterDesc: cudnnFilterDescriptor_t) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Selects the softmax implementation algorithm.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnSoftmaxAlgorithm_t {
    #[doc = "< Straightforward softmax without overflow protection. > **Since** cuDNN 9.0.0"]
    CUDNN_SOFTMAX_FAST = 0,
    #[doc = "< Scales by max value to avoid floating-point overflow. > **Since** cuDNN 9.0.0"]
    CUDNN_SOFTMAX_ACCURATE = 1,
    #[doc = "< Log-softmax with max-value scaling for overflow protection. > **Since** cuDNN 9.0.0"]
    CUDNN_SOFTMAX_LOG = 2,
}
#[repr(u32)]
#[doc = "Selects the scope over which the softmax computation is performed.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnSoftmaxMode_t {
    #[doc = "< Compute softmax over all C, H, W for each image (N). > **Since** cuDNN 9.0.0"]
    CUDNN_SOFTMAX_MODE_INSTANCE = 0,
    #[doc = "< Compute softmax over channel (C) for each spatial location (H, W) and image (N). > **Since** cuDNN 9.0.0"]
    CUDNN_SOFTMAX_MODE_CHANNEL = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs forward softmax computation.\nComputes y = alpha * softmax(x) + beta * y.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `algo` [in]  -    Softmax algorithm to use.\n* `mode` [in]  -    Softmax computation scope.\n* `alpha` [in]  -   Scaling factor for the result.\n* `xDesc` [in]  -   Input tensor descriptor.\n* `x` [in]  -       Pointer to input tensor data.\n* `beta` [in]  -    Scaling factor for the destination tensor.\n* `yDesc` [in]  -   Output tensor descriptor.\n* `y` [in,out]  -       Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSoftmaxBackward`]"]
    pub fn cudnnSoftmaxForward(
        handle: cudnnHandle_t,
        algo: cudnnSoftmaxAlgorithm_t,
        mode: cudnnSoftmaxMode_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Selects the pooling method used in pooling operations.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnPoolingMode_t {
    #[doc = "< Maximum value in the pooling window. > **Since** cuDNN 9.0.0"]
    CUDNN_POOLING_MAX = 0,
    #[doc = "< Average pooling; element count includes padded positions. > **Since** cuDNN 9.0.0"]
    CUDNN_POOLING_AVERAGE_COUNT_INCLUDE_PADDING = 1,
    #[doc = "< Average pooling; element count excludes padded positions. > **Since** cuDNN 9.0.0"]
    CUDNN_POOLING_AVERAGE_COUNT_EXCLUDE_PADDING = 2,
    #[doc = "< Deterministic max pooling (reproducible results). > **Since** cuDNN 9.0.0"]
    CUDNN_POOLING_MAX_DETERMINISTIC = 3,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [out]  -  Pointer to the newly created pooling descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyPoolingDescriptor`]"]
    pub fn cudnnCreatePoolingDescriptor(poolingDesc: *mut cudnnPoolingDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures a 2D pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in,out]  -       Pooling descriptor to configure.\n* `mode` [in]  -              Pooling mode (max, average, etc.).\n* `maxpoolingNanOpt` [in]  -  NaN propagation policy for max pooling.\n* `windowHeight` [in]  -      Height of the pooling window.\n* `windowWidth` [in]  -       Width of the pooling window.\n* `verticalPadding` [in]  -   Vertical padding size.\n* `horizontalPadding` [in]  - Horizontal padding size.\n* `verticalStride` [in]  -    Vertical stride.\n* `horizontalStride` [in]  -  Horizontal stride.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetPooling2dDescriptor`]"]
    pub fn cudnnSetPooling2dDescriptor(
        poolingDesc: cudnnPoolingDescriptor_t,
        mode: cudnnPoolingMode_t,
        maxpoolingNanOpt: cudnnNanPropagation_t,
        windowHeight: ::core::ffi::c_int,
        windowWidth: ::core::ffi::c_int,
        verticalPadding: ::core::ffi::c_int,
        horizontalPadding: ::core::ffi::c_int,
        verticalStride: ::core::ffi::c_int,
        horizontalStride: ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of a 2D pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in]  -       Pooling descriptor to query.\n* `mode` [out]  -              Pooling mode.\n* `maxpoolingNanOpt` [out]  -  NaN propagation policy.\n* `windowHeight` [out]  -      Height of the pooling window.\n* `windowWidth` [out]  -       Width of the pooling window.\n* `verticalPadding` [out]  -   Vertical padding size.\n* `horizontalPadding` [out]  - Horizontal padding size.\n* `verticalStride` [out]  -    Vertical stride.\n* `horizontalStride` [out]  -  Horizontal stride.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetPooling2dDescriptor`]"]
    pub fn cudnnGetPooling2dDescriptor(
        poolingDesc: cudnnPoolingDescriptor_t,
        mode: *mut cudnnPoolingMode_t,
        maxpoolingNanOpt: *mut cudnnNanPropagation_t,
        windowHeight: *mut ::core::ffi::c_int,
        windowWidth: *mut ::core::ffi::c_int,
        verticalPadding: *mut ::core::ffi::c_int,
        horizontalPadding: *mut ::core::ffi::c_int,
        verticalStride: *mut ::core::ffi::c_int,
        horizontalStride: *mut ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures an N-dimensional pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in,out]  -     Pooling descriptor to configure.\n* `mode` [in]  -            Pooling mode.\n* `maxpoolingNanOpt` [in]  - NaN propagation policy for max pooling.\n* `nbDims` [in]  -          Number of dimensions.\n* `windowDimA` [in]  -      Array of pooling window sizes (length nbDims).\n* `paddingA` [in]  -        Array of padding sizes (length nbDims).\n* `strideA` [in]  -         Array of strides (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetPoolingNdDescriptor`]"]
    pub fn cudnnSetPoolingNdDescriptor(
        poolingDesc: cudnnPoolingDescriptor_t,
        mode: cudnnPoolingMode_t,
        maxpoolingNanOpt: cudnnNanPropagation_t,
        nbDims: ::core::ffi::c_int,
        windowDimA: *const ::core::ffi::c_int,
        paddingA: *const ::core::ffi::c_int,
        strideA: *const ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of an N-dimensional pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in]  -     Pooling descriptor to query.\n* `nbDimsRequested` [in]  - Number of dimensions to retrieve.\n* `mode` [out]  -            Pooling mode.\n* `maxpoolingNanOpt` [out]  - NaN propagation policy.\n* `nbDims` [out]  -          Actual number of dimensions.\n* `windowDimA` [out]  -      Array to receive window sizes.\n* `paddingA` [out]  -        Array to receive padding sizes.\n* `strideA` [out]  -         Array to receive strides.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetPoolingNdDescriptor`]"]
    pub fn cudnnGetPoolingNdDescriptor(
        poolingDesc: cudnnPoolingDescriptor_t,
        nbDimsRequested: ::core::ffi::c_int,
        mode: *mut cudnnPoolingMode_t,
        maxpoolingNanOpt: *mut cudnnNanPropagation_t,
        nbDims: *mut ::core::ffi::c_int,
        windowDimA: *mut ::core::ffi::c_int,
        paddingA: *mut ::core::ffi::c_int,
        strideA: *mut ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes the output dimensions of an N-dimensional pooling operation.\n\n# Arguments\n\n* `poolingDesc` [in]  -      Pooling descriptor.\n* `inputTensorDesc` [in]  -  Input tensor descriptor.\n* `nbDims` [in]  -           Number of dimensions.\n* `outputTensorDimA` [out]  - Array to receive output dimension sizes.\n@retval CUDNN_STATUS_SUCCESS     The dimensions were computed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
    pub fn cudnnGetPoolingNdForwardOutputDim(poolingDesc: cudnnPoolingDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, nbDims: ::core::ffi::c_int, outputTensorDimA: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes the output dimensions of a 2D pooling operation.\n\n# Arguments\n\n* `poolingDesc` [in]  -     Pooling descriptor.\n* `inputTensorDesc` [in]  - Input tensor descriptor.\n* `n` [out]  -               Output batch size.\n* `c` [out]  -               Output number of channels.\n* `h` [out]  -               Output height.\n* `w` [out]  -               Output width.\n@retval CUDNN_STATUS_SUCCESS     The dimensions were computed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
    pub fn cudnnGetPooling2dForwardOutputDim(poolingDesc: cudnnPoolingDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, n: *mut ::core::ffi::c_int, c: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int, w: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in]  -  Pooling descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreatePoolingDescriptor`]"]
    pub fn cudnnDestroyPoolingDescriptor(poolingDesc: cudnnPoolingDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs forward pooling.\nComputes y = alpha * pool(x) + beta * y.\n\n# Arguments\n\n* `handle` [in]  -      cuDNN library handle.\n* `poolingDesc` [in]  - Pooling descriptor.\n* `alpha` [in]  -       Scaling factor for the pooling result.\n* `xDesc` [in]  -       Input tensor descriptor.\n* `x` [in]  -           Pointer to input tensor data.\n* `beta` [in]  -        Scaling factor for the destination tensor.\n* `yDesc` [in]  -       Output tensor descriptor.\n* `y` [in,out]  -           Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnPoolingBackward`]"]
    pub fn cudnnPoolingForward(
        handle: cudnnHandle_t,
        poolingDesc: cudnnPoolingDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates an activation descriptor.\n\n# Arguments\n\n* `activationDesc` [out]  -  Pointer to the newly created activation descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyActivationDescriptor`]"]
    pub fn cudnnCreateActivationDescriptor(activationDesc: *mut cudnnActivationDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures an activation descriptor.\n\n# Arguments\n\n* `activationDesc` [in,out]  -  Activation descriptor to configure.\n* `mode` [in]  -            Activation function type.\n* `reluNanOpt` [in]  -      NaN propagation policy for ReLU.\n* `coef` [in]  -            Ceiling for clipped ReLU, or alpha for ELU.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetActivationDescriptor`]"]
    pub fn cudnnSetActivationDescriptor(activationDesc: cudnnActivationDescriptor_t, mode: cudnnActivationMode_t, reluNanOpt: cudnnNanPropagation_t, coef: f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of an activation descriptor.\n\n# Arguments\n\n* `activationDesc` [in]  -  Activation descriptor to query.\n* `mode` [out]  -            Activation function type.\n* `reluNanOpt` [out]  -      NaN propagation policy.\n* `coef` [out]  -            Ceiling for clipped ReLU, or alpha for ELU.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetActivationDescriptor`]"]
    pub fn cudnnGetActivationDescriptor(activationDesc: cudnnActivationDescriptor_t, mode: *mut cudnnActivationMode_t, reluNanOpt: *mut cudnnNanPropagation_t, coef: *mut f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Sets the beta parameter for Swish activation.\n\n# Arguments\n\n* `activationDesc` [in,out]  -  Activation descriptor to modify.\n* `swish_beta` [in]  -      Beta value for the Swish activation function.\n@retval CUDNN_STATUS_SUCCESS     The parameter was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetActivationDescriptorSwishBeta`]"]
    pub fn cudnnSetActivationDescriptorSwishBeta(activationDesc: cudnnActivationDescriptor_t, swish_beta: f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the beta parameter for Swish activation.\n\n# Arguments\n\n* `activationDesc` [in]  -  Activation descriptor to query.\n* `swish_beta` [out]  -      Beta value for the Swish activation function.\n@retval CUDNN_STATUS_SUCCESS  The parameter was retrieved successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetActivationDescriptorSwishBeta`]"]
    pub fn cudnnGetActivationDescriptorSwishBeta(activationDesc: cudnnActivationDescriptor_t, swish_beta: *mut f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys an activation descriptor.\n\n# Arguments\n\n* `activationDesc` [in]  -  Activation descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateActivationDescriptor`]"]
    pub fn cudnnDestroyActivationDescriptor(activationDesc: cudnnActivationDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs forward activation.\nComputes y = alpha * activation(x) + beta * y.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN library handle.\n* `activationDesc` [in]  -  Activation descriptor.\n* `alpha` [in]  -           Scaling factor for the activation result.\n* `xDesc` [in]  -           Input tensor descriptor.\n* `x` [in]  -               Pointer to input tensor data.\n* `beta` [in]  -            Scaling factor for the destination tensor.\n* `yDesc` [in]  -           Output tensor descriptor.\n* `y` [in,out]  -               Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnActivationBackward`]"]
    pub fn cudnnActivationForward(
        handle: cudnnHandle_t,
        activationDesc: cudnnActivationDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a Local Response Normalization (LRN) descriptor.\nUses lrnN=5, lrnAlpha=1e-4, lrnBeta=0.75, lrnK=2.0 as defaults from\nKrizhevsky'12 ImageNet paper.\n\n# Arguments\n\n* `normDesc` [out]  -  Pointer to the newly created LRN descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroyLRNDescriptor,`] cudnnSetLRNDescriptor"]
    pub fn cudnnCreateLRNDescriptor(normDesc: *mut cudnnLRNDescriptor_t) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Selects the Local Response Normalization (LRN) mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnLRNMode_t {
    #[doc = "< LRN computed across tensor dimension dimA[1]. > **Since** cuDNN 9.0.0"]
    CUDNN_LRN_CROSS_CHANNEL_DIM1 = 0,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures an LRN descriptor.\nUses a window [center-lookBehind, center+lookAhead], where\nlookBehind = floor((lrnN-1)/2), lookAhead = lrnN-lookBehind-1.\nValues of double parameters are cast to the tensor data type.\n\n# Arguments\n\n* `normDesc` [in,out]  -  LRN descriptor to configure.\n* `lrnN` [in]  -      Normalization window size (must be in [CUDNN_LRN_MIN_N, CUDNN_LRN_MAX_N]).\n* `lrnAlpha` [in]  -  Alpha parameter (must be >= CUDNN_LRN_MIN_K).\n* `lrnBeta` [in]  -   Beta parameter (must be >= CUDNN_LRN_MIN_BETA).\n* `lrnK` [in]  -      K parameter.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnGetLRNDescriptor`]"]
    pub fn cudnnSetLRNDescriptor(normDesc: cudnnLRNDescriptor_t, lrnN: ::core::ffi::c_uint, lrnAlpha: f64, lrnBeta: f64, lrnK: f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of an LRN descriptor.\nAny of the output pointers can be NULL (the corresponding value will not be returned).\n\n# Arguments\n\n* `normDesc` [in]  -  LRN descriptor to query.\n* `lrnN` [out]  -      Normalization window size.\n* `lrnAlpha` [out]  -  Alpha parameter.\n* `lrnBeta` [out]  -   Beta parameter.\n* `lrnK` [out]  -      K parameter.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetLRNDescriptor`]"]
    pub fn cudnnGetLRNDescriptor(normDesc: cudnnLRNDescriptor_t, lrnN: *mut ::core::ffi::c_uint, lrnAlpha: *mut f64, lrnBeta: *mut f64, lrnK: *mut f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys an LRN descriptor.\n\n# Arguments\n\n* `lrnDesc` [in]  -  LRN descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateLRNDescriptor`]"]
    pub fn cudnnDestroyLRNDescriptor(lrnDesc: cudnnLRNDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs forward LRN cross-channel normalization.\nComputes y = alpha * normalize(x) + beta * y. Double parameters are cast\nto the tensor data type.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `normDesc` [in]  -  LRN descriptor.\n* `lrnMode` [in]  -   LRN mode.\n* `alpha` [in]  -     Scaling factor for the normalization result.\n* `xDesc` [in]  -     Input tensor descriptor.\n* `x` [in]  -         Pointer to input tensor data.\n* `beta` [in]  -      Scaling factor for the destination tensor.\n* `yDesc` [in]  -     Output tensor descriptor.\n* `y` [in,out]  -         Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnLRNCrossChannelBackward`]"]
    pub fn cudnnLRNCrossChannelForward(
        handle: cudnnHandle_t,
        normDesc: cudnnLRNDescriptor_t,
        lrnMode: cudnnLRNMode_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Selects the divisive normalization mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnDivNormMode_t {
    #[doc = "< Use precomputed means for divisive normalization. > **Since** cuDNN 9.0.0"]
    CUDNN_DIVNORM_PRECOMPUTED_MEANS = 0,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs forward divisive normalization.\nComputes y = alpha * normalize(x) + beta * y. If means is NULL, means are\nassumed to be zero. The xDesc is used for means, temp, and temp2 as well.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `normDesc` [in]  -  LRN descriptor (shared with LRN functions).\n* `mode` [in]  -      Divisive normalization mode.\n* `alpha` [in]  -     Scaling factor for the normalization result.\n* `xDesc` [in]  -     Input tensor descriptor (also used for means, temp, temp2).\n* `x` [in]  -         Pointer to input tensor data.\n* `means` [in]  -     Pointer to means tensor data (NULL for zero means).\n* `temp` [out]  -      Temporary workspace tensor.\n* `temp2` [out]  -     Temporary workspace tensor.\n* `beta` [in]  -      Scaling factor for the destination tensor.\n* `yDesc` [in]  -     Output tensor descriptor.\n* `y` [in,out]  -         Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDivisiveNormalizationBackward`]"]
    pub fn cudnnDivisiveNormalizationForward(
        handle: cudnnHandle_t,
        normDesc: cudnnLRNDescriptor_t,
        mode: cudnnDivNormMode_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        means: *const ::core::ffi::c_void,
        temp: *mut ::core::ffi::c_void,
        temp2: *mut ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Selects the batch normalization mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBatchNormMode_t {
    #[doc = "< Per-activation: bnScale/bnBias shape 1xCxHxW, normalized over N. > **Since** cuDNN 9.0.0"]
    CUDNN_BATCHNORM_PER_ACTIVATION = 0,
    #[doc = "< Spatial: bnScale/bnBias shape 1xCx1x1, normalized over N+spatial dims. > **Since** cuDNN 9.0.0"]
    CUDNN_BATCHNORM_SPATIAL = 1,
    #[doc = "< Like SPATIAL but faster via scaled atomic int reduction. NCHW, CC>=6.0. > **Since** cuDNN 9.0.0"]
    CUDNN_BATCHNORM_SPATIAL_PERSISTENT = 2,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Derives a tensor descriptor for batch normalization parameters.\nComputes the dimensions for bnScale, bnBias, mean, and variance tensors based\non the input tensor descriptor and batch normalization mode. Use this for\nbnScaleBiasMeanVarDesc and bnScaleBiasDiffDesc parameters.\n\n# Arguments\n\n* `derivedBnDesc` [in,out]  -  Tensor descriptor to be derived.\n* `xDesc` [in]  -          Input tensor descriptor.\n* `mode` [in]  -           Batch normalization mode.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was derived successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTraining`]"]
    pub fn cudnnDeriveBNTensorDescriptor(derivedBnDesc: cudnnTensorDescriptor_t, xDesc: cudnnTensorDescriptor_t, mode: cudnnBatchNormMode_t) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Selects the extended batch normalization operation mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnBatchNormOps_t {
    #[doc = "< Batch normalization only. > **Since** cuDNN 9.0.0"]
    CUDNN_BATCHNORM_OPS_BN = 0,
    #[doc = "< Batch normalization followed by activation. > **Since** cuDNN 9.0.0"]
    CUDNN_BATCHNORM_OPS_BN_ACTIVATION = 1,
    #[doc = "< Batch normalization, element-wise add, then activation. > **Since** cuDNN 9.0.0"]
    CUDNN_BATCHNORM_OPS_BN_ADD_ACTIVATION = 2,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs batch normalization during inference.\nComputes y[i] = bnScale[k]*(x[i]-estimatedMean[k])/sqrt(epsilon+estimatedVariance[k]) + bnBias[k],\nwith tensors indexed according to spatial or per-activation mode.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `mode` [in]  -                    Batch normalization mode.\n* `alpha` [in]  -                   Result blend factor.\n* `beta` [in]  -                    Destination layer blend factor.\n* `xDesc` [in]  -                   Input tensor descriptor.\n* `x` [in]  -                       Pointer to input tensor data (NxCxHxW).\n* `yDesc` [in]  -                   Output tensor descriptor.\n* `y` [in,out]  -                       Pointer to output tensor data (NxCxHxW).\n* `bnScaleBiasMeanVarDesc` [in]  -  Descriptor for scale, bias, mean, variance tensors.\n* `bnScale` [in]  -                 Pointer to scale (gamma) tensor data.\n* `bnBias` [in]  -                  Pointer to bias (beta) tensor data.\n* `estimatedMean` [in]  -           Pointer to running mean tensor data.\n* `estimatedVariance` [in]  -       Pointer to running variance tensor data.\n* `epsilon` [in]  -                 Epsilon value (must be >= CUDNN_BN_MIN_EPSILON).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTraining,`] cudnnDeriveBNTensorDescriptor"]
    pub fn cudnnBatchNormalizationForwardInference(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        bnBias: *const ::core::ffi::c_void,
        estimatedMean: *const ::core::ffi::c_void,
        estimatedVariance: *const ::core::ffi::c_void,
        epsilon: f64,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Selects the normalization mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnNormMode_t {
    #[doc = "< Norm per activation. > **Since** cuDNN 9.0.0"]
    CUDNN_NORM_PER_ACTIVATION = 0,
    #[doc = "< Norm per channel. > **Since** cuDNN 9.0.0"]
    CUDNN_NORM_PER_CHANNEL = 1,
}
#[repr(u32)]
#[doc = "Selects the normalization algorithm.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnNormAlgo_t {
    #[doc = "< Standard normalization algorithm. > **Since** cuDNN UNPUBLISHED"]
    CUDNN_NORM_ALGO_STANDARD = 0,
    #[doc = "< Persistent normalization (requires compute capability 6.0+). > **Since** cuDNN UNPUBLISHED"]
    CUDNN_NORM_ALGO_PERSIST = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Derives tensor descriptors for normalization parameters.\nComputes the dimensions for normScale, normBias, mean, and variance tensors based\non the input tensor descriptor and normalization mode.\n\n# Arguments\n\n* `derivedNormScaleBiasDesc` [in,out]  -  Descriptor to be derived for scale/bias tensors.\n* `derivedNormMeanVarDesc` [in,out]  -    Descriptor to be derived for mean/variance tensors.\n* `xDesc` [in]  -                     Input tensor descriptor.\n* `mode` [in]  -                      Normalization mode.\n* `groupCnt` [in]  -                  Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS     The descriptors were derived successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining`]"]
    pub fn cudnnDeriveNormTensorDescriptor(derivedNormScaleBiasDesc: cudnnTensorDescriptor_t, derivedNormMeanVarDesc: cudnnTensorDescriptor_t, xDesc: cudnnTensorDescriptor_t, mode: cudnnNormMode_t, groupCnt: ::core::ffi::c_int) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Selects the extended normalization operation mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnNormOps_t {
    #[doc = "< Normalization only. > **Since** cuDNN 9.0.0"]
    CUDNN_NORM_OPS_NORM = 0,
    #[doc = "< Normalization followed by activation. > **Since** cuDNN 9.0.0"]
    CUDNN_NORM_OPS_NORM_ACTIVATION = 1,
    #[doc = "< Normalization, element-wise add, then activation. > **Since** cuDNN 9.0.0"]
    CUDNN_NORM_OPS_NORM_ADD_ACTIVATION = 2,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs normalization during inference.\nComputes y[i] = normScale[k]*(x[i]-estimatedMean[k])/sqrt(epsilon+estimatedVariance[k]) + normBias[k],\nwith tensors indexed according to per-channel or per-activation mode.\n\n# Arguments\n\n* `handle` [in]  -             cuDNN library handle.\n* `mode` [in]  -               Normalization mode.\n* `normOps` [in]  -            Extended normalization operation mode.\n* `algo` [in]  -               Normalization algorithm.\n* `alpha` [in]  -              Result blend factor.\n* `beta` [in]  -               Destination layer blend factor.\n* `xDesc` [in]  -              Input tensor descriptor.\n* `x` [in]  -                  Pointer to input tensor data (NxCxHxW).\n* `normScaleBiasDesc` [in]  -  Descriptor for normalization scale/bias tensors.\n* `normScale` [in]  -          Pointer to normalization scale tensor data.\n* `normBias` [in]  -           Pointer to normalization bias tensor data.\n* `normMeanVarDesc` [in]  -    Descriptor for mean/variance tensors.\n* `estimatedMean` [in]  -      Pointer to running mean tensor data.\n* `estimatedVariance` [in]  -  Pointer to running variance tensor data.\n* `zDesc` [in]  -              Descriptor for z tensor (used with add operations).\n* `z` [in]  -                  Pointer to z tensor data.\n* `activationDesc` [in]  -     Activation descriptor (used with activation operations).\n* `yDesc` [in]  -              Output tensor descriptor.\n* `y` [in,out]  -                  Pointer to output tensor data (NxCxHxW).\n* `epsilon` [in]  -            Epsilon value (must be >= 0).\n* `groupCnt` [in]  -           Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining,`] cudnnDeriveNormTensorDescriptor"]
    pub fn cudnnNormalizationForwardInference(
        handle: cudnnHandle_t,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        normScaleBiasDesc: cudnnTensorDescriptor_t,
        normScale: *const ::core::ffi::c_void,
        normBias: *const ::core::ffi::c_void,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        estimatedMean: *const ::core::ffi::c_void,
        estimatedVariance: *const ::core::ffi::c_void,
        zDesc: cudnnTensorDescriptor_t,
        z: *const ::core::ffi::c_void,
        activationDesc: cudnnActivationDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        epsilon: f64,
        groupCnt: ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Selects the spatial sampler type for spatial transformer networks.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnSamplerType_t {
    #[doc = "< Bilinear sampler. > **Since** cuDNN 9.0.0"]
    CUDNN_SAMPLER_BILINEAR = 0,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a spatial transformer descriptor.\n\n# Arguments\n\n* `stDesc` [out]  -  Pointer to the newly created spatial transformer descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroySpatialTransformerDescriptor`]"]
    pub fn cudnnCreateSpatialTransformerDescriptor(stDesc: *mut cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures an N-dimensional spatial transformer descriptor.\n\n# Arguments\n\n* `stDesc` [in,out]  -      Spatial transformer descriptor to configure.\n* `samplerType` [in]  - Type of sampler to use.\n* `dataType` [in]  -    Data type of the tensors.\n* `nbDims` [in]  -      Number of dimensions.\n* `dimA` [in]  -        Array of dimension sizes (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfGridGeneratorForward,`] cudnnSpatialTfSamplerForward"]
    pub fn cudnnSetSpatialTransformerNdDescriptor(stDesc: cudnnSpatialTransformerDescriptor_t, samplerType: cudnnSamplerType_t, dataType: cudnnDataType_t, nbDims: ::core::ffi::c_int, dimA: *const ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a spatial transformer descriptor.\n\n# Arguments\n\n* `stDesc` [in]  -  Spatial transformer descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateSpatialTransformerDescriptor`]"]
    pub fn cudnnDestroySpatialTransformerDescriptor(stDesc: cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Generates a sampling grid for a spatial transformer (forward).\nGenerates a grid of sampling coordinates from the affine transformation matrix theta.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `stDesc` [in]  -  Spatial transformer descriptor.\n* `theta` [in]  -   Pointer to affine transformation matrices.\n* `grid` [out]  -    Pointer to output sampling grid data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfGridGeneratorBackward`]"]
    pub fn cudnnSpatialTfGridGeneratorForward(handle: cudnnHandle_t, stDesc: cudnnSpatialTransformerDescriptor_t, theta: *const ::core::ffi::c_void, grid: *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs spatial transformer sampling (forward).\nSamples the input tensor at the grid coordinates to produce the output tensor.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `stDesc` [in]  -  Spatial transformer descriptor.\n* `alpha` [in]  -   Scaling factor for the sampled result.\n* `xDesc` [in]  -   Input tensor descriptor.\n* `x` [in]  -       Pointer to input tensor data.\n* `grid` [in]  -    Pointer to sampling grid data.\n* `beta` [in]  -    Scaling factor for the destination tensor.\n* `yDesc` [in]  -   Output tensor descriptor.\n* `y` [in,out]  -       Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfSamplerBackward`]"]
    pub fn cudnnSpatialTfSamplerForward(
        handle: cudnnHandle_t,
        stDesc: cudnnSpatialTransformerDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        grid: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnDropoutStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for dropout operations. > **Since** cuDNN 9.0.0"]
pub type cudnnDropoutDescriptor_t = *mut cudnnDropoutStruct;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a dropout descriptor.\n\n# Arguments\n\n* `dropoutDesc` [out]  -  Pointer to the newly created dropout descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroyDropoutDescriptor`]"]
    pub fn cudnnCreateDropoutDescriptor(dropoutDesc: *mut cudnnDropoutDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a dropout descriptor.\n\n# Arguments\n\n* `dropoutDesc` [in]  -  Dropout descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateDropoutDescriptor`]"]
    pub fn cudnnDestroyDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the size of the states buffer required for dropout.\n\n# Arguments\n\n* `handle` [in]  -      cuDNN library handle.\n* `sizeInBytes` [out]  - Size of the required states buffer in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetDropoutDescriptor`]"]
    pub fn cudnnDropoutGetStatesSize(handle: cudnnHandle_t, sizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the size of the reserve space required for dropout forward/backward.\n\n# Arguments\n\n* `xdesc` [in]  -       Input tensor descriptor.\n* `sizeInBytes` [out]  - Size of the required reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDropoutForward,`] cudnnDropoutBackward"]
    pub fn cudnnDropoutGetReserveSpaceSize(xdesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures a dropout descriptor and initializes random state.\n\n# Arguments\n\n* `dropoutDesc` [in,out]  -      Dropout descriptor to configure.\n* `handle` [in]  -           cuDNN library handle.\n* `dropout` [in]  -          Probability of dropping (0 = no dropout, 1 = all dropped).\n* `states` [in,out]  -           Pointer to device memory for RNG state storage.\n* `stateSizeInBytes` [in]  - Size of the states buffer in bytes.\n* `seed` [in]  -             Seed for the random number generator.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was configured successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnGetDropoutDescriptor,`] cudnnRestoreDropoutDescriptor"]
    pub fn cudnnSetDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t, handle: cudnnHandle_t, dropout: f32, states: *mut ::core::ffi::c_void, stateSizeInBytes: usize, seed: ::core::ffi::c_ulonglong) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Restores a dropout descriptor to a previously saved state.\n\n# Arguments\n\n* `dropoutDesc` [in,out]  -      Dropout descriptor to restore.\n* `handle` [in]  -           cuDNN library handle.\n* `dropout` [in]  -          Dropout probability.\n* `states` [in]  -           Pointer to previously saved RNG state.\n* `stateSizeInBytes` [in]  - Size of the states buffer in bytes.\n* `seed` [in]  -             Seed used to initialize the original state.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was restored successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetDropoutDescriptor`]"]
    pub fn cudnnRestoreDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t, handle: cudnnHandle_t, dropout: f32, states: *mut ::core::ffi::c_void, stateSizeInBytes: usize, seed: ::core::ffi::c_ulonglong) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves the settings of a dropout descriptor.\n\n# Arguments\n\n* `dropoutDesc` [in]  -  Dropout descriptor to query.\n* `handle` [in]  -       cuDNN library handle.\n* `dropout` [out]  -      Dropout probability.\n* `states` [out]  -       Pointer to RNG state memory.\n* `seed` [out]  -         Seed used for the RNG.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetDropoutDescriptor`]"]
    pub fn cudnnGetDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t, handle: cudnnHandle_t, dropout: *mut f32, states: *mut *mut ::core::ffi::c_void, seed: *mut ::core::ffi::c_ulonglong) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs forward dropout.\nRandomly sets elements to zero based on the dropout probability. The reserve\nspace stores the mask for use in the backward pass.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `dropoutDesc` [in]  -             Dropout descriptor.\n* `xdesc` [in]  -                   Input tensor descriptor.\n* `x` [in]  -                       Pointer to input tensor data.\n* `ydesc` [in]  -                   Output tensor descriptor.\n* `y` [out]  -                       Pointer to output tensor data.\n* `reserveSpace` [out]  -            Pointer to reserve space for the dropout mask.\n* `reserveSpaceSizeInBytes` [in]  - Size of reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDropoutBackward,`] cudnnDropoutGetReserveSpaceSize"]
    pub fn cudnnDropoutForward(
        handle: cudnnHandle_t,
        dropoutDesc: cudnnDropoutDescriptor_t,
        xdesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        ydesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Enumerates convolution forward algorithms.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnConvolutionFwdAlgo_t {
    #[doc = "< Implicit GEMM: matrix product without forming input matrix. No extra workspace. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM = 0,
    #[doc = "< Implicit GEMM with precomputed indices. Needs workspace for index precomputation. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM = 1,
    #[doc = "< Explicit GEMM: forms input matrix explicitly. Requires significant workspace. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_FWD_ALGO_GEMM = 2,
    #[doc = "< Direct convolution without matrix multiplication. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_FWD_ALGO_DIRECT = 3,
    #[doc = "< FFT-based convolution. Requires significant workspace. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_FWD_ALGO_FFT = 4,
    #[doc = "< FFT with tiled inputs. Significant workspace but less than FFT for large inputs. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_FWD_ALGO_FFT_TILING = 5,
    #[doc = "< Winograd transform. Moderate workspace. Not supported on Hopper+. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD = 6,
    #[doc = "< Winograd non-fused variant. May require significant workspace. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD_NONFUSED = 7,
    #[doc = "< Number of forward convolution algorithms. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_FWD_ALGO_COUNT = 8,
}
#[repr(u32)]
#[doc = "Enumerates convolution backward filter algorithms.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnConvolutionBwdFilterAlgo_t {
    #[doc = "< Sum of matrix products with atomic adds. Non-deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_FILTER_ALGO_0 = 0,
    #[doc = "< Implicit GEMM without forming input matrix. Deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_FILTER_ALGO_1 = 1,
    #[doc = "< FFT-based. Significant workspace. Deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT = 2,
    #[doc = "< Like ALGO_0 with precomputed indices. Non-deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_FILTER_ALGO_3 = 3,
    #[doc = "< Winograd transform (not implemented). > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD = 4,
    #[doc = "< Winograd non-fused. Significant workspace. Deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD_NONFUSED = 5,
    #[doc = "< FFT with tiling. Significant workspace. Deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT_TILING = 6,
    #[doc = "< Number of backward filter algorithms. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_FILTER_ALGO_COUNT = 7,
}
#[repr(u32)]
#[doc = "Enumerates convolution backward data algorithms.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnConvolutionBwdDataAlgo_t {
    #[doc = "< Sum of matrix products with atomic adds. Non-deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_DATA_ALGO_0 = 0,
    #[doc = "< Implicit GEMM without forming input matrix. Deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_DATA_ALGO_1 = 1,
    #[doc = "< FFT-based. Significant workspace. Deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT = 2,
    #[doc = "< FFT with tiling. Significant workspace. Deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT_TILING = 3,
    #[doc = "< Winograd transform. Moderate workspace. Deterministic. Not on Hopper+. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD = 4,
    #[doc = "< Winograd non-fused. Significant workspace. Deterministic. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD_NONFUSED = 5,
    #[doc = "< Number of backward data algorithms. > **Since** cuDNN 9.0.0"]
    CUDNN_CONVOLUTION_BWD_DATA_ALGO_COUNT = 6,
}
#[repr(u32)]
#[doc = "Enumerates CTC loss computation algorithms.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnCTCLossAlgo_t {
    #[doc = "< Deterministic CTC loss. > **Since** cuDNN UNPUBLISHED"]
    CUDNN_CTC_LOSS_ALGO_DETERMINISTIC = 0,
    #[doc = "< Non-deterministic CTC loss. > **Since** cuDNN UNPUBLISHED"]
    CUDNN_CTC_LOSS_ALGO_NON_DETERMINISTIC = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Cross-library version checker for the ops sub-library.\nThis function is implemented differently in each sub-library. Each sub-library\nchecks whether its own version matches that of its dependencies.\n@retval CUDNN_STATUS_SUCCESS                       The version check passed.\n@retval CUDNN_STATUS_SUBLIBRARY_VERSION_MISMATCH   The versions are inconsistent.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnOpsVersionCheck() -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs backward softmax computation.\nComputes the gradient of the softmax function.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `algo` [in]  -    Softmax algorithm used in the forward pass.\n* `mode` [in]  -    Softmax computation scope.\n* `alpha` [in]  -   Scaling factor for the result.\n* `yDesc` [in]  -   Output tensor descriptor (from forward pass).\n* `y` [in]  -       Pointer to output tensor data (from forward pass).\n* `dyDesc` [in]  -  Output gradient tensor descriptor.\n* `dy` [in]  -      Pointer to output gradient tensor data.\n* `beta` [in]  -    Scaling factor for the destination tensor.\n* `dxDesc` [in]  -  Input gradient tensor descriptor.\n* `dx` [in,out]  -      Pointer to input gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSoftmaxForward`]"]
    pub fn cudnnSoftmaxBackward(
        handle: cudnnHandle_t,
        algo: cudnnSoftmaxAlgorithm_t,
        mode: cudnnSoftmaxMode_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs backward pooling.\nComputes the gradient of the pooling operation.\n\n# Arguments\n\n* `handle` [in]  -      cuDNN library handle.\n* `poolingDesc` [in]  - Pooling descriptor.\n* `alpha` [in]  -       Scaling factor for the result.\n* `yDesc` [in]  -       Output tensor descriptor (from forward pass).\n* `y` [in]  -           Pointer to output tensor data (from forward pass).\n* `dyDesc` [in]  -      Output gradient tensor descriptor.\n* `dy` [in]  -          Pointer to output gradient tensor data.\n* `xDesc` [in]  -       Input tensor descriptor (from forward pass).\n* `x` [in]  -           Pointer to input tensor data (from forward pass).\n* `beta` [in]  -        Scaling factor for the destination tensor.\n* `dxDesc` [in]  -      Input gradient tensor descriptor.\n* `dx` [in,out]  -          Pointer to input gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnPoolingForward`]"]
    pub fn cudnnPoolingBackward(
        handle: cudnnHandle_t,
        poolingDesc: cudnnPoolingDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs backward activation.\nComputes the gradient of the activation function.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN library handle.\n* `activationDesc` [in]  -  Activation descriptor.\n* `alpha` [in]  -           Scaling factor for the result.\n* `yDesc` [in]  -           Output tensor descriptor (from forward pass).\n* `y` [in]  -               Pointer to output tensor data (from forward pass).\n* `dyDesc` [in]  -          Output gradient tensor descriptor.\n* `dy` [in]  -              Pointer to output gradient tensor data.\n* `xDesc` [in]  -           Input tensor descriptor (from forward pass).\n* `x` [in]  -               Pointer to input tensor data (from forward pass).\n* `beta` [in]  -            Scaling factor for the destination tensor.\n* `dxDesc` [in]  -          Input gradient tensor descriptor.\n* `dx` [in,out]  -              Pointer to input gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnActivationForward`]"]
    pub fn cudnnActivationBackward(
        handle: cudnnHandle_t,
        activationDesc: cudnnActivationDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs backward LRN cross-channel normalization.\nComputes the gradient of the LRN cross-channel normalization. Double\nparameters are cast to the tensor data type.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `normDesc` [in]  -  LRN descriptor.\n* `lrnMode` [in]  -   LRN mode.\n* `alpha` [in]  -     Scaling factor for the result.\n* `yDesc` [in]  -     Output tensor descriptor (from forward pass).\n* `y` [in]  -         Pointer to output tensor data (from forward pass).\n* `dyDesc` [in]  -    Output gradient tensor descriptor.\n* `dy` [in]  -        Pointer to output gradient tensor data.\n* `xDesc` [in]  -     Input tensor descriptor (from forward pass).\n* `x` [in]  -         Pointer to input tensor data (from forward pass).\n* `beta` [in]  -      Scaling factor for the destination tensor.\n* `dxDesc` [in]  -    Input gradient tensor descriptor.\n* `dx` [in,out]  -        Pointer to input gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnLRNCrossChannelForward`]"]
    pub fn cudnnLRNCrossChannelBackward(
        handle: cudnnHandle_t,
        normDesc: cudnnLRNDescriptor_t,
        lrnMode: cudnnLRNMode_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs backward divisive normalization.\nComputes the gradients of the divisive normalization operation. If means is NULL,\nmeans are assumed to be zero.\n\n# Arguments\n\n* `handle` [in]  -       cuDNN library handle.\n* `normDesc` [in]  -     LRN descriptor (shared with LRN functions).\n* `mode` [in]  -         Divisive normalization mode.\n* `alpha` [in]  -        Scaling factor for the result.\n* `xDesc` [in]  -        Input tensor descriptor (also used for means, dy, temp, temp2).\n* `x` [in]  -            Pointer to input tensor data.\n* `means` [in]  -        Pointer to means tensor data (NULL for zero means).\n* `dy` [in]  -           Pointer to output gradient tensor data.\n* `temp` [out]  -         Temporary workspace tensor.\n* `temp2` [out]  -        Temporary workspace tensor.\n* `beta` [in]  -         Scaling factor for the destination tensors.\n* `dXdMeansDesc` [in]  - Descriptor for dx and dMeans tensors.\n* `dx` [in,out]  -           Pointer to input gradient tensor data.\n* `dMeans` [in,out]  -       Pointer to means gradient tensor data (can be NULL).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDivisiveNormalizationForward`]"]
    pub fn cudnnDivisiveNormalizationBackward(
        handle: cudnnHandle_t,
        normDesc: cudnnLRNDescriptor_t,
        mode: cudnnDivNormMode_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        means: *const ::core::ffi::c_void,
        dy: *const ::core::ffi::c_void,
        temp: *mut ::core::ffi::c_void,
        temp2: *mut ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dXdMeansDesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        dMeans: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the workspace size for extended batch normalization forward training.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `mode` [in]  -                    Batch normalization mode.\n* `bnOps` [in]  -                   Extended batch normalization operation.\n* `xDesc` [in]  -                   Input tensor descriptor.\n* `zDesc` [in]  -                   Z tensor descriptor (for add operations).\n* `yDesc` [in]  -                   Output tensor descriptor.\n* `bnScaleBiasMeanVarDesc` [in]  -  Descriptor for BN parameter tensors.\n* `activationDesc` [in]  -          Activation descriptor.\n* `sizeInBytes` [out]  -             Required workspace size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTrainingEx`]"]
    pub fn cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        xDesc: cudnnTensorDescriptor_t,
        zDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the workspace size for extended batch normalization backward.\n\n# Arguments\n\n* `handle` [in]  -            cuDNN library handle.\n* `mode` [in]  -              Batch normalization mode.\n* `bnOps` [in]  -             Extended batch normalization operation.\n* `xDesc` [in]  -             Input tensor descriptor.\n* `yDesc` [in]  -             Output tensor descriptor.\n* `dyDesc` [in]  -            Output gradient tensor descriptor.\n* `dzDesc` [in]  -            Z gradient tensor descriptor.\n* `dxDesc` [in]  -            Input gradient tensor descriptor.\n* `dBnScaleBiasDesc` [in]  -  Descriptor for BN parameter gradient tensors.\n* `activationDesc` [in]  -    Activation descriptor.\n* `sizeInBytes` [out]  -       Required workspace size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationBackwardEx`]"]
    pub fn cudnnGetBatchNormalizationBackwardExWorkspaceSize(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        xDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        dzDesc: cudnnTensorDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        dBnScaleBiasDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the reserve space size for extended batch normalization training.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN library handle.\n* `mode` [in]  -            Batch normalization mode.\n* `bnOps` [in]  -           Extended batch normalization operation.\n* `activationDesc` [in]  -  Activation descriptor.\n* `xDesc` [in]  -           Input tensor descriptor.\n* `sizeInBytes` [out]  -     Required reserve space size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTrainingEx`]"]
    pub fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(handle: cudnnHandle_t, mode: cudnnBatchNormMode_t, bnOps: cudnnBatchNormOps_t, activationDesc: cudnnActivationDescriptor_t, xDesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs batch normalization forward training.\nComputes y = BN(x). Also accumulates moving averages of mean and inverse variances.\n\n# Arguments\n\n* `handle` [in]  -                     cuDNN library handle.\n* `mode` [in]  -                       Batch normalization mode.\n* `alpha` [in]  -                      Result blend factor.\n* `beta` [in]  -                       Destination layer blend factor.\n* `xDesc` [in]  -                      Input tensor descriptor.\n* `x` [in]  -                          Pointer to input tensor data (NxCxHxW).\n* `yDesc` [in]  -                      Output tensor descriptor.\n* `y` [out]  -                          Pointer to output tensor data (NxCxHxW).\n* `bnScaleBiasMeanVarDesc` [in]  -     Descriptor for BN parameter tensors.\n* `bnScale` [in]  -                    Pointer to scale (gamma) tensor data.\n* `bnBias` [in]  -                     Pointer to bias (beta) tensor data.\n* `exponentialAverageFactor` [in]  -   Factor for computing running averages.\n* `resultRunningMean` [in,out]  -          Running mean (updated with exponential average).\n* `resultRunningVariance` [in,out]  -      Running variance (updated with exponential average).\n* `epsilon` [in]  -                    Epsilon value (must be >= CUDNN_BN_MIN_EPSILON).\n* `resultSaveMean` [out]  -             Optionally cached mean for backward pass (NULL if unused).\n* `resultSaveInvVariance` [out]  -      Optionally cached inverse variance for backward pass (NULL if unused).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationBackward,`] cudnnDeriveBNTensorDescriptor"]
    pub fn cudnnBatchNormalizationForwardTraining(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        bnBias: *const ::core::ffi::c_void,
        exponentialAverageFactor: f64,
        resultRunningMean: *mut ::core::ffi::c_void,
        resultRunningVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::core::ffi::c_void,
        resultSaveInvVariance: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs extended batch normalization forward training with optional activation.\nComputes y = relu(BN(x) + z). Also accumulates moving averages of mean and inverse variances.\nSupports fused batch normalization + activation and batch normalization + add + activation.\n\n# Arguments\n\n* `handle` [in]  -                     cuDNN library handle.\n* `mode` [in]  -                       Batch normalization mode.\n* `bnOps` [in]  -                      Extended batch normalization operation.\n* `alpha` [in]  -                      Result blend factor.\n* `beta` [in]  -                       Destination layer blend factor.\n* `xDesc` [in]  -                      Input tensor descriptor.\n* `xData` [in]  -                      Pointer to input tensor data.\n* `zDesc` [in]  -                      Z tensor descriptor (for add operations).\n* `zData` [in]  -                      Pointer to z tensor data.\n* `yDesc` [in]  -                      Output tensor descriptor.\n* `yData` [out]  -                      Pointer to output tensor data.\n* `bnScaleBiasMeanVarDesc` [in]  -     Descriptor for BN parameter tensors.\n* `bnScale` [in]  -                    Pointer to scale tensor data.\n* `bnBias` [in]  -                     Pointer to bias tensor data.\n* `exponentialAverageFactor` [in]  -   Factor for computing running averages.\n* `resultRunningMean` [in,out]  -          Running mean.\n* `resultRunningVariance` [in,out]  -      Running variance.\n* `epsilon` [in]  -                    Epsilon value (must be >= CUDNN_BN_MIN_EPSILON).\n* `resultSaveMean` [out]  -             Cached mean for backward pass (NULL if unused).\n* `resultSaveInvVariance` [out]  -      Cached inverse variance for backward pass (NULL if unused).\n* `activationDesc` [in]  -             Activation descriptor.\n* `workspace` [in,out]  -                  Pointer to workspace memory.\n* `workSpaceSizeInBytes` [in]  -       Size of workspace in bytes.\n* `reserveSpace` [in,out]  -               Pointer to reserve space memory.\n* `reserveSpaceSizeInBytes` [in]  -    Size of reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationBackwardEx,`] cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize"]
    pub fn cudnnBatchNormalizationForwardTrainingEx(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        xData: *const ::core::ffi::c_void,
        zDesc: cudnnTensorDescriptor_t,
        zData: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        yData: *mut ::core::ffi::c_void,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        bnBias: *const ::core::ffi::c_void,
        exponentialAverageFactor: f64,
        resultRunningMean: *mut ::core::ffi::c_void,
        resultRunningVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::core::ffi::c_void,
        resultSaveInvVariance: *mut ::core::ffi::c_void,
        activationDesc: cudnnActivationDescriptor_t,
        workspace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs backward batch normalization.\nComputes gradients for x, bnScale, and bnBias.\n\n# Arguments\n\n* `handle` [in]  -              cuDNN library handle.\n* `mode` [in]  -                Batch normalization mode.\n* `alphaDataDiff` [in]  -       Scaling factor for dx result.\n* `betaDataDiff` [in]  -        Scaling factor for dx destination.\n* `alphaParamDiff` [in]  -      Scaling factor for parameter gradient results.\n* `betaParamDiff` [in]  -       Scaling factor for parameter gradient destinations.\n* `xDesc` [in]  -               Input tensor descriptor (same for x, dx, dy).\n* `x` [in]  -                   Pointer to input tensor data.\n* `dyDesc` [in]  -              Output gradient tensor descriptor.\n* `dy` [in]  -                  Pointer to output gradient tensor data.\n* `dxDesc` [in]  -              Input gradient tensor descriptor.\n* `dx` [in,out]  -                  Pointer to input gradient tensor data.\n* `dBnScaleBiasDesc` [in]  -    Shared descriptor for parameter gradient tensors.\n* `bnScale` [in]  -             Pointer to scale tensor data.\n* `dBnScaleResult` [out]  -      Pointer to scale gradient result.\n* `dBnBiasResult` [out]  -       Pointer to bias gradient result.\n* `epsilon` [in]  -             Same epsilon as forward pass.\n* `savedMean` [in]  -           Optionally cached mean from forward pass.\n* `savedInvVariance` [in]  -    Optionally cached inverse variance from forward pass.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTraining`]"]
    pub fn cudnnBatchNormalizationBackward(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        alphaDataDiff: *const ::core::ffi::c_void,
        betaDataDiff: *const ::core::ffi::c_void,
        alphaParamDiff: *const ::core::ffi::c_void,
        betaParamDiff: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        dBnScaleBiasDesc: cudnnTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        dBnScaleResult: *mut ::core::ffi::c_void,
        dBnBiasResult: *mut ::core::ffi::c_void,
        epsilon: f64,
        savedMean: *const ::core::ffi::c_void,
        savedInvVariance: *const ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs extended backward batch normalization with optional activation.\nComputes gradients for the fused batch normalization + activation operations.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `mode` [in]  -                    Batch normalization mode.\n* `bnOps` [in]  -                   Extended batch normalization operation.\n* `alphaDataDiff` [in]  -           Scaling factor for data gradient results.\n* `betaDataDiff` [in]  -            Scaling factor for data gradient destinations.\n* `alphaParamDiff` [in]  -          Scaling factor for parameter gradient results.\n* `betaParamDiff` [in]  -           Scaling factor for parameter gradient destinations.\n* `xDesc` [in]  -                   Input tensor descriptor.\n* `xData` [in]  -                   Pointer to input tensor data.\n* `yDesc` [in]  -                   Output tensor descriptor.\n* `yData` [in]  -                   Pointer to output tensor data.\n* `dyDesc` [in]  -                  Output gradient tensor descriptor.\n* `dyData` [in]  -                  Pointer to output gradient tensor data.\n* `dzDesc` [in]  -                  Z gradient tensor descriptor.\n* `dzData` [in,out]  -                  Pointer to z gradient tensor data.\n* `dxDesc` [in]  -                  Input gradient tensor descriptor.\n* `dxData` [in,out]  -                  Pointer to input gradient tensor data.\n* `dBnScaleBiasDesc` [in]  -        Shared descriptor for parameter gradient tensors.\n* `bnScaleData` [in]  -             Pointer to scale tensor data.\n* `bnBiasData` [in]  -              Pointer to bias tensor data (needed for activation).\n* `dBnScaleData` [out]  -            Pointer to scale gradient result.\n* `dBnBiasData` [out]  -             Pointer to bias gradient result.\n* `epsilon` [in]  -                 Same epsilon as forward pass.\n* `savedMean` [in]  -               Optionally cached mean from forward pass.\n* `savedInvVariance` [in]  -        Optionally cached inverse variance from forward pass.\n* `activationDesc` [in]  -          Activation descriptor.\n* `workSpace` [in,out]  -               Pointer to workspace memory.\n* `workSpaceSizeInBytes` [in]  -    Size of workspace in bytes.\n* `reserveSpace` [in,out]  -            Pointer to reserve space memory.\n* `reserveSpaceSizeInBytes` [in]  - Size of reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTrainingEx`]"]
    pub fn cudnnBatchNormalizationBackwardEx(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        alphaDataDiff: *const ::core::ffi::c_void,
        betaDataDiff: *const ::core::ffi::c_void,
        alphaParamDiff: *const ::core::ffi::c_void,
        betaParamDiff: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        xData: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        yData: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dyData: *const ::core::ffi::c_void,
        dzDesc: cudnnTensorDescriptor_t,
        dzData: *mut ::core::ffi::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dxData: *mut ::core::ffi::c_void,
        dBnScaleBiasDesc: cudnnTensorDescriptor_t,
        bnScaleData: *const ::core::ffi::c_void,
        bnBiasData: *const ::core::ffi::c_void,
        dBnScaleData: *mut ::core::ffi::c_void,
        dBnBiasData: *mut ::core::ffi::c_void,
        epsilon: f64,
        savedMean: *const ::core::ffi::c_void,
        savedInvVariance: *const ::core::ffi::c_void,
        activationDesc: cudnnActivationDescriptor_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the workspace size for normalization forward training.\n\n# Arguments\n\n* `handle` [in]  -             cuDNN library handle.\n* `mode` [in]  -               Normalization mode.\n* `normOps` [in]  -            Extended normalization operation.\n* `algo` [in]  -               Normalization algorithm.\n* `xDesc` [in]  -              Input tensor descriptor.\n* `zDesc` [in]  -              Z tensor descriptor (for add operations).\n* `yDesc` [in]  -              Output tensor descriptor.\n* `normScaleBiasDesc` [in]  -  Descriptor for normalization scale/bias tensors.\n* `activationDesc` [in]  -     Activation descriptor.\n* `normMeanVarDesc` [in]  -    Descriptor for mean/variance tensors.\n* `sizeInBytes` [out]  -        Required workspace size in bytes.\n* `groupCnt` [in]  -           Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining`]"]
    pub fn cudnnGetNormalizationForwardTrainingWorkspaceSize(
        handle: cudnnHandle_t,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        xDesc: cudnnTensorDescriptor_t,
        zDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        normScaleBiasDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
        groupCnt: ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the workspace size for normalization backward.\n\n# Arguments\n\n* `handle` [in]  -              cuDNN library handle.\n* `mode` [in]  -                Normalization mode.\n* `normOps` [in]  -             Extended normalization operation.\n* `algo` [in]  -                Normalization algorithm.\n* `xDesc` [in]  -               Input tensor descriptor.\n* `yDesc` [in]  -               Output tensor descriptor.\n* `dyDesc` [in]  -              Output gradient tensor descriptor.\n* `dzDesc` [in]  -              Z gradient tensor descriptor.\n* `dxDesc` [in]  -              Input gradient tensor descriptor.\n* `dNormScaleBiasDesc` [in]  -  Descriptor for normalization parameter gradient tensors.\n* `activationDesc` [in]  -      Activation descriptor.\n* `normMeanVarDesc` [in]  -     Descriptor for mean/variance tensors.\n* `sizeInBytes` [out]  -         Required workspace size in bytes.\n* `groupCnt` [in]  -            Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationBackward`]"]
    pub fn cudnnGetNormalizationBackwardWorkspaceSize(
        handle: cudnnHandle_t,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        xDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        dzDesc: cudnnTensorDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        dNormScaleBiasDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
        groupCnt: ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the reserve space size for normalization training.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN library handle.\n* `mode` [in]  -            Normalization mode.\n* `normOps` [in]  -         Extended normalization operation.\n* `algo` [in]  -            Normalization algorithm.\n* `activationDesc` [in]  -  Activation descriptor.\n* `xDesc` [in]  -           Input tensor descriptor.\n* `sizeInBytes` [out]  -     Required reserve space size in bytes.\n* `groupCnt` [in]  -        Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining`]"]
    pub fn cudnnGetNormalizationTrainingReserveSpaceSize(handle: cudnnHandle_t, mode: cudnnNormMode_t, normOps: cudnnNormOps_t, algo: cudnnNormAlgo_t, activationDesc: cudnnActivationDescriptor_t, xDesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize, groupCnt: ::core::ffi::c_int)
    -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs normalization forward training with optional activation.\nComputes y = relu(Norm(x) + z). Also accumulates moving averages of mean\nand inverse variances.\n\n# Arguments\n\n* `handle` [in]  -                     cuDNN library handle.\n* `mode` [in]  -                       Normalization mode.\n* `normOps` [in]  -                    Extended normalization operation.\n* `algo` [in]  -                       Normalization algorithm.\n* `alpha` [in]  -                      Result blend factor.\n* `beta` [in]  -                       Destination layer blend factor.\n* `xDesc` [in]  -                      Input tensor descriptor.\n* `xData` [in]  -                      Pointer to input tensor data.\n* `normScaleBiasDesc` [in]  -          Descriptor for normalization scale/bias tensors.\n* `normScale` [in]  -                  Pointer to scale tensor data.\n* `normBias` [in]  -                   Pointer to bias tensor data.\n* `exponentialAverageFactor` [in]  -   Factor for computing running averages.\n* `normMeanVarDesc` [in]  -            Descriptor for mean/variance tensors.\n* `resultRunningMean` [in,out]  -          Running mean.\n* `resultRunningVariance` [in,out]  -      Running variance.\n* `epsilon` [in]  -                    Epsilon value (must be >= 0).\n* `resultSaveMean` [out]  -             Cached mean for backward pass (NULL if unused).\n* `resultSaveInvVariance` [out]  -      Cached inverse variance for backward pass (NULL if unused).\n* `activationDesc` [in]  -             Activation descriptor.\n* `zDesc` [in]  -                      Z tensor descriptor (for add operations).\n* `zData` [in]  -                      Pointer to z tensor data.\n* `yDesc` [in]  -                      Output tensor descriptor.\n* `yData` [out]  -                      Pointer to output tensor data.\n* `workspace` [in,out]  -                  Pointer to workspace memory.\n* `workSpaceSizeInBytes` [in]  -       Size of workspace in bytes.\n* `reserveSpace` [in,out]  -               Pointer to reserve space memory.\n* `reserveSpaceSizeInBytes` [in]  -    Size of reserve space in bytes.\n* `groupCnt` [in]  -                   Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationBackward,`] cudnnGetNormalizationForwardTrainingWorkspaceSize"]
    pub fn cudnnNormalizationForwardTraining(
        handle: cudnnHandle_t,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        xData: *const ::core::ffi::c_void,
        normScaleBiasDesc: cudnnTensorDescriptor_t,
        normScale: *const ::core::ffi::c_void,
        normBias: *const ::core::ffi::c_void,
        exponentialAverageFactor: f64,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        resultRunningMean: *mut ::core::ffi::c_void,
        resultRunningVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::core::ffi::c_void,
        resultSaveInvVariance: *mut ::core::ffi::c_void,
        activationDesc: cudnnActivationDescriptor_t,
        zDesc: cudnnTensorDescriptor_t,
        zData: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        yData: *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        groupCnt: ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs backward normalization.\nComputes gradients for the normalization operation, including optional activation\nand element-wise add gradients.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `mode` [in]  -                    Normalization mode.\n* `normOps` [in]  -                 Extended normalization operation.\n* `algo` [in]  -                    Normalization algorithm.\n* `alphaDataDiff` [in]  -           Scaling factor for data gradient results.\n* `betaDataDiff` [in]  -            Scaling factor for data gradient destinations.\n* `alphaParamDiff` [in]  -          Scaling factor for parameter gradient results.\n* `betaParamDiff` [in]  -           Scaling factor for parameter gradient destinations.\n* `xDesc` [in]  -                   Input tensor descriptor.\n* `xData` [in]  -                   Pointer to input tensor data.\n* `yDesc` [in]  -                   Output tensor descriptor.\n* `yData` [in]  -                   Pointer to output tensor data.\n* `dyDesc` [in]  -                  Output gradient tensor descriptor.\n* `dyData` [in]  -                  Pointer to output gradient tensor data.\n* `dzDesc` [in]  -                  Z gradient tensor descriptor.\n* `dzData` [in,out]  -                  Pointer to z gradient tensor data.\n* `dxDesc` [in]  -                  Input gradient tensor descriptor.\n* `dxData` [in,out]  -                  Pointer to input gradient tensor data.\n* `dNormScaleBiasDesc` [in]  -      Shared descriptor for parameter gradient tensors.\n* `normScaleData` [in]  -           Pointer to scale tensor data.\n* `normBiasData` [in]  -            Pointer to bias tensor data (needed for activation).\n* `dNormScaleData` [out]  -          Pointer to scale gradient result.\n* `dNormBiasData` [out]  -           Pointer to bias gradient result.\n* `epsilon` [in]  -                 Same epsilon as forward pass.\n* `normMeanVarDesc` [in]  -         Descriptor for mean/variance tensors.\n* `savedMean` [in]  -               Optionally cached mean from forward pass.\n* `savedInvVariance` [in]  -        Optionally cached inverse variance from forward pass.\n* `activationDesc` [in]  -          Activation descriptor.\n* `workSpace` [in,out]  -               Pointer to workspace memory.\n* `workSpaceSizeInBytes` [in]  -    Size of workspace in bytes.\n* `reserveSpace` [in,out]  -            Pointer to reserve space memory.\n* `reserveSpaceSizeInBytes` [in]  - Size of reserve space in bytes.\n* `groupCnt` [in]  -                Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining`]"]
    pub fn cudnnNormalizationBackward(
        handle: cudnnHandle_t,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        alphaDataDiff: *const ::core::ffi::c_void,
        betaDataDiff: *const ::core::ffi::c_void,
        alphaParamDiff: *const ::core::ffi::c_void,
        betaParamDiff: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        xData: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        yData: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dyData: *const ::core::ffi::c_void,
        dzDesc: cudnnTensorDescriptor_t,
        dzData: *mut ::core::ffi::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dxData: *mut ::core::ffi::c_void,
        dNormScaleBiasDesc: cudnnTensorDescriptor_t,
        normScaleData: *const ::core::ffi::c_void,
        normBiasData: *const ::core::ffi::c_void,
        dNormScaleData: *mut ::core::ffi::c_void,
        dNormBiasData: *mut ::core::ffi::c_void,
        epsilon: f64,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        savedMean: *const ::core::ffi::c_void,
        savedInvVariance: *const ::core::ffi::c_void,
        activationDesc: cudnnActivationDescriptor_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        groupCnt: ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes the gradient of the spatial transformer grid generator (backward).\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `stDesc` [in]  -  Spatial transformer descriptor.\n* `dgrid` [in]  -   Pointer to the grid gradient data.\n* `dtheta` [out]  -  Pointer to the theta gradient data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfGridGeneratorForward`]"]
    pub fn cudnnSpatialTfGridGeneratorBackward(handle: cudnnHandle_t, stDesc: cudnnSpatialTransformerDescriptor_t, dgrid: *const ::core::ffi::c_void, dtheta: *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs spatial transformer sampling backward.\nComputes the gradients of the spatial transformer sampler.\n\n# Arguments\n\n* `handle` [in]  -     cuDNN library handle.\n* `stDesc` [in]  -     Spatial transformer descriptor.\n* `alpha` [in]  -      Scaling factor for the dx result.\n* `xDesc` [in]  -      Input tensor descriptor.\n* `x` [in]  -          Pointer to input tensor data.\n* `beta` [in]  -       Scaling factor for the dx destination.\n* `dxDesc` [in]  -     Input gradient tensor descriptor.\n* `dx` [in,out]  -         Pointer to input gradient tensor data.\n* `alphaDgrid` [in]  - Scaling factor for the dgrid result.\n* `dyDesc` [in]  -     Output gradient tensor descriptor.\n* `dy` [in]  -         Pointer to output gradient tensor data.\n* `grid` [in]  -       Pointer to sampling grid data.\n* `betaDgrid` [in]  -  Scaling factor for the dgrid destination.\n* `dgrid` [in,out]  -      Pointer to grid gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfSamplerForward`]"]
    pub fn cudnnSpatialTfSamplerBackward(
        handle: cudnnHandle_t,
        stDesc: cudnnSpatialTransformerDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        alphaDgrid: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        grid: *const ::core::ffi::c_void,
        betaDgrid: *const ::core::ffi::c_void,
        dgrid: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Performs backward dropout.\nApplies the same dropout mask from the forward pass (stored in reserveSpace)\nto the gradient tensor.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `dropoutDesc` [in]  -             Dropout descriptor.\n* `dydesc` [in]  -                  Output gradient tensor descriptor.\n* `dy` [in]  -                      Pointer to output gradient tensor data.\n* `dxdesc` [in]  -                  Input gradient tensor descriptor.\n* `dx` [out]  -                      Pointer to input gradient tensor data.\n* `reserveSpace` [in]  -            Pointer to reserve space from forward pass.\n* `reserveSpaceSizeInBytes` [in]  - Size of reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDropoutForward`]"]
    pub fn cudnnDropoutBackward(
        handle: cudnnHandle_t,
        dropoutDesc: cudnnDropoutDescriptor_t,
        dydesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dxdesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "RNN computation algorithm selection.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnRNNAlgo_t {
    #[doc = "< Standard cuBLASLt-based algorithm. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_ALGO_STANDARD = 0,
    #[doc = "< Persistent kernel with static compilation. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_ALGO_PERSIST_STATIC = 1,
    #[doc = "< Runtime-compiled persistent kernels via NVRTC. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_ALGO_PERSIST_DYNAMIC = 2,
    #[doc = "< Register-based approach for smaller hidden states. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_ALGO_PERSIST_STATIC_SMALL_H = 3,
    #[doc = "< Number of RNN algorithms. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_ALGO_COUNT = 4,
}
#[repr(u32)]
#[doc = "Specifies inference or training mode for RNN forward pass.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnForwardMode_t {
    #[doc = "< Inference mode. > **Since** cuDNN 9.0.0"]
    CUDNN_FWD_MODE_INFERENCE = 0,
    #[doc = "< Training mode (reserves space for backward pass). > **Since** cuDNN 9.0.0"]
    CUDNN_FWD_MODE_TRAINING = 1,
}
#[repr(u32)]
#[doc = "RNN cell type selection.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnRNNMode_t {
    #[doc = "< Single-gate RNN cell with ReLU activation. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_RELU = 0,
    #[doc = "< Single-gate RNN cell with tanh activation. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_TANH = 1,
    #[doc = "< Four-gate LSTM with optional recurrent projection and clipping. > **Since** cuDNN 9.0.0"]
    CUDNN_LSTM = 2,
    #[doc = "< Three-gate GRU network. > **Since** cuDNN 9.0.0"]
    CUDNN_GRU = 3,
}
#[repr(u32)]
#[doc = "Number of bias vectors used in RNN cell formulas.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnRNNBiasMode_t {
    #[doc = "< No biases used. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_NO_BIAS = 0,
    #[doc = "< One input bias in input GEMM. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_SINGLE_INP_BIAS = 1,
    #[doc = "< Two bias vectors (default). > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_DOUBLE_BIAS = 2,
    #[doc = "< One recurrent bias in recurrent GEMM. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_SINGLE_REC_BIAS = 3,
}
#[repr(u32)]
#[doc = "RNN recurrence direction mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnDirectionMode_t {
    #[doc = "< Single direction, first input to last. > **Since** cuDNN 9.0.0"]
    CUDNN_UNIDIRECTIONAL = 0,
    #[doc = "< Both directions, outputs concatenated at each layer. > **Since** cuDNN 9.0.0"]
    CUDNN_BIDIRECTIONAL = 1,
}
#[repr(u32)]
#[doc = "RNN first layer input behavior.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnRNNInputMode_t {
    #[doc = "< Biased matrix multiplication at first layer. > **Since** cuDNN 9.0.0"]
    CUDNN_LINEAR_INPUT = 0,
    #[doc = "< Fixed identity matrix at first layer (no operation). > **Since** cuDNN 9.0.0"]
    CUDNN_SKIP_INPUT = 1,
}
#[repr(u32)]
#[doc = "LSTM cell clipping mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnRNNClipMode_t {
    #[doc = "< Disables LSTM cell clipping. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_CLIP_NONE = 0,
    #[doc = "< Enables LSTM cell clipping. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_CLIP_MINMAX = 1,
}
#[repr(u32)]
#[doc = "RNN data memory layout.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnRNNDataLayout_t {
    #[doc = "< Padded, sequence-major layout. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_UNPACKED = 0,
    #[doc = "< Packed, sequence-major layout. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_PACKED = 1,
    #[doc = "< Padded, batch-major layout. > **Since** cuDNN 9.0.0"]
    CUDNN_RNN_DATA_LAYOUT_BATCH_MAJOR_UNPACKED = 2,
}
#[doc = "Opaque RNN descriptor. > **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnRNNStruct {
    _unused: [u8; 0],
}
pub type cudnnRNNDescriptor_t = *mut cudnnRNNStruct;
#[doc = "Opaque RNN data descriptor. > **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnRNNDataStruct {
    _unused: [u8; 0],
}
pub type cudnnRNNDataDescriptor_t = *mut cudnnRNNDataStruct;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates an RNN descriptor.\n\n# Arguments\n\n* `rnnDesc` [out]  -  Pointer to the created RNN descriptor.\n@retval CUDNN_STATUS_SUCCESS  Descriptor created successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroyRNNDescriptor`]"]
    pub fn cudnnCreateRNNDescriptor(rnnDesc: *mut cudnnRNNDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys an RNN descriptor.\n\n# Arguments\n\n* `rnnDesc` [in]  -  RNN descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  Descriptor destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateRNNDescriptor`]"]
    pub fn cudnnDestroyRNNDescriptor(rnnDesc: cudnnRNNDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures an RNN descriptor with network parameters.\n\n# Arguments\n\n* `rnnDesc` [in,out]  -     RNN descriptor to configure.\n* `algo` [in]  -        RNN computation algorithm.\n* `cellMode` [in]  -    RNN cell type (RELU, TANH, LSTM, GRU).\n* `biasMode` [in]  -    Bias configuration.\n* `dirMode` [in]  -     Unidirectional or bidirectional.\n* `inputMode` [in]  -   First layer input behavior.\n* `dataType` [in]  -    Input/output and weight data type.\n* `mathPrec` [in]  -    Compute precision.\n* `mathType` [in]  -    Tensor Core usage preference.\n* `inputSize` [in]  -   Input vector size.\n* `hiddenSize` [in]  -  Hidden state size.\n* `projSize` [in]  -    Recurrent projection size (0 to disable).\n* `numLayers` [in]  -   Number of stacked RNN layers.\n* `dropoutDesc` [in]  - Dropout descriptor for inter-layer dropout.\n* `auxFlags` [in]  -    Auxiliary flags (e.g., CUDNN_RNN_PADDED_IO_ENABLED).\n@retval CUDNN_STATUS_SUCCESS       Descriptor configured successfully.\n@retval CUDNN_STATUS_BAD_PARAM     Invalid parameter.\n@retval CUDNN_STATUS_NOT_SUPPORTED Unsupported configuration.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnGetRNNDescriptor_v8`]"]
    pub fn cudnnSetRNNDescriptor_v8(
        rnnDesc: cudnnRNNDescriptor_t,
        algo: cudnnRNNAlgo_t,
        cellMode: cudnnRNNMode_t,
        biasMode: cudnnRNNBiasMode_t,
        dirMode: cudnnDirectionMode_t,
        inputMode: cudnnRNNInputMode_t,
        dataType: cudnnDataType_t,
        mathPrec: cudnnDataType_t,
        mathType: cudnnMathType_t,
        inputSize: i32,
        hiddenSize: i32,
        projSize: i32,
        numLayers: i32,
        dropoutDesc: cudnnDropoutDescriptor_t,
        auxFlags: u32,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves RNN descriptor parameters.\n\n# Arguments\n\n* `rnnDesc` [in]  -     RNN descriptor to query.\n* `algo` [out]  -        RNN algorithm.\n* `cellMode` [out]  -    Cell type.\n* `biasMode` [out]  -    Bias configuration.\n* `dirMode` [out]  -     Direction mode.\n* `inputMode` [out]  -   Input mode.\n* `dataType` [out]  -    Data type.\n* `mathPrec` [out]  -    Math precision.\n* `mathType` [out]  -    Math type.\n* `inputSize` [out]  -   Input size.\n* `hiddenSize` [out]  -  Hidden size.\n* `projSize` [out]  -    Projection size.\n* `numLayers` [out]  -   Number of layers.\n* `dropoutDesc` [out]  - Dropout descriptor.\n* `auxFlags` [out]  -    Auxiliary flags.\n@retval CUDNN_STATUS_SUCCESS  Query succeeded.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetRNNDescriptor_v8`]"]
    pub fn cudnnGetRNNDescriptor_v8(
        rnnDesc: cudnnRNNDescriptor_t,
        algo: *mut cudnnRNNAlgo_t,
        cellMode: *mut cudnnRNNMode_t,
        biasMode: *mut cudnnRNNBiasMode_t,
        dirMode: *mut cudnnDirectionMode_t,
        inputMode: *mut cudnnRNNInputMode_t,
        dataType: *mut cudnnDataType_t,
        mathPrec: *mut cudnnDataType_t,
        mathType: *mut cudnnMathType_t,
        inputSize: *mut i32,
        hiddenSize: *mut i32,
        projSize: *mut i32,
        numLayers: *mut i32,
        dropoutDesc: *mut cudnnDropoutDescriptor_t,
        auxFlags: *mut u32,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures LSTM cell clipping parameters.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnRNNSetClip_v9 instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnRNNSetClip_v8(rnnDesc: cudnnRNNDescriptor_t, clipMode: cudnnRNNClipMode_t, clipNanOpt: cudnnNanPropagation_t, lclip: f64, rclip: f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures LSTM cell clipping parameters.\n\n# Arguments\n\n* `rnnDesc` [in,out]  -   RNN descriptor.\n* `clipMode` [in]  -  Clipping mode (NONE or MINMAX).\n* `lclip` [in]  -     Left (minimum) clipping value.\n* `rclip` [in]  -     Right (maximum) clipping value.\n@retval CUDNN_STATUS_SUCCESS  Clipping configured.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNGetClip_v9`]"]
    pub fn cudnnRNNSetClip_v9(rnnDesc: cudnnRNNDescriptor_t, clipMode: cudnnRNNClipMode_t, lclip: f64, rclip: f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves LSTM cell clipping settings.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnRNNGetClip_v9 instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnRNNGetClip_v8(rnnDesc: cudnnRNNDescriptor_t, clipMode: *mut cudnnRNNClipMode_t, clipNanOpt: *mut cudnnNanPropagation_t, lclip: *mut f64, rclip: *mut f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves LSTM cell clipping settings.\n\n# Arguments\n\n* `rnnDesc` [in]  -   RNN descriptor.\n* `clipMode` [out]  -  Clipping mode.\n* `lclip` [out]  -     Left clipping value.\n* `rclip` [out]  -     Right clipping value.\n@retval CUDNN_STATUS_SUCCESS  Query succeeded.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNSetClip_v9`]"]
    pub fn cudnnRNNGetClip_v9(rnnDesc: cudnnRNNDescriptor_t, clipMode: *mut cudnnRNNClipMode_t, lclip: *mut f64, rclip: *mut f64) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Compiles persistent RNN code using NVRTC for dynamic algorithm.\n\n# Arguments\n\n* `handle` [in]  -     cuDNN handle.\n* `rnnDesc` [in]  -    RNN descriptor (must use PERSIST_DYNAMIC algorithm).\n* `miniBatch` [in]  -  Exact mini-batch size for compilation.\n@retval CUDNN_STATUS_SUCCESS       Compilation succeeded.\n@retval CUDNN_STATUS_NOT_SUPPORTED Unsupported configuration.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnBuildRNNDynamic(handle: cudnnHandle_t, rnnDesc: cudnnRNNDescriptor_t, miniBatch: ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes workspace and reserve space buffer sizes for RNN.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN handle.\n* `rnnDesc` [in]  -         RNN descriptor.\n* `fwdMode` [in]  -         Inference or training mode.\n* `xDesc` [in]  -           Input data descriptor.\n* `workSpaceSize` [out]  -   Required workspace size in bytes.\n* `reserveSpaceSize` [out]  - Required reserve space size in bytes (training only).\n@retval CUDNN_STATUS_SUCCESS  Sizes computed successfully.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetRNNTempSpaceSizes(handle: cudnnHandle_t, rnnDesc: cudnnRNNDescriptor_t, fwdMode: cudnnForwardMode_t, xDesc: cudnnRNNDataDescriptor_t, workSpaceSize: *mut usize, reserveSpaceSize: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Reports required GPU memory for all RNN weight parameters.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN handle.\n* `rnnDesc` [in]  -         RNN descriptor.\n* `weightSpaceSize` [out]  - Required weight space size in bytes.\n@retval CUDNN_STATUS_SUCCESS  Size computed.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetRNNWeightSpaceSize(handle: cudnnHandle_t, rnnDesc: cudnnRNNDescriptor_t, weightSpaceSize: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Obtains start address and shape of RNN weight matrices and bias vectors.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN handle.\n* `rnnDesc` [in]  -         RNN descriptor.\n* `pseudoLayer` [in]  -     Pseudo-layer index (physical layer and direction).\n* `weightSpaceSize` [in]  - Total weight space size.\n* `weightSpace` [in]  -     Pointer to weight space.\n* `linLayerID` [in]  -      Linear layer ID within the RNN cell.\n* `mDesc` [out]  -           Tensor descriptor for the weight matrix.\n* `mAddr` [out]  -           Start address of the weight matrix.\n* `bDesc` [out]  -           Tensor descriptor for the bias vector.\n* `bAddr` [out]  -           Start address of the bias vector.\n@retval CUDNN_STATUS_SUCCESS  Parameters retrieved.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetRNNWeightParams(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        pseudoLayer: i32,
        weightSpaceSize: usize,
        weightSpace: *const ::core::ffi::c_void,
        linLayerID: i32,
        mDesc: cudnnTensorDescriptor_t,
        mAddr: *mut *mut ::core::ffi::c_void,
        bDesc: cudnnTensorDescriptor_t,
        bAddr: *mut *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates an RNN data descriptor.\n\n# Arguments\n\n* `rnnDataDesc` [out]  -  Pointer to created descriptor.\n@retval CUDNN_STATUS_SUCCESS  Descriptor created.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCreateRNNDataDescriptor(rnnDataDesc: *mut cudnnRNNDataDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys an RNN data descriptor.\n\n# Arguments\n\n* `rnnDataDesc` [in]  -  Descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  Descriptor destroyed.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnDestroyRNNDataDescriptor(rnnDataDesc: cudnnRNNDataDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures an RNN data descriptor with layout and sequence information.\n\n# Arguments\n\n* `rnnDataDesc` [in,out]  -     RNN data descriptor.\n* `dataType` [in]  -        Data type.\n* `layout` [in]  -          Data layout (sequence-major or batch-major).\n* `maxSeqLength` [in]  -    Maximum sequence length.\n* `batchSize` [in]  -       Batch size.\n* `vectorSize` [in]  -      Input vector size.\n* `seqLengthArray` [in]  -  Length of each sequence in the batch.\n* `paddingFill` [in,out]  -     Symbol for filling padding positions.\n@retval CUDNN_STATUS_SUCCESS  Descriptor configured.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetRNNDataDescriptor(
        rnnDataDesc: cudnnRNNDataDescriptor_t,
        dataType: cudnnDataType_t,
        layout: cudnnRNNDataLayout_t,
        maxSeqLength: ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
        vectorSize: ::core::ffi::c_int,
        seqLengthArray: *const ::core::ffi::c_int,
        paddingFill: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves RNN data descriptor parameters.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetRNNDataDescriptor(
        rnnDataDesc: cudnnRNNDataDescriptor_t,
        dataType: *mut cudnnDataType_t,
        layout: *mut cudnnRNNDataLayout_t,
        maxSeqLength: *mut ::core::ffi::c_int,
        batchSize: *mut ::core::ffi::c_int,
        vectorSize: *mut ::core::ffi::c_int,
        arrayLengthRequested: ::core::ffi::c_int,
        seqLengthArray: *mut ::core::ffi::c_int,
        paddingFill: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes the forward pass of an RNN network.\n\n# Arguments\n\n* `handle` [in]  -           cuDNN handle.\n* `rnnDesc` [in]  -          RNN descriptor.\n* `fwdMode` [in]  -          Inference or training mode.\n* `devSeqLengths` [in]  -    Per-batch sequence lengths (device memory).\n* `xDesc` [in]  -            Input data descriptor.\n* `x` [in]  -                Input data pointer.\n* `yDesc` [in]  -            Output data descriptor.\n* `y` [out]  -                Output data pointer.\n* `hDesc` [in]  -            Hidden state descriptor.\n* `hx` [in]  -               Initial hidden state (NULL for zero).\n* `hy` [out]  -               Final hidden state (NULL to discard).\n* `cDesc` [in]  -            Cell state descriptor (LSTM only).\n* `cx` [in]  -               Initial cell state (NULL for zero).\n* `cy` [out]  -               Final cell state (NULL to discard).\n* `weightSpaceSize` [in]  -  Weight space size in bytes.\n* `weightSpace` [in]  -      Weight space pointer.\n* `workSpaceSize` [in]  -    Workspace size in bytes.\n* `workSpace` [in,out]  -        Workspace pointer.\n* `reserveSpaceSize` [in]  - Reserve space size (training only).\n* `reserveSpace` [in,out]  -     Reserve space pointer (training only).\n@retval CUDNN_STATUS_SUCCESS        Forward pass completed.\n@retval CUDNN_STATUS_BAD_PARAM      Invalid parameter.\n@retval CUDNN_STATUS_EXECUTION_FAILED  Execution failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNBackwardData_v8,`] cudnnRNNBackwardWeights_v8"]
    pub fn cudnnRNNForward(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        fwdMode: cudnnForwardMode_t,
        devSeqLengths: *const i32,
        xDesc: cudnnRNNDataDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: cudnnRNNDataDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hDesc: cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        hy: *mut ::core::ffi::c_void,
        cDesc: cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        cy: *mut ::core::ffi::c_void,
        weightSpaceSize: usize,
        weightSpace: *const ::core::ffi::c_void,
        workSpaceSize: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSize: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Sequence data dimension indices.\n> **Deprecated** Since cuDNN 9.0.0. Use RNN data descriptors instead.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnSeqDataAxis_t {
    #[doc = "< Time/sequence length dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_SEQDATA_TIME_DIM = 0,
    #[doc = "< Batch dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_SEQDATA_BATCH_DIM = 1,
    #[doc = "< Beam dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_SEQDATA_BEAM_DIM = 2,
    #[doc = "< Vector dimension. > **Since** cuDNN 9.0.0"]
    CUDNN_SEQDATA_VECT_DIM = 3,
}
#[doc = "Opaque sequence data descriptor. > **Deprecated** Since cuDNN 9.0.0. > **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnSeqDataStruct {
    _unused: [u8; 0],
}
pub type cudnnSeqDataDescriptor_t = *mut cudnnSeqDataStruct;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a sequence data descriptor.\n> **Deprecated** Since cuDNN 9.0.0. Use RNN data descriptors instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCreateSeqDataDescriptor(seqDataDesc: *mut cudnnSeqDataDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a sequence data descriptor.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnDestroySeqDataDescriptor(seqDataDesc: cudnnSeqDataDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures a sequence data descriptor.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetSeqDataDescriptor(
        seqDataDesc: cudnnSeqDataDescriptor_t,
        dataType: cudnnDataType_t,
        nbDims: ::core::ffi::c_int,
        dimA: *const ::core::ffi::c_int,
        axes: *const cudnnSeqDataAxis_t,
        seqLengthArraySize: usize,
        seqLengthArray: *const ::core::ffi::c_int,
        paddingFill: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves sequence data descriptor parameters.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetSeqDataDescriptor(
        seqDataDesc: cudnnSeqDataDescriptor_t,
        dataType: *mut cudnnDataType_t,
        nbDims: *mut ::core::ffi::c_int,
        nbDimsRequested: ::core::ffi::c_int,
        dimA: *mut ::core::ffi::c_int,
        axes: *mut cudnnSeqDataAxis_t,
        seqLengthArraySize: *mut usize,
        seqLengthSizeRequested: usize,
        seqLengthArray: *mut ::core::ffi::c_int,
        paddingFill: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[doc = "Opaque multi-head attention descriptor. > **Deprecated** Since cuDNN 9.0.0. > **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnAttnStruct {
    _unused: [u8; 0],
}
pub type cudnnAttnDescriptor_t = *mut cudnnAttnStruct;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a multi-head attention descriptor.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCreateAttnDescriptor(attnDesc: *mut cudnnAttnDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a multi-head attention descriptor.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnDestroyAttnDescriptor(attnDesc: cudnnAttnDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures a multi-head attention descriptor.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetAttnDescriptor(
        attnDesc: cudnnAttnDescriptor_t,
        attnMode: ::core::ffi::c_uint,
        nHeads: ::core::ffi::c_int,
        smScaler: f64,
        dataType: cudnnDataType_t,
        computePrec: cudnnDataType_t,
        mathType: cudnnMathType_t,
        attnDropoutDesc: cudnnDropoutDescriptor_t,
        postDropoutDesc: cudnnDropoutDescriptor_t,
        qSize: ::core::ffi::c_int,
        kSize: ::core::ffi::c_int,
        vSize: ::core::ffi::c_int,
        qProjSize: ::core::ffi::c_int,
        kProjSize: ::core::ffi::c_int,
        vProjSize: ::core::ffi::c_int,
        oProjSize: ::core::ffi::c_int,
        qoMaxSeqLength: ::core::ffi::c_int,
        kvMaxSeqLength: ::core::ffi::c_int,
        maxBatchSize: ::core::ffi::c_int,
        maxBeamSize: ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves multi-head attention descriptor parameters.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetAttnDescriptor(
        attnDesc: cudnnAttnDescriptor_t,
        attnMode: *mut ::core::ffi::c_uint,
        nHeads: *mut ::core::ffi::c_int,
        smScaler: *mut f64,
        dataType: *mut cudnnDataType_t,
        computePrec: *mut cudnnDataType_t,
        mathType: *mut cudnnMathType_t,
        attnDropoutDesc: *mut cudnnDropoutDescriptor_t,
        postDropoutDesc: *mut cudnnDropoutDescriptor_t,
        qSize: *mut ::core::ffi::c_int,
        kSize: *mut ::core::ffi::c_int,
        vSize: *mut ::core::ffi::c_int,
        qProjSize: *mut ::core::ffi::c_int,
        kProjSize: *mut ::core::ffi::c_int,
        vProjSize: *mut ::core::ffi::c_int,
        oProjSize: *mut ::core::ffi::c_int,
        qoMaxSeqLength: *mut ::core::ffi::c_int,
        kvMaxSeqLength: *mut ::core::ffi::c_int,
        maxBatchSize: *mut ::core::ffi::c_int,
        maxBeamSize: *mut ::core::ffi::c_int,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes weight, workspace, and reserve space sizes for multi-head attention.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetMultiHeadAttnBuffers(handle: cudnnHandle_t, attnDesc: cudnnAttnDescriptor_t, weightSizeInBytes: *mut usize, workSpaceSizeInBytes: *mut usize, reserveSpaceSizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Specifies weight/bias groups in multi-head attention layers.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnMultiHeadAttnWeightKind_t {
    #[doc = "< Query projection weights. > **Since** cuDNN 9.0.0"]
    CUDNN_MH_ATTN_Q_WEIGHTS = 0,
    #[doc = "< Key projection weights. > **Since** cuDNN 9.0.0"]
    CUDNN_MH_ATTN_K_WEIGHTS = 1,
    #[doc = "< Value projection weights. > **Since** cuDNN 9.0.0"]
    CUDNN_MH_ATTN_V_WEIGHTS = 2,
    #[doc = "< Output projection weights. > **Since** cuDNN 9.0.0"]
    CUDNN_MH_ATTN_O_WEIGHTS = 3,
    #[doc = "< Query projection biases. > **Since** cuDNN 9.0.0"]
    CUDNN_MH_ATTN_Q_BIASES = 4,
    #[doc = "< Key projection biases. > **Since** cuDNN 9.0.0"]
    CUDNN_MH_ATTN_K_BIASES = 5,
    #[doc = "< Value projection biases. > **Since** cuDNN 9.0.0"]
    CUDNN_MH_ATTN_V_BIASES = 6,
    #[doc = "< Output projection biases. > **Since** cuDNN 9.0.0"]
    CUDNN_MH_ATTN_O_BIASES = 7,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Obtains shape and start address of attention weight/bias tensors.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetMultiHeadAttnWeights(handle: cudnnHandle_t, attnDesc: cudnnAttnDescriptor_t, wKind: cudnnMultiHeadAttnWeightKind_t, weightSizeInBytes: usize, weights: *const ::core::ffi::c_void, wDesc: cudnnTensorDescriptor_t, wAddr: *mut *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes multi-head attention forward pass.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnMultiHeadAttnForward(
        handle: cudnnHandle_t,
        attnDesc: cudnnAttnDescriptor_t,
        currIdx: ::core::ffi::c_int,
        loWinIdx: *const ::core::ffi::c_int,
        hiWinIdx: *const ::core::ffi::c_int,
        devSeqLengthsQO: *const ::core::ffi::c_int,
        devSeqLengthsKV: *const ::core::ffi::c_int,
        qDesc: cudnnSeqDataDescriptor_t,
        queries: *const ::core::ffi::c_void,
        residuals: *const ::core::ffi::c_void,
        kDesc: cudnnSeqDataDescriptor_t,
        keys: *const ::core::ffi::c_void,
        vDesc: cudnnSeqDataDescriptor_t,
        values: *const ::core::ffi::c_void,
        oDesc: cudnnSeqDataDescriptor_t,
        out: *mut ::core::ffi::c_void,
        weightSizeInBytes: usize,
        weights: *const ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cudnnAdvVersionCheck() -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Weight gradient accumulation mode.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnWgradMode_t {
    #[doc = "< Add partial gradients to existing buffer. > **Since** cuDNN 9.0.0"]
    CUDNN_WGRAD_MODE_ADD = 0,
    #[doc = "< Overwrite buffer with partial gradients. > **Since** cuDNN 9.0.0"]
    CUDNN_WGRAD_MODE_SET = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes RNN data gradients (backward pass with respect to inputs).\n\n# Arguments\n\n* `handle` [in]  -           cuDNN handle.\n* `rnnDesc` [in]  -          RNN descriptor.\n* `devSeqLengths` [in]  -    Per-batch sequence lengths (device memory).\n* `yDesc` [in]  -            Output data descriptor.\n* `y` [in]  -                Forward output data.\n* `dy` [in]  -               Output gradient data.\n* `xDesc` [in]  -            Input data descriptor.\n* `dx` [out]  -               Computed input gradient.\n* `hDesc` [in]  -            Hidden state descriptor.\n* `hx` [in]  -               Initial hidden state from forward pass.\n* `dhy` [in]  -              Hidden state gradient (from upstream).\n* `dhx` [out]  -              Computed initial hidden state gradient.\n* `cDesc` [in]  -            Cell state descriptor (LSTM only).\n* `cx` [in]  -               Initial cell state from forward pass.\n* `dcy` [in]  -              Cell state gradient (from upstream).\n* `dcx` [out]  -              Computed initial cell state gradient.\n* `weightSpaceSize` [in]  -  Weight space size.\n* `weightSpace` [in]  -      Weight space pointer.\n* `workSpaceSize` [in]  -    Workspace size.\n* `workSpace` [in,out]  -        Workspace pointer.\n* `reserveSpaceSize` [in]  - Reserve space size.\n* `reserveSpace` [in,out]  -     Reserve space (from forward training pass).\n@retval CUDNN_STATUS_SUCCESS  Backward data pass completed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNForward,`] cudnnRNNBackwardWeights_v8"]
    pub fn cudnnRNNBackwardData_v8(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        devSeqLengths: *const i32,
        yDesc: cudnnRNNDataDescriptor_t,
        y: *const ::core::ffi::c_void,
        dy: *const ::core::ffi::c_void,
        xDesc: cudnnRNNDataDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        hDesc: cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        dhy: *const ::core::ffi::c_void,
        dhx: *mut ::core::ffi::c_void,
        cDesc: cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        dcy: *const ::core::ffi::c_void,
        dcx: *mut ::core::ffi::c_void,
        weightSpaceSize: usize,
        weightSpace: *const ::core::ffi::c_void,
        workSpaceSize: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSize: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes RNN weight gradients (backward pass with respect to parameters).\n\n# Arguments\n\n* `handle` [in]  -           cuDNN handle.\n* `rnnDesc` [in]  -          RNN descriptor.\n* `addGrad` [in]  -          Accumulate (ADD) or overwrite (SET) gradients.\n* `devSeqLengths` [in]  -    Per-batch sequence lengths (device memory).\n* `xDesc` [in]  -            Input data descriptor.\n* `x` [in]  -                Input data.\n* `hDesc` [in]  -            Hidden state descriptor.\n* `hx` [in]  -               Initial hidden state.\n* `yDesc` [in]  -            Output data descriptor.\n* `y` [in]  -                Forward output data.\n* `weightSpaceSize` [in]  -  Weight space size.\n* `dweightSpace` [in,out]  -     Computed weight gradients.\n* `workSpaceSize` [in]  -    Workspace size.\n* `workSpace` [in,out]  -        Workspace pointer.\n* `reserveSpaceSize` [in]  - Reserve space size.\n* `reserveSpace` [in,out]  -     Reserve space (from forward training pass).\n@retval CUDNN_STATUS_SUCCESS  Weight gradients computed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNForward,`] cudnnRNNBackwardData_v8"]
    pub fn cudnnRNNBackwardWeights_v8(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        addGrad: cudnnWgradMode_t,
        devSeqLengths: *const i32,
        xDesc: cudnnRNNDataDescriptor_t,
        x: *const ::core::ffi::c_void,
        hDesc: cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        yDesc: cudnnRNNDataDescriptor_t,
        y: *const ::core::ffi::c_void,
        weightSpaceSize: usize,
        dweightSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSize: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes multi-head attention data gradients.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnMultiHeadAttnBackwardData(
        handle: cudnnHandle_t,
        attnDesc: cudnnAttnDescriptor_t,
        loWinIdx: *const ::core::ffi::c_int,
        hiWinIdx: *const ::core::ffi::c_int,
        devSeqLengthsDQDO: *const ::core::ffi::c_int,
        devSeqLengthsDKDV: *const ::core::ffi::c_int,
        doDesc: cudnnSeqDataDescriptor_t,
        dout: *const ::core::ffi::c_void,
        dqDesc: cudnnSeqDataDescriptor_t,
        dqueries: *mut ::core::ffi::c_void,
        queries: *const ::core::ffi::c_void,
        dkDesc: cudnnSeqDataDescriptor_t,
        dkeys: *mut ::core::ffi::c_void,
        keys: *const ::core::ffi::c_void,
        dvDesc: cudnnSeqDataDescriptor_t,
        dvalues: *mut ::core::ffi::c_void,
        values: *const ::core::ffi::c_void,
        weightSizeInBytes: usize,
        weights: *const ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes multi-head attention weight gradients.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnMultiHeadAttnBackwardWeights(
        handle: cudnnHandle_t,
        attnDesc: cudnnAttnDescriptor_t,
        addGrad: cudnnWgradMode_t,
        qDesc: cudnnSeqDataDescriptor_t,
        queries: *const ::core::ffi::c_void,
        kDesc: cudnnSeqDataDescriptor_t,
        keys: *const ::core::ffi::c_void,
        vDesc: cudnnSeqDataDescriptor_t,
        values: *const ::core::ffi::c_void,
        doDesc: cudnnSeqDataDescriptor_t,
        dout: *const ::core::ffi::c_void,
        weightSizeInBytes: usize,
        weights: *const ::core::ffi::c_void,
        dweights: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Input normalization mode for loss functions.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnLossNormalizationMode_t {
    #[doc = "< Input treated as normalized probability. > **Since** cuDNN 9.0.0"]
    CUDNN_LOSS_NORMALIZATION_NONE = 0,
    #[doc = "< Input treated as unnormalized activation (softmax applied). > **Since** cuDNN 9.0.0"]
    CUDNN_LOSS_NORMALIZATION_SOFTMAX = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Creates a CTC loss descriptor.\n\n# Arguments\n\n* `ctcLossDesc` [out]  -  Pointer to created descriptor.\n@retval CUDNN_STATUS_SUCCESS  Descriptor created.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCreateCTCLossDescriptor(ctcLossDesc: *mut cudnnCTCLossDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures a CTC loss descriptor with compute type.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnSetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures CTC loss with normalization mode.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnSetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetCTCLossDescriptorEx(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t, normMode: cudnnLossNormalizationMode_t, gradMode: cudnnNanPropagation_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures CTC loss with normalization, gradient mode, and max label length.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnSetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetCTCLossDescriptor_v8(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t, normMode: cudnnLossNormalizationMode_t, gradMode: cudnnNanPropagation_t, maxLabelLength: ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Configures CTC loss with normalization, CTC gradient mode, and max label length.\n\n# Arguments\n\n* `ctcLossDesc` [in,out]  -    CTC loss descriptor.\n* `compType` [in]  -       Compute data type.\n* `normMode` [in]  -       Loss normalization mode.\n* `ctcGradMode` [in]  -    Gradient mode for out-of-bounds samples.\n* `maxLabelLength` [in]  - Maximum label length.\n@retval CUDNN_STATUS_SUCCESS  Descriptor configured.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetCTCLossDescriptor_v9(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t, normMode: cudnnLossNormalizationMode_t, ctcGradMode: cudnnCTCGradMode_t, maxLabelLength: ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves CTC loss compute type.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnGetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: *mut cudnnDataType_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves CTC loss extended parameters.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnGetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetCTCLossDescriptorEx(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: *mut cudnnDataType_t, normMode: *mut cudnnLossNormalizationMode_t, gradMode: *mut cudnnNanPropagation_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves CTC loss v8 parameters.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnGetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetCTCLossDescriptor_v8(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: *mut cudnnDataType_t, normMode: *mut cudnnLossNormalizationMode_t, gradMode: *mut cudnnNanPropagation_t, maxLabelLength: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Retrieves CTC loss v9 parameters.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetCTCLossDescriptor_v9(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: *mut cudnnDataType_t, normMode: *mut cudnnLossNormalizationMode_t, ctcGradMode: *mut cudnnCTCGradMode_t, maxLabelLength: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroys a CTC loss descriptor.\n\n# Arguments\n\n* `ctcLossDesc` [in]  -  Descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  Descriptor destroyed.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnDestroyCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes CTC loss and gradients given probabilities and labels.\nLabels and sequence lengths are in CPU memory. For GPU-memory variant, use cudnnCTCLoss_v8.\n\n# Arguments\n\n* `handle` [in]  -              cuDNN handle.\n* `probsDesc` [in]  -           Tensor descriptor for probabilities (T x N x A).\n* `probs` [in]  -               Probabilities after softmax (GPU memory).\n* `hostLabels` [in]  -           Labels (CPU memory).\n* `hostLabelLengths` [in]  -     Length of each label (CPU memory).\n* `hostInputLengths` [in]  -     Timing step lengths per batch (CPU memory).\n* `costs` [out]  -               CTC costs (GPU memory).\n* `gradientsDesc` [in]  -       Tensor descriptor for gradients (T x N x A).\n* `gradients` [out]  -           CTC gradients (GPU memory, NULL for costs only).\n* `algo` [in]  -                CTC loss algorithm.\n* `ctcLossDesc` [in]  -         CTC loss descriptor.\n* `workspace` [in]  -           Workspace (GPU memory).\n* `workSpaceSizeInBytes` [in]  - Workspace size.\n@retval CUDNN_STATUS_SUCCESS  CTC loss computed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCTCLoss_v8,`] cudnnGetCTCLossWorkspaceSize"]
    pub fn cudnnCTCLoss(
        handle: cudnnHandle_t,
        probsDesc: cudnnTensorDescriptor_t,
        probs: *const ::core::ffi::c_void,
        hostLabels: *const ::core::ffi::c_int,
        hostLabelLengths: *const ::core::ffi::c_int,
        hostInputLengths: *const ::core::ffi::c_int,
        costs: *mut ::core::ffi::c_void,
        gradientsDesc: cudnnTensorDescriptor_t,
        gradients: *mut ::core::ffi::c_void,
        algo: cudnnCTCLossAlgo_t,
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        workspace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Computes CTC loss and gradients (v8, supports CUDA graphs with GPU memory labels).\nLabels and sequence lengths are in GPU memory (unlike cudnnCTCLoss which uses CPU memory).\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCTCLoss,`] cudnnGetCTCLossWorkspaceSize_v8"]
    pub fn cudnnCTCLoss_v8(
        handle: cudnnHandle_t,
        algo: cudnnCTCLossAlgo_t,
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        probsDesc: cudnnTensorDescriptor_t,
        probs: *const ::core::ffi::c_void,
        labels: *const ::core::ffi::c_int,
        labelLengths: *const ::core::ffi::c_int,
        inputLengths: *const ::core::ffi::c_int,
        costs: *mut ::core::ffi::c_void,
        gradientsDesc: cudnnTensorDescriptor_t,
        gradients: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        workspace: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the GPU workspace size required for CTC loss computation.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCTCLoss`]"]
    pub fn cudnnGetCTCLossWorkspaceSize(
        handle: cudnnHandle_t,
        probsDesc: cudnnTensorDescriptor_t,
        gradientsDesc: cudnnTensorDescriptor_t,
        labels: *const ::core::ffi::c_int,
        labelLengths: *const ::core::ffi::c_int,
        inputLengths: *const ::core::ffi::c_int,
        algo: cudnnCTCLossAlgo_t,
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Returns the GPU workspace size required for CTC loss v8 computation.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCTCLoss_v8`]"]
    pub fn cudnnGetCTCLossWorkspaceSize_v8(handle: cudnnHandle_t, algo: cudnnCTCLossAlgo_t, ctcLossDesc: cudnnCTCLossDescriptor_t, probsDesc: cudnnTensorDescriptor_t, gradientsDesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnConvolutionStruct {
    _unused: [u8; 0],
}
#[doc = "Opaque descriptor for a convolution operation.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub type cudnnConvolutionDescriptor_t = *mut cudnnConvolutionStruct;
#[doc = "Performance results for forward convolution algorithm selection.\nContains timing, memory usage, and determinism information for a given\nforward convolution algorithm. Returned by algorithm search functions.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnConvolutionFwdAlgoPerfStruct {
    #[doc = "< The forward convolution algorithm."]
    pub algo: cudnnConvolutionFwdAlgo_t,
    #[doc = "< Status returned when running this algorithm."]
    pub status: cudnnStatus_t,
    #[doc = "< Execution time in milliseconds."]
    pub time: f32,
    #[doc = "< Workspace memory required in bytes."]
    pub memory: usize,
    #[doc = "< Whether the algorithm is deterministic."]
    pub determinism: cudnnDeterminism_t,
    #[doc = "< Math type used by the algorithm."]
    pub mathType: cudnnMathType_t,
    #[doc = "< Reserved for future use."]
    pub reserved: [::core::ffi::c_int; 3usize],
}
impl Default for cudnnConvolutionFwdAlgoPerfStruct {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "Performance results for forward convolution algorithm selection.\nContains timing, memory usage, and determinism information for a given\nforward convolution algorithm. Returned by algorithm search functions.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub type cudnnConvolutionFwdAlgoPerf_t = cudnnConvolutionFwdAlgoPerfStruct;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Create an instance of convolution descriptor.\n\n# Arguments\n\n* `convDesc` [out]  - Pointer to receive the newly created convolution descriptor.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCreateConvolutionDescriptor(convDesc: *mut cudnnConvolutionDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroy an instance of convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  - The convolution descriptor to destroy.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnDestroyConvolutionDescriptor(convDesc: cudnnConvolutionDescriptor_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Set the math type for a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in,out]  - The convolution descriptor.\n* `mathType` [in]  - The math type to set.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetConvolutionMathType(convDesc: cudnnConvolutionDescriptor_t, mathType: cudnnMathType_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the math type from a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  - The convolution descriptor.\n* `mathType` [out]  - Pointer to receive the math type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionMathType(convDesc: cudnnConvolutionDescriptor_t, mathType: *mut cudnnMathType_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Set the group count for a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in,out]  -   The convolution descriptor.\n* `groupCount` [in]  - The number of groups for grouped convolution.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetConvolutionGroupCount(convDesc: cudnnConvolutionDescriptor_t, groupCount: ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the group count from a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  -   The convolution descriptor.\n* `groupCount` [out]  - Pointer to receive the group count.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionGroupCount(convDesc: cudnnConvolutionDescriptor_t, groupCount: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Set the reorder type for a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in,out]  -    The convolution descriptor.\n* `reorderType` [in]  - The reorder type to set.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetConvolutionReorderType(convDesc: cudnnConvolutionDescriptor_t, reorderType: cudnnReorderType_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the reorder type from a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  -    The convolution descriptor.\n* `reorderType` [out]  - Pointer to receive the reorder type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionReorderType(convDesc: cudnnConvolutionDescriptor_t, reorderType: *mut cudnnReorderType_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Set a 2D convolution descriptor with padding, stride, dilation, mode, and compute type.\n\n# Arguments\n\n* `convDesc` [in,out]  -    The convolution descriptor to initialize.\n* `pad_h` [in]  -       Zero-padding height.\n* `pad_w` [in]  -       Zero-padding width.\n* `u` [in]  -           Vertical filter stride.\n* `v` [in]  -           Horizontal filter stride.\n* `dilation_h` [in]  -  Filter dilation in the vertical dimension.\n* `dilation_w` [in]  -  Filter dilation in the horizontal dimension.\n* `mode` [in]  -        Convolution mode (cross-correlation or convolution).\n* `computeType` [in]  - Data type for convolution computation.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetConvolution2dDescriptor(
        convDesc: cudnnConvolutionDescriptor_t,
        pad_h: ::core::ffi::c_int,
        pad_w: ::core::ffi::c_int,
        u: ::core::ffi::c_int,
        v: ::core::ffi::c_int,
        dilation_h: ::core::ffi::c_int,
        dilation_w: ::core::ffi::c_int,
        mode: cudnnConvolutionMode_t,
        computeType: cudnnDataType_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the parameters of a 2D convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  -    The convolution descriptor to query.\n* `pad_h` [out]  -       Pointer to receive zero-padding height.\n* `pad_w` [out]  -       Pointer to receive zero-padding width.\n* `u` [out]  -           Pointer to receive vertical filter stride.\n* `v` [out]  -           Pointer to receive horizontal filter stride.\n* `dilation_h` [out]  -  Pointer to receive filter dilation in the vertical dimension.\n* `dilation_w` [out]  -  Pointer to receive filter dilation in the horizontal dimension.\n* `mode` [out]  -        Pointer to receive convolution mode.\n* `computeType` [out]  - Pointer to receive compute data type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolution2dDescriptor(
        convDesc: cudnnConvolutionDescriptor_t,
        pad_h: *mut ::core::ffi::c_int,
        pad_w: *mut ::core::ffi::c_int,
        u: *mut ::core::ffi::c_int,
        v: *mut ::core::ffi::c_int,
        dilation_h: *mut ::core::ffi::c_int,
        dilation_w: *mut ::core::ffi::c_int,
        mode: *mut cudnnConvolutionMode_t,
        computeType: *mut cudnnDataType_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Set an N-dimensional convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in,out]  -      The convolution descriptor to initialize.\n* `arrayLength` [in]  -   Number of dimensions (nbDims-2 size).\n* `padA` [in]  -          Array of zero-padding values per dimension.\n* `filterStrideA` [in]  - Array of filter strides per dimension.\n* `dilationA` [in]  -     Array of dilation values per dimension.\n* `mode` [in]  -          Convolution mode.\n* `computeType` [in]  -   Data type for convolution computation.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetConvolutionNdDescriptor(
        convDesc: cudnnConvolutionDescriptor_t,
        arrayLength: ::core::ffi::c_int,
        padA: *const ::core::ffi::c_int,
        filterStrideA: *const ::core::ffi::c_int,
        dilationA: *const ::core::ffi::c_int,
        mode: cudnnConvolutionMode_t,
        computeType: cudnnDataType_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the parameters of an N-dimensional convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  -             The convolution descriptor to query.\n* `arrayLengthRequested` [in]  - Maximum number of dimensions to retrieve.\n* `arrayLength` [out]  -          Pointer to receive the actual number of dimensions.\n* `padA` [out]  -                 Array to receive zero-padding values.\n* `strideA` [out]  -              Array to receive stride values.\n* `dilationA` [out]  -            Array to receive dilation values.\n* `mode` [out]  -                 Pointer to receive convolution mode.\n* `computeType` [out]  -          Pointer to receive compute data type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionNdDescriptor(
        convDesc: cudnnConvolutionDescriptor_t,
        arrayLengthRequested: ::core::ffi::c_int,
        arrayLength: *mut ::core::ffi::c_int,
        padA: *mut ::core::ffi::c_int,
        strideA: *mut ::core::ffi::c_int,
        dilationA: *mut ::core::ffi::c_int,
        mode: *mut cudnnConvolutionMode_t,
        computeType: *mut cudnnDataType_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Compute the output dimensions of a 2D convolution.\n\n# Arguments\n\n* `convDesc` [in]  -        The convolution descriptor.\n* `inputTensorDesc` [in]  - Descriptor for the input tensor.\n* `filterDesc` [in]  -      Descriptor for the filter.\n* `n` [out]  -               Pointer to receive the output batch size.\n* `c` [out]  -               Pointer to receive the output channels.\n* `h` [out]  -               Pointer to receive the output height.\n* `w` [out]  -               Pointer to receive the output width.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolution2dForwardOutputDim(convDesc: cudnnConvolutionDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, filterDesc: cudnnFilterDescriptor_t, n: *mut ::core::ffi::c_int, c: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int, w: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Compute the output dimensions of an N-dimensional convolution.\n\n# Arguments\n\n* `convDesc` [in]  -         The convolution descriptor.\n* `inputTensorDesc` [in]  -  Descriptor for the input tensor.\n* `filterDesc` [in]  -       Descriptor for the filter.\n* `nbDims` [in]  -           Number of dimensions.\n* `tensorOuputDimA` [out]  -  Array to receive the output dimensions.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionNdForwardOutputDim(convDesc: cudnnConvolutionDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, filterDesc: cudnnFilterDescriptor_t, nbDims: ::core::ffi::c_int, tensorOuputDimA: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the maximum number of forward convolution algorithms available.\n\n# Arguments\n\n* `handle` [in]  - The cuDNN handle.\n* `count` [out]  -  Pointer to receive the maximum algorithm count.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionForwardAlgorithmMaxCount(handle: cudnnHandle_t, count: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get forward convolution algorithm recommendations without executing them.\nReturns a list of algorithms sorted by expected performance. Does not\nrequire a workspace or run actual convolutions.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `srcDesc` [in]  -             Descriptor for the input tensor.\n* `filterDesc` [in]  -          Descriptor for the filter.\n* `convDesc` [in]  -            The convolution descriptor.\n* `destDesc` [in]  -            Descriptor for the output tensor.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to return.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionForwardAlgorithm_v7(
        handle: cudnnHandle_t,
        srcDesc: cudnnTensorDescriptor_t,
        filterDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        destDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Find the best forward convolution algorithm by running benchmarks.\nExecutes all applicable algorithms and returns performance results\nsorted by execution time. Does not require user-allocated workspace.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `xDesc` [in]  -               Descriptor for the input tensor.\n* `wDesc` [in]  -               Descriptor for the filter.\n* `convDesc` [in]  -            The convolution descriptor.\n* `yDesc` [in]  -               Descriptor for the output tensor.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnFindConvolutionForwardAlgorithm(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        wDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Find the best forward convolution algorithm by running benchmarks with user-provided buffers.\nSimilar to cudnnFindConvolutionForwardAlgorithm but uses caller-provided\ndata buffers and workspace.\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `yDesc` [in]  -                 Descriptor for the output tensor.\n* `y` [out]  -                     Pointer to output data in device memory.\n* `requestedAlgoCount` [in]  -    Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -     Pointer to receive the actual number returned.\n* `perfResults` [out]  -           Array to receive the algorithm performance results.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnFindConvolutionForwardAlgorithmEx(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Perform the Im2Col transform for convolution.\nRearranges image data into a column matrix suitable for matrix multiplication\nbased convolution.\n\n# Arguments\n\n* `handle` [in]  -   The cuDNN handle.\n* `xDesc` [in]  -    Descriptor for the input tensor.\n* `x` [in]  -        Pointer to input data in device memory.\n* `wDesc` [in]  -    Descriptor for the filter.\n* `convDesc` [in]  - The convolution descriptor.\n* `colBuffer` [out]  - Pointer to the output column buffer in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnIm2Col(handle: cudnnHandle_t, xDesc: cudnnTensorDescriptor_t, x: *const ::core::ffi::c_void, wDesc: cudnnFilterDescriptor_t, convDesc: cudnnConvolutionDescriptor_t, colBuffer: *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Reorder filter and bias data for optimized convolution execution.\nRearranges filter and optionally bias data into a layout optimized for\nthe specified reorder type.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `filterDesc` [in]  -          Descriptor for the filter.\n* `reorderType` [in]  -         The reorder type to apply.\n* `filterData` [in]  -          Pointer to source filter data in device memory.\n* `reorderedFilterData` [out]  - Pointer to destination filter data in device memory.\n* `reorderBias` [in]  -         Non-zero to also reorder bias data.\n* `biasData` [in]  -            Pointer to source bias data in device memory (may be NULL).\n* `reorderedBiasData` [out]  -   Pointer to destination bias data in device memory (may be NULL).\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnReorderFilterAndBias(
        handle: cudnnHandle_t,
        filterDesc: cudnnFilterDescriptor_t,
        reorderType: cudnnReorderType_t,
        filterData: *const ::core::ffi::c_void,
        reorderedFilterData: *mut ::core::ffi::c_void,
        reorderBias: ::core::ffi::c_int,
        biasData: *const ::core::ffi::c_void,
        reorderedBiasData: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the minimum workspace size required for a forward convolution algorithm.\n\n# Arguments\n\n* `handle` [in]  -       The cuDNN handle.\n* `xDesc` [in]  -        Descriptor for the input tensor.\n* `wDesc` [in]  -        Descriptor for the filter.\n* `convDesc` [in]  -     The convolution descriptor.\n* `yDesc` [in]  -        Descriptor for the output tensor.\n* `algo` [in]  -         The forward convolution algorithm.\n* `sizeInBytes` [out]  -  Pointer to receive the required workspace size.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionForwardWorkspaceSize(handle: cudnnHandle_t, xDesc: cudnnTensorDescriptor_t, wDesc: cudnnFilterDescriptor_t, convDesc: cudnnConvolutionDescriptor_t, yDesc: cudnnTensorDescriptor_t, algo: cudnnConvolutionFwdAlgo_t, sizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Execute a forward convolution.\nComputes: y = alpha * conv(x, w) + beta * y\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `alpha` [in]  -                 Pointer to scaling factor for the convolution result.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `algo` [in]  -                  The forward convolution algorithm to use.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n* `beta` [in]  -                  Pointer to scaling factor for the prior output.\n* `yDesc` [in]  -                 Descriptor for the output tensor.\n* `y` [in,out]  -                     Pointer to output data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnConvolutionForward(
        handle: cudnnHandle_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionFwdAlgo_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        beta: *const ::core::ffi::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Execute a fused convolution with bias and activation.\nComputes: y = Act( alpha1 * conv(x) + alpha2 * z + bias )\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `alpha1` [in]  -                Pointer to scaling factor for the convolution result.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `algo` [in]  -                  The forward convolution algorithm to use.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n* `alpha2` [in]  -                Pointer to scaling factor for the residual input z.\n* `zDesc` [in]  -                 Descriptor for the residual input tensor.\n* `z` [in]  -                     Pointer to residual data in device memory.\n* `biasDesc` [in]  -              Descriptor for the bias tensor.\n* `bias` [in]  -                  Pointer to bias data in device memory.\n* `activationDesc` [in]  -        Descriptor for the activation operation.\n* `yDesc` [in]  -                 Descriptor for the output tensor.\n* `y` [in,out]  -                     Pointer to output data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnConvolutionBiasActivationForward(
        handle: cudnnHandle_t,
        alpha1: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionFwdAlgo_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        alpha2: *const ::core::ffi::c_void,
        zDesc: cudnnTensorDescriptor_t,
        z: *const ::core::ffi::c_void,
        biasDesc: cudnnTensorDescriptor_t,
        bias: *const ::core::ffi::c_void,
        activationDesc: cudnnActivationDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[doc = "Performance results for backward data convolution algorithm selection.\nContains timing, memory usage, and determinism information for a given\nbackward data convolution algorithm. Returned by algorithm search functions.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnConvolutionBwdDataAlgoPerfStruct {
    #[doc = "< The backward data convolution algorithm."]
    pub algo: cudnnConvolutionBwdDataAlgo_t,
    #[doc = "< Status returned when running this algorithm."]
    pub status: cudnnStatus_t,
    #[doc = "< Execution time in milliseconds."]
    pub time: f32,
    #[doc = "< Workspace memory required in bytes."]
    pub memory: usize,
    #[doc = "< Whether the algorithm is deterministic."]
    pub determinism: cudnnDeterminism_t,
    #[doc = "< Math type used by the algorithm."]
    pub mathType: cudnnMathType_t,
    #[doc = "< Reserved for future use."]
    pub reserved: [::core::ffi::c_int; 3usize],
}
impl Default for cudnnConvolutionBwdDataAlgoPerfStruct {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "Performance results for backward data convolution algorithm selection.\nContains timing, memory usage, and determinism information for a given\nbackward data convolution algorithm. Returned by algorithm search functions.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub type cudnnConvolutionBwdDataAlgoPerf_t = cudnnConvolutionBwdDataAlgoPerfStruct;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the maximum number of backward data convolution algorithms available.\n\n# Arguments\n\n* `handle` [in]  - The cuDNN handle.\n* `count` [out]  -  Pointer to receive the maximum algorithm count.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(handle: cudnnHandle_t, count: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Find the best backward data convolution algorithm by running benchmarks.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `wDesc` [in]  -               Descriptor for the filter.\n* `dyDesc` [in]  -              Descriptor for the gradient output tensor.\n* `convDesc` [in]  -            The convolution descriptor.\n* `dxDesc` [in]  -              Descriptor for the gradient input tensor.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnFindConvolutionBackwardDataAlgorithm(
        handle: cudnnHandle_t,
        wDesc: cudnnFilterDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Find the best backward data convolution algorithm with user-provided buffers.\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `dyDesc` [in]  -                Descriptor for the gradient output tensor.\n* `dy` [in]  -                    Pointer to gradient output data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `dxDesc` [in]  -                Descriptor for the gradient input tensor.\n* `dx` [out]  -                    Pointer to gradient input data in device memory.\n* `requestedAlgoCount` [in]  -    Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -     Pointer to receive the actual number returned.\n* `perfResults` [out]  -           Array to receive the algorithm performance results.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnFindConvolutionBackwardDataAlgorithmEx(
        handle: cudnnHandle_t,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get backward data convolution algorithm recommendations without executing them.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `filterDesc` [in]  -          Descriptor for the filter.\n* `diffDesc` [in]  -            Descriptor for the gradient output tensor.\n* `convDesc` [in]  -            The convolution descriptor.\n* `gradDesc` [in]  -            Descriptor for the gradient input tensor.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to return.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
        handle: cudnnHandle_t,
        filterDesc: cudnnFilterDescriptor_t,
        diffDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the minimum workspace size required for a backward data convolution algorithm.\n\n# Arguments\n\n* `handle` [in]  -       The cuDNN handle.\n* `wDesc` [in]  -        Descriptor for the filter.\n* `dyDesc` [in]  -       Descriptor for the gradient output tensor.\n* `convDesc` [in]  -     The convolution descriptor.\n* `dxDesc` [in]  -       Descriptor for the gradient input tensor.\n* `algo` [in]  -         The backward data convolution algorithm.\n* `sizeInBytes` [out]  -  Pointer to receive the required workspace size.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionBackwardDataWorkspaceSize(handle: cudnnHandle_t, wDesc: cudnnFilterDescriptor_t, dyDesc: cudnnTensorDescriptor_t, convDesc: cudnnConvolutionDescriptor_t, dxDesc: cudnnTensorDescriptor_t, algo: cudnnConvolutionBwdDataAlgo_t, sizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Execute a backward data convolution (compute gradient with respect to input data).\nComputes: dx = alpha * dconv(w, dy) + beta * dx\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `alpha` [in]  -                 Pointer to scaling factor for the convolution result.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `dyDesc` [in]  -                Descriptor for the gradient output tensor.\n* `dy` [in]  -                    Pointer to gradient output data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `algo` [in]  -                  The backward data convolution algorithm.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n* `beta` [in]  -                  Pointer to scaling factor for the prior gradient input.\n* `dxDesc` [in]  -                Descriptor for the gradient input tensor.\n* `dx` [in,out]  -                    Pointer to gradient input data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnConvolutionBackwardData(
        handle: cudnnHandle_t,
        alpha: *const ::core::ffi::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionBwdDataAlgo_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        beta: *const ::core::ffi::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Calculate folding descriptors for backward data convolution (dgrad).\nComputes the folded descriptors needed for tensor transform operations\nused in backward data gradient computation.\n\n# Arguments\n\n* `handle` [in]  -                 The cuDNN handle.\n* `filterDesc` [in]  -             Descriptor for the filter.\n* `diffDesc` [in]  -               Descriptor for the gradient output tensor.\n* `convDesc` [in]  -               The convolution descriptor.\n* `gradDesc` [in]  -               Descriptor for the gradient input tensor.\n* `transformFormat` [in]  -         The tensor format for the transform.\n* `foldedFilterDesc` [out]  -       Descriptor for the folded filter.\n* `paddedDiffDesc` [out]  -         Descriptor for the padded gradient output.\n* `foldedConvDesc` [out]  -         Descriptor for the folded convolution.\n* `foldedGradDesc` [out]  -         Descriptor for the folded gradient input.\n* `filterFoldTransDesc` [out]  -    Transform descriptor for filter folding.\n* `diffPadTransDesc` [out]  -       Transform descriptor for diff padding.\n* `gradFoldTransDesc` [out]  -      Transform descriptor for gradient folding.\n* `gradUnfoldTransDesc` [out]  -    Transform descriptor for gradient unfolding.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetFoldedConvBackwardDataDescriptors(
        handle: cudnnHandle_t,
        filterDesc: cudnnFilterDescriptor_t,
        diffDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnTensorDescriptor_t,
        transformFormat: cudnnTensorFormat_t,
        foldedFilterDesc: cudnnFilterDescriptor_t,
        paddedDiffDesc: cudnnTensorDescriptor_t,
        foldedConvDesc: cudnnConvolutionDescriptor_t,
        foldedGradDesc: cudnnTensorDescriptor_t,
        filterFoldTransDesc: cudnnTensorTransformDescriptor_t,
        diffPadTransDesc: cudnnTensorTransformDescriptor_t,
        gradFoldTransDesc: cudnnTensorTransformDescriptor_t,
        gradUnfoldTransDesc: cudnnTensorTransformDescriptor_t,
    ) -> cudnnStatus_t;
}
#[doc = "Opaque descriptor for fused operations constant parameter pack.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnFusedOpsConstParamStruct {
    _unused: [u8; 0],
}
#[doc = "Handle to a fused operations constant parameter pack.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub type cudnnFusedOpsConstParamPack_t = *mut cudnnFusedOpsConstParamStruct;
#[doc = "Opaque descriptor for fused operations variant parameter pack.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnFusedOpsVariantParamStruct {
    _unused: [u8; 0],
}
#[doc = "Handle to a fused operations variant parameter pack.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub type cudnnFusedOpsVariantParamPack_t = *mut cudnnFusedOpsVariantParamStruct;
#[doc = "Opaque descriptor for a fused operations execution plan.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnFusedOpsPlanStruct {
    _unused: [u8; 0],
}
#[doc = "Handle to a fused operations execution plan.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub type cudnnFusedOpsPlan_t = *mut cudnnFusedOpsPlanStruct;
#[repr(u32)]
#[doc = "Enumeration of fused operation sequences.\nSpecifies which sequence of operations to fuse together for optimized execution.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnFusedOps_t {
    #[doc = "< Fused scale, bias, activation, convolution, and optional BN stats generation."]
    CUDNN_FUSED_SCALE_BIAS_ACTIVATION_CONV_BNSTATS = 0,
    #[doc = "< Fused scale, bias, activation, and backward weight convolution."]
    CUDNN_FUSED_SCALE_BIAS_ACTIVATION_WGRAD = 1,
    #[doc = "< Finalize batch normalization statistics for training."]
    CUDNN_FUSED_BN_FINALIZE_STATISTICS_TRAINING = 2,
    #[doc = "< Finalize batch normalization statistics for inference."]
    CUDNN_FUSED_BN_FINALIZE_STATISTICS_INFERENCE = 3,
    #[doc = "< Fused convolution, scale, bias, residual add, and activation."]
    CUDNN_FUSED_CONV_SCALE_BIAS_ADD_ACTIVATION = 4,
    #[doc = "< Fused scale, bias, residual add, activation, and bitmask generation."]
    CUDNN_FUSED_SCALE_BIAS_ADD_ACTIVATION_GEN_BITMASK = 5,
    #[doc = "< Fused backward activation fork and backward batch normalization."]
    CUDNN_FUSED_DACTIVATION_FORK_DBATCHNORM = 6,
}
#[repr(u32)]
#[doc = "Labels for constant parameters in a fused operations parameter pack.\nUsed to identify which constant parameter to set or get in a\ncudnnFusedOpsConstParamPack_t.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnFusedOpsConstParamLabel_t {
    #[doc = "< Input tensor descriptor (X)."]
    CUDNN_PARAM_XDESC = 0,
    #[doc = "< Placeholder for input data pointer alignment (X)."]
    CUDNN_PARAM_XDATA_PLACEHOLDER = 1,
    #[doc = "< Batch normalization mode."]
    CUDNN_PARAM_BN_MODE = 2,
    #[doc = "< Equivalent scale/bias descriptor for BN fusion."]
    CUDNN_PARAM_BN_EQSCALEBIAS_DESC = 3,
    #[doc = "< Placeholder for BN equivalent scale pointer alignment."]
    CUDNN_PARAM_BN_EQSCALE_PLACEHOLDER = 4,
    #[doc = "< Placeholder for BN equivalent bias pointer alignment."]
    CUDNN_PARAM_BN_EQBIAS_PLACEHOLDER = 5,
    #[doc = "< Activation descriptor."]
    CUDNN_PARAM_ACTIVATION_DESC = 6,
    #[doc = "< Convolution descriptor."]
    CUDNN_PARAM_CONV_DESC = 7,
    #[doc = "< Filter descriptor (W)."]
    CUDNN_PARAM_WDESC = 8,
    #[doc = "< Placeholder for filter data pointer alignment (W)."]
    CUDNN_PARAM_WDATA_PLACEHOLDER = 9,
    #[doc = "< Filter gradient descriptor (dW)."]
    CUDNN_PARAM_DWDESC = 10,
    #[doc = "< Placeholder for filter gradient data pointer alignment (dW)."]
    CUDNN_PARAM_DWDATA_PLACEHOLDER = 11,
    #[doc = "< Output tensor descriptor (Y)."]
    CUDNN_PARAM_YDESC = 12,
    #[doc = "< Placeholder for output data pointer alignment (Y)."]
    CUDNN_PARAM_YDATA_PLACEHOLDER = 13,
    #[doc = "< Output gradient tensor descriptor (dY)."]
    CUDNN_PARAM_DYDESC = 14,
    #[doc = "< Placeholder for output gradient data pointer alignment (dY)."]
    CUDNN_PARAM_DYDATA_PLACEHOLDER = 15,
    #[doc = "< Output statistics tensor descriptor."]
    CUDNN_PARAM_YSTATS_DESC = 16,
    #[doc = "< Placeholder for Y sum pointer alignment."]
    CUDNN_PARAM_YSUM_PLACEHOLDER = 17,
    #[doc = "< Placeholder for Y squared sum pointer alignment."]
    CUDNN_PARAM_YSQSUM_PLACEHOLDER = 18,
    #[doc = "< BN scale/bias/mean/variance tensor descriptor."]
    CUDNN_PARAM_BN_SCALEBIAS_MEANVAR_DESC = 19,
    #[doc = "< Placeholder for BN scale pointer alignment."]
    CUDNN_PARAM_BN_SCALE_PLACEHOLDER = 20,
    #[doc = "< Placeholder for BN bias pointer alignment."]
    CUDNN_PARAM_BN_BIAS_PLACEHOLDER = 21,
    #[doc = "< Placeholder for BN saved mean pointer alignment."]
    CUDNN_PARAM_BN_SAVED_MEAN_PLACEHOLDER = 22,
    #[doc = "< Placeholder for BN saved inverse standard deviation pointer alignment."]
    CUDNN_PARAM_BN_SAVED_INVSTD_PLACEHOLDER = 23,
    #[doc = "< Placeholder for BN running mean pointer alignment."]
    CUDNN_PARAM_BN_RUNNING_MEAN_PLACEHOLDER = 24,
    #[doc = "< Placeholder for BN running variance pointer alignment."]
    CUDNN_PARAM_BN_RUNNING_VAR_PLACEHOLDER = 25,
    #[doc = "< Residual input tensor descriptor (Z)."]
    CUDNN_PARAM_ZDESC = 26,
    #[doc = "< Placeholder for residual input data pointer alignment (Z)."]
    CUDNN_PARAM_ZDATA_PLACEHOLDER = 27,
    #[doc = "< BN equivalent scale/bias descriptor for Z branch."]
    CUDNN_PARAM_BN_Z_EQSCALEBIAS_DESC = 28,
    #[doc = "< Placeholder for BN Z-branch equivalent scale pointer alignment."]
    CUDNN_PARAM_BN_Z_EQSCALE_PLACEHOLDER = 29,
    #[doc = "< Placeholder for BN Z-branch equivalent bias pointer alignment."]
    CUDNN_PARAM_BN_Z_EQBIAS_PLACEHOLDER = 30,
    #[doc = "< Activation bitmask tensor descriptor."]
    CUDNN_PARAM_ACTIVATION_BITMASK_DESC = 31,
    #[doc = "< Placeholder for activation bitmask pointer alignment."]
    CUDNN_PARAM_ACTIVATION_BITMASK_PLACEHOLDER = 32,
    #[doc = "< Input gradient tensor descriptor (dX)."]
    CUDNN_PARAM_DXDESC = 33,
    #[doc = "< Placeholder for input gradient data pointer alignment (dX)."]
    CUDNN_PARAM_DXDATA_PLACEHOLDER = 34,
    #[doc = "< Residual input gradient tensor descriptor (dZ)."]
    CUDNN_PARAM_DZDESC = 35,
    #[doc = "< Placeholder for residual input gradient data pointer alignment (dZ)."]
    CUDNN_PARAM_DZDATA_PLACEHOLDER = 36,
    #[doc = "< Placeholder for BN scale gradient pointer alignment."]
    CUDNN_PARAM_BN_DSCALE_PLACEHOLDER = 37,
    #[doc = "< Placeholder for BN bias gradient pointer alignment."]
    CUDNN_PARAM_BN_DBIAS_PLACEHOLDER = 38,
}
#[repr(u32)]
#[doc = "Pointer alignment modes for fused operations.\nSpecifies the alignment guarantee of pointers passed to fused operations,\nallowing the runtime to select optimized code paths.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnFusedOpsPointerPlaceHolder_t {
    #[doc = "< Null pointer (parameter disabled)."]
    CUDNN_PTR_NULL = 0,
    #[doc = "< Pointer is element-aligned."]
    CUDNN_PTR_ELEM_ALIGNED = 1,
    #[doc = "< Pointer is 16-byte aligned."]
    CUDNN_PTR_16B_ALIGNED = 2,
}
#[repr(u32)]
#[doc = "Labels for variant (per-execution) parameters in a fused operations parameter pack.\nUsed to identify which variant parameter to set or get in a\ncudnnFusedOpsVariantParamPack_t. These include device memory pointers\nand scalar host-side values.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnFusedOpsVariantParamLabel_t {
    #[doc = "< Pointer to input data (X) in device memory."]
    CUDNN_PTR_XDATA = 0,
    #[doc = "< Pointer to BN equivalent scale in device memory."]
    CUDNN_PTR_BN_EQSCALE = 1,
    #[doc = "< Pointer to BN equivalent bias in device memory."]
    CUDNN_PTR_BN_EQBIAS = 2,
    #[doc = "< Pointer to filter data (W) in device memory."]
    CUDNN_PTR_WDATA = 3,
    #[doc = "< Pointer to filter gradient data (dW) in device memory."]
    CUDNN_PTR_DWDATA = 4,
    #[doc = "< Pointer to output data (Y) in device memory."]
    CUDNN_PTR_YDATA = 5,
    #[doc = "< Pointer to output gradient data (dY) in device memory."]
    CUDNN_PTR_DYDATA = 6,
    #[doc = "< Pointer to Y sum accumulator in device memory."]
    CUDNN_PTR_YSUM = 7,
    #[doc = "< Pointer to Y squared sum accumulator in device memory."]
    CUDNN_PTR_YSQSUM = 8,
    #[doc = "< Pointer to workspace in device memory."]
    CUDNN_PTR_WORKSPACE = 9,
    #[doc = "< Pointer to BN scale in device memory."]
    CUDNN_PTR_BN_SCALE = 10,
    #[doc = "< Pointer to BN bias in device memory."]
    CUDNN_PTR_BN_BIAS = 11,
    #[doc = "< Pointer to BN saved mean in device memory."]
    CUDNN_PTR_BN_SAVED_MEAN = 12,
    #[doc = "< Pointer to BN saved inverse standard deviation in device memory."]
    CUDNN_PTR_BN_SAVED_INVSTD = 13,
    #[doc = "< Pointer to BN running mean in device memory."]
    CUDNN_PTR_BN_RUNNING_MEAN = 14,
    #[doc = "< Pointer to BN running variance in device memory."]
    CUDNN_PTR_BN_RUNNING_VAR = 15,
    #[doc = "< Pointer to residual input data (Z) in device memory."]
    CUDNN_PTR_ZDATA = 16,
    #[doc = "< Pointer to BN Z-branch equivalent scale in device memory."]
    CUDNN_PTR_BN_Z_EQSCALE = 17,
    #[doc = "< Pointer to BN Z-branch equivalent bias in device memory."]
    CUDNN_PTR_BN_Z_EQBIAS = 18,
    #[doc = "< Pointer to activation bitmask in device memory."]
    CUDNN_PTR_ACTIVATION_BITMASK = 19,
    #[doc = "< Pointer to input gradient data (dX) in device memory."]
    CUDNN_PTR_DXDATA = 20,
    #[doc = "< Pointer to residual input gradient data (dZ) in device memory."]
    CUDNN_PTR_DZDATA = 21,
    #[doc = "< Pointer to BN scale gradient in device memory."]
    CUDNN_PTR_BN_DSCALE = 22,
    #[doc = "< Pointer to BN bias gradient in device memory."]
    CUDNN_PTR_BN_DBIAS = 23,
    #[doc = "< Workspace size in bytes (host, size_t)."]
    CUDNN_SCALAR_SIZE_T_WORKSPACE_SIZE_IN_BYTES = 100,
    #[doc = "< BN accumulation count (host, int64_t)."]
    CUDNN_SCALAR_INT64_T_BN_ACCUMULATION_COUNT = 101,
    #[doc = "< BN exponential average factor (host, double)."]
    CUDNN_SCALAR_DOUBLE_BN_EXP_AVG_FACTOR = 102,
    #[doc = "< BN epsilon value (host, double)."]
    CUDNN_SCALAR_DOUBLE_BN_EPSILON = 103,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Check the version of the cuDNN CNN library.\nVerifies that the CNN sub-library version matches the core cuDNN version.\n\n# Returns\n\ncudnnStatus_t indicating success or version mismatch.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCnnVersionCheck() -> cudnnStatus_t;
}
#[doc = "Performance results for backward filter convolution algorithm selection.\nContains timing, memory usage, and determinism information for a given\nbackward filter convolution algorithm. Returned by algorithm search functions.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnConvolutionBwdFilterAlgoPerfStruct {
    #[doc = "< The backward filter convolution algorithm."]
    pub algo: cudnnConvolutionBwdFilterAlgo_t,
    #[doc = "< Status returned when running this algorithm."]
    pub status: cudnnStatus_t,
    #[doc = "< Execution time in milliseconds."]
    pub time: f32,
    #[doc = "< Workspace memory required in bytes."]
    pub memory: usize,
    #[doc = "< Whether the algorithm is deterministic."]
    pub determinism: cudnnDeterminism_t,
    #[doc = "< Math type used by the algorithm."]
    pub mathType: cudnnMathType_t,
    #[doc = "< Reserved for future use."]
    pub reserved: [::core::ffi::c_int; 3usize],
}
impl Default for cudnnConvolutionBwdFilterAlgoPerfStruct {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "Performance results for backward filter convolution algorithm selection.\nContains timing, memory usage, and determinism information for a given\nbackward filter convolution algorithm. Returned by algorithm search functions.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub type cudnnConvolutionBwdFilterAlgoPerf_t = cudnnConvolutionBwdFilterAlgoPerfStruct;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the maximum number of backward filter convolution algorithms available.\n\n# Arguments\n\n* `handle` [in]  - The cuDNN handle.\n* `count` [out]  -  Pointer to receive the maximum algorithm count.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(handle: cudnnHandle_t, count: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Find the best backward filter convolution algorithm by running benchmarks.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `xDesc` [in]  -               Descriptor for the input tensor.\n* `dyDesc` [in]  -              Descriptor for the gradient output tensor.\n* `convDesc` [in]  -            The convolution descriptor.\n* `dwDesc` [in]  -              Descriptor for the filter gradient.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnFindConvolutionBackwardFilterAlgorithm(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dwDesc: cudnnFilterDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Find the best backward filter convolution algorithm with user-provided buffers.\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `dyDesc` [in]  -                Descriptor for the gradient output tensor.\n* `y` [in]  -                     Pointer to gradient output data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `dwDesc` [in]  -                Descriptor for the filter gradient.\n* `dw` [out]  -                    Pointer to filter gradient data in device memory.\n* `requestedAlgoCount` [in]  -    Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -     Pointer to receive the actual number returned.\n* `perfResults` [out]  -           Array to receive the algorithm performance results.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnFindConvolutionBackwardFilterAlgorithmEx(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        dwDesc: cudnnFilterDescriptor_t,
        dw: *mut ::core::ffi::c_void,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get backward filter convolution algorithm recommendations without executing them.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `srcDesc` [in]  -             Descriptor for the input tensor.\n* `diffDesc` [in]  -            Descriptor for the gradient output tensor.\n* `convDesc` [in]  -            The convolution descriptor.\n* `gradDesc` [in]  -            Descriptor for the filter gradient.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to return.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
        handle: cudnnHandle_t,
        srcDesc: cudnnTensorDescriptor_t,
        diffDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnFilterDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get the minimum workspace size required for a backward filter convolution algorithm.\n\n# Arguments\n\n* `handle` [in]  -       The cuDNN handle.\n* `xDesc` [in]  -        Descriptor for the input tensor.\n* `dyDesc` [in]  -       Descriptor for the gradient output tensor.\n* `convDesc` [in]  -     The convolution descriptor.\n* `gradDesc` [in]  -     Descriptor for the filter gradient.\n* `algo` [in]  -         The backward filter convolution algorithm.\n* `sizeInBytes` [out]  -  Pointer to receive the required workspace size.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnFilterDescriptor_t,
        algo: cudnnConvolutionBwdFilterAlgo_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Execute a backward filter convolution (compute gradient with respect to filter weights).\nComputes: dw = alpha * dconv(x, dy) + beta * dw\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `alpha` [in]  -                 Pointer to scaling factor for the convolution result.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `dyDesc` [in]  -                Descriptor for the gradient output tensor.\n* `dy` [in]  -                    Pointer to gradient output data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `algo` [in]  -                  The backward filter convolution algorithm.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n* `beta` [in]  -                  Pointer to scaling factor for the prior filter gradient.\n* `dwDesc` [in]  -                Descriptor for the filter gradient.\n* `dw` [in,out]  -                    Pointer to filter gradient data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnConvolutionBackwardFilter(
        handle: cudnnHandle_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionBwdFilterAlgo_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        beta: *const ::core::ffi::c_void,
        dwDesc: cudnnFilterDescriptor_t,
        dw: *mut ::core::ffi::c_void,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Compute the bias gradient for batch convolution.\nComputes: db = alpha * sum(dy) + beta * db, where the sum is over spatial dimensions and batch.\n\n# Arguments\n\n* `handle` [in]  - The cuDNN handle.\n* `alpha` [in]  -  Pointer to scaling factor for the bias gradient result.\n* `dyDesc` [in]  - Descriptor for the gradient output tensor.\n* `dy` [in]  -     Pointer to gradient output data in device memory.\n* `beta` [in]  -   Pointer to scaling factor for the prior bias gradient.\n* `dbDesc` [in]  - Descriptor for the bias gradient tensor.\n* `db` [in,out]  -     Pointer to bias gradient data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnConvolutionBackwardBias(handle: cudnnHandle_t, alpha: *const ::core::ffi::c_void, dyDesc: cudnnTensorDescriptor_t, dy: *const ::core::ffi::c_void, beta: *const ::core::ffi::c_void, dbDesc: cudnnTensorDescriptor_t, db: *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Create a fused operations constant parameter pack.\n\n# Arguments\n\n* `constPack` [out]  - Pointer to receive the newly created constant parameter pack.\n* `ops` [in]  -       The fused operation type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCreateFusedOpsConstParamPack(constPack: *mut cudnnFusedOpsConstParamPack_t, ops: cudnnFusedOps_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroy a fused operations constant parameter pack.\n\n# Arguments\n\n* `constPack` [in]  - The constant parameter pack to destroy.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnDestroyFusedOpsConstParamPack(constPack: cudnnFusedOpsConstParamPack_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Set an attribute on a fused operations constant parameter pack.\n\n# Arguments\n\n* `constPack` [in,out]  -  The constant parameter pack.\n* `paramLabel` [in]  - The label identifying which parameter to set.\n* `param` [in]  -      Pointer to the parameter value.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetFusedOpsConstParamPackAttribute(constPack: cudnnFusedOpsConstParamPack_t, paramLabel: cudnnFusedOpsConstParamLabel_t, param: *const ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get an attribute from a fused operations constant parameter pack.\n\n# Arguments\n\n* `constPack` [in]  -  The constant parameter pack.\n* `paramLabel` [in]  - The label identifying which parameter to get.\n* `param` [out]  -      Pointer to receive the parameter value.\n* `isNULL` [out]  -     Pointer to receive whether the parameter is NULL.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetFusedOpsConstParamPackAttribute(constPack: cudnnFusedOpsConstParamPack_t, paramLabel: cudnnFusedOpsConstParamLabel_t, param: *mut ::core::ffi::c_void, isNULL: *mut ::core::ffi::c_int) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Create a fused operations variant parameter pack.\n\n# Arguments\n\n* `varPack` [out]  - Pointer to receive the newly created variant parameter pack.\n* `ops` [in]  -     The fused operation type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCreateFusedOpsVariantParamPack(varPack: *mut cudnnFusedOpsVariantParamPack_t, ops: cudnnFusedOps_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroy a fused operations variant parameter pack.\n\n# Arguments\n\n* `varPack` [in]  - The variant parameter pack to destroy.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnDestroyFusedOpsVariantParamPack(varPack: cudnnFusedOpsVariantParamPack_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Set an attribute on a fused operations variant parameter pack.\n\n# Arguments\n\n* `varPack` [in,out]  -    The variant parameter pack.\n* `paramLabel` [in]  - The label identifying which parameter to set.\n* `ptr` [in]  -        Pointer to the parameter value.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnSetFusedOpsVariantParamPackAttribute(varPack: cudnnFusedOpsVariantParamPack_t, paramLabel: cudnnFusedOpsVariantParamLabel_t, ptr: *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Get an attribute from a fused operations variant parameter pack.\n\n# Arguments\n\n* `varPack` [in]  -    The variant parameter pack.\n* `paramLabel` [in]  - The label identifying which parameter to get.\n* `ptr` [out]  -        Pointer to receive the parameter value.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnGetFusedOpsVariantParamPackAttribute(varPack: cudnnFusedOpsVariantParamPack_t, paramLabel: cudnnFusedOpsVariantParamLabel_t, ptr: *mut ::core::ffi::c_void) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Create a fused operations execution plan.\n\n# Arguments\n\n* `plan` [out]  - Pointer to receive the newly created fused operations plan.\n* `ops` [in]  -  The fused operation type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnCreateFusedOpsPlan(plan: *mut cudnnFusedOpsPlan_t, ops: cudnnFusedOps_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Destroy a fused operations execution plan.\n\n# Arguments\n\n* `plan` [in]  - The fused operations plan to destroy.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnDestroyFusedOpsPlan(plan: cudnnFusedOpsPlan_t) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Build a fused operations execution plan from constant parameters.\nCompiles the plan and returns the required workspace size.\n\n# Arguments\n\n* `handle` [in]  -               The cuDNN handle.\n* `plan` [in,out]  -                 The fused operations plan to build.\n* `constPack` [in]  -            The constant parameter pack with descriptors.\n* `workspaceSizeInBytes` [out]  - Pointer to receive the required workspace size.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnMakeFusedOpsPlan(handle: cudnnHandle_t, plan: cudnnFusedOpsPlan_t, constPack: cudnnFusedOpsConstParamPack_t, workspaceSizeInBytes: *mut usize) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Execute a fused operations plan.\n\n# Arguments\n\n* `handle` [in]  -  The cuDNN handle.\n* `plan` [in]  -    The fused operations plan to execute.\n* `varPack` [in]  - The variant parameter pack with data pointers and scalar values.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
    pub fn cudnnFusedOpsExecute(handle: cudnnHandle_t, plan: cudnnFusedOpsPlan_t, varPack: cudnnFusedOpsVariantParamPack_t) -> cudnnStatus_t;
}
#[repr(u32)]
#[doc = "Activation mode for causal conv1d operations.\n> **Since** cuDNN 9.22.0"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cudnnCausalConv1dActivation_t {
    #[doc = "< Identity (no activation)."]
    CUDNN_CAUSAL_CONV1D_ACTIVATION_IDENTITY = 0,
    #[doc = "< SiLU (Sigmoid Linear Unit) activation."]
    CUDNN_CAUSAL_CONV1D_ACTIVATION_SILU = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Check the version of the cuDNN SubquadraticOps library.\nVerifies that the SubquadraticOps sub-library version matches the core cuDNN version.\n\n# Returns\n\ncudnnStatus_t indicating success or version mismatch.\n> **Since** cuDNN 9.22.0"]
    pub fn cudnnSubquadraticOpsVersionCheck() -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Compute a causal (left-padded) depthwise 1D convolution with optional SiLU activation.\nComputes: y = Act( conv1d_causal(x, weight) + bias )\nCausal padding inserts (kernel_size - 1) zeros on the left and 0 on the right.\nThe convolution is depthwise: each channel is convolved independently with its\nown 1D filter.\n\n# Arguments\n\n* `stream` [in]  -      CUDA stream for kernel launch.\n* `x` [in]  -           Input tensor in device memory, layout (batch, dim, seq_len), contiguous.\n* `weight` [in]  -      Filter tensor in device memory, layout (dim, kernel_size), contiguous.\n* `bias` [in]  -        Bias tensor in device memory, layout (dim,), contiguous. Must be non-NULL.\n* `y` [out]  -           Output tensor in device memory, layout (batch, dim, seq_len), contiguous.\n* `batch` [in]  -       Batch size.\n* `dim` [in]  -         Number of channels (feature dimension).\n* `seqLen` [in]  -      Sequence length.\n* `kernelSize` [in]  -  Convolution kernel width. Supported: 2-8, 16, 32, 64, 128, 256.\n* `dataType` [in]  -    Element type for x, weight, bias, y. Supported: FLOAT, HALF, BFLOAT16.\n* `activation` [in]  -  Activation to apply after convolution + bias.\n> **Note** Not supported on Windows.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Since** cuDNN 9.22.0"]
    pub fn cudnnCausalConv1dForward(
        stream: cudaStream_t,
        x: *const ::core::ffi::c_void,
        weight: *const ::core::ffi::c_void,
        bias: *const ::core::ffi::c_void,
        y: *mut ::core::ffi::c_void,
        batch: ::core::ffi::c_int,
        dim: ::core::ffi::c_int,
        seqLen: ::core::ffi::c_int,
        kernelSize: ::core::ffi::c_int,
        dataType: cudnnDataType_t,
        activation: cudnnCausalConv1dActivation_t,
    ) -> cudnnStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = "Compute gradients for causal depthwise 1D convolution.\nComputes:\n- dx      = dL/dx       (batch, dim, seq_len)\n- dweight = dL/dweight   (dim, kernel_size) — accumulated via atomicAdd\n- dbias   = dL/dbias     (dim,)             — accumulated via atomicAdd\nThe caller must zero-initialize dweight and dbias before calling this function\nif accumulation across multiple calls is not desired.\n\n# Arguments\n\n* `stream` [in]  -      CUDA stream for kernel launch.\n* `x` [in]  -           Original input tensor (needed for activation backward), device memory.\n* `weight` [in]  -      Original filter tensor in device memory.\n* `bias` [in]  -        Original bias tensor in device memory. Must be non-NULL.\n* `dy` [in]  -          Output gradient tensor in device memory, layout (batch, dim, seq_len).\n* `dx` [out]  -          Input gradient tensor in device memory, layout (batch, dim, seq_len).\n* `dweight` [in,out]  -     Filter gradient tensor (accumulated) in device memory, layout (dim, kernel_size).\n* `dbias` [in,out]  -       Bias gradient tensor (accumulated) in device memory, layout (dim,). Must be non-NULL.\n* `batch` [in]  -       Batch size.\n* `dim` [in]  -         Number of channels.\n* `seqLen` [in]  -      Sequence length.\n* `kernelSize` [in]  -  Convolution kernel width.\n* `dataType` [in]  -    Element type for x, weight, bias, dy, dx. Supported: FLOAT, HALF, BFLOAT16.\n* `dwDataType` [in]  -  Element type for dweight, dbias. Currently only FLOAT is supported.\n* `activation` [in]  -  Activation that was applied in forward (needed for backward recompute).\n> **Note** Not supported on Windows.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Since** cuDNN 9.22.0"]
    pub fn cudnnCausalConv1dBackward(
        stream: cudaStream_t,
        x: *const ::core::ffi::c_void,
        weight: *const ::core::ffi::c_void,
        bias: *const ::core::ffi::c_void,
        dy: *const ::core::ffi::c_void,
        dx: *mut ::core::ffi::c_void,
        dweight: *mut ::core::ffi::c_void,
        dbias: *mut ::core::ffi::c_void,
        batch: ::core::ffi::c_int,
        dim: ::core::ffi::c_int,
        seqLen: ::core::ffi::c_int,
        kernelSize: ::core::ffi::c_int,
        dataType: cudnnDataType_t,
        dwDataType: cudnnDataType_t,
        activation: cudnnCausalConv1dActivation_t,
    ) -> cudnnStatus_t;
}
#[cfg(feature = "runtime-link")]
pub struct DynamicBindings {
    pub cudnnGetVersion: Option<unsafe extern "C" fn() -> usize>,
    pub cudnnGetMaxDeviceVersion: Option<unsafe extern "C" fn() -> usize>,
    pub cudnnGetCudartVersion: Option<unsafe extern "C" fn() -> usize>,
    pub cudnnGetErrorString: Option<unsafe extern "C" fn(cudnnStatus_t) -> *const ::core::ffi::c_char>,
    pub cudnnGetLastErrorString: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char, usize)>,
    pub cudnnQueryRuntimeError: Option<unsafe extern "C" fn(cudnnHandle_t, *mut cudnnStatus_t, cudnnErrQueryMode_t, *mut cudnnRuntimeTag_t) -> cudnnStatus_t>,
    pub cudnnGetProperty: Option<unsafe extern "C" fn(libraryPropertyType, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnCreate: Option<unsafe extern "C" fn(*mut cudnnHandle_t) -> cudnnStatus_t>,
    pub cudnnDestroy: Option<unsafe extern "C" fn(cudnnHandle_t) -> cudnnStatus_t>,
    pub cudnnSetStream: Option<unsafe extern "C" fn(cudnnHandle_t, cudaStream_t) -> cudnnStatus_t>,
    pub cudnnGetStream: Option<unsafe extern "C" fn(cudnnHandle_t, *mut cudaStream_t) -> cudnnStatus_t>,
    pub cudnnSetCallback: Option<unsafe extern "C" fn(::core::ffi::c_uint, *mut ::core::ffi::c_void, cudnnCallback_t) -> cudnnStatus_t>,
    pub cudnnGetCallback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_uint, *mut *mut ::core::ffi::c_void, *mut cudnnCallback_t) -> cudnnStatus_t>,
    pub cudnnGraphVersionCheck: Option<unsafe extern "C" fn() -> cudnnStatus_t>,
    pub cudnnBackendCreateDescriptor: Option<unsafe extern "C" fn(cudnnBackendDescriptorType_t, *mut cudnnBackendDescriptor_t) -> cudnnStatus_t>,
    pub cudnnBackendDestroyDescriptor: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t) -> cudnnStatus_t>,
    pub cudnnBackendInitialize: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t) -> cudnnStatus_t>,
    pub cudnnBackendFinalize: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t) -> cudnnStatus_t>,
    pub cudnnBackendSetAttribute: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t, cudnnBackendAttributeName_t, cudnnBackendAttributeType_t, i64, *const ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnBackendGetAttribute: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t, cudnnBackendAttributeName_t, cudnnBackendAttributeType_t, i64, *mut i64, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnBackendExecute: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBackendDescriptor_t, cudnnBackendDescriptor_t) -> cudnnStatus_t>,
    pub cudnnBackendPopulateCudaGraph: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBackendDescriptor_t, cudnnBackendDescriptor_t, cudaGraph_t) -> cudnnStatus_t>,
    pub cudnnBackendUpdateCudaGraph: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBackendDescriptor_t, cudnnBackendDescriptor_t, cudaGraph_t) -> cudnnStatus_t>,
    pub cudnnCreateTensorDescriptor: Option<unsafe extern "C" fn(*mut cudnnTensorDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetTensor4dDescriptor: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnTensorFormat_t, cudnnDataType_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnSetTensor4dDescriptorEx: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnDataType_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetTensor4dDescriptor: Option<
        unsafe extern "C" fn(cudnnTensorDescriptor_t, *mut cudnnDataType_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t,
    >,
    pub cudnnSetTensorNdDescriptor: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnDataType_t, ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnSetTensorNdDescriptorEx: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnTensorFormat_t, cudnnDataType_t, ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetTensorNdDescriptor: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut cudnnDataType_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetTensorSizeInBytes: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnDestroyTensorDescriptor: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t) -> cudnnStatus_t>,
    pub cudnnInitTransformDest: Option<unsafe extern "C" fn(cudnnTensorTransformDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnCreateTensorTransformDescriptor: Option<unsafe extern "C" fn(*mut cudnnTensorTransformDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetTensorTransformDescriptor: Option<unsafe extern "C" fn(cudnnTensorTransformDescriptor_t, u32, cudnnTensorFormat_t, *const i32, *const i32, *const u32, cudnnFoldingDirection_t) -> cudnnStatus_t>,
    pub cudnnGetTensorTransformDescriptor: Option<unsafe extern "C" fn(cudnnTensorTransformDescriptor_t, u32, *mut cudnnTensorFormat_t, *mut i32, *mut i32, *mut u32, *mut cudnnFoldingDirection_t) -> cudnnStatus_t>,
    pub cudnnDestroyTensorTransformDescriptor: Option<unsafe extern "C" fn(cudnnTensorTransformDescriptor_t) -> cudnnStatus_t>,
    pub cudnnTransformTensor: Option<unsafe extern "C" fn(cudnnHandle_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnTransformTensorEx: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorTransformDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnAddTensor: Option<unsafe extern "C" fn(cudnnHandle_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreateOpTensorDescriptor: Option<unsafe extern "C" fn(*mut cudnnOpTensorDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetOpTensorDescriptor: Option<unsafe extern "C" fn(cudnnOpTensorDescriptor_t, cudnnOpTensorOp_t, cudnnDataType_t, cudnnNanPropagation_t) -> cudnnStatus_t>,
    pub cudnnGetOpTensorDescriptor: Option<unsafe extern "C" fn(cudnnOpTensorDescriptor_t, *mut cudnnOpTensorOp_t, *mut cudnnDataType_t, *mut cudnnNanPropagation_t) -> cudnnStatus_t>,
    pub cudnnDestroyOpTensorDescriptor: Option<unsafe extern "C" fn(cudnnOpTensorDescriptor_t) -> cudnnStatus_t>,
    pub cudnnOpTensor: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnOpTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnCreateReduceTensorDescriptor: Option<unsafe extern "C" fn(*mut cudnnReduceTensorDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetReduceTensorDescriptor: Option<unsafe extern "C" fn(cudnnReduceTensorDescriptor_t, cudnnReduceTensorOp_t, cudnnDataType_t, cudnnNanPropagation_t, cudnnReduceTensorIndices_t, cudnnIndicesType_t) -> cudnnStatus_t>,
    pub cudnnGetReduceTensorDescriptor: Option<unsafe extern "C" fn(cudnnReduceTensorDescriptor_t, *mut cudnnReduceTensorOp_t, *mut cudnnDataType_t, *mut cudnnNanPropagation_t, *mut cudnnReduceTensorIndices_t, *mut cudnnIndicesType_t) -> cudnnStatus_t>,
    pub cudnnDestroyReduceTensorDescriptor: Option<unsafe extern "C" fn(cudnnReduceTensorDescriptor_t) -> cudnnStatus_t>,
    pub cudnnGetReductionIndicesSize: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnReduceTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnGetReductionWorkspaceSize: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnReduceTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnReduceTensor: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnReduceTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnSetTensor: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnScaleTensor: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreateFilterDescriptor: Option<unsafe extern "C" fn(*mut cudnnFilterDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetFilter4dDescriptor: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, cudnnDataType_t, cudnnTensorFormat_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetFilter4dDescriptor: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, *mut cudnnDataType_t, *mut cudnnTensorFormat_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnSetFilterNdDescriptor: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, cudnnDataType_t, cudnnTensorFormat_t, ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetFilterNdDescriptor: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, ::core::ffi::c_int, *mut cudnnDataType_t, *mut cudnnTensorFormat_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetFilterSizeInBytes: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnTransformFilter: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorTransformDescriptor_t, *const ::core::ffi::c_void, cudnnFilterDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnFilterDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnDestroyFilterDescriptor: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSoftmaxForward: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnSoftmaxAlgorithm_t, cudnnSoftmaxMode_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreatePoolingDescriptor: Option<unsafe extern "C" fn(*mut cudnnPoolingDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetPooling2dDescriptor: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, cudnnPoolingMode_t, cudnnNanPropagation_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetPooling2dDescriptor:
        Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, *mut cudnnPoolingMode_t, *mut cudnnNanPropagation_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnSetPoolingNdDescriptor: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, cudnnPoolingMode_t, cudnnNanPropagation_t, ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetPoolingNdDescriptor: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, ::core::ffi::c_int, *mut cudnnPoolingMode_t, *mut cudnnNanPropagation_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetPoolingNdForwardOutputDim: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetPooling2dForwardOutputDim: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, cudnnTensorDescriptor_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnDestroyPoolingDescriptor: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t) -> cudnnStatus_t>,
    pub cudnnPoolingForward: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnPoolingDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreateActivationDescriptor: Option<unsafe extern "C" fn(*mut cudnnActivationDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetActivationDescriptor: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t, cudnnActivationMode_t, cudnnNanPropagation_t, f64) -> cudnnStatus_t>,
    pub cudnnGetActivationDescriptor: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t, *mut cudnnActivationMode_t, *mut cudnnNanPropagation_t, *mut f64) -> cudnnStatus_t>,
    pub cudnnSetActivationDescriptorSwishBeta: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t, f64) -> cudnnStatus_t>,
    pub cudnnGetActivationDescriptorSwishBeta: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t, *mut f64) -> cudnnStatus_t>,
    pub cudnnDestroyActivationDescriptor: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t) -> cudnnStatus_t>,
    pub cudnnActivationForward: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnActivationDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreateLRNDescriptor: Option<unsafe extern "C" fn(*mut cudnnLRNDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetLRNDescriptor: Option<unsafe extern "C" fn(cudnnLRNDescriptor_t, ::core::ffi::c_uint, f64, f64, f64) -> cudnnStatus_t>,
    pub cudnnGetLRNDescriptor: Option<unsafe extern "C" fn(cudnnLRNDescriptor_t, *mut ::core::ffi::c_uint, *mut f64, *mut f64, *mut f64) -> cudnnStatus_t>,
    pub cudnnDestroyLRNDescriptor: Option<unsafe extern "C" fn(cudnnLRNDescriptor_t) -> cudnnStatus_t>,
    pub cudnnLRNCrossChannelForward: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnLRNDescriptor_t, cudnnLRNMode_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnDivisiveNormalizationForward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnLRNDescriptor_t,
            cudnnDivNormMode_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnDeriveBNTensorDescriptor: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnBatchNormMode_t) -> cudnnStatus_t>,
    pub cudnnBatchNormalizationForwardInference: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnBatchNormMode_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            f64,
        ) -> cudnnStatus_t,
    >,
    pub cudnnDeriveNormTensorDescriptor: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnNormMode_t, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnNormalizationForwardInference: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnNormMode_t,
            cudnnNormOps_t,
            cudnnNormAlgo_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnActivationDescriptor_t,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            f64,
            ::core::ffi::c_int,
        ) -> cudnnStatus_t,
    >,
    pub cudnnCreateSpatialTransformerDescriptor: Option<unsafe extern "C" fn(*mut cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetSpatialTransformerNdDescriptor: Option<unsafe extern "C" fn(cudnnSpatialTransformerDescriptor_t, cudnnSamplerType_t, cudnnDataType_t, ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnDestroySpatialTransformerDescriptor: Option<unsafe extern "C" fn(cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSpatialTfGridGeneratorForward: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnSpatialTransformerDescriptor_t, *const ::core::ffi::c_void, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnSpatialTfSamplerForward:
        Option<unsafe extern "C" fn(cudnnHandle_t, cudnnSpatialTransformerDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreateDropoutDescriptor: Option<unsafe extern "C" fn(*mut cudnnDropoutDescriptor_t) -> cudnnStatus_t>,
    pub cudnnDestroyDropoutDescriptor: Option<unsafe extern "C" fn(cudnnDropoutDescriptor_t) -> cudnnStatus_t>,
    pub cudnnDropoutGetStatesSize: Option<unsafe extern "C" fn(cudnnHandle_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnDropoutGetReserveSpaceSize: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnSetDropoutDescriptor: Option<unsafe extern "C" fn(cudnnDropoutDescriptor_t, cudnnHandle_t, f32, *mut ::core::ffi::c_void, usize, ::core::ffi::c_ulonglong) -> cudnnStatus_t>,
    pub cudnnRestoreDropoutDescriptor: Option<unsafe extern "C" fn(cudnnDropoutDescriptor_t, cudnnHandle_t, f32, *mut ::core::ffi::c_void, usize, ::core::ffi::c_ulonglong) -> cudnnStatus_t>,
    pub cudnnGetDropoutDescriptor: Option<unsafe extern "C" fn(cudnnDropoutDescriptor_t, cudnnHandle_t, *mut f32, *mut *mut ::core::ffi::c_void, *mut ::core::ffi::c_ulonglong) -> cudnnStatus_t>,
    pub cudnnDropoutForward: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnDropoutDescriptor_t, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, usize) -> cudnnStatus_t>,
    pub cudnnOpsVersionCheck: Option<unsafe extern "C" fn() -> cudnnStatus_t>,
    pub cudnnSoftmaxBackward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnSoftmaxAlgorithm_t,
            cudnnSoftmaxMode_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnPoolingBackward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnPoolingDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnActivationBackward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnActivationDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnLRNCrossChannelBackward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnLRNDescriptor_t,
            cudnnLRNMode_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnDivisiveNormalizationBackward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnLRNDescriptor_t,
            cudnnDivNormMode_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize:
        Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBatchNormMode_t, cudnnBatchNormOps_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnActivationDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnGetBatchNormalizationBackwardExWorkspaceSize: Option<
        unsafe extern "C" fn(cudnnHandle_t, cudnnBatchNormMode_t, cudnnBatchNormOps_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnActivationDescriptor_t, *mut usize) -> cudnnStatus_t,
    >,
    pub cudnnGetBatchNormalizationTrainingExReserveSpaceSize: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBatchNormMode_t, cudnnBatchNormOps_t, cudnnActivationDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnBatchNormalizationForwardTraining: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnBatchNormMode_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            f64,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            f64,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnBatchNormalizationForwardTrainingEx: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnBatchNormMode_t,
            cudnnBatchNormOps_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            f64,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            f64,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            cudnnActivationDescriptor_t,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
        ) -> cudnnStatus_t,
    >,
    pub cudnnBatchNormalizationBackward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnBatchNormMode_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            f64,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnBatchNormalizationBackwardEx: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnBatchNormMode_t,
            cudnnBatchNormOps_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            f64,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnActivationDescriptor_t,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetNormalizationForwardTrainingWorkspaceSize: Option<
        unsafe extern "C" fn(cudnnHandle_t, cudnnNormMode_t, cudnnNormOps_t, cudnnNormAlgo_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnActivationDescriptor_t, cudnnTensorDescriptor_t, *mut usize, ::core::ffi::c_int) -> cudnnStatus_t,
    >,
    pub cudnnGetNormalizationBackwardWorkspaceSize: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnNormMode_t,
            cudnnNormOps_t,
            cudnnNormAlgo_t,
            cudnnTensorDescriptor_t,
            cudnnTensorDescriptor_t,
            cudnnTensorDescriptor_t,
            cudnnTensorDescriptor_t,
            cudnnTensorDescriptor_t,
            cudnnTensorDescriptor_t,
            cudnnActivationDescriptor_t,
            cudnnTensorDescriptor_t,
            *mut usize,
            ::core::ffi::c_int,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetNormalizationTrainingReserveSpaceSize: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnNormMode_t, cudnnNormOps_t, cudnnNormAlgo_t, cudnnActivationDescriptor_t, cudnnTensorDescriptor_t, *mut usize, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnNormalizationForwardTraining: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnNormMode_t,
            cudnnNormOps_t,
            cudnnNormAlgo_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            f64,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            f64,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            cudnnActivationDescriptor_t,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            ::core::ffi::c_int,
        ) -> cudnnStatus_t,
    >,
    pub cudnnNormalizationBackward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnNormMode_t,
            cudnnNormOps_t,
            cudnnNormAlgo_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            f64,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnActivationDescriptor_t,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            ::core::ffi::c_int,
        ) -> cudnnStatus_t,
    >,
    pub cudnnSpatialTfGridGeneratorBackward: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnSpatialTransformerDescriptor_t, *const ::core::ffi::c_void, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnSpatialTfSamplerBackward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnSpatialTransformerDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnDropoutBackward: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnDropoutDescriptor_t, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, usize) -> cudnnStatus_t>,
    pub cudnnCreateRNNDescriptor: Option<unsafe extern "C" fn(*mut cudnnRNNDescriptor_t) -> cudnnStatus_t>,
    pub cudnnDestroyRNNDescriptor: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetRNNDescriptor_v8: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, cudnnRNNAlgo_t, cudnnRNNMode_t, cudnnRNNBiasMode_t, cudnnDirectionMode_t, cudnnRNNInputMode_t, cudnnDataType_t, cudnnDataType_t, cudnnMathType_t, i32, i32, i32, i32, cudnnDropoutDescriptor_t, u32) -> cudnnStatus_t>,
    pub cudnnGetRNNDescriptor_v8: Option<
        unsafe extern "C" fn(
            cudnnRNNDescriptor_t,
            *mut cudnnRNNAlgo_t,
            *mut cudnnRNNMode_t,
            *mut cudnnRNNBiasMode_t,
            *mut cudnnDirectionMode_t,
            *mut cudnnRNNInputMode_t,
            *mut cudnnDataType_t,
            *mut cudnnDataType_t,
            *mut cudnnMathType_t,
            *mut i32,
            *mut i32,
            *mut i32,
            *mut i32,
            *mut cudnnDropoutDescriptor_t,
            *mut u32,
        ) -> cudnnStatus_t,
    >,
    pub cudnnRNNSetClip_v8: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, cudnnRNNClipMode_t, cudnnNanPropagation_t, f64, f64) -> cudnnStatus_t>,
    pub cudnnRNNSetClip_v9: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, cudnnRNNClipMode_t, f64, f64) -> cudnnStatus_t>,
    pub cudnnRNNGetClip_v8: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, *mut cudnnRNNClipMode_t, *mut cudnnNanPropagation_t, *mut f64, *mut f64) -> cudnnStatus_t>,
    pub cudnnRNNGetClip_v9: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, *mut cudnnRNNClipMode_t, *mut f64, *mut f64) -> cudnnStatus_t>,
    pub cudnnBuildRNNDynamic: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnRNNDescriptor_t, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetRNNTempSpaceSizes: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnRNNDescriptor_t, cudnnForwardMode_t, cudnnRNNDataDescriptor_t, *mut usize, *mut usize) -> cudnnStatus_t>,
    pub cudnnGetRNNWeightSpaceSize: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnRNNDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnGetRNNWeightParams: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnRNNDescriptor_t, i32, usize, *const ::core::ffi::c_void, i32, cudnnTensorDescriptor_t, *mut *mut ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreateRNNDataDescriptor: Option<unsafe extern "C" fn(*mut cudnnRNNDataDescriptor_t) -> cudnnStatus_t>,
    pub cudnnDestroyRNNDataDescriptor: Option<unsafe extern "C" fn(cudnnRNNDataDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetRNNDataDescriptor: Option<unsafe extern "C" fn(cudnnRNNDataDescriptor_t, cudnnDataType_t, cudnnRNNDataLayout_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, *const ::core::ffi::c_int, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnGetRNNDataDescriptor:
        Option<unsafe extern "C" fn(cudnnRNNDataDescriptor_t, *mut cudnnDataType_t, *mut cudnnRNNDataLayout_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnRNNForward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnRNNDescriptor_t,
            cudnnForwardMode_t,
            *const i32,
            cudnnRNNDataDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnRNNDataDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnCreateSeqDataDescriptor: Option<unsafe extern "C" fn(*mut cudnnSeqDataDescriptor_t) -> cudnnStatus_t>,
    pub cudnnDestroySeqDataDescriptor: Option<unsafe extern "C" fn(cudnnSeqDataDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetSeqDataDescriptor: Option<unsafe extern "C" fn(cudnnSeqDataDescriptor_t, cudnnDataType_t, ::core::ffi::c_int, *const ::core::ffi::c_int, *const cudnnSeqDataAxis_t, usize, *const ::core::ffi::c_int, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnGetSeqDataDescriptor: Option<unsafe extern "C" fn(cudnnSeqDataDescriptor_t, *mut cudnnDataType_t, *mut ::core::ffi::c_int, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnSeqDataAxis_t, *mut usize, usize, *mut ::core::ffi::c_int, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreateAttnDescriptor: Option<unsafe extern "C" fn(*mut cudnnAttnDescriptor_t) -> cudnnStatus_t>,
    pub cudnnDestroyAttnDescriptor: Option<unsafe extern "C" fn(cudnnAttnDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetAttnDescriptor: Option<
        unsafe extern "C" fn(
            cudnnAttnDescriptor_t,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
            f64,
            cudnnDataType_t,
            cudnnDataType_t,
            cudnnMathType_t,
            cudnnDropoutDescriptor_t,
            cudnnDropoutDescriptor_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetAttnDescriptor: Option<
        unsafe extern "C" fn(
            cudnnAttnDescriptor_t,
            *mut ::core::ffi::c_uint,
            *mut ::core::ffi::c_int,
            *mut f64,
            *mut cudnnDataType_t,
            *mut cudnnDataType_t,
            *mut cudnnMathType_t,
            *mut cudnnDropoutDescriptor_t,
            *mut cudnnDropoutDescriptor_t,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetMultiHeadAttnBuffers: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnAttnDescriptor_t, *mut usize, *mut usize, *mut usize) -> cudnnStatus_t>,
    pub cudnnGetMultiHeadAttnWeights: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnAttnDescriptor_t, cudnnMultiHeadAttnWeightKind_t, usize, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnMultiHeadAttnForward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnAttnDescriptor_t,
            ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            cudnnSeqDataDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnSeqDataDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnSeqDataDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnSeqDataDescriptor_t,
            *mut ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnAdvVersionCheck: Option<unsafe extern "C" fn() -> cudnnStatus_t>,
    pub cudnnRNNBackwardData_v8: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnRNNDescriptor_t,
            *const i32,
            cudnnRNNDataDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnRNNDataDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnRNNBackwardWeights_v8: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnRNNDescriptor_t,
            cudnnWgradMode_t,
            *const i32,
            cudnnRNNDataDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnRNNDataDescriptor_t,
            *const ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnMultiHeadAttnBackwardData: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnAttnDescriptor_t,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            cudnnSeqDataDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnSeqDataDescriptor_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnSeqDataDescriptor_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            cudnnSeqDataDescriptor_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnMultiHeadAttnBackwardWeights: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnAttnDescriptor_t,
            cudnnWgradMode_t,
            cudnnSeqDataDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnSeqDataDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnSeqDataDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnSeqDataDescriptor_t,
            *const ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnCreateCTCLossDescriptor: Option<unsafe extern "C" fn(*mut cudnnCTCLossDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetCTCLossDescriptor: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, cudnnDataType_t) -> cudnnStatus_t>,
    pub cudnnSetCTCLossDescriptorEx: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnNanPropagation_t) -> cudnnStatus_t>,
    pub cudnnSetCTCLossDescriptor_v8: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnNanPropagation_t, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnSetCTCLossDescriptor_v9: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnCTCGradMode_t, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetCTCLossDescriptor: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, *mut cudnnDataType_t) -> cudnnStatus_t>,
    pub cudnnGetCTCLossDescriptorEx: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, *mut cudnnDataType_t, *mut cudnnLossNormalizationMode_t, *mut cudnnNanPropagation_t) -> cudnnStatus_t>,
    pub cudnnGetCTCLossDescriptor_v8: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, *mut cudnnDataType_t, *mut cudnnLossNormalizationMode_t, *mut cudnnNanPropagation_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetCTCLossDescriptor_v9: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, *mut cudnnDataType_t, *mut cudnnLossNormalizationMode_t, *mut cudnnCTCGradMode_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnDestroyCTCLossDescriptor: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t) -> cudnnStatus_t>,
    pub cudnnCTCLoss: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            cudnnCTCLossAlgo_t,
            cudnnCTCLossDescriptor_t,
            *mut ::core::ffi::c_void,
            usize,
        ) -> cudnnStatus_t,
    >,
    pub cudnnCTCLoss_v8: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnCTCLossAlgo_t,
            cudnnCTCLossDescriptor_t,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *const ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            usize,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetCTCLossWorkspaceSize: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *const ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int, cudnnCTCLossAlgo_t, cudnnCTCLossDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnGetCTCLossWorkspaceSize_v8: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnCTCLossAlgo_t, cudnnCTCLossDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnCreateConvolutionDescriptor: Option<unsafe extern "C" fn(*mut cudnnConvolutionDescriptor_t) -> cudnnStatus_t>,
    pub cudnnDestroyConvolutionDescriptor: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t) -> cudnnStatus_t>,
    pub cudnnSetConvolutionMathType: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, cudnnMathType_t) -> cudnnStatus_t>,
    pub cudnnGetConvolutionMathType: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, *mut cudnnMathType_t) -> cudnnStatus_t>,
    pub cudnnSetConvolutionGroupCount: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetConvolutionGroupCount: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnSetConvolutionReorderType: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, cudnnReorderType_t) -> cudnnStatus_t>,
    pub cudnnGetConvolutionReorderType: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, *mut cudnnReorderType_t) -> cudnnStatus_t>,
    pub cudnnSetConvolution2dDescriptor: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cudnnConvolutionMode_t, cudnnDataType_t) -> cudnnStatus_t>,
    pub cudnnGetConvolution2dDescriptor:
        Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionMode_t, *mut cudnnDataType_t) -> cudnnStatus_t>,
    pub cudnnSetConvolutionNdDescriptor: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int, cudnnConvolutionMode_t, cudnnDataType_t) -> cudnnStatus_t>,
    pub cudnnGetConvolutionNdDescriptor: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionMode_t, *mut cudnnDataType_t) -> cudnnStatus_t>,
    pub cudnnGetConvolution2dForwardOutputDim: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetConvolutionNdForwardOutputDim: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetConvolutionForwardAlgorithmMaxCount: Option<unsafe extern "C" fn(cudnnHandle_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnGetConvolutionForwardAlgorithm_v7: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionFwdAlgoPerf_t) -> cudnnStatus_t>,
    pub cudnnFindConvolutionForwardAlgorithm: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionFwdAlgoPerf_t) -> cudnnStatus_t>,
    pub cudnnFindConvolutionForwardAlgorithmEx: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnFilterDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnConvolutionDescriptor_t,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut cudnnConvolutionFwdAlgoPerf_t,
            *mut ::core::ffi::c_void,
            usize,
        ) -> cudnnStatus_t,
    >,
    pub cudnnIm2Col: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, cudnnFilterDescriptor_t, cudnnConvolutionDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnReorderFilterAndBias: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFilterDescriptor_t, cudnnReorderType_t, *const ::core::ffi::c_void, *mut ::core::ffi::c_void, ::core::ffi::c_int, *const ::core::ffi::c_void, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnGetConvolutionForwardWorkspaceSize: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionFwdAlgo_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnConvolutionForward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnFilterDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnConvolutionDescriptor_t,
            cudnnConvolutionFwdAlgo_t,
            *mut ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnConvolutionBiasActivationForward: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnFilterDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnConvolutionDescriptor_t,
            cudnnConvolutionFwdAlgo_t,
            *mut ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnActivationDescriptor_t,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetConvolutionBackwardDataAlgorithmMaxCount: Option<unsafe extern "C" fn(cudnnHandle_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnFindConvolutionBackwardDataAlgorithm:
        Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFilterDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionBwdDataAlgoPerf_t) -> cudnnStatus_t>,
    pub cudnnFindConvolutionBackwardDataAlgorithmEx: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnFilterDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnConvolutionDescriptor_t,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut cudnnConvolutionBwdDataAlgoPerf_t,
            *mut ::core::ffi::c_void,
            usize,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetConvolutionBackwardDataAlgorithm_v7:
        Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFilterDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionBwdDataAlgoPerf_t) -> cudnnStatus_t>,
    pub cudnnGetConvolutionBackwardDataWorkspaceSize: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFilterDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionBwdDataAlgo_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnConvolutionBackwardData: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            *const ::core::ffi::c_void,
            cudnnFilterDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnConvolutionDescriptor_t,
            cudnnConvolutionBwdDataAlgo_t,
            *mut ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetFoldedConvBackwardDataDescriptors: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnFilterDescriptor_t,
            cudnnTensorDescriptor_t,
            cudnnConvolutionDescriptor_t,
            cudnnTensorDescriptor_t,
            cudnnTensorFormat_t,
            cudnnFilterDescriptor_t,
            cudnnTensorDescriptor_t,
            cudnnConvolutionDescriptor_t,
            cudnnTensorDescriptor_t,
            cudnnTensorTransformDescriptor_t,
            cudnnTensorTransformDescriptor_t,
            cudnnTensorTransformDescriptor_t,
            cudnnTensorTransformDescriptor_t,
        ) -> cudnnStatus_t,
    >,
    pub cudnnCnnVersionCheck: Option<unsafe extern "C" fn() -> cudnnStatus_t>,
    pub cudnnGetConvolutionBackwardFilterAlgorithmMaxCount: Option<unsafe extern "C" fn(cudnnHandle_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnFindConvolutionBackwardFilterAlgorithm:
        Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnFilterDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionBwdFilterAlgoPerf_t) -> cudnnStatus_t>,
    pub cudnnFindConvolutionBackwardFilterAlgorithmEx: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnConvolutionDescriptor_t,
            cudnnFilterDescriptor_t,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut cudnnConvolutionBwdFilterAlgoPerf_t,
            *mut ::core::ffi::c_void,
            usize,
        ) -> cudnnStatus_t,
    >,
    pub cudnnGetConvolutionBackwardFilterAlgorithm_v7:
        Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnFilterDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionBwdFilterAlgoPerf_t) -> cudnnStatus_t>,
    pub cudnnGetConvolutionBackwardFilterWorkspaceSize: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnFilterDescriptor_t, cudnnConvolutionBwdFilterAlgo_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnConvolutionBackwardFilter: Option<
        unsafe extern "C" fn(
            cudnnHandle_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnTensorDescriptor_t,
            *const ::core::ffi::c_void,
            cudnnConvolutionDescriptor_t,
            cudnnConvolutionBwdFilterAlgo_t,
            *mut ::core::ffi::c_void,
            usize,
            *const ::core::ffi::c_void,
            cudnnFilterDescriptor_t,
            *mut ::core::ffi::c_void,
        ) -> cudnnStatus_t,
    >,
    pub cudnnConvolutionBackwardBias: Option<unsafe extern "C" fn(cudnnHandle_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreateFusedOpsConstParamPack: Option<unsafe extern "C" fn(*mut cudnnFusedOpsConstParamPack_t, cudnnFusedOps_t) -> cudnnStatus_t>,
    pub cudnnDestroyFusedOpsConstParamPack: Option<unsafe extern "C" fn(cudnnFusedOpsConstParamPack_t) -> cudnnStatus_t>,
    pub cudnnSetFusedOpsConstParamPackAttribute: Option<unsafe extern "C" fn(cudnnFusedOpsConstParamPack_t, cudnnFusedOpsConstParamLabel_t, *const ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnGetFusedOpsConstParamPackAttribute: Option<unsafe extern "C" fn(cudnnFusedOpsConstParamPack_t, cudnnFusedOpsConstParamLabel_t, *mut ::core::ffi::c_void, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    pub cudnnCreateFusedOpsVariantParamPack: Option<unsafe extern "C" fn(*mut cudnnFusedOpsVariantParamPack_t, cudnnFusedOps_t) -> cudnnStatus_t>,
    pub cudnnDestroyFusedOpsVariantParamPack: Option<unsafe extern "C" fn(cudnnFusedOpsVariantParamPack_t) -> cudnnStatus_t>,
    pub cudnnSetFusedOpsVariantParamPackAttribute: Option<unsafe extern "C" fn(cudnnFusedOpsVariantParamPack_t, cudnnFusedOpsVariantParamLabel_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnGetFusedOpsVariantParamPackAttribute: Option<unsafe extern "C" fn(cudnnFusedOpsVariantParamPack_t, cudnnFusedOpsVariantParamLabel_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    pub cudnnCreateFusedOpsPlan: Option<unsafe extern "C" fn(*mut cudnnFusedOpsPlan_t, cudnnFusedOps_t) -> cudnnStatus_t>,
    pub cudnnDestroyFusedOpsPlan: Option<unsafe extern "C" fn(cudnnFusedOpsPlan_t) -> cudnnStatus_t>,
    pub cudnnMakeFusedOpsPlan: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFusedOpsPlan_t, cudnnFusedOpsConstParamPack_t, *mut usize) -> cudnnStatus_t>,
    pub cudnnFusedOpsExecute: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFusedOpsPlan_t, cudnnFusedOpsVariantParamPack_t) -> cudnnStatus_t>,
    pub cudnnSubquadraticOpsVersionCheck: Option<unsafe extern "C" fn() -> cudnnStatus_t>,
    pub cudnnCausalConv1dForward: Option<
        unsafe extern "C" fn(cudaStream_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, *const ::core::ffi::c_void, *mut ::core::ffi::c_void, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cudnnDataType_t, cudnnCausalConv1dActivation_t) -> cudnnStatus_t,
    >,
    pub cudnnCausalConv1dBackward: Option<
        unsafe extern "C" fn(
            cudaStream_t,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            cudnnDataType_t,
            cudnnDataType_t,
            cudnnCausalConv1dActivation_t,
        ) -> cudnnStatus_t,
    >,
}
#[cfg(feature = "runtime-link")]
unsafe impl Send for DynamicBindings {}
#[cfg(feature = "runtime-link")]
unsafe impl Sync for DynamicBindings {}
#[cfg(feature = "runtime-link")]
pub static DYNAMIC_BINDINGS: std::sync::OnceLock<Box<DynamicBindings>> = std::sync::OnceLock::new();
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetVersion() -> usize {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetVersion {
        Some(____func) => unsafe { ____func() },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetVersion"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetMaxDeviceVersion() -> usize {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetMaxDeviceVersion {
        Some(____func) => unsafe { ____func() },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetMaxDeviceVersion"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetCudartVersion() -> usize {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetCudartVersion {
        Some(____func) => unsafe { ____func() },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetCudartVersion"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetErrorString(status: cudnnStatus_t) -> *const ::core::ffi::c_char {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetErrorString {
        Some(____func) => unsafe { ____func(status) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetErrorString"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetLastErrorString(message: *mut ::core::ffi::c_char, max_size: usize) {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetLastErrorString {
        Some(____func) => unsafe { ____func(message, max_size) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetLastErrorString"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnQueryRuntimeError(handle: cudnnHandle_t, rstatus: *mut cudnnStatus_t, mode: cudnnErrQueryMode_t, tag: *mut cudnnRuntimeTag_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnQueryRuntimeError {
        Some(____func) => unsafe { ____func(handle, rstatus, mode, tag) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnQueryRuntimeError"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetProperty(type_: libraryPropertyType, value: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetProperty {
        Some(____func) => unsafe { ____func(type_, value) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetProperty"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreate(handle: *mut cudnnHandle_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreate {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnCreate"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroy(handle: cudnnHandle_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroy {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnDestroy"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetStream(handle: cudnnHandle_t, streamId: cudaStream_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetStream {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnSetStream"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetStream(handle: cudnnHandle_t, streamId: *mut cudaStream_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetStream {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetStream"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetCallback(mask: ::core::ffi::c_uint, udata: *mut ::core::ffi::c_void, fptr: cudnnCallback_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetCallback {
        Some(____func) => unsafe { ____func(mask, udata, fptr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnSetCallback"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetCallback(mask: *mut ::core::ffi::c_uint, udata: *mut *mut ::core::ffi::c_void, fptr: *mut cudnnCallback_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetCallback {
        Some(____func) => unsafe { ____func(mask, udata, fptr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetCallback"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGraphVersionCheck() -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGraphVersionCheck {
        Some(____func) => unsafe { ____func() },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGraphVersionCheck"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBackendCreateDescriptor(descriptorType: cudnnBackendDescriptorType_t, descriptor: *mut cudnnBackendDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBackendCreateDescriptor {
        Some(____func) => unsafe { ____func(descriptorType, descriptor) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnBackendCreateDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBackendDestroyDescriptor(descriptor: cudnnBackendDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBackendDestroyDescriptor {
        Some(____func) => unsafe { ____func(descriptor) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnBackendDestroyDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBackendInitialize(descriptor: cudnnBackendDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBackendInitialize {
        Some(____func) => unsafe { ____func(descriptor) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnBackendInitialize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBackendFinalize(descriptor: cudnnBackendDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBackendFinalize {
        Some(____func) => unsafe { ____func(descriptor) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnBackendFinalize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBackendSetAttribute(descriptor: cudnnBackendDescriptor_t, attributeName: cudnnBackendAttributeName_t, attributeType: cudnnBackendAttributeType_t, elementCount: i64, arrayOfElements: *const ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBackendSetAttribute {
        Some(____func) => unsafe { ____func(descriptor, attributeName, attributeType, elementCount, arrayOfElements) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnBackendSetAttribute"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBackendGetAttribute(descriptor: cudnnBackendDescriptor_t, attributeName: cudnnBackendAttributeName_t, attributeType: cudnnBackendAttributeType_t, requestedElementCount: i64, elementCount: *mut i64, arrayOfElements: *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBackendGetAttribute {
        Some(____func) => unsafe { ____func(descriptor, attributeName, attributeType, requestedElementCount, elementCount, arrayOfElements) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnBackendGetAttribute"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBackendExecute(handle: cudnnHandle_t, executionPlan: cudnnBackendDescriptor_t, variantPack: cudnnBackendDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBackendExecute {
        Some(____func) => unsafe { ____func(handle, executionPlan, variantPack) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnBackendExecute"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBackendPopulateCudaGraph(handle: cudnnHandle_t, executionPlan: cudnnBackendDescriptor_t, variantPack: cudnnBackendDescriptor_t, graph: cudaGraph_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBackendPopulateCudaGraph {
        Some(____func) => unsafe { ____func(handle, executionPlan, variantPack, graph) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnBackendPopulateCudaGraph"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBackendUpdateCudaGraph(handle: cudnnHandle_t, executionPlan: cudnnBackendDescriptor_t, variantPack: cudnnBackendDescriptor_t, graph: cudaGraph_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBackendUpdateCudaGraph {
        Some(____func) => unsafe { ____func(handle, executionPlan, variantPack, graph) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnBackendUpdateCudaGraph"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateTensorDescriptor(tensorDesc: *mut cudnnTensorDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateTensorDescriptor {
        Some(____func) => unsafe { ____func(tensorDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetTensor4dDescriptor(tensorDesc: cudnnTensorDescriptor_t, format: cudnnTensorFormat_t, dataType: cudnnDataType_t, n: ::core::ffi::c_int, c: ::core::ffi::c_int, h: ::core::ffi::c_int, w: ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetTensor4dDescriptor {
        Some(____func) => unsafe { ____func(tensorDesc, format, dataType, n, c, h, w) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetTensor4dDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetTensor4dDescriptorEx(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: cudnnDataType_t,
    n: ::core::ffi::c_int,
    c: ::core::ffi::c_int,
    h: ::core::ffi::c_int,
    w: ::core::ffi::c_int,
    nStride: ::core::ffi::c_int,
    cStride: ::core::ffi::c_int,
    hStride: ::core::ffi::c_int,
    wStride: ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetTensor4dDescriptorEx {
        Some(____func) => unsafe { ____func(tensorDesc, dataType, n, c, h, w, nStride, cStride, hStride, wStride) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetTensor4dDescriptorEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetTensor4dDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: *mut cudnnDataType_t,
    n: *mut ::core::ffi::c_int,
    c: *mut ::core::ffi::c_int,
    h: *mut ::core::ffi::c_int,
    w: *mut ::core::ffi::c_int,
    nStride: *mut ::core::ffi::c_int,
    cStride: *mut ::core::ffi::c_int,
    hStride: *mut ::core::ffi::c_int,
    wStride: *mut ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetTensor4dDescriptor {
        Some(____func) => unsafe { ____func(tensorDesc, dataType, n, c, h, w, nStride, cStride, hStride, wStride) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetTensor4dDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetTensorNdDescriptor(tensorDesc: cudnnTensorDescriptor_t, dataType: cudnnDataType_t, nbDims: ::core::ffi::c_int, dimA: *const ::core::ffi::c_int, strideA: *const ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetTensorNdDescriptor {
        Some(____func) => unsafe { ____func(tensorDesc, dataType, nbDims, dimA, strideA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetTensorNdDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetTensorNdDescriptorEx(tensorDesc: cudnnTensorDescriptor_t, format: cudnnTensorFormat_t, dataType: cudnnDataType_t, nbDims: ::core::ffi::c_int, dimA: *const ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetTensorNdDescriptorEx {
        Some(____func) => unsafe { ____func(tensorDesc, format, dataType, nbDims, dimA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetTensorNdDescriptorEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetTensorNdDescriptor(tensorDesc: cudnnTensorDescriptor_t, nbDimsRequested: ::core::ffi::c_int, dataType: *mut cudnnDataType_t, nbDims: *mut ::core::ffi::c_int, dimA: *mut ::core::ffi::c_int, strideA: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetTensorNdDescriptor {
        Some(____func) => unsafe { ____func(tensorDesc, nbDimsRequested, dataType, nbDims, dimA, strideA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetTensorNdDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetTensorSizeInBytes(tensorDesc: cudnnTensorDescriptor_t, size: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetTensorSizeInBytes {
        Some(____func) => unsafe { ____func(tensorDesc, size) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetTensorSizeInBytes"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyTensorDescriptor(tensorDesc: cudnnTensorDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyTensorDescriptor {
        Some(____func) => unsafe { ____func(tensorDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnInitTransformDest(transformDesc: cudnnTensorTransformDescriptor_t, srcDesc: cudnnTensorDescriptor_t, destDesc: cudnnTensorDescriptor_t, destSizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnInitTransformDest {
        Some(____func) => unsafe { ____func(transformDesc, srcDesc, destDesc, destSizeInBytes) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnInitTransformDest"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateTensorTransformDescriptor(transformDesc: *mut cudnnTensorTransformDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateTensorTransformDescriptor {
        Some(____func) => unsafe { ____func(transformDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateTensorTransformDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetTensorTransformDescriptor(transformDesc: cudnnTensorTransformDescriptor_t, nbDims: u32, destFormat: cudnnTensorFormat_t, padBeforeA: *const i32, padAfterA: *const i32, foldA: *const u32, direction: cudnnFoldingDirection_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetTensorTransformDescriptor {
        Some(____func) => unsafe { ____func(transformDesc, nbDims, destFormat, padBeforeA, padAfterA, foldA, direction) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetTensorTransformDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetTensorTransformDescriptor(transformDesc: cudnnTensorTransformDescriptor_t, nbDimsRequested: u32, destFormat: *mut cudnnTensorFormat_t, padBeforeA: *mut i32, padAfterA: *mut i32, foldA: *mut u32, direction: *mut cudnnFoldingDirection_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetTensorTransformDescriptor {
        Some(____func) => unsafe { ____func(transformDesc, nbDimsRequested, destFormat, padBeforeA, padAfterA, foldA, direction) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetTensorTransformDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyTensorTransformDescriptor(transformDesc: cudnnTensorTransformDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyTensorTransformDescriptor {
        Some(____func) => unsafe { ____func(transformDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyTensorTransformDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnTransformTensor(handle: cudnnHandle_t, alpha: *const ::core::ffi::c_void, xDesc: cudnnTensorDescriptor_t, x: *const ::core::ffi::c_void, beta: *const ::core::ffi::c_void, yDesc: cudnnTensorDescriptor_t, y: *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnTransformTensor {
        Some(____func) => unsafe { ____func(handle, alpha, xDesc, x, beta, yDesc, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnTransformTensor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnTransformTensorEx(
    handle: cudnnHandle_t,
    transDesc: cudnnTensorTransformDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    srcDesc: cudnnTensorDescriptor_t,
    srcData: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    destDesc: cudnnTensorDescriptor_t,
    destData: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnTransformTensorEx {
        Some(____func) => unsafe { ____func(handle, transDesc, alpha, srcDesc, srcData, beta, destDesc, destData) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnTransformTensorEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnAddTensor(handle: cudnnHandle_t, alpha: *const ::core::ffi::c_void, aDesc: cudnnTensorDescriptor_t, A: *const ::core::ffi::c_void, beta: *const ::core::ffi::c_void, cDesc: cudnnTensorDescriptor_t, C: *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnAddTensor {
        Some(____func) => unsafe { ____func(handle, alpha, aDesc, A, beta, cDesc, C) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnAddTensor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateOpTensorDescriptor(opTensorDesc: *mut cudnnOpTensorDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateOpTensorDescriptor {
        Some(____func) => unsafe { ____func(opTensorDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateOpTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t, opTensorOp: cudnnOpTensorOp_t, opTensorCompType: cudnnDataType_t, opTensorNanOpt: cudnnNanPropagation_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetOpTensorDescriptor {
        Some(____func) => unsafe { ____func(opTensorDesc, opTensorOp, opTensorCompType, opTensorNanOpt) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetOpTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t, opTensorOp: *mut cudnnOpTensorOp_t, opTensorCompType: *mut cudnnDataType_t, opTensorNanOpt: *mut cudnnNanPropagation_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetOpTensorDescriptor {
        Some(____func) => unsafe { ____func(opTensorDesc, opTensorOp, opTensorCompType, opTensorNanOpt) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetOpTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyOpTensorDescriptor {
        Some(____func) => unsafe { ____func(opTensorDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyOpTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnOpTensor(
    handle: cudnnHandle_t,
    opTensorDesc: cudnnOpTensorDescriptor_t,
    alpha1: *const ::core::ffi::c_void,
    aDesc: cudnnTensorDescriptor_t,
    A: *const ::core::ffi::c_void,
    alpha2: *const ::core::ffi::c_void,
    bDesc: cudnnTensorDescriptor_t,
    B: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    cDesc: cudnnTensorDescriptor_t,
    C: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnOpTensor {
        Some(____func) => unsafe { ____func(handle, opTensorDesc, alpha1, aDesc, A, alpha2, bDesc, B, beta, cDesc, C) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnOpTensor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateReduceTensorDescriptor(reduceTensorDesc: *mut cudnnReduceTensorDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateReduceTensorDescriptor {
        Some(____func) => unsafe { ____func(reduceTensorDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateReduceTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetReduceTensorDescriptor(
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    reduceTensorOp: cudnnReduceTensorOp_t,
    reduceTensorCompType: cudnnDataType_t,
    reduceTensorNanOpt: cudnnNanPropagation_t,
    reduceTensorIndices: cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: cudnnIndicesType_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetReduceTensorDescriptor {
        Some(____func) => unsafe { ____func(reduceTensorDesc, reduceTensorOp, reduceTensorCompType, reduceTensorNanOpt, reduceTensorIndices, reduceTensorIndicesType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetReduceTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetReduceTensorDescriptor(
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    reduceTensorOp: *mut cudnnReduceTensorOp_t,
    reduceTensorCompType: *mut cudnnDataType_t,
    reduceTensorNanOpt: *mut cudnnNanPropagation_t,
    reduceTensorIndices: *mut cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: *mut cudnnIndicesType_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetReduceTensorDescriptor {
        Some(____func) => unsafe { ____func(reduceTensorDesc, reduceTensorOp, reduceTensorCompType, reduceTensorNanOpt, reduceTensorIndices, reduceTensorIndicesType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetReduceTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyReduceTensorDescriptor(reduceTensorDesc: cudnnReduceTensorDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyReduceTensorDescriptor {
        Some(____func) => unsafe { ____func(reduceTensorDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyReduceTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetReductionIndicesSize(handle: cudnnHandle_t, reduceTensorDesc: cudnnReduceTensorDescriptor_t, aDesc: cudnnTensorDescriptor_t, cDesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetReductionIndicesSize {
        Some(____func) => unsafe { ____func(handle, reduceTensorDesc, aDesc, cDesc, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetReductionIndicesSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetReductionWorkspaceSize(handle: cudnnHandle_t, reduceTensorDesc: cudnnReduceTensorDescriptor_t, aDesc: cudnnTensorDescriptor_t, cDesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetReductionWorkspaceSize {
        Some(____func) => unsafe { ____func(handle, reduceTensorDesc, aDesc, cDesc, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetReductionWorkspaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnReduceTensor(
    handle: cudnnHandle_t,
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    indices: *mut ::core::ffi::c_void,
    indicesSizeInBytes: usize,
    workspace: *mut ::core::ffi::c_void,
    workspaceSizeInBytes: usize,
    alpha: *const ::core::ffi::c_void,
    aDesc: cudnnTensorDescriptor_t,
    A: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    cDesc: cudnnTensorDescriptor_t,
    C: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnReduceTensor {
        Some(____func) => unsafe { ____func(handle, reduceTensorDesc, indices, indicesSizeInBytes, workspace, workspaceSizeInBytes, alpha, aDesc, A, beta, cDesc, C) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnReduceTensor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetTensor(handle: cudnnHandle_t, yDesc: cudnnTensorDescriptor_t, y: *mut ::core::ffi::c_void, valuePtr: *const ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetTensor {
        Some(____func) => unsafe { ____func(handle, yDesc, y, valuePtr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnSetTensor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnScaleTensor(handle: cudnnHandle_t, yDesc: cudnnTensorDescriptor_t, y: *mut ::core::ffi::c_void, alpha: *const ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnScaleTensor {
        Some(____func) => unsafe { ____func(handle, yDesc, y, alpha) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnScaleTensor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateFilterDescriptor(filterDesc: *mut cudnnFilterDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateFilterDescriptor {
        Some(____func) => unsafe { ____func(filterDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateFilterDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetFilter4dDescriptor(filterDesc: cudnnFilterDescriptor_t, dataType: cudnnDataType_t, format: cudnnTensorFormat_t, k: ::core::ffi::c_int, c: ::core::ffi::c_int, h: ::core::ffi::c_int, w: ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetFilter4dDescriptor {
        Some(____func) => unsafe { ____func(filterDesc, dataType, format, k, c, h, w) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetFilter4dDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetFilter4dDescriptor(filterDesc: cudnnFilterDescriptor_t, dataType: *mut cudnnDataType_t, format: *mut cudnnTensorFormat_t, k: *mut ::core::ffi::c_int, c: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int, w: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetFilter4dDescriptor {
        Some(____func) => unsafe { ____func(filterDesc, dataType, format, k, c, h, w) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetFilter4dDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetFilterNdDescriptor(filterDesc: cudnnFilterDescriptor_t, dataType: cudnnDataType_t, format: cudnnTensorFormat_t, nbDims: ::core::ffi::c_int, filterDimA: *const ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetFilterNdDescriptor {
        Some(____func) => unsafe { ____func(filterDesc, dataType, format, nbDims, filterDimA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetFilterNdDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetFilterNdDescriptor(filterDesc: cudnnFilterDescriptor_t, nbDimsRequested: ::core::ffi::c_int, dataType: *mut cudnnDataType_t, format: *mut cudnnTensorFormat_t, nbDims: *mut ::core::ffi::c_int, filterDimA: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetFilterNdDescriptor {
        Some(____func) => unsafe { ____func(filterDesc, nbDimsRequested, dataType, format, nbDims, filterDimA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetFilterNdDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetFilterSizeInBytes(filterDesc: cudnnFilterDescriptor_t, size: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetFilterSizeInBytes {
        Some(____func) => unsafe { ____func(filterDesc, size) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetFilterSizeInBytes"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnTransformFilter(
    handle: cudnnHandle_t,
    transDesc: cudnnTensorTransformDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    srcDesc: cudnnFilterDescriptor_t,
    srcData: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    destDesc: cudnnFilterDescriptor_t,
    destData: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnTransformFilter {
        Some(____func) => unsafe { ____func(handle, transDesc, alpha, srcDesc, srcData, beta, destDesc, destData) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnTransformFilter"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyFilterDescriptor(filterDesc: cudnnFilterDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyFilterDescriptor {
        Some(____func) => unsafe { ____func(filterDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyFilterDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSoftmaxForward(
    handle: cudnnHandle_t,
    algo: cudnnSoftmaxAlgorithm_t,
    mode: cudnnSoftmaxMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSoftmaxForward {
        Some(____func) => unsafe { ____func(handle, algo, mode, alpha, xDesc, x, beta, yDesc, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnSoftmaxForward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreatePoolingDescriptor(poolingDesc: *mut cudnnPoolingDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreatePoolingDescriptor {
        Some(____func) => unsafe { ____func(poolingDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreatePoolingDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetPooling2dDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    windowHeight: ::core::ffi::c_int,
    windowWidth: ::core::ffi::c_int,
    verticalPadding: ::core::ffi::c_int,
    horizontalPadding: ::core::ffi::c_int,
    verticalStride: ::core::ffi::c_int,
    horizontalStride: ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetPooling2dDescriptor {
        Some(____func) => unsafe { ____func(poolingDesc, mode, maxpoolingNanOpt, windowHeight, windowWidth, verticalPadding, horizontalPadding, verticalStride, horizontalStride) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetPooling2dDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetPooling2dDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: *mut cudnnPoolingMode_t,
    maxpoolingNanOpt: *mut cudnnNanPropagation_t,
    windowHeight: *mut ::core::ffi::c_int,
    windowWidth: *mut ::core::ffi::c_int,
    verticalPadding: *mut ::core::ffi::c_int,
    horizontalPadding: *mut ::core::ffi::c_int,
    verticalStride: *mut ::core::ffi::c_int,
    horizontalStride: *mut ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetPooling2dDescriptor {
        Some(____func) => unsafe { ____func(poolingDesc, mode, maxpoolingNanOpt, windowHeight, windowWidth, verticalPadding, horizontalPadding, verticalStride, horizontalStride) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetPooling2dDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetPoolingNdDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    nbDims: ::core::ffi::c_int,
    windowDimA: *const ::core::ffi::c_int,
    paddingA: *const ::core::ffi::c_int,
    strideA: *const ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetPoolingNdDescriptor {
        Some(____func) => unsafe { ____func(poolingDesc, mode, maxpoolingNanOpt, nbDims, windowDimA, paddingA, strideA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetPoolingNdDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetPoolingNdDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    nbDimsRequested: ::core::ffi::c_int,
    mode: *mut cudnnPoolingMode_t,
    maxpoolingNanOpt: *mut cudnnNanPropagation_t,
    nbDims: *mut ::core::ffi::c_int,
    windowDimA: *mut ::core::ffi::c_int,
    paddingA: *mut ::core::ffi::c_int,
    strideA: *mut ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetPoolingNdDescriptor {
        Some(____func) => unsafe { ____func(poolingDesc, nbDimsRequested, mode, maxpoolingNanOpt, nbDims, windowDimA, paddingA, strideA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetPoolingNdDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetPoolingNdForwardOutputDim(poolingDesc: cudnnPoolingDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, nbDims: ::core::ffi::c_int, outputTensorDimA: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetPoolingNdForwardOutputDim {
        Some(____func) => unsafe { ____func(poolingDesc, inputTensorDesc, nbDims, outputTensorDimA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetPoolingNdForwardOutputDim"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetPooling2dForwardOutputDim(poolingDesc: cudnnPoolingDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, n: *mut ::core::ffi::c_int, c: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int, w: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetPooling2dForwardOutputDim {
        Some(____func) => unsafe { ____func(poolingDesc, inputTensorDesc, n, c, h, w) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetPooling2dForwardOutputDim"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyPoolingDescriptor(poolingDesc: cudnnPoolingDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyPoolingDescriptor {
        Some(____func) => unsafe { ____func(poolingDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyPoolingDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnPoolingForward(
    handle: cudnnHandle_t,
    poolingDesc: cudnnPoolingDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnPoolingForward {
        Some(____func) => unsafe { ____func(handle, poolingDesc, alpha, xDesc, x, beta, yDesc, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnPoolingForward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateActivationDescriptor(activationDesc: *mut cudnnActivationDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateActivationDescriptor {
        Some(____func) => unsafe { ____func(activationDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateActivationDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetActivationDescriptor(activationDesc: cudnnActivationDescriptor_t, mode: cudnnActivationMode_t, reluNanOpt: cudnnNanPropagation_t, coef: f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetActivationDescriptor {
        Some(____func) => unsafe { ____func(activationDesc, mode, reluNanOpt, coef) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetActivationDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetActivationDescriptor(activationDesc: cudnnActivationDescriptor_t, mode: *mut cudnnActivationMode_t, reluNanOpt: *mut cudnnNanPropagation_t, coef: *mut f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetActivationDescriptor {
        Some(____func) => unsafe { ____func(activationDesc, mode, reluNanOpt, coef) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetActivationDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetActivationDescriptorSwishBeta(activationDesc: cudnnActivationDescriptor_t, swish_beta: f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetActivationDescriptorSwishBeta {
        Some(____func) => unsafe { ____func(activationDesc, swish_beta) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetActivationDescriptorSwishBeta"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetActivationDescriptorSwishBeta(activationDesc: cudnnActivationDescriptor_t, swish_beta: *mut f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetActivationDescriptorSwishBeta {
        Some(____func) => unsafe { ____func(activationDesc, swish_beta) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetActivationDescriptorSwishBeta"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyActivationDescriptor(activationDesc: cudnnActivationDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyActivationDescriptor {
        Some(____func) => unsafe { ____func(activationDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyActivationDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnActivationForward(
    handle: cudnnHandle_t,
    activationDesc: cudnnActivationDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnActivationForward {
        Some(____func) => unsafe { ____func(handle, activationDesc, alpha, xDesc, x, beta, yDesc, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnActivationForward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateLRNDescriptor(normDesc: *mut cudnnLRNDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateLRNDescriptor {
        Some(____func) => unsafe { ____func(normDesc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnCreateLRNDescriptor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetLRNDescriptor(normDesc: cudnnLRNDescriptor_t, lrnN: ::core::ffi::c_uint, lrnAlpha: f64, lrnBeta: f64, lrnK: f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetLRNDescriptor {
        Some(____func) => unsafe { ____func(normDesc, lrnN, lrnAlpha, lrnBeta, lrnK) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnSetLRNDescriptor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetLRNDescriptor(normDesc: cudnnLRNDescriptor_t, lrnN: *mut ::core::ffi::c_uint, lrnAlpha: *mut f64, lrnBeta: *mut f64, lrnK: *mut f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetLRNDescriptor {
        Some(____func) => unsafe { ____func(normDesc, lrnN, lrnAlpha, lrnBeta, lrnK) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetLRNDescriptor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyLRNDescriptor(lrnDesc: cudnnLRNDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyLRNDescriptor {
        Some(____func) => unsafe { ____func(lrnDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyLRNDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnLRNCrossChannelForward(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    lrnMode: cudnnLRNMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnLRNCrossChannelForward {
        Some(____func) => unsafe { ____func(handle, normDesc, lrnMode, alpha, xDesc, x, beta, yDesc, y) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnLRNCrossChannelForward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDivisiveNormalizationForward(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    mode: cudnnDivNormMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    means: *const ::core::ffi::c_void,
    temp: *mut ::core::ffi::c_void,
    temp2: *mut ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDivisiveNormalizationForward {
        Some(____func) => unsafe { ____func(handle, normDesc, mode, alpha, xDesc, x, means, temp, temp2, beta, yDesc, y) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDivisiveNormalizationForward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDeriveBNTensorDescriptor(derivedBnDesc: cudnnTensorDescriptor_t, xDesc: cudnnTensorDescriptor_t, mode: cudnnBatchNormMode_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDeriveBNTensorDescriptor {
        Some(____func) => unsafe { ____func(derivedBnDesc, xDesc, mode) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDeriveBNTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBatchNormalizationForwardInference(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    bnScale: *const ::core::ffi::c_void,
    bnBias: *const ::core::ffi::c_void,
    estimatedMean: *const ::core::ffi::c_void,
    estimatedVariance: *const ::core::ffi::c_void,
    epsilon: f64,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBatchNormalizationForwardInference {
        Some(____func) => unsafe { ____func(handle, mode, alpha, beta, xDesc, x, yDesc, y, bnScaleBiasMeanVarDesc, bnScale, bnBias, estimatedMean, estimatedVariance, epsilon) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnBatchNormalizationForwardInference"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDeriveNormTensorDescriptor(derivedNormScaleBiasDesc: cudnnTensorDescriptor_t, derivedNormMeanVarDesc: cudnnTensorDescriptor_t, xDesc: cudnnTensorDescriptor_t, mode: cudnnNormMode_t, groupCnt: ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDeriveNormTensorDescriptor {
        Some(____func) => unsafe { ____func(derivedNormScaleBiasDesc, derivedNormMeanVarDesc, xDesc, mode, groupCnt) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDeriveNormTensorDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnNormalizationForwardInference(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    normScaleBiasDesc: cudnnTensorDescriptor_t,
    normScale: *const ::core::ffi::c_void,
    normBias: *const ::core::ffi::c_void,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    estimatedMean: *const ::core::ffi::c_void,
    estimatedVariance: *const ::core::ffi::c_void,
    zDesc: cudnnTensorDescriptor_t,
    z: *const ::core::ffi::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    epsilon: f64,
    groupCnt: ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnNormalizationForwardInference {
        Some(____func) => unsafe {
            ____func(
                handle,
                mode,
                normOps,
                algo,
                alpha,
                beta,
                xDesc,
                x,
                normScaleBiasDesc,
                normScale,
                normBias,
                normMeanVarDesc,
                estimatedMean,
                estimatedVariance,
                zDesc,
                z,
                activationDesc,
                yDesc,
                y,
                epsilon,
                groupCnt,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnNormalizationForwardInference"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateSpatialTransformerDescriptor(stDesc: *mut cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateSpatialTransformerDescriptor {
        Some(____func) => unsafe { ____func(stDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateSpatialTransformerDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetSpatialTransformerNdDescriptor(stDesc: cudnnSpatialTransformerDescriptor_t, samplerType: cudnnSamplerType_t, dataType: cudnnDataType_t, nbDims: ::core::ffi::c_int, dimA: *const ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetSpatialTransformerNdDescriptor {
        Some(____func) => unsafe { ____func(stDesc, samplerType, dataType, nbDims, dimA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetSpatialTransformerNdDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroySpatialTransformerDescriptor(stDesc: cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroySpatialTransformerDescriptor {
        Some(____func) => unsafe { ____func(stDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroySpatialTransformerDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSpatialTfGridGeneratorForward(handle: cudnnHandle_t, stDesc: cudnnSpatialTransformerDescriptor_t, theta: *const ::core::ffi::c_void, grid: *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSpatialTfGridGeneratorForward {
        Some(____func) => unsafe { ____func(handle, stDesc, theta, grid) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSpatialTfGridGeneratorForward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSpatialTfSamplerForward(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    grid: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSpatialTfSamplerForward {
        Some(____func) => unsafe { ____func(handle, stDesc, alpha, xDesc, x, grid, beta, yDesc, y) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSpatialTfSamplerForward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateDropoutDescriptor(dropoutDesc: *mut cudnnDropoutDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateDropoutDescriptor {
        Some(____func) => unsafe { ____func(dropoutDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateDropoutDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyDropoutDescriptor {
        Some(____func) => unsafe { ____func(dropoutDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyDropoutDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDropoutGetStatesSize(handle: cudnnHandle_t, sizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDropoutGetStatesSize {
        Some(____func) => unsafe { ____func(handle, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDropoutGetStatesSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDropoutGetReserveSpaceSize(xdesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDropoutGetReserveSpaceSize {
        Some(____func) => unsafe { ____func(xdesc, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDropoutGetReserveSpaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t, handle: cudnnHandle_t, dropout: f32, states: *mut ::core::ffi::c_void, stateSizeInBytes: usize, seed: ::core::ffi::c_ulonglong) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetDropoutDescriptor {
        Some(____func) => unsafe { ____func(dropoutDesc, handle, dropout, states, stateSizeInBytes, seed) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetDropoutDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnRestoreDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t, handle: cudnnHandle_t, dropout: f32, states: *mut ::core::ffi::c_void, stateSizeInBytes: usize, seed: ::core::ffi::c_ulonglong) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnRestoreDropoutDescriptor {
        Some(____func) => unsafe { ____func(dropoutDesc, handle, dropout, states, stateSizeInBytes, seed) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnRestoreDropoutDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t, handle: cudnnHandle_t, dropout: *mut f32, states: *mut *mut ::core::ffi::c_void, seed: *mut ::core::ffi::c_ulonglong) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetDropoutDescriptor {
        Some(____func) => unsafe { ____func(dropoutDesc, handle, dropout, states, seed) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetDropoutDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDropoutForward(
    handle: cudnnHandle_t,
    dropoutDesc: cudnnDropoutDescriptor_t,
    xdesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    ydesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    reserveSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDropoutForward {
        Some(____func) => unsafe { ____func(handle, dropoutDesc, xdesc, x, ydesc, y, reserveSpace, reserveSpaceSizeInBytes) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnDropoutForward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnOpsVersionCheck() -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnOpsVersionCheck {
        Some(____func) => unsafe { ____func() },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnOpsVersionCheck"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSoftmaxBackward(
    handle: cudnnHandle_t,
    algo: cudnnSoftmaxAlgorithm_t,
    mode: cudnnSoftmaxMode_t,
    alpha: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSoftmaxBackward {
        Some(____func) => unsafe { ____func(handle, algo, mode, alpha, yDesc, y, dyDesc, dy, beta, dxDesc, dx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnSoftmaxBackward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnPoolingBackward(
    handle: cudnnHandle_t,
    poolingDesc: cudnnPoolingDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnPoolingBackward {
        Some(____func) => unsafe { ____func(handle, poolingDesc, alpha, yDesc, y, dyDesc, dy, xDesc, x, beta, dxDesc, dx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnPoolingBackward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnActivationBackward(
    handle: cudnnHandle_t,
    activationDesc: cudnnActivationDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnActivationBackward {
        Some(____func) => unsafe { ____func(handle, activationDesc, alpha, yDesc, y, dyDesc, dy, xDesc, x, beta, dxDesc, dx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnActivationBackward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnLRNCrossChannelBackward(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    lrnMode: cudnnLRNMode_t,
    alpha: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnLRNCrossChannelBackward {
        Some(____func) => unsafe { ____func(handle, normDesc, lrnMode, alpha, yDesc, y, dyDesc, dy, xDesc, x, beta, dxDesc, dx) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnLRNCrossChannelBackward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDivisiveNormalizationBackward(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    mode: cudnnDivNormMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    means: *const ::core::ffi::c_void,
    dy: *const ::core::ffi::c_void,
    temp: *mut ::core::ffi::c_void,
    temp2: *mut ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dXdMeansDesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    dMeans: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDivisiveNormalizationBackward {
        Some(____func) => unsafe { ____func(handle, normDesc, mode, alpha, xDesc, x, means, dy, temp, temp2, beta, dXdMeansDesc, dx, dMeans) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDivisiveNormalizationBackward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    xDesc: cudnnTensorDescriptor_t,
    zDesc: cudnnTensorDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    activationDesc: cudnnActivationDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize {
        Some(____func) => unsafe { ____func(handle, mode, bnOps, xDesc, zDesc, yDesc, bnScaleBiasMeanVarDesc, activationDesc, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetBatchNormalizationBackwardExWorkspaceSize(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    xDesc: cudnnTensorDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    dzDesc: cudnnTensorDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    dBnScaleBiasDesc: cudnnTensorDescriptor_t,
    activationDesc: cudnnActivationDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetBatchNormalizationBackwardExWorkspaceSize {
        Some(____func) => unsafe { ____func(handle, mode, bnOps, xDesc, yDesc, dyDesc, dzDesc, dxDesc, dBnScaleBiasDesc, activationDesc, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetBatchNormalizationBackwardExWorkspaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(handle: cudnnHandle_t, mode: cudnnBatchNormMode_t, bnOps: cudnnBatchNormOps_t, activationDesc: cudnnActivationDescriptor_t, xDesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetBatchNormalizationTrainingExReserveSpaceSize {
        Some(____func) => unsafe { ____func(handle, mode, bnOps, activationDesc, xDesc, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetBatchNormalizationTrainingExReserveSpaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBatchNormalizationForwardTraining(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    bnScale: *const ::core::ffi::c_void,
    bnBias: *const ::core::ffi::c_void,
    exponentialAverageFactor: f64,
    resultRunningMean: *mut ::core::ffi::c_void,
    resultRunningVariance: *mut ::core::ffi::c_void,
    epsilon: f64,
    resultSaveMean: *mut ::core::ffi::c_void,
    resultSaveInvVariance: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBatchNormalizationForwardTraining {
        Some(____func) => unsafe {
            ____func(
                handle,
                mode,
                alpha,
                beta,
                xDesc,
                x,
                yDesc,
                y,
                bnScaleBiasMeanVarDesc,
                bnScale,
                bnBias,
                exponentialAverageFactor,
                resultRunningMean,
                resultRunningVariance,
                epsilon,
                resultSaveMean,
                resultSaveInvVariance,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnBatchNormalizationForwardTraining"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBatchNormalizationForwardTrainingEx(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    xData: *const ::core::ffi::c_void,
    zDesc: cudnnTensorDescriptor_t,
    zData: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    yData: *mut ::core::ffi::c_void,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    bnScale: *const ::core::ffi::c_void,
    bnBias: *const ::core::ffi::c_void,
    exponentialAverageFactor: f64,
    resultRunningMean: *mut ::core::ffi::c_void,
    resultRunningVariance: *mut ::core::ffi::c_void,
    epsilon: f64,
    resultSaveMean: *mut ::core::ffi::c_void,
    resultSaveInvVariance: *mut ::core::ffi::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    workspace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    reserveSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBatchNormalizationForwardTrainingEx {
        Some(____func) => unsafe {
            ____func(
                handle,
                mode,
                bnOps,
                alpha,
                beta,
                xDesc,
                xData,
                zDesc,
                zData,
                yDesc,
                yData,
                bnScaleBiasMeanVarDesc,
                bnScale,
                bnBias,
                exponentialAverageFactor,
                resultRunningMean,
                resultRunningVariance,
                epsilon,
                resultSaveMean,
                resultSaveInvVariance,
                activationDesc,
                workspace,
                workSpaceSizeInBytes,
                reserveSpace,
                reserveSpaceSizeInBytes,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnBatchNormalizationForwardTrainingEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBatchNormalizationBackward(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    alphaDataDiff: *const ::core::ffi::c_void,
    betaDataDiff: *const ::core::ffi::c_void,
    alphaParamDiff: *const ::core::ffi::c_void,
    betaParamDiff: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    dBnScaleBiasDesc: cudnnTensorDescriptor_t,
    bnScale: *const ::core::ffi::c_void,
    dBnScaleResult: *mut ::core::ffi::c_void,
    dBnBiasResult: *mut ::core::ffi::c_void,
    epsilon: f64,
    savedMean: *const ::core::ffi::c_void,
    savedInvVariance: *const ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBatchNormalizationBackward {
        Some(____func) => unsafe {
            ____func(
                handle,
                mode,
                alphaDataDiff,
                betaDataDiff,
                alphaParamDiff,
                betaParamDiff,
                xDesc,
                x,
                dyDesc,
                dy,
                dxDesc,
                dx,
                dBnScaleBiasDesc,
                bnScale,
                dBnScaleResult,
                dBnBiasResult,
                epsilon,
                savedMean,
                savedInvVariance,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnBatchNormalizationBackward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBatchNormalizationBackwardEx(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    alphaDataDiff: *const ::core::ffi::c_void,
    betaDataDiff: *const ::core::ffi::c_void,
    alphaParamDiff: *const ::core::ffi::c_void,
    betaParamDiff: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    xData: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    yData: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dyData: *const ::core::ffi::c_void,
    dzDesc: cudnnTensorDescriptor_t,
    dzData: *mut ::core::ffi::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dxData: *mut ::core::ffi::c_void,
    dBnScaleBiasDesc: cudnnTensorDescriptor_t,
    bnScaleData: *const ::core::ffi::c_void,
    bnBiasData: *const ::core::ffi::c_void,
    dBnScaleData: *mut ::core::ffi::c_void,
    dBnBiasData: *mut ::core::ffi::c_void,
    epsilon: f64,
    savedMean: *const ::core::ffi::c_void,
    savedInvVariance: *const ::core::ffi::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    reserveSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBatchNormalizationBackwardEx {
        Some(____func) => unsafe {
            ____func(
                handle,
                mode,
                bnOps,
                alphaDataDiff,
                betaDataDiff,
                alphaParamDiff,
                betaParamDiff,
                xDesc,
                xData,
                yDesc,
                yData,
                dyDesc,
                dyData,
                dzDesc,
                dzData,
                dxDesc,
                dxData,
                dBnScaleBiasDesc,
                bnScaleData,
                bnBiasData,
                dBnScaleData,
                dBnBiasData,
                epsilon,
                savedMean,
                savedInvVariance,
                activationDesc,
                workSpace,
                workSpaceSizeInBytes,
                reserveSpace,
                reserveSpaceSizeInBytes,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnBatchNormalizationBackwardEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetNormalizationForwardTrainingWorkspaceSize(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    xDesc: cudnnTensorDescriptor_t,
    zDesc: cudnnTensorDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    normScaleBiasDesc: cudnnTensorDescriptor_t,
    activationDesc: cudnnActivationDescriptor_t,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
    groupCnt: ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetNormalizationForwardTrainingWorkspaceSize {
        Some(____func) => unsafe { ____func(handle, mode, normOps, algo, xDesc, zDesc, yDesc, normScaleBiasDesc, activationDesc, normMeanVarDesc, sizeInBytes, groupCnt) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetNormalizationForwardTrainingWorkspaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetNormalizationBackwardWorkspaceSize(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    xDesc: cudnnTensorDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    dzDesc: cudnnTensorDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    dNormScaleBiasDesc: cudnnTensorDescriptor_t,
    activationDesc: cudnnActivationDescriptor_t,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
    groupCnt: ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetNormalizationBackwardWorkspaceSize {
        Some(____func) => unsafe { ____func(handle, mode, normOps, algo, xDesc, yDesc, dyDesc, dzDesc, dxDesc, dNormScaleBiasDesc, activationDesc, normMeanVarDesc, sizeInBytes, groupCnt) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetNormalizationBackwardWorkspaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetNormalizationTrainingReserveSpaceSize(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    activationDesc: cudnnActivationDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
    groupCnt: ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetNormalizationTrainingReserveSpaceSize {
        Some(____func) => unsafe { ____func(handle, mode, normOps, algo, activationDesc, xDesc, sizeInBytes, groupCnt) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetNormalizationTrainingReserveSpaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnNormalizationForwardTraining(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    xData: *const ::core::ffi::c_void,
    normScaleBiasDesc: cudnnTensorDescriptor_t,
    normScale: *const ::core::ffi::c_void,
    normBias: *const ::core::ffi::c_void,
    exponentialAverageFactor: f64,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    resultRunningMean: *mut ::core::ffi::c_void,
    resultRunningVariance: *mut ::core::ffi::c_void,
    epsilon: f64,
    resultSaveMean: *mut ::core::ffi::c_void,
    resultSaveInvVariance: *mut ::core::ffi::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    zDesc: cudnnTensorDescriptor_t,
    zData: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    yData: *mut ::core::ffi::c_void,
    workspace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    reserveSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
    groupCnt: ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnNormalizationForwardTraining {
        Some(____func) => unsafe {
            ____func(
                handle,
                mode,
                normOps,
                algo,
                alpha,
                beta,
                xDesc,
                xData,
                normScaleBiasDesc,
                normScale,
                normBias,
                exponentialAverageFactor,
                normMeanVarDesc,
                resultRunningMean,
                resultRunningVariance,
                epsilon,
                resultSaveMean,
                resultSaveInvVariance,
                activationDesc,
                zDesc,
                zData,
                yDesc,
                yData,
                workspace,
                workSpaceSizeInBytes,
                reserveSpace,
                reserveSpaceSizeInBytes,
                groupCnt,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnNormalizationForwardTraining"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnNormalizationBackward(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    alphaDataDiff: *const ::core::ffi::c_void,
    betaDataDiff: *const ::core::ffi::c_void,
    alphaParamDiff: *const ::core::ffi::c_void,
    betaParamDiff: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    xData: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    yData: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dyData: *const ::core::ffi::c_void,
    dzDesc: cudnnTensorDescriptor_t,
    dzData: *mut ::core::ffi::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dxData: *mut ::core::ffi::c_void,
    dNormScaleBiasDesc: cudnnTensorDescriptor_t,
    normScaleData: *const ::core::ffi::c_void,
    normBiasData: *const ::core::ffi::c_void,
    dNormScaleData: *mut ::core::ffi::c_void,
    dNormBiasData: *mut ::core::ffi::c_void,
    epsilon: f64,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    savedMean: *const ::core::ffi::c_void,
    savedInvVariance: *const ::core::ffi::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    reserveSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
    groupCnt: ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnNormalizationBackward {
        Some(____func) => unsafe {
            ____func(
                handle,
                mode,
                normOps,
                algo,
                alphaDataDiff,
                betaDataDiff,
                alphaParamDiff,
                betaParamDiff,
                xDesc,
                xData,
                yDesc,
                yData,
                dyDesc,
                dyData,
                dzDesc,
                dzData,
                dxDesc,
                dxData,
                dNormScaleBiasDesc,
                normScaleData,
                normBiasData,
                dNormScaleData,
                dNormBiasData,
                epsilon,
                normMeanVarDesc,
                savedMean,
                savedInvVariance,
                activationDesc,
                workSpace,
                workSpaceSizeInBytes,
                reserveSpace,
                reserveSpaceSizeInBytes,
                groupCnt,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnNormalizationBackward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSpatialTfGridGeneratorBackward(handle: cudnnHandle_t, stDesc: cudnnSpatialTransformerDescriptor_t, dgrid: *const ::core::ffi::c_void, dtheta: *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSpatialTfGridGeneratorBackward {
        Some(____func) => unsafe { ____func(handle, stDesc, dgrid, dtheta) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSpatialTfGridGeneratorBackward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSpatialTfSamplerBackward(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    alphaDgrid: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    grid: *const ::core::ffi::c_void,
    betaDgrid: *const ::core::ffi::c_void,
    dgrid: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSpatialTfSamplerBackward {
        Some(____func) => unsafe { ____func(handle, stDesc, alpha, xDesc, x, beta, dxDesc, dx, alphaDgrid, dyDesc, dy, grid, betaDgrid, dgrid) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSpatialTfSamplerBackward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDropoutBackward(
    handle: cudnnHandle_t,
    dropoutDesc: cudnnDropoutDescriptor_t,
    dydesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    dxdesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    reserveSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDropoutBackward {
        Some(____func) => unsafe { ____func(handle, dropoutDesc, dydesc, dy, dxdesc, dx, reserveSpace, reserveSpaceSizeInBytes) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnDropoutBackward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateRNNDescriptor(rnnDesc: *mut cudnnRNNDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateRNNDescriptor {
        Some(____func) => unsafe { ____func(rnnDesc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnCreateRNNDescriptor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyRNNDescriptor(rnnDesc: cudnnRNNDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyRNNDescriptor {
        Some(____func) => unsafe { ____func(rnnDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyRNNDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetRNNDescriptor_v8(
    rnnDesc: cudnnRNNDescriptor_t,
    algo: cudnnRNNAlgo_t,
    cellMode: cudnnRNNMode_t,
    biasMode: cudnnRNNBiasMode_t,
    dirMode: cudnnDirectionMode_t,
    inputMode: cudnnRNNInputMode_t,
    dataType: cudnnDataType_t,
    mathPrec: cudnnDataType_t,
    mathType: cudnnMathType_t,
    inputSize: i32,
    hiddenSize: i32,
    projSize: i32,
    numLayers: i32,
    dropoutDesc: cudnnDropoutDescriptor_t,
    auxFlags: u32,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetRNNDescriptor_v8 {
        Some(____func) => unsafe { ____func(rnnDesc, algo, cellMode, biasMode, dirMode, inputMode, dataType, mathPrec, mathType, inputSize, hiddenSize, projSize, numLayers, dropoutDesc, auxFlags) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnSetRNNDescriptor_v8"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetRNNDescriptor_v8(
    rnnDesc: cudnnRNNDescriptor_t,
    algo: *mut cudnnRNNAlgo_t,
    cellMode: *mut cudnnRNNMode_t,
    biasMode: *mut cudnnRNNBiasMode_t,
    dirMode: *mut cudnnDirectionMode_t,
    inputMode: *mut cudnnRNNInputMode_t,
    dataType: *mut cudnnDataType_t,
    mathPrec: *mut cudnnDataType_t,
    mathType: *mut cudnnMathType_t,
    inputSize: *mut i32,
    hiddenSize: *mut i32,
    projSize: *mut i32,
    numLayers: *mut i32,
    dropoutDesc: *mut cudnnDropoutDescriptor_t,
    auxFlags: *mut u32,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetRNNDescriptor_v8 {
        Some(____func) => unsafe { ____func(rnnDesc, algo, cellMode, biasMode, dirMode, inputMode, dataType, mathPrec, mathType, inputSize, hiddenSize, projSize, numLayers, dropoutDesc, auxFlags) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetRNNDescriptor_v8"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnRNNSetClip_v8(rnnDesc: cudnnRNNDescriptor_t, clipMode: cudnnRNNClipMode_t, clipNanOpt: cudnnNanPropagation_t, lclip: f64, rclip: f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnRNNSetClip_v8 {
        Some(____func) => unsafe { ____func(rnnDesc, clipMode, clipNanOpt, lclip, rclip) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnRNNSetClip_v8"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnRNNSetClip_v9(rnnDesc: cudnnRNNDescriptor_t, clipMode: cudnnRNNClipMode_t, lclip: f64, rclip: f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnRNNSetClip_v9 {
        Some(____func) => unsafe { ____func(rnnDesc, clipMode, lclip, rclip) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnRNNSetClip_v9"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnRNNGetClip_v8(rnnDesc: cudnnRNNDescriptor_t, clipMode: *mut cudnnRNNClipMode_t, clipNanOpt: *mut cudnnNanPropagation_t, lclip: *mut f64, rclip: *mut f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnRNNGetClip_v8 {
        Some(____func) => unsafe { ____func(rnnDesc, clipMode, clipNanOpt, lclip, rclip) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnRNNGetClip_v8"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnRNNGetClip_v9(rnnDesc: cudnnRNNDescriptor_t, clipMode: *mut cudnnRNNClipMode_t, lclip: *mut f64, rclip: *mut f64) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnRNNGetClip_v9 {
        Some(____func) => unsafe { ____func(rnnDesc, clipMode, lclip, rclip) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnRNNGetClip_v9"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnBuildRNNDynamic(handle: cudnnHandle_t, rnnDesc: cudnnRNNDescriptor_t, miniBatch: ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnBuildRNNDynamic {
        Some(____func) => unsafe { ____func(handle, rnnDesc, miniBatch) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnBuildRNNDynamic"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetRNNTempSpaceSizes(handle: cudnnHandle_t, rnnDesc: cudnnRNNDescriptor_t, fwdMode: cudnnForwardMode_t, xDesc: cudnnRNNDataDescriptor_t, workSpaceSize: *mut usize, reserveSpaceSize: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetRNNTempSpaceSizes {
        Some(____func) => unsafe { ____func(handle, rnnDesc, fwdMode, xDesc, workSpaceSize, reserveSpaceSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetRNNTempSpaceSizes"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetRNNWeightSpaceSize(handle: cudnnHandle_t, rnnDesc: cudnnRNNDescriptor_t, weightSpaceSize: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetRNNWeightSpaceSize {
        Some(____func) => unsafe { ____func(handle, rnnDesc, weightSpaceSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetRNNWeightSpaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetRNNWeightParams(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    pseudoLayer: i32,
    weightSpaceSize: usize,
    weightSpace: *const ::core::ffi::c_void,
    linLayerID: i32,
    mDesc: cudnnTensorDescriptor_t,
    mAddr: *mut *mut ::core::ffi::c_void,
    bDesc: cudnnTensorDescriptor_t,
    bAddr: *mut *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetRNNWeightParams {
        Some(____func) => unsafe { ____func(handle, rnnDesc, pseudoLayer, weightSpaceSize, weightSpace, linLayerID, mDesc, mAddr, bDesc, bAddr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetRNNWeightParams"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateRNNDataDescriptor(rnnDataDesc: *mut cudnnRNNDataDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateRNNDataDescriptor {
        Some(____func) => unsafe { ____func(rnnDataDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateRNNDataDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyRNNDataDescriptor(rnnDataDesc: cudnnRNNDataDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyRNNDataDescriptor {
        Some(____func) => unsafe { ____func(rnnDataDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyRNNDataDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetRNNDataDescriptor(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
    dataType: cudnnDataType_t,
    layout: cudnnRNNDataLayout_t,
    maxSeqLength: ::core::ffi::c_int,
    batchSize: ::core::ffi::c_int,
    vectorSize: ::core::ffi::c_int,
    seqLengthArray: *const ::core::ffi::c_int,
    paddingFill: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetRNNDataDescriptor {
        Some(____func) => unsafe { ____func(rnnDataDesc, dataType, layout, maxSeqLength, batchSize, vectorSize, seqLengthArray, paddingFill) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetRNNDataDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetRNNDataDescriptor(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
    dataType: *mut cudnnDataType_t,
    layout: *mut cudnnRNNDataLayout_t,
    maxSeqLength: *mut ::core::ffi::c_int,
    batchSize: *mut ::core::ffi::c_int,
    vectorSize: *mut ::core::ffi::c_int,
    arrayLengthRequested: ::core::ffi::c_int,
    seqLengthArray: *mut ::core::ffi::c_int,
    paddingFill: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetRNNDataDescriptor {
        Some(____func) => unsafe { ____func(rnnDataDesc, dataType, layout, maxSeqLength, batchSize, vectorSize, arrayLengthRequested, seqLengthArray, paddingFill) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetRNNDataDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnRNNForward(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    fwdMode: cudnnForwardMode_t,
    devSeqLengths: *const i32,
    xDesc: cudnnRNNDataDescriptor_t,
    x: *const ::core::ffi::c_void,
    yDesc: cudnnRNNDataDescriptor_t,
    y: *mut ::core::ffi::c_void,
    hDesc: cudnnTensorDescriptor_t,
    hx: *const ::core::ffi::c_void,
    hy: *mut ::core::ffi::c_void,
    cDesc: cudnnTensorDescriptor_t,
    cx: *const ::core::ffi::c_void,
    cy: *mut ::core::ffi::c_void,
    weightSpaceSize: usize,
    weightSpace: *const ::core::ffi::c_void,
    workSpaceSize: usize,
    workSpace: *mut ::core::ffi::c_void,
    reserveSpaceSize: usize,
    reserveSpace: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnRNNForward {
        Some(____func) => unsafe { ____func(handle, rnnDesc, fwdMode, devSeqLengths, xDesc, x, yDesc, y, hDesc, hx, hy, cDesc, cx, cy, weightSpaceSize, weightSpace, workSpaceSize, workSpace, reserveSpaceSize, reserveSpace) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnRNNForward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateSeqDataDescriptor(seqDataDesc: *mut cudnnSeqDataDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateSeqDataDescriptor {
        Some(____func) => unsafe { ____func(seqDataDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateSeqDataDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroySeqDataDescriptor(seqDataDesc: cudnnSeqDataDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroySeqDataDescriptor {
        Some(____func) => unsafe { ____func(seqDataDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroySeqDataDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetSeqDataDescriptor(
    seqDataDesc: cudnnSeqDataDescriptor_t,
    dataType: cudnnDataType_t,
    nbDims: ::core::ffi::c_int,
    dimA: *const ::core::ffi::c_int,
    axes: *const cudnnSeqDataAxis_t,
    seqLengthArraySize: usize,
    seqLengthArray: *const ::core::ffi::c_int,
    paddingFill: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetSeqDataDescriptor {
        Some(____func) => unsafe { ____func(seqDataDesc, dataType, nbDims, dimA, axes, seqLengthArraySize, seqLengthArray, paddingFill) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetSeqDataDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetSeqDataDescriptor(
    seqDataDesc: cudnnSeqDataDescriptor_t,
    dataType: *mut cudnnDataType_t,
    nbDims: *mut ::core::ffi::c_int,
    nbDimsRequested: ::core::ffi::c_int,
    dimA: *mut ::core::ffi::c_int,
    axes: *mut cudnnSeqDataAxis_t,
    seqLengthArraySize: *mut usize,
    seqLengthSizeRequested: usize,
    seqLengthArray: *mut ::core::ffi::c_int,
    paddingFill: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetSeqDataDescriptor {
        Some(____func) => unsafe { ____func(seqDataDesc, dataType, nbDims, nbDimsRequested, dimA, axes, seqLengthArraySize, seqLengthSizeRequested, seqLengthArray, paddingFill) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetSeqDataDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateAttnDescriptor(attnDesc: *mut cudnnAttnDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateAttnDescriptor {
        Some(____func) => unsafe { ____func(attnDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateAttnDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyAttnDescriptor(attnDesc: cudnnAttnDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyAttnDescriptor {
        Some(____func) => unsafe { ____func(attnDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyAttnDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
    attnMode: ::core::ffi::c_uint,
    nHeads: ::core::ffi::c_int,
    smScaler: f64,
    dataType: cudnnDataType_t,
    computePrec: cudnnDataType_t,
    mathType: cudnnMathType_t,
    attnDropoutDesc: cudnnDropoutDescriptor_t,
    postDropoutDesc: cudnnDropoutDescriptor_t,
    qSize: ::core::ffi::c_int,
    kSize: ::core::ffi::c_int,
    vSize: ::core::ffi::c_int,
    qProjSize: ::core::ffi::c_int,
    kProjSize: ::core::ffi::c_int,
    vProjSize: ::core::ffi::c_int,
    oProjSize: ::core::ffi::c_int,
    qoMaxSeqLength: ::core::ffi::c_int,
    kvMaxSeqLength: ::core::ffi::c_int,
    maxBatchSize: ::core::ffi::c_int,
    maxBeamSize: ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetAttnDescriptor {
        Some(____func) => unsafe {
            ____func(
                attnDesc,
                attnMode,
                nHeads,
                smScaler,
                dataType,
                computePrec,
                mathType,
                attnDropoutDesc,
                postDropoutDesc,
                qSize,
                kSize,
                vSize,
                qProjSize,
                kProjSize,
                vProjSize,
                oProjSize,
                qoMaxSeqLength,
                kvMaxSeqLength,
                maxBatchSize,
                maxBeamSize,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnSetAttnDescriptor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
    attnMode: *mut ::core::ffi::c_uint,
    nHeads: *mut ::core::ffi::c_int,
    smScaler: *mut f64,
    dataType: *mut cudnnDataType_t,
    computePrec: *mut cudnnDataType_t,
    mathType: *mut cudnnMathType_t,
    attnDropoutDesc: *mut cudnnDropoutDescriptor_t,
    postDropoutDesc: *mut cudnnDropoutDescriptor_t,
    qSize: *mut ::core::ffi::c_int,
    kSize: *mut ::core::ffi::c_int,
    vSize: *mut ::core::ffi::c_int,
    qProjSize: *mut ::core::ffi::c_int,
    kProjSize: *mut ::core::ffi::c_int,
    vProjSize: *mut ::core::ffi::c_int,
    oProjSize: *mut ::core::ffi::c_int,
    qoMaxSeqLength: *mut ::core::ffi::c_int,
    kvMaxSeqLength: *mut ::core::ffi::c_int,
    maxBatchSize: *mut ::core::ffi::c_int,
    maxBeamSize: *mut ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetAttnDescriptor {
        Some(____func) => unsafe {
            ____func(
                attnDesc,
                attnMode,
                nHeads,
                smScaler,
                dataType,
                computePrec,
                mathType,
                attnDropoutDesc,
                postDropoutDesc,
                qSize,
                kSize,
                vSize,
                qProjSize,
                kProjSize,
                vProjSize,
                oProjSize,
                qoMaxSeqLength,
                kvMaxSeqLength,
                maxBatchSize,
                maxBeamSize,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnGetAttnDescriptor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetMultiHeadAttnBuffers(handle: cudnnHandle_t, attnDesc: cudnnAttnDescriptor_t, weightSizeInBytes: *mut usize, workSpaceSizeInBytes: *mut usize, reserveSpaceSizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetMultiHeadAttnBuffers {
        Some(____func) => unsafe { ____func(handle, attnDesc, weightSizeInBytes, workSpaceSizeInBytes, reserveSpaceSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetMultiHeadAttnBuffers"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetMultiHeadAttnWeights(handle: cudnnHandle_t, attnDesc: cudnnAttnDescriptor_t, wKind: cudnnMultiHeadAttnWeightKind_t, weightSizeInBytes: usize, weights: *const ::core::ffi::c_void, wDesc: cudnnTensorDescriptor_t, wAddr: *mut *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetMultiHeadAttnWeights {
        Some(____func) => unsafe { ____func(handle, attnDesc, wKind, weightSizeInBytes, weights, wDesc, wAddr) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetMultiHeadAttnWeights"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnMultiHeadAttnForward(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    currIdx: ::core::ffi::c_int,
    loWinIdx: *const ::core::ffi::c_int,
    hiWinIdx: *const ::core::ffi::c_int,
    devSeqLengthsQO: *const ::core::ffi::c_int,
    devSeqLengthsKV: *const ::core::ffi::c_int,
    qDesc: cudnnSeqDataDescriptor_t,
    queries: *const ::core::ffi::c_void,
    residuals: *const ::core::ffi::c_void,
    kDesc: cudnnSeqDataDescriptor_t,
    keys: *const ::core::ffi::c_void,
    vDesc: cudnnSeqDataDescriptor_t,
    values: *const ::core::ffi::c_void,
    oDesc: cudnnSeqDataDescriptor_t,
    out: *mut ::core::ffi::c_void,
    weightSizeInBytes: usize,
    weights: *const ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    workSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
    reserveSpace: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnMultiHeadAttnForward {
        Some(____func) => unsafe {
            ____func(
                handle,
                attnDesc,
                currIdx,
                loWinIdx,
                hiWinIdx,
                devSeqLengthsQO,
                devSeqLengthsKV,
                qDesc,
                queries,
                residuals,
                kDesc,
                keys,
                vDesc,
                values,
                oDesc,
                out,
                weightSizeInBytes,
                weights,
                workSpaceSizeInBytes,
                workSpace,
                reserveSpaceSizeInBytes,
                reserveSpace,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnMultiHeadAttnForward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnAdvVersionCheck() -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnAdvVersionCheck {
        Some(____func) => unsafe { ____func() },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnAdvVersionCheck"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnRNNBackwardData_v8(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    devSeqLengths: *const i32,
    yDesc: cudnnRNNDataDescriptor_t,
    y: *const ::core::ffi::c_void,
    dy: *const ::core::ffi::c_void,
    xDesc: cudnnRNNDataDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    hDesc: cudnnTensorDescriptor_t,
    hx: *const ::core::ffi::c_void,
    dhy: *const ::core::ffi::c_void,
    dhx: *mut ::core::ffi::c_void,
    cDesc: cudnnTensorDescriptor_t,
    cx: *const ::core::ffi::c_void,
    dcy: *const ::core::ffi::c_void,
    dcx: *mut ::core::ffi::c_void,
    weightSpaceSize: usize,
    weightSpace: *const ::core::ffi::c_void,
    workSpaceSize: usize,
    workSpace: *mut ::core::ffi::c_void,
    reserveSpaceSize: usize,
    reserveSpace: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnRNNBackwardData_v8 {
        Some(____func) => unsafe {
            ____func(
                handle,
                rnnDesc,
                devSeqLengths,
                yDesc,
                y,
                dy,
                xDesc,
                dx,
                hDesc,
                hx,
                dhy,
                dhx,
                cDesc,
                cx,
                dcy,
                dcx,
                weightSpaceSize,
                weightSpace,
                workSpaceSize,
                workSpace,
                reserveSpaceSize,
                reserveSpace,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnRNNBackwardData_v8"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnRNNBackwardWeights_v8(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    addGrad: cudnnWgradMode_t,
    devSeqLengths: *const i32,
    xDesc: cudnnRNNDataDescriptor_t,
    x: *const ::core::ffi::c_void,
    hDesc: cudnnTensorDescriptor_t,
    hx: *const ::core::ffi::c_void,
    yDesc: cudnnRNNDataDescriptor_t,
    y: *const ::core::ffi::c_void,
    weightSpaceSize: usize,
    dweightSpace: *mut ::core::ffi::c_void,
    workSpaceSize: usize,
    workSpace: *mut ::core::ffi::c_void,
    reserveSpaceSize: usize,
    reserveSpace: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnRNNBackwardWeights_v8 {
        Some(____func) => unsafe { ____func(handle, rnnDesc, addGrad, devSeqLengths, xDesc, x, hDesc, hx, yDesc, y, weightSpaceSize, dweightSpace, workSpaceSize, workSpace, reserveSpaceSize, reserveSpace) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnRNNBackwardWeights_v8"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnMultiHeadAttnBackwardData(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    loWinIdx: *const ::core::ffi::c_int,
    hiWinIdx: *const ::core::ffi::c_int,
    devSeqLengthsDQDO: *const ::core::ffi::c_int,
    devSeqLengthsDKDV: *const ::core::ffi::c_int,
    doDesc: cudnnSeqDataDescriptor_t,
    dout: *const ::core::ffi::c_void,
    dqDesc: cudnnSeqDataDescriptor_t,
    dqueries: *mut ::core::ffi::c_void,
    queries: *const ::core::ffi::c_void,
    dkDesc: cudnnSeqDataDescriptor_t,
    dkeys: *mut ::core::ffi::c_void,
    keys: *const ::core::ffi::c_void,
    dvDesc: cudnnSeqDataDescriptor_t,
    dvalues: *mut ::core::ffi::c_void,
    values: *const ::core::ffi::c_void,
    weightSizeInBytes: usize,
    weights: *const ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    workSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
    reserveSpace: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnMultiHeadAttnBackwardData {
        Some(____func) => unsafe {
            ____func(
                handle,
                attnDesc,
                loWinIdx,
                hiWinIdx,
                devSeqLengthsDQDO,
                devSeqLengthsDKDV,
                doDesc,
                dout,
                dqDesc,
                dqueries,
                queries,
                dkDesc,
                dkeys,
                keys,
                dvDesc,
                dvalues,
                values,
                weightSizeInBytes,
                weights,
                workSpaceSizeInBytes,
                workSpace,
                reserveSpaceSizeInBytes,
                reserveSpace,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnMultiHeadAttnBackwardData"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnMultiHeadAttnBackwardWeights(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    addGrad: cudnnWgradMode_t,
    qDesc: cudnnSeqDataDescriptor_t,
    queries: *const ::core::ffi::c_void,
    kDesc: cudnnSeqDataDescriptor_t,
    keys: *const ::core::ffi::c_void,
    vDesc: cudnnSeqDataDescriptor_t,
    values: *const ::core::ffi::c_void,
    doDesc: cudnnSeqDataDescriptor_t,
    dout: *const ::core::ffi::c_void,
    weightSizeInBytes: usize,
    weights: *const ::core::ffi::c_void,
    dweights: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    workSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
    reserveSpace: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnMultiHeadAttnBackwardWeights {
        Some(____func) => unsafe {
            ____func(
                handle,
                attnDesc,
                addGrad,
                qDesc,
                queries,
                kDesc,
                keys,
                vDesc,
                values,
                doDesc,
                dout,
                weightSizeInBytes,
                weights,
                dweights,
                workSpaceSizeInBytes,
                workSpace,
                reserveSpaceSizeInBytes,
                reserveSpace,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnMultiHeadAttnBackwardWeights"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateCTCLossDescriptor(ctcLossDesc: *mut cudnnCTCLossDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateCTCLossDescriptor {
        Some(____func) => unsafe { ____func(ctcLossDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateCTCLossDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetCTCLossDescriptor {
        Some(____func) => unsafe { ____func(ctcLossDesc, compType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetCTCLossDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetCTCLossDescriptorEx(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t, normMode: cudnnLossNormalizationMode_t, gradMode: cudnnNanPropagation_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetCTCLossDescriptorEx {
        Some(____func) => unsafe { ____func(ctcLossDesc, compType, normMode, gradMode) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetCTCLossDescriptorEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetCTCLossDescriptor_v8(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t, normMode: cudnnLossNormalizationMode_t, gradMode: cudnnNanPropagation_t, maxLabelLength: ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetCTCLossDescriptor_v8 {
        Some(____func) => unsafe { ____func(ctcLossDesc, compType, normMode, gradMode, maxLabelLength) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetCTCLossDescriptor_v8"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetCTCLossDescriptor_v9(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t, normMode: cudnnLossNormalizationMode_t, ctcGradMode: cudnnCTCGradMode_t, maxLabelLength: ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetCTCLossDescriptor_v9 {
        Some(____func) => unsafe { ____func(ctcLossDesc, compType, normMode, ctcGradMode, maxLabelLength) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetCTCLossDescriptor_v9"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: *mut cudnnDataType_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetCTCLossDescriptor {
        Some(____func) => unsafe { ____func(ctcLossDesc, compType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetCTCLossDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetCTCLossDescriptorEx(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: *mut cudnnDataType_t, normMode: *mut cudnnLossNormalizationMode_t, gradMode: *mut cudnnNanPropagation_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetCTCLossDescriptorEx {
        Some(____func) => unsafe { ____func(ctcLossDesc, compType, normMode, gradMode) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetCTCLossDescriptorEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetCTCLossDescriptor_v8(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: *mut cudnnDataType_t, normMode: *mut cudnnLossNormalizationMode_t, gradMode: *mut cudnnNanPropagation_t, maxLabelLength: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetCTCLossDescriptor_v8 {
        Some(____func) => unsafe { ____func(ctcLossDesc, compType, normMode, gradMode, maxLabelLength) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetCTCLossDescriptor_v8"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetCTCLossDescriptor_v9(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: *mut cudnnDataType_t, normMode: *mut cudnnLossNormalizationMode_t, ctcGradMode: *mut cudnnCTCGradMode_t, maxLabelLength: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetCTCLossDescriptor_v9 {
        Some(____func) => unsafe { ____func(ctcLossDesc, compType, normMode, ctcGradMode, maxLabelLength) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetCTCLossDescriptor_v9"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyCTCLossDescriptor {
        Some(____func) => unsafe { ____func(ctcLossDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyCTCLossDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCTCLoss(
    handle: cudnnHandle_t,
    probsDesc: cudnnTensorDescriptor_t,
    probs: *const ::core::ffi::c_void,
    hostLabels: *const ::core::ffi::c_int,
    hostLabelLengths: *const ::core::ffi::c_int,
    hostInputLengths: *const ::core::ffi::c_int,
    costs: *mut ::core::ffi::c_void,
    gradientsDesc: cudnnTensorDescriptor_t,
    gradients: *mut ::core::ffi::c_void,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    workspace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCTCLoss {
        Some(____func) => unsafe { ____func(handle, probsDesc, probs, hostLabels, hostLabelLengths, hostInputLengths, costs, gradientsDesc, gradients, algo, ctcLossDesc, workspace, workSpaceSizeInBytes) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnCTCLoss"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCTCLoss_v8(
    handle: cudnnHandle_t,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    probsDesc: cudnnTensorDescriptor_t,
    probs: *const ::core::ffi::c_void,
    labels: *const ::core::ffi::c_int,
    labelLengths: *const ::core::ffi::c_int,
    inputLengths: *const ::core::ffi::c_int,
    costs: *mut ::core::ffi::c_void,
    gradientsDesc: cudnnTensorDescriptor_t,
    gradients: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    workspace: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCTCLoss_v8 {
        Some(____func) => unsafe { ____func(handle, algo, ctcLossDesc, probsDesc, probs, labels, labelLengths, inputLengths, costs, gradientsDesc, gradients, workSpaceSizeInBytes, workspace) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnCTCLoss_v8"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetCTCLossWorkspaceSize(
    handle: cudnnHandle_t,
    probsDesc: cudnnTensorDescriptor_t,
    gradientsDesc: cudnnTensorDescriptor_t,
    labels: *const ::core::ffi::c_int,
    labelLengths: *const ::core::ffi::c_int,
    inputLengths: *const ::core::ffi::c_int,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetCTCLossWorkspaceSize {
        Some(____func) => unsafe { ____func(handle, probsDesc, gradientsDesc, labels, labelLengths, inputLengths, algo, ctcLossDesc, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetCTCLossWorkspaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetCTCLossWorkspaceSize_v8(handle: cudnnHandle_t, algo: cudnnCTCLossAlgo_t, ctcLossDesc: cudnnCTCLossDescriptor_t, probsDesc: cudnnTensorDescriptor_t, gradientsDesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetCTCLossWorkspaceSize_v8 {
        Some(____func) => unsafe { ____func(handle, algo, ctcLossDesc, probsDesc, gradientsDesc, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetCTCLossWorkspaceSize_v8"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateConvolutionDescriptor(convDesc: *mut cudnnConvolutionDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateConvolutionDescriptor {
        Some(____func) => unsafe { ____func(convDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateConvolutionDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyConvolutionDescriptor(convDesc: cudnnConvolutionDescriptor_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyConvolutionDescriptor {
        Some(____func) => unsafe { ____func(convDesc) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyConvolutionDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetConvolutionMathType(convDesc: cudnnConvolutionDescriptor_t, mathType: cudnnMathType_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetConvolutionMathType {
        Some(____func) => unsafe { ____func(convDesc, mathType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetConvolutionMathType"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionMathType(convDesc: cudnnConvolutionDescriptor_t, mathType: *mut cudnnMathType_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionMathType {
        Some(____func) => unsafe { ____func(convDesc, mathType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionMathType"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetConvolutionGroupCount(convDesc: cudnnConvolutionDescriptor_t, groupCount: ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetConvolutionGroupCount {
        Some(____func) => unsafe { ____func(convDesc, groupCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetConvolutionGroupCount"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionGroupCount(convDesc: cudnnConvolutionDescriptor_t, groupCount: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionGroupCount {
        Some(____func) => unsafe { ____func(convDesc, groupCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionGroupCount"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetConvolutionReorderType(convDesc: cudnnConvolutionDescriptor_t, reorderType: cudnnReorderType_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetConvolutionReorderType {
        Some(____func) => unsafe { ____func(convDesc, reorderType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetConvolutionReorderType"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionReorderType(convDesc: cudnnConvolutionDescriptor_t, reorderType: *mut cudnnReorderType_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionReorderType {
        Some(____func) => unsafe { ____func(convDesc, reorderType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionReorderType"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetConvolution2dDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    pad_h: ::core::ffi::c_int,
    pad_w: ::core::ffi::c_int,
    u: ::core::ffi::c_int,
    v: ::core::ffi::c_int,
    dilation_h: ::core::ffi::c_int,
    dilation_w: ::core::ffi::c_int,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetConvolution2dDescriptor {
        Some(____func) => unsafe { ____func(convDesc, pad_h, pad_w, u, v, dilation_h, dilation_w, mode, computeType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetConvolution2dDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolution2dDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    pad_h: *mut ::core::ffi::c_int,
    pad_w: *mut ::core::ffi::c_int,
    u: *mut ::core::ffi::c_int,
    v: *mut ::core::ffi::c_int,
    dilation_h: *mut ::core::ffi::c_int,
    dilation_w: *mut ::core::ffi::c_int,
    mode: *mut cudnnConvolutionMode_t,
    computeType: *mut cudnnDataType_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolution2dDescriptor {
        Some(____func) => unsafe { ____func(convDesc, pad_h, pad_w, u, v, dilation_h, dilation_w, mode, computeType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolution2dDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetConvolutionNdDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    arrayLength: ::core::ffi::c_int,
    padA: *const ::core::ffi::c_int,
    filterStrideA: *const ::core::ffi::c_int,
    dilationA: *const ::core::ffi::c_int,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetConvolutionNdDescriptor {
        Some(____func) => unsafe { ____func(convDesc, arrayLength, padA, filterStrideA, dilationA, mode, computeType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetConvolutionNdDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionNdDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    arrayLengthRequested: ::core::ffi::c_int,
    arrayLength: *mut ::core::ffi::c_int,
    padA: *mut ::core::ffi::c_int,
    strideA: *mut ::core::ffi::c_int,
    dilationA: *mut ::core::ffi::c_int,
    mode: *mut cudnnConvolutionMode_t,
    computeType: *mut cudnnDataType_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionNdDescriptor {
        Some(____func) => unsafe { ____func(convDesc, arrayLengthRequested, arrayLength, padA, strideA, dilationA, mode, computeType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionNdDescriptor"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolution2dForwardOutputDim(
    convDesc: cudnnConvolutionDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
    n: *mut ::core::ffi::c_int,
    c: *mut ::core::ffi::c_int,
    h: *mut ::core::ffi::c_int,
    w: *mut ::core::ffi::c_int,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolution2dForwardOutputDim {
        Some(____func) => unsafe { ____func(convDesc, inputTensorDesc, filterDesc, n, c, h, w) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolution2dForwardOutputDim"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionNdForwardOutputDim(convDesc: cudnnConvolutionDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, filterDesc: cudnnFilterDescriptor_t, nbDims: ::core::ffi::c_int, tensorOuputDimA: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionNdForwardOutputDim {
        Some(____func) => unsafe { ____func(convDesc, inputTensorDesc, filterDesc, nbDims, tensorOuputDimA) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionNdForwardOutputDim"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionForwardAlgorithmMaxCount(handle: cudnnHandle_t, count: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionForwardAlgorithmMaxCount {
        Some(____func) => unsafe { ____func(handle, count) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionForwardAlgorithmMaxCount"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionForwardAlgorithm_v7(
    handle: cudnnHandle_t,
    srcDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    destDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionForwardAlgorithm_v7 {
        Some(____func) => unsafe { ____func(handle, srcDesc, filterDesc, convDesc, destDesc, requestedAlgoCount, returnedAlgoCount, perfResults) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionForwardAlgorithm_v7"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnFindConvolutionForwardAlgorithm(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    wDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnFindConvolutionForwardAlgorithm {
        Some(____func) => unsafe { ____func(handle, xDesc, wDesc, convDesc, yDesc, requestedAlgoCount, returnedAlgoCount, perfResults) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnFindConvolutionForwardAlgorithm"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnFindConvolutionForwardAlgorithmEx(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnFindConvolutionForwardAlgorithmEx {
        Some(____func) => unsafe { ____func(handle, xDesc, x, wDesc, w, convDesc, yDesc, y, requestedAlgoCount, returnedAlgoCount, perfResults, workSpace, workSpaceSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnFindConvolutionForwardAlgorithmEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnIm2Col(handle: cudnnHandle_t, xDesc: cudnnTensorDescriptor_t, x: *const ::core::ffi::c_void, wDesc: cudnnFilterDescriptor_t, convDesc: cudnnConvolutionDescriptor_t, colBuffer: *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnIm2Col {
        Some(____func) => unsafe { ____func(handle, xDesc, x, wDesc, convDesc, colBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnIm2Col"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnReorderFilterAndBias(
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    reorderType: cudnnReorderType_t,
    filterData: *const ::core::ffi::c_void,
    reorderedFilterData: *mut ::core::ffi::c_void,
    reorderBias: ::core::ffi::c_int,
    biasData: *const ::core::ffi::c_void,
    reorderedBiasData: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnReorderFilterAndBias {
        Some(____func) => unsafe { ____func(handle, filterDesc, reorderType, filterData, reorderedFilterData, reorderBias, biasData, reorderedBiasData) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnReorderFilterAndBias"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionForwardWorkspaceSize(handle: cudnnHandle_t, xDesc: cudnnTensorDescriptor_t, wDesc: cudnnFilterDescriptor_t, convDesc: cudnnConvolutionDescriptor_t, yDesc: cudnnTensorDescriptor_t, algo: cudnnConvolutionFwdAlgo_t, sizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionForwardWorkspaceSize {
        Some(____func) => unsafe { ____func(handle, xDesc, wDesc, convDesc, yDesc, algo, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionForwardWorkspaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnConvolutionForward(
    handle: cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionFwdAlgo_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    beta: *const ::core::ffi::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnConvolutionForward {
        Some(____func) => unsafe { ____func(handle, alpha, xDesc, x, wDesc, w, convDesc, algo, workSpace, workSpaceSizeInBytes, beta, yDesc, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnConvolutionForward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnConvolutionBiasActivationForward(
    handle: cudnnHandle_t,
    alpha1: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionFwdAlgo_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    alpha2: *const ::core::ffi::c_void,
    zDesc: cudnnTensorDescriptor_t,
    z: *const ::core::ffi::c_void,
    biasDesc: cudnnTensorDescriptor_t,
    bias: *const ::core::ffi::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnConvolutionBiasActivationForward {
        Some(____func) => unsafe { ____func(handle, alpha1, xDesc, x, wDesc, w, convDesc, algo, workSpace, workSpaceSizeInBytes, alpha2, zDesc, z, biasDesc, bias, activationDesc, yDesc, y) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnConvolutionBiasActivationForward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(handle: cudnnHandle_t, count: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionBackwardDataAlgorithmMaxCount {
        Some(____func) => unsafe { ____func(handle, count) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionBackwardDataAlgorithmMaxCount"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnFindConvolutionBackwardDataAlgorithm(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnFindConvolutionBackwardDataAlgorithm {
        Some(____func) => unsafe { ____func(handle, wDesc, dyDesc, convDesc, dxDesc, requestedAlgoCount, returnedAlgoCount, perfResults) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnFindConvolutionBackwardDataAlgorithm"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnFindConvolutionBackwardDataAlgorithmEx(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnFindConvolutionBackwardDataAlgorithmEx {
        Some(____func) => unsafe { ____func(handle, wDesc, w, dyDesc, dy, convDesc, dxDesc, dx, requestedAlgoCount, returnedAlgoCount, perfResults, workSpace, workSpaceSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnFindConvolutionBackwardDataAlgorithmEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionBackwardDataAlgorithm_v7 {
        Some(____func) => unsafe { ____func(handle, filterDesc, diffDesc, convDesc, gradDesc, requestedAlgoCount, returnedAlgoCount, perfResults) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionBackwardDataAlgorithm_v7"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardDataWorkspaceSize(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    algo: cudnnConvolutionBwdDataAlgo_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionBackwardDataWorkspaceSize {
        Some(____func) => unsafe { ____func(handle, wDesc, dyDesc, convDesc, dxDesc, algo, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionBackwardDataWorkspaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnConvolutionBackwardData(
    handle: cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionBwdDataAlgo_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    beta: *const ::core::ffi::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnConvolutionBackwardData {
        Some(____func) => unsafe { ____func(handle, alpha, wDesc, w, dyDesc, dy, convDesc, algo, workSpace, workSpaceSizeInBytes, beta, dxDesc, dx) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnConvolutionBackwardData"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetFoldedConvBackwardDataDescriptors(
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnTensorDescriptor_t,
    transformFormat: cudnnTensorFormat_t,
    foldedFilterDesc: cudnnFilterDescriptor_t,
    paddedDiffDesc: cudnnTensorDescriptor_t,
    foldedConvDesc: cudnnConvolutionDescriptor_t,
    foldedGradDesc: cudnnTensorDescriptor_t,
    filterFoldTransDesc: cudnnTensorTransformDescriptor_t,
    diffPadTransDesc: cudnnTensorTransformDescriptor_t,
    gradFoldTransDesc: cudnnTensorTransformDescriptor_t,
    gradUnfoldTransDesc: cudnnTensorTransformDescriptor_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetFoldedConvBackwardDataDescriptors {
        Some(____func) => unsafe {
            ____func(
                handle,
                filterDesc,
                diffDesc,
                convDesc,
                gradDesc,
                transformFormat,
                foldedFilterDesc,
                paddedDiffDesc,
                foldedConvDesc,
                foldedGradDesc,
                filterFoldTransDesc,
                diffPadTransDesc,
                gradFoldTransDesc,
                gradUnfoldTransDesc,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetFoldedConvBackwardDataDescriptors"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCnnVersionCheck() -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCnnVersionCheck {
        Some(____func) => unsafe { ____func() },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnCnnVersionCheck"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(handle: cudnnHandle_t, count: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionBackwardFilterAlgorithmMaxCount {
        Some(____func) => unsafe { ____func(handle, count) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionBackwardFilterAlgorithmMaxCount"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnFindConvolutionBackwardFilterAlgorithm(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dwDesc: cudnnFilterDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnFindConvolutionBackwardFilterAlgorithm {
        Some(____func) => unsafe { ____func(handle, xDesc, dyDesc, convDesc, dwDesc, requestedAlgoCount, returnedAlgoCount, perfResults) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnFindConvolutionBackwardFilterAlgorithm"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnFindConvolutionBackwardFilterAlgorithmEx(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    dwDesc: cudnnFilterDescriptor_t,
    dw: *mut ::core::ffi::c_void,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnFindConvolutionBackwardFilterAlgorithmEx {
        Some(____func) => unsafe { ____func(handle, xDesc, x, dyDesc, y, convDesc, dwDesc, dw, requestedAlgoCount, returnedAlgoCount, perfResults, workSpace, workSpaceSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnFindConvolutionBackwardFilterAlgorithmEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
    handle: cudnnHandle_t,
    srcDesc: cudnnTensorDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnFilterDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionBackwardFilterAlgorithm_v7 {
        Some(____func) => unsafe { ____func(handle, srcDesc, diffDesc, convDesc, gradDesc, requestedAlgoCount, returnedAlgoCount, perfResults) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionBackwardFilterAlgorithm_v7"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnFilterDescriptor_t,
    algo: cudnnConvolutionBwdFilterAlgo_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetConvolutionBackwardFilterWorkspaceSize {
        Some(____func) => unsafe { ____func(handle, xDesc, dyDesc, convDesc, gradDesc, algo, sizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetConvolutionBackwardFilterWorkspaceSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnConvolutionBackwardFilter(
    handle: cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionBwdFilterAlgo_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    beta: *const ::core::ffi::c_void,
    dwDesc: cudnnFilterDescriptor_t,
    dw: *mut ::core::ffi::c_void,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnConvolutionBackwardFilter {
        Some(____func) => unsafe { ____func(handle, alpha, xDesc, x, dyDesc, dy, convDesc, algo, workSpace, workSpaceSizeInBytes, beta, dwDesc, dw) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnConvolutionBackwardFilter"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnConvolutionBackwardBias(handle: cudnnHandle_t, alpha: *const ::core::ffi::c_void, dyDesc: cudnnTensorDescriptor_t, dy: *const ::core::ffi::c_void, beta: *const ::core::ffi::c_void, dbDesc: cudnnTensorDescriptor_t, db: *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnConvolutionBackwardBias {
        Some(____func) => unsafe { ____func(handle, alpha, dyDesc, dy, beta, dbDesc, db) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnConvolutionBackwardBias"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateFusedOpsConstParamPack(constPack: *mut cudnnFusedOpsConstParamPack_t, ops: cudnnFusedOps_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateFusedOpsConstParamPack {
        Some(____func) => unsafe { ____func(constPack, ops) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateFusedOpsConstParamPack"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyFusedOpsConstParamPack(constPack: cudnnFusedOpsConstParamPack_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyFusedOpsConstParamPack {
        Some(____func) => unsafe { ____func(constPack) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyFusedOpsConstParamPack"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetFusedOpsConstParamPackAttribute(constPack: cudnnFusedOpsConstParamPack_t, paramLabel: cudnnFusedOpsConstParamLabel_t, param: *const ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetFusedOpsConstParamPackAttribute {
        Some(____func) => unsafe { ____func(constPack, paramLabel, param) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetFusedOpsConstParamPackAttribute"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetFusedOpsConstParamPackAttribute(constPack: cudnnFusedOpsConstParamPack_t, paramLabel: cudnnFusedOpsConstParamLabel_t, param: *mut ::core::ffi::c_void, isNULL: *mut ::core::ffi::c_int) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetFusedOpsConstParamPackAttribute {
        Some(____func) => unsafe { ____func(constPack, paramLabel, param, isNULL) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetFusedOpsConstParamPackAttribute"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateFusedOpsVariantParamPack(varPack: *mut cudnnFusedOpsVariantParamPack_t, ops: cudnnFusedOps_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateFusedOpsVariantParamPack {
        Some(____func) => unsafe { ____func(varPack, ops) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCreateFusedOpsVariantParamPack"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyFusedOpsVariantParamPack(varPack: cudnnFusedOpsVariantParamPack_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyFusedOpsVariantParamPack {
        Some(____func) => unsafe { ____func(varPack) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnDestroyFusedOpsVariantParamPack"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSetFusedOpsVariantParamPackAttribute(varPack: cudnnFusedOpsVariantParamPack_t, paramLabel: cudnnFusedOpsVariantParamLabel_t, ptr: *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSetFusedOpsVariantParamPackAttribute {
        Some(____func) => unsafe { ____func(varPack, paramLabel, ptr) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSetFusedOpsVariantParamPackAttribute"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnGetFusedOpsVariantParamPackAttribute(varPack: cudnnFusedOpsVariantParamPack_t, paramLabel: cudnnFusedOpsVariantParamLabel_t, ptr: *mut ::core::ffi::c_void) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnGetFusedOpsVariantParamPackAttribute {
        Some(____func) => unsafe { ____func(varPack, paramLabel, ptr) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnGetFusedOpsVariantParamPackAttribute"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCreateFusedOpsPlan(plan: *mut cudnnFusedOpsPlan_t, ops: cudnnFusedOps_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCreateFusedOpsPlan {
        Some(____func) => unsafe { ____func(plan, ops) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnCreateFusedOpsPlan"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnDestroyFusedOpsPlan(plan: cudnnFusedOpsPlan_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnDestroyFusedOpsPlan {
        Some(____func) => unsafe { ____func(plan) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnDestroyFusedOpsPlan"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnMakeFusedOpsPlan(handle: cudnnHandle_t, plan: cudnnFusedOpsPlan_t, constPack: cudnnFusedOpsConstParamPack_t, workspaceSizeInBytes: *mut usize) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnMakeFusedOpsPlan {
        Some(____func) => unsafe { ____func(handle, plan, constPack, workspaceSizeInBytes) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnMakeFusedOpsPlan"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnFusedOpsExecute(handle: cudnnHandle_t, plan: cudnnFusedOpsPlan_t, varPack: cudnnFusedOpsVariantParamPack_t) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnFusedOpsExecute {
        Some(____func) => unsafe { ____func(handle, plan, varPack) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnFusedOpsExecute"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnSubquadraticOpsVersionCheck() -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnSubquadraticOpsVersionCheck {
        Some(____func) => unsafe { ____func() },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnSubquadraticOpsVersionCheck"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCausalConv1dForward(
    stream: cudaStream_t,
    x: *const ::core::ffi::c_void,
    weight: *const ::core::ffi::c_void,
    bias: *const ::core::ffi::c_void,
    y: *mut ::core::ffi::c_void,
    batch: ::core::ffi::c_int,
    dim: ::core::ffi::c_int,
    seqLen: ::core::ffi::c_int,
    kernelSize: ::core::ffi::c_int,
    dataType: cudnnDataType_t,
    activation: cudnnCausalConv1dActivation_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCausalConv1dForward {
        Some(____func) => unsafe { ____func(stream, x, weight, bias, y, batch, dim, seqLen, kernelSize, dataType, activation) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cudnnCausalConv1dForward"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cudnnCausalConv1dBackward(
    stream: cudaStream_t,
    x: *const ::core::ffi::c_void,
    weight: *const ::core::ffi::c_void,
    bias: *const ::core::ffi::c_void,
    dy: *const ::core::ffi::c_void,
    dx: *mut ::core::ffi::c_void,
    dweight: *mut ::core::ffi::c_void,
    dbias: *mut ::core::ffi::c_void,
    batch: ::core::ffi::c_int,
    dim: ::core::ffi::c_int,
    seqLen: ::core::ffi::c_int,
    kernelSize: ::core::ffi::c_int,
    dataType: cudnnDataType_t,
    dwDataType: cudnnDataType_t,
    activation: cudnnCausalConv1dActivation_t,
) -> cudnnStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cudnnCausalConv1dBackward {
        Some(____func) => unsafe { ____func(stream, x, weight, bias, dy, dx, dweight, dbias, batch, dim, seqLen, kernelSize, dataType, dwDataType, activation) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cudnnCausalConv1dBackward"
        ),
    }
}
#[cfg(feature = "runtime-link")]
pub unsafe fn load_dynamic_bindings(lib: *mut std::ffi::c_void, get_proc_addr: unsafe fn(*mut std::ffi::c_void, *const u8) -> *mut std::ffi::c_void) {
    let bindings = Box::new(DynamicBindings {
        cudnnGetVersion: {
            let p = get_proc_addr(lib, b"cudnnGetVersion\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetMaxDeviceVersion: {
            let p = get_proc_addr(lib, b"cudnnGetMaxDeviceVersion\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetCudartVersion: {
            let p = get_proc_addr(lib, b"cudnnGetCudartVersion\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetErrorString: {
            let p = get_proc_addr(lib, b"cudnnGetErrorString\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetLastErrorString: {
            let p = get_proc_addr(lib, b"cudnnGetLastErrorString\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnQueryRuntimeError: {
            let p = get_proc_addr(lib, b"cudnnQueryRuntimeError\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetProperty: {
            let p = get_proc_addr(lib, b"cudnnGetProperty\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreate: {
            let p = get_proc_addr(lib, b"cudnnCreate\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroy: {
            let p = get_proc_addr(lib, b"cudnnDestroy\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetStream: {
            let p = get_proc_addr(lib, b"cudnnSetStream\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetStream: {
            let p = get_proc_addr(lib, b"cudnnGetStream\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetCallback: {
            let p = get_proc_addr(lib, b"cudnnSetCallback\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetCallback: {
            let p = get_proc_addr(lib, b"cudnnGetCallback\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGraphVersionCheck: {
            let p = get_proc_addr(lib, b"cudnnGraphVersionCheck\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBackendCreateDescriptor: {
            let p = get_proc_addr(lib, b"cudnnBackendCreateDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBackendDestroyDescriptor: {
            let p = get_proc_addr(lib, b"cudnnBackendDestroyDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBackendInitialize: {
            let p = get_proc_addr(lib, b"cudnnBackendInitialize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBackendFinalize: {
            let p = get_proc_addr(lib, b"cudnnBackendFinalize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBackendSetAttribute: {
            let p = get_proc_addr(lib, b"cudnnBackendSetAttribute\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBackendGetAttribute: {
            let p = get_proc_addr(lib, b"cudnnBackendGetAttribute\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBackendExecute: {
            let p = get_proc_addr(lib, b"cudnnBackendExecute\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBackendPopulateCudaGraph: {
            let p = get_proc_addr(lib, b"cudnnBackendPopulateCudaGraph\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBackendUpdateCudaGraph: {
            let p = get_proc_addr(lib, b"cudnnBackendUpdateCudaGraph\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetTensor4dDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetTensor4dDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetTensor4dDescriptorEx: {
            let p = get_proc_addr(lib, b"cudnnSetTensor4dDescriptorEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetTensor4dDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetTensor4dDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetTensorNdDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetTensorNdDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetTensorNdDescriptorEx: {
            let p = get_proc_addr(lib, b"cudnnSetTensorNdDescriptorEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetTensorNdDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetTensorNdDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetTensorSizeInBytes: {
            let p = get_proc_addr(lib, b"cudnnGetTensorSizeInBytes\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnInitTransformDest: {
            let p = get_proc_addr(lib, b"cudnnInitTransformDest\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateTensorTransformDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateTensorTransformDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetTensorTransformDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetTensorTransformDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetTensorTransformDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetTensorTransformDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyTensorTransformDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyTensorTransformDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnTransformTensor: {
            let p = get_proc_addr(lib, b"cudnnTransformTensor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnTransformTensorEx: {
            let p = get_proc_addr(lib, b"cudnnTransformTensorEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnAddTensor: {
            let p = get_proc_addr(lib, b"cudnnAddTensor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateOpTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateOpTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetOpTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetOpTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetOpTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetOpTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyOpTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyOpTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnOpTensor: {
            let p = get_proc_addr(lib, b"cudnnOpTensor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateReduceTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateReduceTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetReduceTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetReduceTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetReduceTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetReduceTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyReduceTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyReduceTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetReductionIndicesSize: {
            let p = get_proc_addr(lib, b"cudnnGetReductionIndicesSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetReductionWorkspaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetReductionWorkspaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnReduceTensor: {
            let p = get_proc_addr(lib, b"cudnnReduceTensor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetTensor: {
            let p = get_proc_addr(lib, b"cudnnSetTensor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnScaleTensor: {
            let p = get_proc_addr(lib, b"cudnnScaleTensor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateFilterDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateFilterDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetFilter4dDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetFilter4dDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetFilter4dDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetFilter4dDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetFilterNdDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetFilterNdDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetFilterNdDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetFilterNdDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetFilterSizeInBytes: {
            let p = get_proc_addr(lib, b"cudnnGetFilterSizeInBytes\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnTransformFilter: {
            let p = get_proc_addr(lib, b"cudnnTransformFilter\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyFilterDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyFilterDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSoftmaxForward: {
            let p = get_proc_addr(lib, b"cudnnSoftmaxForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreatePoolingDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreatePoolingDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetPooling2dDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetPooling2dDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetPooling2dDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetPooling2dDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetPoolingNdDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetPoolingNdDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetPoolingNdDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetPoolingNdDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetPoolingNdForwardOutputDim: {
            let p = get_proc_addr(lib, b"cudnnGetPoolingNdForwardOutputDim\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetPooling2dForwardOutputDim: {
            let p = get_proc_addr(lib, b"cudnnGetPooling2dForwardOutputDim\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyPoolingDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyPoolingDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnPoolingForward: {
            let p = get_proc_addr(lib, b"cudnnPoolingForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateActivationDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateActivationDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetActivationDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetActivationDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetActivationDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetActivationDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetActivationDescriptorSwishBeta: {
            let p = get_proc_addr(lib, b"cudnnSetActivationDescriptorSwishBeta\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetActivationDescriptorSwishBeta: {
            let p = get_proc_addr(lib, b"cudnnGetActivationDescriptorSwishBeta\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyActivationDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyActivationDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnActivationForward: {
            let p = get_proc_addr(lib, b"cudnnActivationForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateLRNDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateLRNDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetLRNDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetLRNDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetLRNDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetLRNDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyLRNDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyLRNDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnLRNCrossChannelForward: {
            let p = get_proc_addr(lib, b"cudnnLRNCrossChannelForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDivisiveNormalizationForward: {
            let p = get_proc_addr(lib, b"cudnnDivisiveNormalizationForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDeriveBNTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDeriveBNTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBatchNormalizationForwardInference: {
            let p = get_proc_addr(lib, b"cudnnBatchNormalizationForwardInference\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDeriveNormTensorDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDeriveNormTensorDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnNormalizationForwardInference: {
            let p = get_proc_addr(lib, b"cudnnNormalizationForwardInference\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateSpatialTransformerDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateSpatialTransformerDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetSpatialTransformerNdDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetSpatialTransformerNdDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroySpatialTransformerDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroySpatialTransformerDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSpatialTfGridGeneratorForward: {
            let p = get_proc_addr(lib, b"cudnnSpatialTfGridGeneratorForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSpatialTfSamplerForward: {
            let p = get_proc_addr(lib, b"cudnnSpatialTfSamplerForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateDropoutDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateDropoutDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyDropoutDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyDropoutDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDropoutGetStatesSize: {
            let p = get_proc_addr(lib, b"cudnnDropoutGetStatesSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDropoutGetReserveSpaceSize: {
            let p = get_proc_addr(lib, b"cudnnDropoutGetReserveSpaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetDropoutDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetDropoutDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnRestoreDropoutDescriptor: {
            let p = get_proc_addr(lib, b"cudnnRestoreDropoutDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetDropoutDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetDropoutDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDropoutForward: {
            let p = get_proc_addr(lib, b"cudnnDropoutForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnOpsVersionCheck: {
            let p = get_proc_addr(lib, b"cudnnOpsVersionCheck\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSoftmaxBackward: {
            let p = get_proc_addr(lib, b"cudnnSoftmaxBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnPoolingBackward: {
            let p = get_proc_addr(lib, b"cudnnPoolingBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnActivationBackward: {
            let p = get_proc_addr(lib, b"cudnnActivationBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnLRNCrossChannelBackward: {
            let p = get_proc_addr(lib, b"cudnnLRNCrossChannelBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDivisiveNormalizationBackward: {
            let p = get_proc_addr(lib, b"cudnnDivisiveNormalizationBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetBatchNormalizationBackwardExWorkspaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetBatchNormalizationBackwardExWorkspaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetBatchNormalizationTrainingExReserveSpaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetBatchNormalizationTrainingExReserveSpaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBatchNormalizationForwardTraining: {
            let p = get_proc_addr(lib, b"cudnnBatchNormalizationForwardTraining\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBatchNormalizationForwardTrainingEx: {
            let p = get_proc_addr(lib, b"cudnnBatchNormalizationForwardTrainingEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBatchNormalizationBackward: {
            let p = get_proc_addr(lib, b"cudnnBatchNormalizationBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBatchNormalizationBackwardEx: {
            let p = get_proc_addr(lib, b"cudnnBatchNormalizationBackwardEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetNormalizationForwardTrainingWorkspaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetNormalizationForwardTrainingWorkspaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetNormalizationBackwardWorkspaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetNormalizationBackwardWorkspaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetNormalizationTrainingReserveSpaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetNormalizationTrainingReserveSpaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnNormalizationForwardTraining: {
            let p = get_proc_addr(lib, b"cudnnNormalizationForwardTraining\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnNormalizationBackward: {
            let p = get_proc_addr(lib, b"cudnnNormalizationBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSpatialTfGridGeneratorBackward: {
            let p = get_proc_addr(lib, b"cudnnSpatialTfGridGeneratorBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSpatialTfSamplerBackward: {
            let p = get_proc_addr(lib, b"cudnnSpatialTfSamplerBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDropoutBackward: {
            let p = get_proc_addr(lib, b"cudnnDropoutBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateRNNDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateRNNDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyRNNDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyRNNDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetRNNDescriptor_v8: {
            let p = get_proc_addr(lib, b"cudnnSetRNNDescriptor_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetRNNDescriptor_v8: {
            let p = get_proc_addr(lib, b"cudnnGetRNNDescriptor_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnRNNSetClip_v8: {
            let p = get_proc_addr(lib, b"cudnnRNNSetClip_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnRNNSetClip_v9: {
            let p = get_proc_addr(lib, b"cudnnRNNSetClip_v9\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnRNNGetClip_v8: {
            let p = get_proc_addr(lib, b"cudnnRNNGetClip_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnRNNGetClip_v9: {
            let p = get_proc_addr(lib, b"cudnnRNNGetClip_v9\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnBuildRNNDynamic: {
            let p = get_proc_addr(lib, b"cudnnBuildRNNDynamic\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetRNNTempSpaceSizes: {
            let p = get_proc_addr(lib, b"cudnnGetRNNTempSpaceSizes\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetRNNWeightSpaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetRNNWeightSpaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetRNNWeightParams: {
            let p = get_proc_addr(lib, b"cudnnGetRNNWeightParams\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateRNNDataDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateRNNDataDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyRNNDataDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyRNNDataDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetRNNDataDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetRNNDataDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetRNNDataDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetRNNDataDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnRNNForward: {
            let p = get_proc_addr(lib, b"cudnnRNNForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateSeqDataDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateSeqDataDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroySeqDataDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroySeqDataDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetSeqDataDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetSeqDataDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetSeqDataDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetSeqDataDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateAttnDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateAttnDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyAttnDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyAttnDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetAttnDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetAttnDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetAttnDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetAttnDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetMultiHeadAttnBuffers: {
            let p = get_proc_addr(lib, b"cudnnGetMultiHeadAttnBuffers\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetMultiHeadAttnWeights: {
            let p = get_proc_addr(lib, b"cudnnGetMultiHeadAttnWeights\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnMultiHeadAttnForward: {
            let p = get_proc_addr(lib, b"cudnnMultiHeadAttnForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnAdvVersionCheck: {
            let p = get_proc_addr(lib, b"cudnnAdvVersionCheck\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnRNNBackwardData_v8: {
            let p = get_proc_addr(lib, b"cudnnRNNBackwardData_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnRNNBackwardWeights_v8: {
            let p = get_proc_addr(lib, b"cudnnRNNBackwardWeights_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnMultiHeadAttnBackwardData: {
            let p = get_proc_addr(lib, b"cudnnMultiHeadAttnBackwardData\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnMultiHeadAttnBackwardWeights: {
            let p = get_proc_addr(lib, b"cudnnMultiHeadAttnBackwardWeights\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateCTCLossDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateCTCLossDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetCTCLossDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetCTCLossDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetCTCLossDescriptorEx: {
            let p = get_proc_addr(lib, b"cudnnSetCTCLossDescriptorEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetCTCLossDescriptor_v8: {
            let p = get_proc_addr(lib, b"cudnnSetCTCLossDescriptor_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetCTCLossDescriptor_v9: {
            let p = get_proc_addr(lib, b"cudnnSetCTCLossDescriptor_v9\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetCTCLossDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetCTCLossDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetCTCLossDescriptorEx: {
            let p = get_proc_addr(lib, b"cudnnGetCTCLossDescriptorEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetCTCLossDescriptor_v8: {
            let p = get_proc_addr(lib, b"cudnnGetCTCLossDescriptor_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetCTCLossDescriptor_v9: {
            let p = get_proc_addr(lib, b"cudnnGetCTCLossDescriptor_v9\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyCTCLossDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyCTCLossDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCTCLoss: {
            let p = get_proc_addr(lib, b"cudnnCTCLoss\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCTCLoss_v8: {
            let p = get_proc_addr(lib, b"cudnnCTCLoss_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetCTCLossWorkspaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetCTCLossWorkspaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetCTCLossWorkspaceSize_v8: {
            let p = get_proc_addr(lib, b"cudnnGetCTCLossWorkspaceSize_v8\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateConvolutionDescriptor: {
            let p = get_proc_addr(lib, b"cudnnCreateConvolutionDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyConvolutionDescriptor: {
            let p = get_proc_addr(lib, b"cudnnDestroyConvolutionDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetConvolutionMathType: {
            let p = get_proc_addr(lib, b"cudnnSetConvolutionMathType\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionMathType: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionMathType\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetConvolutionGroupCount: {
            let p = get_proc_addr(lib, b"cudnnSetConvolutionGroupCount\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionGroupCount: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionGroupCount\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetConvolutionReorderType: {
            let p = get_proc_addr(lib, b"cudnnSetConvolutionReorderType\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionReorderType: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionReorderType\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetConvolution2dDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetConvolution2dDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolution2dDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetConvolution2dDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetConvolutionNdDescriptor: {
            let p = get_proc_addr(lib, b"cudnnSetConvolutionNdDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionNdDescriptor: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionNdDescriptor\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolution2dForwardOutputDim: {
            let p = get_proc_addr(lib, b"cudnnGetConvolution2dForwardOutputDim\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionNdForwardOutputDim: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionNdForwardOutputDim\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionForwardAlgorithmMaxCount: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionForwardAlgorithmMaxCount\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionForwardAlgorithm_v7: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionForwardAlgorithm_v7\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnFindConvolutionForwardAlgorithm: {
            let p = get_proc_addr(lib, b"cudnnFindConvolutionForwardAlgorithm\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnFindConvolutionForwardAlgorithmEx: {
            let p = get_proc_addr(lib, b"cudnnFindConvolutionForwardAlgorithmEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnIm2Col: {
            let p = get_proc_addr(lib, b"cudnnIm2Col\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnReorderFilterAndBias: {
            let p = get_proc_addr(lib, b"cudnnReorderFilterAndBias\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionForwardWorkspaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionForwardWorkspaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnConvolutionForward: {
            let p = get_proc_addr(lib, b"cudnnConvolutionForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnConvolutionBiasActivationForward: {
            let p = get_proc_addr(lib, b"cudnnConvolutionBiasActivationForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionBackwardDataAlgorithmMaxCount: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionBackwardDataAlgorithmMaxCount\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnFindConvolutionBackwardDataAlgorithm: {
            let p = get_proc_addr(lib, b"cudnnFindConvolutionBackwardDataAlgorithm\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnFindConvolutionBackwardDataAlgorithmEx: {
            let p = get_proc_addr(lib, b"cudnnFindConvolutionBackwardDataAlgorithmEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionBackwardDataAlgorithm_v7: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionBackwardDataAlgorithm_v7\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionBackwardDataWorkspaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionBackwardDataWorkspaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnConvolutionBackwardData: {
            let p = get_proc_addr(lib, b"cudnnConvolutionBackwardData\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetFoldedConvBackwardDataDescriptors: {
            let p = get_proc_addr(lib, b"cudnnGetFoldedConvBackwardDataDescriptors\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCnnVersionCheck: {
            let p = get_proc_addr(lib, b"cudnnCnnVersionCheck\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionBackwardFilterAlgorithmMaxCount: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionBackwardFilterAlgorithmMaxCount\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnFindConvolutionBackwardFilterAlgorithm: {
            let p = get_proc_addr(lib, b"cudnnFindConvolutionBackwardFilterAlgorithm\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnFindConvolutionBackwardFilterAlgorithmEx: {
            let p = get_proc_addr(lib, b"cudnnFindConvolutionBackwardFilterAlgorithmEx\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionBackwardFilterAlgorithm_v7: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionBackwardFilterAlgorithm_v7\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetConvolutionBackwardFilterWorkspaceSize: {
            let p = get_proc_addr(lib, b"cudnnGetConvolutionBackwardFilterWorkspaceSize\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnConvolutionBackwardFilter: {
            let p = get_proc_addr(lib, b"cudnnConvolutionBackwardFilter\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnConvolutionBackwardBias: {
            let p = get_proc_addr(lib, b"cudnnConvolutionBackwardBias\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateFusedOpsConstParamPack: {
            let p = get_proc_addr(lib, b"cudnnCreateFusedOpsConstParamPack\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyFusedOpsConstParamPack: {
            let p = get_proc_addr(lib, b"cudnnDestroyFusedOpsConstParamPack\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetFusedOpsConstParamPackAttribute: {
            let p = get_proc_addr(lib, b"cudnnSetFusedOpsConstParamPackAttribute\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetFusedOpsConstParamPackAttribute: {
            let p = get_proc_addr(lib, b"cudnnGetFusedOpsConstParamPackAttribute\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateFusedOpsVariantParamPack: {
            let p = get_proc_addr(lib, b"cudnnCreateFusedOpsVariantParamPack\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyFusedOpsVariantParamPack: {
            let p = get_proc_addr(lib, b"cudnnDestroyFusedOpsVariantParamPack\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSetFusedOpsVariantParamPackAttribute: {
            let p = get_proc_addr(lib, b"cudnnSetFusedOpsVariantParamPackAttribute\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnGetFusedOpsVariantParamPackAttribute: {
            let p = get_proc_addr(lib, b"cudnnGetFusedOpsVariantParamPackAttribute\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCreateFusedOpsPlan: {
            let p = get_proc_addr(lib, b"cudnnCreateFusedOpsPlan\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnDestroyFusedOpsPlan: {
            let p = get_proc_addr(lib, b"cudnnDestroyFusedOpsPlan\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnMakeFusedOpsPlan: {
            let p = get_proc_addr(lib, b"cudnnMakeFusedOpsPlan\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnFusedOpsExecute: {
            let p = get_proc_addr(lib, b"cudnnFusedOpsExecute\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnSubquadraticOpsVersionCheck: {
            let p = get_proc_addr(lib, b"cudnnSubquadraticOpsVersionCheck\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCausalConv1dForward: {
            let p = get_proc_addr(lib, b"cudnnCausalConv1dForward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
        cudnnCausalConv1dBackward: {
            let p = get_proc_addr(lib, b"cudnnCausalConv1dBackward\0".as_ptr());
            if p.is_null() { None } else { Some(std::mem::transmute(p)) }
        },
    });
    DYNAMIC_BINDINGS.set(bindings).ok();
}
unsafe impl Send for CUgraph_st {}
unsafe impl Sync for CUgraph_st {}
unsafe impl Send for cudnnContext {}
unsafe impl Sync for cudnnContext {}
unsafe impl Send for cudnnStatus_t {}
unsafe impl Sync for cudnnStatus_t {}
unsafe impl Send for cudnnRuntimeTag_t {}
unsafe impl Sync for cudnnRuntimeTag_t {}
unsafe impl Send for cudnnErrQueryMode_t {}
unsafe impl Sync for cudnnErrQueryMode_t {}
unsafe impl Send for cudnnDataType_t {}
unsafe impl Sync for cudnnDataType_t {}
unsafe impl Send for cudnnMathType_t {}
unsafe impl Sync for cudnnMathType_t {}
unsafe impl Send for cudnnNanPropagation_t {}
unsafe impl Sync for cudnnNanPropagation_t {}
unsafe impl Send for cudnnCTCGradMode_t {}
unsafe impl Sync for cudnnCTCGradMode_t {}
unsafe impl Send for cudnnTensorFormat_t {}
unsafe impl Sync for cudnnTensorFormat_t {}
unsafe impl Send for cudnnReduceTensorOp_t {}
unsafe impl Sync for cudnnReduceTensorOp_t {}
unsafe impl Send for cudnnActivationMode_t {}
unsafe impl Sync for cudnnActivationMode_t {}
unsafe impl Send for cudnnSeverity_t {}
unsafe impl Sync for cudnnSeverity_t {}
unsafe impl Send for cudnnDebugStruct {}
unsafe impl Sync for cudnnDebugStruct {}
unsafe impl Send for cudnnConvolutionMode_t {}
unsafe impl Sync for cudnnConvolutionMode_t {}
unsafe impl Send for cudnnReorderType_t {}
unsafe impl Sync for cudnnReorderType_t {}
unsafe impl Send for cudnnFractionStruct {}
unsafe impl Sync for cudnnFractionStruct {}
unsafe impl Send for cudnnPointwiseMode_t {}
unsafe impl Sync for cudnnPointwiseMode_t {}
unsafe impl Send for cudnnResampleMode_t {}
unsafe impl Sync for cudnnResampleMode_t {}
unsafe impl Send for cudnnSignalMode_t {}
unsafe impl Sync for cudnnSignalMode_t {}
unsafe impl Send for cudnnGenStatsMode_t {}
unsafe impl Sync for cudnnGenStatsMode_t {}
unsafe impl Send for cudnnBnFinalizeStatsMode_t {}
unsafe impl Sync for cudnnBnFinalizeStatsMode_t {}
unsafe impl Send for cudnnRngDistribution_t {}
unsafe impl Sync for cudnnRngDistribution_t {}
unsafe impl Send for cudnnMoeGroupedMatmulMode_t {}
unsafe impl Sync for cudnnMoeGroupedMatmulMode_t {}
unsafe impl Send for cudnnBackendAttributeName_t {}
unsafe impl Sync for cudnnBackendAttributeName_t {}
unsafe impl Send for cudnnBackendAttributeType_t {}
unsafe impl Sync for cudnnBackendAttributeType_t {}
unsafe impl Send for cudnnBackendDescriptorType_t {}
unsafe impl Sync for cudnnBackendDescriptorType_t {}
unsafe impl Send for cudnnBackendNumericalNote_t {}
unsafe impl Sync for cudnnBackendNumericalNote_t {}
unsafe impl Send for cudnnBackendBehaviorNote_t {}
unsafe impl Sync for cudnnBackendBehaviorNote_t {}
unsafe impl Send for cudnnBackendKnobType_t {}
unsafe impl Sync for cudnnBackendKnobType_t {}
unsafe impl Send for cudnnBackendLayoutType_t {}
unsafe impl Sync for cudnnBackendLayoutType_t {}
unsafe impl Send for cudnnBackendHeurMode_t {}
unsafe impl Sync for cudnnBackendHeurMode_t {}
unsafe impl Send for cudnnBackendTensorReordering_t {}
unsafe impl Sync for cudnnBackendTensorReordering_t {}
unsafe impl Send for cudnnPaddingMode_t {}
unsafe impl Sync for cudnnPaddingMode_t {}
unsafe impl Send for cudnnBackendNormMode_t {}
unsafe impl Sync for cudnnBackendNormMode_t {}
unsafe impl Send for cudnnBackendNormFwdPhase_t {}
unsafe impl Sync for cudnnBackendNormFwdPhase_t {}
unsafe impl Send for cudnnBackendReshapeMode_t {}
unsafe impl Sync for cudnnBackendReshapeMode_t {}
unsafe impl Send for cudnnTensorStruct {}
unsafe impl Sync for cudnnTensorStruct {}
unsafe impl Send for cudnnPoolingStruct {}
unsafe impl Sync for cudnnPoolingStruct {}
unsafe impl Send for cudnnFilterStruct {}
unsafe impl Sync for cudnnFilterStruct {}
unsafe impl Send for cudnnLRNStruct {}
unsafe impl Sync for cudnnLRNStruct {}
unsafe impl Send for cudnnActivationStruct {}
unsafe impl Sync for cudnnActivationStruct {}
unsafe impl Send for cudnnSpatialTransformerStruct {}
unsafe impl Sync for cudnnSpatialTransformerStruct {}
unsafe impl Send for cudnnOpTensorStruct {}
unsafe impl Sync for cudnnOpTensorStruct {}
unsafe impl Send for cudnnReduceTensorStruct {}
unsafe impl Sync for cudnnReduceTensorStruct {}
unsafe impl Send for cudnnCTCLossStruct {}
unsafe impl Sync for cudnnCTCLossStruct {}
unsafe impl Send for cudnnTensorTransformStruct {}
unsafe impl Sync for cudnnTensorTransformStruct {}
unsafe impl Send for cudnnDeterminism_t {}
unsafe impl Sync for cudnnDeterminism_t {}
unsafe impl Send for cudnnFoldingDirection_t {}
unsafe impl Sync for cudnnFoldingDirection_t {}
unsafe impl Send for cudnnOpTensorOp_t {}
unsafe impl Sync for cudnnOpTensorOp_t {}
unsafe impl Send for cudnnReduceTensorIndices_t {}
unsafe impl Sync for cudnnReduceTensorIndices_t {}
unsafe impl Send for cudnnIndicesType_t {}
unsafe impl Sync for cudnnIndicesType_t {}
unsafe impl Send for cudnnSoftmaxAlgorithm_t {}
unsafe impl Sync for cudnnSoftmaxAlgorithm_t {}
unsafe impl Send for cudnnSoftmaxMode_t {}
unsafe impl Sync for cudnnSoftmaxMode_t {}
unsafe impl Send for cudnnPoolingMode_t {}
unsafe impl Sync for cudnnPoolingMode_t {}
unsafe impl Send for cudnnLRNMode_t {}
unsafe impl Sync for cudnnLRNMode_t {}
unsafe impl Send for cudnnDivNormMode_t {}
unsafe impl Sync for cudnnDivNormMode_t {}
unsafe impl Send for cudnnBatchNormMode_t {}
unsafe impl Sync for cudnnBatchNormMode_t {}
unsafe impl Send for cudnnBatchNormOps_t {}
unsafe impl Sync for cudnnBatchNormOps_t {}
unsafe impl Send for cudnnNormMode_t {}
unsafe impl Sync for cudnnNormMode_t {}
unsafe impl Send for cudnnNormAlgo_t {}
unsafe impl Sync for cudnnNormAlgo_t {}
unsafe impl Send for cudnnNormOps_t {}
unsafe impl Sync for cudnnNormOps_t {}
unsafe impl Send for cudnnSamplerType_t {}
unsafe impl Sync for cudnnSamplerType_t {}
unsafe impl Send for cudnnDropoutStruct {}
unsafe impl Sync for cudnnDropoutStruct {}
unsafe impl Send for cudnnConvolutionFwdAlgo_t {}
unsafe impl Sync for cudnnConvolutionFwdAlgo_t {}
unsafe impl Send for cudnnConvolutionBwdFilterAlgo_t {}
unsafe impl Sync for cudnnConvolutionBwdFilterAlgo_t {}
unsafe impl Send for cudnnConvolutionBwdDataAlgo_t {}
unsafe impl Sync for cudnnConvolutionBwdDataAlgo_t {}
unsafe impl Send for cudnnCTCLossAlgo_t {}
unsafe impl Sync for cudnnCTCLossAlgo_t {}
unsafe impl Send for cudnnRNNAlgo_t {}
unsafe impl Sync for cudnnRNNAlgo_t {}
unsafe impl Send for cudnnForwardMode_t {}
unsafe impl Sync for cudnnForwardMode_t {}
unsafe impl Send for cudnnRNNMode_t {}
unsafe impl Sync for cudnnRNNMode_t {}
unsafe impl Send for cudnnRNNBiasMode_t {}
unsafe impl Sync for cudnnRNNBiasMode_t {}
unsafe impl Send for cudnnDirectionMode_t {}
unsafe impl Sync for cudnnDirectionMode_t {}
unsafe impl Send for cudnnRNNInputMode_t {}
unsafe impl Sync for cudnnRNNInputMode_t {}
unsafe impl Send for cudnnRNNClipMode_t {}
unsafe impl Sync for cudnnRNNClipMode_t {}
unsafe impl Send for cudnnRNNDataLayout_t {}
unsafe impl Sync for cudnnRNNDataLayout_t {}
unsafe impl Send for cudnnRNNStruct {}
unsafe impl Sync for cudnnRNNStruct {}
unsafe impl Send for cudnnRNNDataStruct {}
unsafe impl Sync for cudnnRNNDataStruct {}
unsafe impl Send for cudnnSeqDataAxis_t {}
unsafe impl Sync for cudnnSeqDataAxis_t {}
unsafe impl Send for cudnnSeqDataStruct {}
unsafe impl Sync for cudnnSeqDataStruct {}
unsafe impl Send for cudnnAttnStruct {}
unsafe impl Sync for cudnnAttnStruct {}
unsafe impl Send for cudnnMultiHeadAttnWeightKind_t {}
unsafe impl Sync for cudnnMultiHeadAttnWeightKind_t {}
unsafe impl Send for cudnnWgradMode_t {}
unsafe impl Sync for cudnnWgradMode_t {}
unsafe impl Send for cudnnLossNormalizationMode_t {}
unsafe impl Sync for cudnnLossNormalizationMode_t {}
unsafe impl Send for cudnnConvolutionStruct {}
unsafe impl Sync for cudnnConvolutionStruct {}
unsafe impl Send for cudnnConvolutionFwdAlgoPerfStruct {}
unsafe impl Sync for cudnnConvolutionFwdAlgoPerfStruct {}
unsafe impl Send for cudnnConvolutionBwdDataAlgoPerfStruct {}
unsafe impl Sync for cudnnConvolutionBwdDataAlgoPerfStruct {}
unsafe impl Send for cudnnFusedOpsConstParamStruct {}
unsafe impl Sync for cudnnFusedOpsConstParamStruct {}
unsafe impl Send for cudnnFusedOpsVariantParamStruct {}
unsafe impl Sync for cudnnFusedOpsVariantParamStruct {}
unsafe impl Send for cudnnFusedOpsPlanStruct {}
unsafe impl Sync for cudnnFusedOpsPlanStruct {}
unsafe impl Send for cudnnFusedOps_t {}
unsafe impl Sync for cudnnFusedOps_t {}
unsafe impl Send for cudnnFusedOpsConstParamLabel_t {}
unsafe impl Sync for cudnnFusedOpsConstParamLabel_t {}
unsafe impl Send for cudnnFusedOpsPointerPlaceHolder_t {}
unsafe impl Sync for cudnnFusedOpsPointerPlaceHolder_t {}
unsafe impl Send for cudnnFusedOpsVariantParamLabel_t {}
unsafe impl Sync for cudnnFusedOpsVariantParamLabel_t {}
unsafe impl Send for cudnnConvolutionBwdFilterAlgoPerfStruct {}
unsafe impl Sync for cudnnConvolutionBwdFilterAlgoPerfStruct {}
unsafe impl Send for cudnnCausalConv1dActivation_t {}
unsafe impl Sync for cudnnCausalConv1dActivation_t {}
impl std::fmt::Display for cudnnStatus_t {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for cudnnStatus_t {}
