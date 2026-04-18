use cuda_libs_cudart::sys::*;
pub const CUBLAS_VER_MAJOR: u32 = 13;
pub const CUBLAS_VER_MINOR: u32 = 4;
pub const CUBLAS_VER_PATCH: u32 = 0;
pub const CUBLAS_VER_BUILD: u32 = 1;
pub const CUBLAS_VERSION: u32 = 130400;
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct float2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct double2 {
    pub x: f64,
    pub y: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
pub type cuFloatComplex = float2;
pub type cuDoubleComplex = double2;
pub type cuComplex = cuFloatComplex;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum libraryPropertyType_t {
    MAJOR_VERSION = 0,
    MINOR_VERSION = 1,
    PATCH_LEVEL = 2,
}
pub use self::libraryPropertyType_t as libraryPropertyType;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasStatus_t {
    CUBLAS_STATUS_SUCCESS = 0,
    CUBLAS_STATUS_NOT_INITIALIZED = 1,
    CUBLAS_STATUS_ALLOC_FAILED = 3,
    CUBLAS_STATUS_INVALID_VALUE = 7,
    CUBLAS_STATUS_ARCH_MISMATCH = 8,
    CUBLAS_STATUS_MAPPING_ERROR = 11,
    CUBLAS_STATUS_EXECUTION_FAILED = 13,
    CUBLAS_STATUS_INTERNAL_ERROR = 14,
    CUBLAS_STATUS_NOT_SUPPORTED = 15,
    CUBLAS_STATUS_LICENSE_ERROR = 16,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasFillMode_t {
    CUBLAS_FILL_MODE_LOWER = 0,
    CUBLAS_FILL_MODE_UPPER = 1,
    CUBLAS_FILL_MODE_FULL = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasDiagType_t {
    CUBLAS_DIAG_NON_UNIT = 0,
    CUBLAS_DIAG_UNIT = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasSideMode_t {
    CUBLAS_SIDE_LEFT = 0,
    CUBLAS_SIDE_RIGHT = 1,
}
impl cublasOperation_t {
    pub const CUBLAS_OP_HERMITAN: cublasOperation_t = cublasOperation_t::CUBLAS_OP_C;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasOperation_t {
    CUBLAS_OP_N = 0,
    CUBLAS_OP_T = 1,
    CUBLAS_OP_C = 2,
    CUBLAS_OP_CONJG = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasPointerMode_t {
    CUBLAS_POINTER_MODE_HOST = 0,
    CUBLAS_POINTER_MODE_DEVICE = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasAtomicsMode_t {
    CUBLAS_ATOMICS_NOT_ALLOWED = 0,
    CUBLAS_ATOMICS_ALLOWED = 1,
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_DEFAULT: cublasGemmAlgo_t = cublasGemmAlgo_t::CUBLAS_GEMM_DFALT;
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_DFALT_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t::CUBLAS_GEMM_DEFAULT_TENSOR_OP;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasGemmAlgo_t {
    CUBLAS_GEMM_DFALT = -1,
    CUBLAS_GEMM_ALGO0 = 0,
    CUBLAS_GEMM_ALGO1 = 1,
    CUBLAS_GEMM_ALGO2 = 2,
    CUBLAS_GEMM_ALGO3 = 3,
    CUBLAS_GEMM_ALGO4 = 4,
    CUBLAS_GEMM_ALGO5 = 5,
    CUBLAS_GEMM_ALGO6 = 6,
    CUBLAS_GEMM_ALGO7 = 7,
    CUBLAS_GEMM_ALGO8 = 8,
    CUBLAS_GEMM_ALGO9 = 9,
    CUBLAS_GEMM_ALGO10 = 10,
    CUBLAS_GEMM_ALGO11 = 11,
    CUBLAS_GEMM_ALGO12 = 12,
    CUBLAS_GEMM_ALGO13 = 13,
    CUBLAS_GEMM_ALGO14 = 14,
    CUBLAS_GEMM_ALGO15 = 15,
    CUBLAS_GEMM_ALGO16 = 16,
    CUBLAS_GEMM_ALGO17 = 17,
    CUBLAS_GEMM_ALGO18 = 18,
    CUBLAS_GEMM_ALGO19 = 19,
    CUBLAS_GEMM_ALGO20 = 20,
    CUBLAS_GEMM_ALGO21 = 21,
    CUBLAS_GEMM_ALGO22 = 22,
    CUBLAS_GEMM_ALGO23 = 23,
    CUBLAS_GEMM_DEFAULT_TENSOR_OP = 99,
    CUBLAS_GEMM_ALGO0_TENSOR_OP = 100,
    CUBLAS_GEMM_ALGO1_TENSOR_OP = 101,
    CUBLAS_GEMM_ALGO2_TENSOR_OP = 102,
    CUBLAS_GEMM_ALGO3_TENSOR_OP = 103,
    CUBLAS_GEMM_ALGO4_TENSOR_OP = 104,
    CUBLAS_GEMM_ALGO5_TENSOR_OP = 105,
    CUBLAS_GEMM_ALGO6_TENSOR_OP = 106,
    CUBLAS_GEMM_ALGO7_TENSOR_OP = 107,
    CUBLAS_GEMM_ALGO8_TENSOR_OP = 108,
    CUBLAS_GEMM_ALGO9_TENSOR_OP = 109,
    CUBLAS_GEMM_ALGO10_TENSOR_OP = 110,
    CUBLAS_GEMM_ALGO11_TENSOR_OP = 111,
    CUBLAS_GEMM_ALGO12_TENSOR_OP = 112,
    CUBLAS_GEMM_ALGO13_TENSOR_OP = 113,
    CUBLAS_GEMM_ALGO14_TENSOR_OP = 114,
    CUBLAS_GEMM_ALGO15_TENSOR_OP = 115,
    CUBLAS_GEMM_AUTOTUNE = 999,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasMath_t {
    CUBLAS_DEFAULT_MATH = 0,
    CUBLAS_TENSOR_OP_MATH = 1,
    CUBLAS_PEDANTIC_MATH = 2,
    CUBLAS_TF32_TENSOR_OP_MATH = 3,
    CUBLAS_FP32_EMULATED_BF16X9_MATH = 4,
    CUBLAS_FP64_EMULATED_FIXEDPOINT_MATH = 8,
    CUBLAS_MATH_DISALLOW_REDUCED_PRECISION_REDUCTION = 16,
}
pub use cuda_libs_cudart::sys::cudaDataType as cublasDataType_t;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasComputeType_t {
    CUBLAS_COMPUTE_16F = 64,
    CUBLAS_COMPUTE_16F_PEDANTIC = 65,
    CUBLAS_COMPUTE_32F = 68,
    CUBLAS_COMPUTE_32F_PEDANTIC = 69,
    CUBLAS_COMPUTE_32F_FAST_16F = 74,
    CUBLAS_COMPUTE_32F_FAST_16BF = 75,
    CUBLAS_COMPUTE_32F_FAST_TF32 = 77,
    CUBLAS_COMPUTE_32F_EMULATED_16BFX9 = 78,
    CUBLAS_COMPUTE_64F = 70,
    CUBLAS_COMPUTE_64F_PEDANTIC = 71,
    CUBLAS_COMPUTE_64F_EMULATED_FIXEDPOINT = 79,
    CUBLAS_COMPUTE_32I = 72,
    CUBLAS_COMPUTE_32I_PEDANTIC = 73,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasEmulationStrategy_t {
    CUBLAS_EMULATION_STRATEGY_DEFAULT = 0,
    CUBLAS_EMULATION_STRATEGY_PERFORMANT = 1,
    CUBLAS_EMULATION_STRATEGY_EAGER = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cublasContext {
    _unused: [u8; 0],
}
pub type cublasHandle_t = *mut cublasContext;
pub type cublasLogCallback = ::std::option::Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char)>;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCreate_v2(handle: *mut cublasHandle_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDestroy_v2(handle: cublasHandle_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetVersion_v2(handle: cublasHandle_t, version: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetProperty(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetCudartVersion() -> usize;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetWorkspace_v2(handle: cublasHandle_t, workspace: *mut ::std::os::raw::c_void, workspaceSizeInBytes: usize) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetStream_v2(handle: cublasHandle_t, streamId: cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetStream_v2(handle: cublasHandle_t, streamId: *mut cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetPointerMode_v2(handle: cublasHandle_t, mode: *mut cublasPointerMode_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetPointerMode_v2(handle: cublasHandle_t, mode: cublasPointerMode_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetAtomicsMode(handle: cublasHandle_t, mode: *mut cublasAtomicsMode_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetAtomicsMode(handle: cublasHandle_t, mode: cublasAtomicsMode_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetMathMode(handle: cublasHandle_t, mode: *mut cublasMath_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetMathMode(handle: cublasHandle_t, mode: cublasMath_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetSmCountTarget(handle: cublasHandle_t, smCountTarget: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetSmCountTarget(handle: cublasHandle_t, smCountTarget: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetEmulationStrategy(handle: cublasHandle_t, emulationStrategy: *mut cublasEmulationStrategy_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetEmulationStrategy(handle: cublasHandle_t, emulationStrategy: cublasEmulationStrategy_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetEmulationSpecialValuesSupport(handle: cublasHandle_t, mask: *mut cudaEmulationSpecialValuesSupport) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetEmulationSpecialValuesSupport(handle: cublasHandle_t, mask: cudaEmulationSpecialValuesSupport) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetFixedPointEmulationMantissaControl(handle: cublasHandle_t, mantissaControl: *mut cudaEmulationMantissaControl) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetFixedPointEmulationMantissaControl(handle: cublasHandle_t, mantissaControl: cudaEmulationMantissaControl) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetFixedPointEmulationMaxMantissaBitCount(handle: cublasHandle_t, maxMantissaBitCount: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetFixedPointEmulationMaxMantissaBitCount(handle: cublasHandle_t, maxMantissaBitCount: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetFixedPointEmulationMantissaBitOffset(handle: cublasHandle_t, mantissaBitOffset: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetFixedPointEmulationMantissaBitOffset(handle: cublasHandle_t, mantissaBitOffset: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetFixedPointEmulationMantissaBitCountPointer(handle: cublasHandle_t, mantissaBitCount: *mut *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetFixedPointEmulationMantissaBitCountPointer(handle: cublasHandle_t, mantissaBitCount: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetStatusName(status: cublasStatus_t) -> *const ::std::os::raw::c_char;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetStatusString(status: cublasStatus_t) -> *const ::std::os::raw::c_char;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasLoggerConfigure(logIsOn: ::std::os::raw::c_int, logToStdOut: ::std::os::raw::c_int, logToStdErr: ::std::os::raw::c_int, logFileName: *const ::std::os::raw::c_char) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetLoggerCallback(userCallback: cublasLogCallback) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetLoggerCallback(userCallback: *mut cublasLogCallback) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetVector(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, devicePtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetVector_64(n: i64, elemSize: i64, x: *const ::std::os::raw::c_void, incx: i64, devicePtr: *mut ::std::os::raw::c_void, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetVector(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetVector_64(n: i64, elemSize: i64, x: *const ::std::os::raw::c_void, incx: i64, y: *mut ::std::os::raw::c_void, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetMatrix(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetMatrix_64(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetMatrix(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetMatrix_64(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetVectorAsync(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, hostPtr: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, devicePtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetVectorAsync_64(n: i64, elemSize: i64, hostPtr: *const ::std::os::raw::c_void, incx: i64, devicePtr: *mut ::std::os::raw::c_void, incy: i64, stream: cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetVectorAsync(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, devicePtr: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, hostPtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetVectorAsync_64(n: i64, elemSize: i64, devicePtr: *const ::std::os::raw::c_void, incx: i64, hostPtr: *mut ::std::os::raw::c_void, incy: i64, stream: cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetMatrixAsync(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSetMatrixAsync_64(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64, stream: cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetMatrixAsync(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGetMatrixAsync_64(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64, stream: cudaStream_t) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasXerbla(srName: *const ::std::os::raw::c_char, info: ::std::os::raw::c_int);
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasNrm2Ex(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executionType: cudaDataType) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasNrm2Ex_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executionType: cudaDataType) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSnrm2_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSnrm2_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDnrm2_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDnrm2_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasScnrm2_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasScnrm2_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDznrm2_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDznrm2_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDotEx(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        x: *const ::std::os::raw::c_void,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        y: *const ::std::os::raw::c_void,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        result: *mut ::std::os::raw::c_void,
        resultType: cudaDataType,
        executionType: cudaDataType,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDotEx_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *const ::std::os::raw::c_void, yType: cudaDataType, incy: i64, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executionType: cudaDataType)
    -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDotcEx(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        x: *const ::std::os::raw::c_void,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        y: *const ::std::os::raw::c_void,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        result: *mut ::std::os::raw::c_void,
        resultType: cudaDataType,
        executionType: cudaDataType,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDotcEx_64(
        handle: cublasHandle_t,
        n: i64,
        x: *const ::std::os::raw::c_void,
        xType: cudaDataType,
        incx: i64,
        y: *const ::std::os::raw::c_void,
        yType: cudaDataType,
        incy: i64,
        result: *mut ::std::os::raw::c_void,
        resultType: cudaDataType,
        executionType: cudaDataType,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSdot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSdot_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, y: *const f32, incy: i64, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDdot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDdot_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, y: *const f64, incy: i64, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCdotu_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, result: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCdotu_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, result: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCdotc_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, result: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCdotc_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, result: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdotu_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, result: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdotu_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, result: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdotc_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, result: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdotc_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, result: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasScalEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const ::std::os::raw::c_void, alphaType: cudaDataType, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, executionType: cudaDataType) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasScalEx_64(handle: cublasHandle_t, n: i64, alpha: *const ::std::os::raw::c_void, alphaType: cudaDataType, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, executionType: cudaDataType) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *mut cuComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasAxpyEx(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        alpha: *const ::std::os::raw::c_void,
        alphaType: cudaDataType,
        x: *const ::std::os::raw::c_void,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_void,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        executiontype: cudaDataType,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasAxpyEx_64(handle: cublasHandle_t, n: i64, alpha: *const ::std::os::raw::c_void, alphaType: cudaDataType, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64, executiontype: cudaDataType) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSaxpy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSaxpy_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDaxpy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDaxpy_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCaxpy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCaxpy_v2_64(handle: cublasHandle_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZaxpy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZaxpy_v2_64(handle: cublasHandle_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCopyEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCopyEx_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasScopy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasScopy_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDcopy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDcopy_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCcopy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCcopy_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZcopy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZcopy_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSswap_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSswap_v2_64(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDswap_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDswap_v2_64(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCswap_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCswap_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZswap_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZswap_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSwapEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSwapEx_64(handle: cublasHandle_t, n: i64, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIsamax_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIsamax_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIdamax_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIdamax_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIcamax_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIcamax_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIzamax_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIzamax_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIamaxEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIamaxEx_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIsamin_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIsamin_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIdamin_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIdamin_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIcamin_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIcamin_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIzamin_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIzamin_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIaminEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasIaminEx_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasAsumEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasAsumEx_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSasum_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSasum_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDasum_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDasum_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasScasum_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasScasum_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDzasum_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDzasum_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int, c: *const f32, s: *const f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64, c: *const f32, s: *const f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int, c: *const f64, s: *const f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64, c: *const f64, s: *const f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int, c: *const f32, s: *const cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64, c: *const f32, s: *const cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int, c: *const f32, s: *const f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64, c: *const f32, s: *const f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int, c: *const f64, s: *const cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64, c: *const f64, s: *const cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int, c: *const f64, s: *const f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64, c: *const f64, s: *const f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasRotEx(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        x: *mut ::std::os::raw::c_void,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_void,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        c: *const ::std::os::raw::c_void,
        s: *const ::std::os::raw::c_void,
        csType: cudaDataType,
        executiontype: cudaDataType,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasRotEx_64(
        handle: cublasHandle_t,
        n: i64,
        x: *mut ::std::os::raw::c_void,
        xType: cudaDataType,
        incx: i64,
        y: *mut ::std::os::raw::c_void,
        yType: cudaDataType,
        incy: i64,
        c: *const ::std::os::raw::c_void,
        s: *const ::std::os::raw::c_void,
        csType: cudaDataType,
        executiontype: cudaDataType,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSrotg_v2(handle: cublasHandle_t, a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDrotg_v2(handle: cublasHandle_t, a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCrotg_v2(handle: cublasHandle_t, a: *mut cuComplex, b: *mut cuComplex, c: *mut f32, s: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZrotg_v2(handle: cublasHandle_t, a: *mut cuDoubleComplex, b: *mut cuDoubleComplex, c: *mut f64, s: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasRotgEx(handle: cublasHandle_t, a: *mut ::std::os::raw::c_void, b: *mut ::std::os::raw::c_void, abType: cudaDataType, c: *mut ::std::os::raw::c_void, s: *mut ::std::os::raw::c_void, csType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSrotm_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int, param: *const f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSrotm_v2_64(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64, param: *const f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDrotm_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int, param: *const f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDrotm_v2_64(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64, param: *const f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasRotmEx(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        x: *mut ::std::os::raw::c_void,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_void,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        param: *const ::std::os::raw::c_void,
        paramType: cudaDataType,
        executiontype: cudaDataType,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasRotmEx_64(handle: cublasHandle_t, n: i64, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64, param: *const ::std::os::raw::c_void, paramType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSrotmg_v2(handle: cublasHandle_t, d1: *mut f32, d2: *mut f32, x1: *mut f32, y1: *const f32, param: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDrotmg_v2(handle: cublasHandle_t, d1: *mut f64, d2: *mut f64, x1: *mut f64, y1: *const f64, param: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasRotmgEx(
        handle: cublasHandle_t,
        d1: *mut ::std::os::raw::c_void,
        d1Type: cudaDataType,
        d2: *mut ::std::os::raw::c_void,
        d2Type: cudaDataType,
        x1: *mut ::std::os::raw::c_void,
        x1Type: cudaDataType,
        y1: *const ::std::os::raw::c_void,
        y1Type: cudaDataType,
        param: *mut ::std::os::raw::c_void,
        paramType: cudaDataType,
        executiontype: cudaDataType,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemv_v2(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        x: *const f32,
        incx: ::std::os::raw::c_int,
        beta: *const f32,
        y: *mut f32,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemv_v2(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        x: *const f64,
        incx: ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemv_v2(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuComplex,
        y: *mut cuComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemv_v2(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        y: *mut cuDoubleComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgbmv_v2(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kl: ::std::os::raw::c_int,
        ku: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        x: *const f32,
        incx: ::std::os::raw::c_int,
        beta: *const f32,
        y: *mut f32,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgbmv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgbmv_v2(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kl: ::std::os::raw::c_int,
        ku: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        x: *const f64,
        incx: ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgbmv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgbmv_v2(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kl: ::std::os::raw::c_int,
        ku: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuComplex,
        y: *mut cuComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgbmv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgbmv_v2(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kl: ::std::os::raw::c_int,
        ku: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        y: *mut cuDoubleComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgbmv_v2_64(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        x: *const cuDoubleComplex,
        incx: i64,
        beta: *const cuDoubleComplex,
        y: *mut cuDoubleComplex,
        incy: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStbmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtbmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtbmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtbmv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        x: *mut cuDoubleComplex,
        incx: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStpmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtpmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtpmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtpmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStpsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStpsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtpsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtpsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtpsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtpsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtpsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtpsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStbsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStbsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtbsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtbsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtbsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtbsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtbsv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        x: *mut cuDoubleComplex,
        incx: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtbsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsymv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, A: *const f32, lda: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, beta: *const f32, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsymv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsymv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, A: *const f64, lda: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, beta: *const f64, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsymv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsymv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuComplex,
        y: *mut cuComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsymv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsymv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        y: *mut cuDoubleComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsymv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChemv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuComplex,
        y: *mut cuComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChemv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhemv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        y: *mut cuDoubleComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhemv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsbmv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        x: *const f32,
        incx: ::std::os::raw::c_int,
        beta: *const f32,
        y: *mut f32,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsbmv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        x: *const f64,
        incx: ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChbmv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuComplex,
        y: *mut cuComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhbmv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        y: *mut cuDoubleComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSspmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, AP: *const f32, x: *const f32, incx: ::std::os::raw::c_int, beta: *const f32, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSspmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, AP: *const f32, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDspmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, AP: *const f64, x: *const f64, incx: ::std::os::raw::c_int, beta: *const f64, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDspmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, AP: *const f64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChpmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, AP: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, beta: *const cuComplex, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, AP: *const cuComplex, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhpmv_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        AP: *const cuDoubleComplex,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        y: *mut cuDoubleComplex,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, AP: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSger_v2(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSger_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, A: *mut f32, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDger_v2(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDger_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, A: *mut f64, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgeru_v2(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgeru_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgerc_v2(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgerc_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgeru_v2(
        handle: cublasHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        y: *const cuDoubleComplex,
        incy: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgeru_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgerc_v2(
        handle: cublasHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        y: *const cuDoubleComplex,
        incy: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgerc_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, A: *mut f32, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, A: *mut f64, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCher_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const cuComplex, incx: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCher_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const cuComplex, incx: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZher_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZher_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const cuDoubleComplex, incx: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSspr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSspr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, AP: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDspr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDspr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, AP: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChpr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const cuComplex, incx: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChpr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const cuComplex, incx: i64, AP: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhpr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhpr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const cuDoubleComplex, incx: i64, AP: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, A: *mut f32, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, A: *mut f64, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyr2_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        y: *const cuDoubleComplex,
        incy: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCher2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCher2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZher2_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        y: *const cuDoubleComplex,
        incy: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZher2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSspr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSspr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, AP: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDspr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDspr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, AP: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChpr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChpr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, AP: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhpr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhpr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, AP: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemvBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        Aarray: *const *const f32,
        lda: ::std::os::raw::c_int,
        xarray: *const *const f32,
        incx: ::std::os::raw::c_int,
        beta: *const f32,
        yarray: *const *mut f32,
        incy: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemvBatched_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f32, Aarray: *const *const f32, lda: i64, xarray: *const *const f32, incx: i64, beta: *const f32, yarray: *const *mut f32, incy: i64, batchCount: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemvBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f64,
        Aarray: *const *const f64,
        lda: ::std::os::raw::c_int,
        xarray: *const *const f64,
        incx: ::std::os::raw::c_int,
        beta: *const f64,
        yarray: *const *mut f64,
        incy: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemvBatched_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f64, Aarray: *const *const f64, lda: i64, xarray: *const *const f64, incx: i64, beta: *const f64, yarray: *const *mut f64, incy: i64, batchCount: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemvBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        Aarray: *const *const cuComplex,
        lda: ::std::os::raw::c_int,
        xarray: *const *const cuComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuComplex,
        yarray: *const *mut cuComplex,
        incy: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemvBatched_64(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: *const cuComplex,
        Aarray: *const *const cuComplex,
        lda: i64,
        xarray: *const *const cuComplex,
        incx: i64,
        beta: *const cuComplex,
        yarray: *const *mut cuComplex,
        incy: i64,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemvBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        Aarray: *const *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        xarray: *const *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        yarray: *const *mut cuDoubleComplex,
        incy: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemvBatched_64(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: *const cuDoubleComplex,
        Aarray: *const *const cuDoubleComplex,
        lda: i64,
        xarray: *const *const cuDoubleComplex,
        incx: i64,
        beta: *const cuDoubleComplex,
        yarray: *const *mut cuDoubleComplex,
        incy: i64,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemvStridedBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        x: *const f32,
        incx: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_longlong,
        beta: *const f32,
        y: *mut f32,
        incy: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemvStridedBatched_64(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        x: *const f32,
        incx: i64,
        stridex: ::std::os::raw::c_longlong,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
        stridey: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemvStridedBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        x: *const f64,
        incx: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_longlong,
        beta: *const f64,
        y: *mut f64,
        incy: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemvStridedBatched_64(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        x: *const f64,
        incx: i64,
        stridex: ::std::os::raw::c_longlong,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
        stridey: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemvStridedBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        x: *const cuComplex,
        incx: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_longlong,
        beta: *const cuComplex,
        y: *mut cuComplex,
        incy: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemvStridedBatched_64(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        x: *const cuComplex,
        incx: i64,
        stridex: ::std::os::raw::c_longlong,
        beta: *const cuComplex,
        y: *mut cuComplex,
        incy: i64,
        stridey: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemvStridedBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_longlong,
        beta: *const cuDoubleComplex,
        y: *mut cuDoubleComplex,
        incy: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemvStridedBatched_64(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        x: *const cuDoubleComplex,
        incx: i64,
        stridex: ::std::os::raw::c_longlong,
        beta: *const cuDoubleComplex,
        y: *mut cuDoubleComplex,
        incy: i64,
        stridey: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemm_v2(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemm_v2_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemm_v2(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemm_v2_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm_v2(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm_v2_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm3m(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm3m_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm3mEx(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm3mEx_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuComplex,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: i64,
        beta: *const cuComplex,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemm_v2(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemm_v2_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        B: *const cuDoubleComplex,
        ldb: i64,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemm3m(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemm3m_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        B: *const cuDoubleComplex,
        ldb: i64,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemmEx(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemmEx_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: i64,
        beta: *const f32,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGemmEx(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const ::std::os::raw::c_void,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: *const ::std::os::raw::c_void,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGemmEx_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const ::std::os::raw::c_void,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: i64,
        beta: *const ::std::os::raw::c_void,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemmEx(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemmEx_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuComplex,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: i64,
        beta: *const cuComplex,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyrk_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, alpha: *const f32, A: *const f32, lda: ::std::os::raw::c_int, beta: *const f32, C: *mut f32, ldc: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyrk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyrk_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, alpha: *const f64, A: *const f64, lda: ::std::os::raw::c_int, beta: *const f64, C: *mut f64, ldc: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyrk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyrk_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyrk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyrk_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyrk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, beta: *const cuDoubleComplex, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyrkEx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyrkEx_64(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: i64,
        k: i64,
        alpha: *const cuComplex,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        beta: *const cuComplex,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyrk3mEx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyrk3mEx_64(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: i64,
        k: i64,
        alpha: *const cuComplex,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        beta: *const cuComplex,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCherk_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCherk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const cuComplex, lda: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZherk_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        beta: *const f64,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZherk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const cuDoubleComplex, lda: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCherkEx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCherkEx_64(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        beta: *const f32,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCherk3mEx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCherk3mEx_64(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        beta: *const f32,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyr2k_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyr2k_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyr2k_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyr2k_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyr2k_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyr2k_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyr2k_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyr2k_v2_64(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: i64,
        k: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        B: *const cuDoubleComplex,
        ldb: i64,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCher2k_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCher2k_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZher2k_v2(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const f64,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZher2k_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, B: *const cuDoubleComplex, ldb: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyrkx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsyrkx_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyrkx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsyrkx_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyrkx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsyrkx_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyrkx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsyrkx_64(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: i64,
        k: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        B: *const cuDoubleComplex,
        ldb: i64,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCherkx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCherkx_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZherkx(
        handle: cublasHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const f64,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZherkx_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, B: *const cuDoubleComplex, ldb: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsymm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSsymm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsymm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDsymm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsymm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCsymm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsymm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZsymm_v2_64(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: i64,
        n: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        B: *const cuDoubleComplex,
        ldb: i64,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChemm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasChemm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhemm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZhemm_v2_64(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: i64,
        n: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        B: *const cuDoubleComplex,
        ldb: i64,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrsm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *mut f32,
        ldb: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrsm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *mut f32, ldb: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrsm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *mut f64,
        ldb: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrsm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *mut f64, ldb: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrsm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *mut cuComplex,
        ldb: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrsm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *mut cuComplex, ldb: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrsm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *mut cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrsm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, B: *mut cuDoubleComplex, ldb: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrmm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrmm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, C: *mut f32, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrmm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrmm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, C: *mut f64, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrmm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrmm_v2_64(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: i64,
        n: i64,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: i64,
        B: *const cuComplex,
        ldb: i64,
        C: *mut cuComplex,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrmm_v2(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrmm_v2_64(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: i64,
        n: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        B: *const cuDoubleComplex,
        ldb: i64,
        C: *mut cuDoubleComplex,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemmBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        Aarray: *const *const f32,
        lda: ::std::os::raw::c_int,
        Barray: *const *const f32,
        ldb: ::std::os::raw::c_int,
        beta: *const f32,
        Carray: *const *mut f32,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemmBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f32,
        Aarray: *const *const f32,
        lda: i64,
        Barray: *const *const f32,
        ldb: i64,
        beta: *const f32,
        Carray: *const *mut f32,
        ldc: i64,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemmBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f64,
        Aarray: *const *const f64,
        lda: ::std::os::raw::c_int,
        Barray: *const *const f64,
        ldb: ::std::os::raw::c_int,
        beta: *const f64,
        Carray: *const *mut f64,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemmBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f64,
        Aarray: *const *const f64,
        lda: i64,
        Barray: *const *const f64,
        ldb: i64,
        beta: *const f64,
        Carray: *const *mut f64,
        ldc: i64,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemmBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        Aarray: *const *const cuComplex,
        lda: ::std::os::raw::c_int,
        Barray: *const *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        Carray: *const *mut cuComplex,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemmBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuComplex,
        Aarray: *const *const cuComplex,
        lda: i64,
        Barray: *const *const cuComplex,
        ldb: i64,
        beta: *const cuComplex,
        Carray: *const *mut cuComplex,
        ldc: i64,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm3mBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        Aarray: *const *const cuComplex,
        lda: ::std::os::raw::c_int,
        Barray: *const *const cuComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuComplex,
        Carray: *const *mut cuComplex,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm3mBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuComplex,
        Aarray: *const *const cuComplex,
        lda: i64,
        Barray: *const *const cuComplex,
        ldb: i64,
        beta: *const cuComplex,
        Carray: *const *mut cuComplex,
        ldc: i64,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemmBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        Aarray: *const *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        Barray: *const *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        Carray: *const *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemmBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuDoubleComplex,
        Aarray: *const *const cuDoubleComplex,
        lda: i64,
        Barray: *const *const cuDoubleComplex,
        ldb: i64,
        beta: *const cuDoubleComplex,
        Carray: *const *mut cuDoubleComplex,
        ldc: i64,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemmStridedBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: *const f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemmStridedBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: *const f32,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemmStridedBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: *const f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemmStridedBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: *const f64,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemmStridedBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemmStridedBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: *const cuComplex,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm3mStridedBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgemm3mStridedBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: *const cuComplex,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemmStridedBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgemmStridedBatched_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: *const cuDoubleComplex,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGemmBatchedEx(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const ::std::os::raw::c_void,
        Aarray: *const *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        Barray: *const *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: *const ::std::os::raw::c_void,
        Carray: *const *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGemmBatchedEx_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const ::std::os::raw::c_void,
        Aarray: *const *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        Barray: *const *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: i64,
        beta: *const ::std::os::raw::c_void,
        Carray: *const *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
        batchCount: i64,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGemmStridedBatchedEx(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const ::std::os::raw::c_void,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: *const ::std::os::raw::c_void,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGemmStridedBatchedEx_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const ::std::os::raw::c_void,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: *const ::std::os::raw::c_void,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemmGroupedBatched(
        handle: cublasHandle_t,
        transa_array: *const cublasOperation_t,
        transb_array: *const cublasOperation_t,
        m_array: *const ::std::os::raw::c_int,
        n_array: *const ::std::os::raw::c_int,
        k_array: *const ::std::os::raw::c_int,
        alpha_array: *const f32,
        Aarray: *const *const f32,
        lda_array: *const ::std::os::raw::c_int,
        Barray: *const *const f32,
        ldb_array: *const ::std::os::raw::c_int,
        beta_array: *const f32,
        Carray: *const *mut f32,
        ldc_array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgemmGroupedBatched_64(
        handle: cublasHandle_t,
        transa_array: *const cublasOperation_t,
        transb_array: *const cublasOperation_t,
        m_array: *const i64,
        n_array: *const i64,
        k_array: *const i64,
        alpha_array: *const f32,
        Aarray: *const *const f32,
        lda_array: *const i64,
        Barray: *const *const f32,
        ldb_array: *const i64,
        beta_array: *const f32,
        Carray: *const *mut f32,
        ldc_array: *const i64,
        group_count: i64,
        group_size: *const i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemmGroupedBatched(
        handle: cublasHandle_t,
        transa_array: *const cublasOperation_t,
        transb_array: *const cublasOperation_t,
        m_array: *const ::std::os::raw::c_int,
        n_array: *const ::std::os::raw::c_int,
        k_array: *const ::std::os::raw::c_int,
        alpha_array: *const f64,
        Aarray: *const *const f64,
        lda_array: *const ::std::os::raw::c_int,
        Barray: *const *const f64,
        ldb_array: *const ::std::os::raw::c_int,
        beta_array: *const f64,
        Carray: *const *mut f64,
        ldc_array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgemmGroupedBatched_64(
        handle: cublasHandle_t,
        transa_array: *const cublasOperation_t,
        transb_array: *const cublasOperation_t,
        m_array: *const i64,
        n_array: *const i64,
        k_array: *const i64,
        alpha_array: *const f64,
        Aarray: *const *const f64,
        lda_array: *const i64,
        Barray: *const *const f64,
        ldb_array: *const i64,
        beta_array: *const f64,
        Carray: *const *mut f64,
        ldc_array: *const i64,
        group_count: i64,
        group_size: *const i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGemmGroupedBatchedEx(
        handle: cublasHandle_t,
        transa_array: *const cublasOperation_t,
        transb_array: *const cublasOperation_t,
        m_array: *const ::std::os::raw::c_int,
        n_array: *const ::std::os::raw::c_int,
        k_array: *const ::std::os::raw::c_int,
        alpha_array: *const ::std::os::raw::c_void,
        Aarray: *const *const ::std::os::raw::c_void,
        Atype: cudaDataType_t,
        lda_array: *const ::std::os::raw::c_int,
        Barray: *const *const ::std::os::raw::c_void,
        Btype: cudaDataType_t,
        ldb_array: *const ::std::os::raw::c_int,
        beta_array: *const ::std::os::raw::c_void,
        Carray: *const *mut ::std::os::raw::c_void,
        Ctype: cudaDataType_t,
        ldc_array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
        computeType: cublasComputeType_t,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasGemmGroupedBatchedEx_64(
        handle: cublasHandle_t,
        transa_array: *const cublasOperation_t,
        transb_array: *const cublasOperation_t,
        m_array: *const i64,
        n_array: *const i64,
        k_array: *const i64,
        alpha_array: *const ::std::os::raw::c_void,
        Aarray: *const *const ::std::os::raw::c_void,
        Atype: cudaDataType_t,
        lda_array: *const i64,
        Barray: *const *const ::std::os::raw::c_void,
        Btype: cudaDataType_t,
        ldb_array: *const i64,
        beta_array: *const ::std::os::raw::c_void,
        Carray: *const *mut ::std::os::raw::c_void,
        Ctype: cudaDataType_t,
        ldc_array: *const i64,
        group_count: i64,
        group_size: *const i64,
        computeType: cublasComputeType_t,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgeam(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        beta: *const f32,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgeam_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, beta: *const f32, B: *const f32, ldb: i64, C: *mut f32, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgeam(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        beta: *const f64,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgeam_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, beta: *const f64, B: *const f64, ldb: i64, C: *mut f64, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgeam(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        beta: *const cuComplex,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgeam_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, beta: *const cuComplex, B: *const cuComplex, ldb: i64, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgeam(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        beta: *const cuDoubleComplex,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgeam_64(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: i64,
        beta: *const cuDoubleComplex,
        B: *const cuDoubleComplex,
        ldb: i64,
        C: *mut cuDoubleComplex,
        ldc: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrsmBatched(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: ::std::os::raw::c_int,
        B: *const *mut f32,
        ldb: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrsmBatched_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const *const f32, lda: i64, B: *const *mut f32, ldb: i64, batchCount: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrsmBatched(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: ::std::os::raw::c_int,
        B: *const *mut f64,
        ldb: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrsmBatched_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const *const f64, lda: i64, B: *const *mut f64, ldb: i64, batchCount: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrsmBatched(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuComplex,
        A: *const *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const *mut cuComplex,
        ldb: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrsmBatched_64(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: i64,
        n: i64,
        alpha: *const cuComplex,
        A: *const *const cuComplex,
        lda: i64,
        B: *const *mut cuComplex,
        ldb: i64,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrsmBatched(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const cuDoubleComplex,
        A: *const *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const *mut cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrsmBatched_64(
        handle: cublasHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: i64,
        n: i64,
        alpha: *const cuDoubleComplex,
        A: *const *const cuDoubleComplex,
        lda: i64,
        B: *const *mut cuDoubleComplex,
        ldb: i64,
        batchCount: i64,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSdgmm(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, C: *mut f32, ldc: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSdgmm_64(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const f32, lda: i64, x: *const f32, incx: i64, C: *mut f32, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDdgmm(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, C: *mut f64, ldc: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDdgmm_64(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const f64, lda: i64, x: *const f64, incx: i64, C: *mut f64, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCdgmm(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, C: *mut cuComplex, ldc: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCdgmm_64(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, C: *mut cuComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdgmm(
        handle: cublasHandle_t,
        mode: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        x: *const cuDoubleComplex,
        incx: ::std::os::raw::c_int,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZdgmm_64(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSmatinvBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f32, lda: ::std::os::raw::c_int, Ainv: *const *mut f32, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDmatinvBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f64, lda: ::std::os::raw::c_int, Ainv: *const *mut f64, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCmatinvBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const cuComplex, lda: ::std::os::raw::c_int, Ainv: *const *mut cuComplex, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZmatinvBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const cuDoubleComplex, lda: ::std::os::raw::c_int, Ainv: *const *mut cuDoubleComplex, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgeqrfBatched(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut f32, lda: ::std::os::raw::c_int, TauArray: *const *mut f32, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgeqrfBatched(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut f64, lda: ::std::os::raw::c_int, TauArray: *const *mut f64, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgeqrfBatched(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut cuComplex, lda: ::std::os::raw::c_int, TauArray: *const *mut cuComplex, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgeqrfBatched(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut cuDoubleComplex, lda: ::std::os::raw::c_int, TauArray: *const *mut cuDoubleComplex, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgelsBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: *const *mut f32,
        lda: ::std::os::raw::c_int,
        Carray: *const *mut f32,
        ldc: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        devInfoArray: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgelsBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: *const *mut f64,
        lda: ::std::os::raw::c_int,
        Carray: *const *mut f64,
        ldc: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        devInfoArray: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgelsBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: *const *mut cuComplex,
        lda: ::std::os::raw::c_int,
        Carray: *const *mut cuComplex,
        ldc: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        devInfoArray: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgelsBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: *const *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        Carray: *const *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        devInfoArray: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStpttr(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const f32, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtpttr(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const f64, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtpttr(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const cuComplex, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtpttr(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasStrttp(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDtrttp(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCtrttp(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZtrttp(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgetrfBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut f32, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgetrfBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut f64, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgetrfBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut cuComplex, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgetrfBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut cuDoubleComplex, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgetriBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f32, lda: ::std::os::raw::c_int, P: *const ::std::os::raw::c_int, C: *const *mut f32, ldc: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgetriBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f64, lda: ::std::os::raw::c_int, P: *const ::std::os::raw::c_int, C: *const *mut f64, ldc: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgetriBatched(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        A: *const *const cuComplex,
        lda: ::std::os::raw::c_int,
        P: *const ::std::os::raw::c_int,
        C: *const *mut cuComplex,
        ldc: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgetriBatched(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        A: *const *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        P: *const ::std::os::raw::c_int,
        C: *const *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasSgetrsBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: *const *const f32,
        lda: ::std::os::raw::c_int,
        devIpiv: *const ::std::os::raw::c_int,
        Barray: *const *mut f32,
        ldb: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasDgetrsBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: *const *const f64,
        lda: ::std::os::raw::c_int,
        devIpiv: *const ::std::os::raw::c_int,
        Barray: *const *mut f64,
        ldb: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasCgetrsBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: *const *const cuComplex,
        lda: ::std::os::raw::c_int,
        devIpiv: *const ::std::os::raw::c_int,
        Barray: *const *mut cuComplex,
        ldb: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasZgetrsBatched(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: *const *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        devIpiv: *const ::std::os::raw::c_int,
        Barray: *const *mut cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cublasUint8gemmBias(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        transc: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const ::std::os::raw::c_uchar,
        A_bias: ::std::os::raw::c_int,
        lda: ::std::os::raw::c_int,
        B: *const ::std::os::raw::c_uchar,
        B_bias: ::std::os::raw::c_int,
        ldb: ::std::os::raw::c_int,
        C: *mut ::std::os::raw::c_uchar,
        C_bias: ::std::os::raw::c_int,
        ldc: ::std::os::raw::c_int,
        C_mult: ::std::os::raw::c_int,
        C_shift: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[cfg(feature = "runtime-link")]
pub struct DynamicBindings {
    pub cublasCreate_v2: Option<unsafe extern "C" fn(handle: *mut cublasHandle_t) -> cublasStatus_t>,
    pub cublasDestroy_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t) -> cublasStatus_t>,
    pub cublasGetVersion_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, version: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasGetProperty: Option<unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasGetCudartVersion: Option<unsafe extern "C" fn() -> usize>,
    pub cublasSetWorkspace_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, workspace: *mut ::std::os::raw::c_void, workspaceSizeInBytes: usize) -> cublasStatus_t>,
    pub cublasSetStream_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, streamId: cudaStream_t) -> cublasStatus_t>,
    pub cublasGetStream_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, streamId: *mut cudaStream_t) -> cublasStatus_t>,
    pub cublasGetPointerMode_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: *mut cublasPointerMode_t) -> cublasStatus_t>,
    pub cublasSetPointerMode_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasPointerMode_t) -> cublasStatus_t>,
    pub cublasGetAtomicsMode: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: *mut cublasAtomicsMode_t) -> cublasStatus_t>,
    pub cublasSetAtomicsMode: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasAtomicsMode_t) -> cublasStatus_t>,
    pub cublasGetMathMode: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: *mut cublasMath_t) -> cublasStatus_t>,
    pub cublasSetMathMode: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasMath_t) -> cublasStatus_t>,
    pub cublasGetSmCountTarget: Option<unsafe extern "C" fn(handle: cublasHandle_t, smCountTarget: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSetSmCountTarget: Option<unsafe extern "C" fn(handle: cublasHandle_t, smCountTarget: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasGetEmulationStrategy: Option<unsafe extern "C" fn(handle: cublasHandle_t, emulationStrategy: *mut cublasEmulationStrategy_t) -> cublasStatus_t>,
    pub cublasSetEmulationStrategy: Option<unsafe extern "C" fn(handle: cublasHandle_t, emulationStrategy: cublasEmulationStrategy_t) -> cublasStatus_t>,
    pub cublasGetEmulationSpecialValuesSupport: Option<unsafe extern "C" fn(handle: cublasHandle_t, mask: *mut cudaEmulationSpecialValuesSupport) -> cublasStatus_t>,
    pub cublasSetEmulationSpecialValuesSupport: Option<unsafe extern "C" fn(handle: cublasHandle_t, mask: cudaEmulationSpecialValuesSupport) -> cublasStatus_t>,
    pub cublasGetFixedPointEmulationMantissaControl: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaControl: *mut cudaEmulationMantissaControl) -> cublasStatus_t>,
    pub cublasSetFixedPointEmulationMantissaControl: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaControl: cudaEmulationMantissaControl) -> cublasStatus_t>,
    pub cublasGetFixedPointEmulationMaxMantissaBitCount: Option<unsafe extern "C" fn(handle: cublasHandle_t, maxMantissaBitCount: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSetFixedPointEmulationMaxMantissaBitCount: Option<unsafe extern "C" fn(handle: cublasHandle_t, maxMantissaBitCount: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasGetFixedPointEmulationMantissaBitOffset: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaBitOffset: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSetFixedPointEmulationMantissaBitOffset: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaBitOffset: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasGetFixedPointEmulationMantissaBitCountPointer: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaBitCount: *mut *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSetFixedPointEmulationMantissaBitCountPointer: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaBitCount: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasGetStatusName: Option<unsafe extern "C" fn(status: cublasStatus_t) -> *const ::std::os::raw::c_char>,
    pub cublasGetStatusString: Option<unsafe extern "C" fn(status: cublasStatus_t) -> *const ::std::os::raw::c_char>,
    pub cublasLoggerConfigure: Option<unsafe extern "C" fn(logIsOn: ::std::os::raw::c_int, logToStdOut: ::std::os::raw::c_int, logToStdErr: ::std::os::raw::c_int, logFileName: *const ::std::os::raw::c_char) -> cublasStatus_t>,
    pub cublasSetLoggerCallback: Option<unsafe extern "C" fn(userCallback: cublasLogCallback) -> cublasStatus_t>,
    pub cublasGetLoggerCallback: Option<unsafe extern "C" fn(userCallback: *mut cublasLogCallback) -> cublasStatus_t>,
    pub cublasSetVector: Option<unsafe extern "C" fn(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, devicePtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSetVector_64: Option<unsafe extern "C" fn(n: i64, elemSize: i64, x: *const ::std::os::raw::c_void, incx: i64, devicePtr: *mut ::std::os::raw::c_void, incy: i64) -> cublasStatus_t>,
    pub cublasGetVector: Option<unsafe extern "C" fn(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasGetVector_64: Option<unsafe extern "C" fn(n: i64, elemSize: i64, x: *const ::std::os::raw::c_void, incx: i64, y: *mut ::std::os::raw::c_void, incy: i64) -> cublasStatus_t>,
    pub cublasSetMatrix: Option<unsafe extern "C" fn(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSetMatrix_64: Option<unsafe extern "C" fn(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64) -> cublasStatus_t>,
    pub cublasGetMatrix: Option<unsafe extern "C" fn(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasGetMatrix_64: Option<unsafe extern "C" fn(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64) -> cublasStatus_t>,
    pub cublasSetVectorAsync: Option<unsafe extern "C" fn(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, hostPtr: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, devicePtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t>,
    pub cublasSetVectorAsync_64: Option<unsafe extern "C" fn(n: i64, elemSize: i64, hostPtr: *const ::std::os::raw::c_void, incx: i64, devicePtr: *mut ::std::os::raw::c_void, incy: i64, stream: cudaStream_t) -> cublasStatus_t>,
    pub cublasGetVectorAsync: Option<unsafe extern "C" fn(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, devicePtr: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, hostPtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t>,
    pub cublasGetVectorAsync_64: Option<unsafe extern "C" fn(n: i64, elemSize: i64, devicePtr: *const ::std::os::raw::c_void, incx: i64, hostPtr: *mut ::std::os::raw::c_void, incy: i64, stream: cudaStream_t) -> cublasStatus_t>,
    pub cublasSetMatrixAsync:
        Option<unsafe extern "C" fn(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t>,
    pub cublasSetMatrixAsync_64: Option<unsafe extern "C" fn(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64, stream: cudaStream_t) -> cublasStatus_t>,
    pub cublasGetMatrixAsync:
        Option<unsafe extern "C" fn(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t>,
    pub cublasGetMatrixAsync_64: Option<unsafe extern "C" fn(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64, stream: cudaStream_t) -> cublasStatus_t>,
    pub cublasXerbla: Option<unsafe extern "C" fn(srName: *const ::std::os::raw::c_char, info: ::std::os::raw::c_int)>,
    pub cublasNrm2Ex: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executionType: cudaDataType) -> cublasStatus_t>,
    pub cublasNrm2Ex_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executionType: cudaDataType) -> cublasStatus_t>,
    pub cublasSnrm2_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>,
    pub cublasSnrm2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut f32) -> cublasStatus_t>,
    pub cublasDnrm2_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>,
    pub cublasDnrm2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut f64) -> cublasStatus_t>,
    pub cublasScnrm2_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>,
    pub cublasScnrm2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut f32) -> cublasStatus_t>,
    pub cublasDznrm2_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>,
    pub cublasDznrm2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut f64) -> cublasStatus_t>,
    pub cublasDotEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: ::std::os::raw::c_int,
            x: *const ::std::os::raw::c_void,
            xType: cudaDataType,
            incx: ::std::os::raw::c_int,
            y: *const ::std::os::raw::c_void,
            yType: cudaDataType,
            incy: ::std::os::raw::c_int,
            result: *mut ::std::os::raw::c_void,
            resultType: cudaDataType,
            executionType: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasDotEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: i64,
            x: *const ::std::os::raw::c_void,
            xType: cudaDataType,
            incx: i64,
            y: *const ::std::os::raw::c_void,
            yType: cudaDataType,
            incy: i64,
            result: *mut ::std::os::raw::c_void,
            resultType: cudaDataType,
            executionType: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasDotcEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: ::std::os::raw::c_int,
            x: *const ::std::os::raw::c_void,
            xType: cudaDataType,
            incx: ::std::os::raw::c_int,
            y: *const ::std::os::raw::c_void,
            yType: cudaDataType,
            incy: ::std::os::raw::c_int,
            result: *mut ::std::os::raw::c_void,
            resultType: cudaDataType,
            executionType: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasDotcEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: i64,
            x: *const ::std::os::raw::c_void,
            xType: cudaDataType,
            incx: i64,
            y: *const ::std::os::raw::c_void,
            yType: cudaDataType,
            incy: i64,
            result: *mut ::std::os::raw::c_void,
            resultType: cudaDataType,
            executionType: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasSdot_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>,
    pub cublasSdot_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, y: *const f32, incy: i64, result: *mut f32) -> cublasStatus_t>,
    pub cublasDdot_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>,
    pub cublasDdot_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, y: *const f64, incy: i64, result: *mut f64) -> cublasStatus_t>,
    pub cublasCdotu_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, result: *mut cuComplex) -> cublasStatus_t>,
    pub cublasCdotu_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, result: *mut cuComplex) -> cublasStatus_t>,
    pub cublasCdotc_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, result: *mut cuComplex) -> cublasStatus_t>,
    pub cublasCdotc_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, result: *mut cuComplex) -> cublasStatus_t>,
    pub cublasZdotu_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, result: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasZdotu_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, result: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasZdotc_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, result: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasZdotc_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, result: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasScalEx: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const ::std::os::raw::c_void, alphaType: cudaDataType, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, executionType: cudaDataType) -> cublasStatus_t>,
    pub cublasScalEx_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const ::std::os::raw::c_void, alphaType: cudaDataType, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, executionType: cudaDataType) -> cublasStatus_t>,
    pub cublasSscal_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSscal_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t>,
    pub cublasDscal_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDscal_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t>,
    pub cublasCscal_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCscal_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t>,
    pub cublasCsscal_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCsscal_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *mut cuComplex, incx: i64) -> cublasStatus_t>,
    pub cublasZscal_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZscal_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>,
    pub cublasZdscal_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZdscal_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>,
    pub cublasAxpyEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: ::std::os::raw::c_int,
            alpha: *const ::std::os::raw::c_void,
            alphaType: cudaDataType,
            x: *const ::std::os::raw::c_void,
            xType: cudaDataType,
            incx: ::std::os::raw::c_int,
            y: *mut ::std::os::raw::c_void,
            yType: cudaDataType,
            incy: ::std::os::raw::c_int,
            executiontype: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasAxpyEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: i64,
            alpha: *const ::std::os::raw::c_void,
            alphaType: cudaDataType,
            x: *const ::std::os::raw::c_void,
            xType: cudaDataType,
            incx: i64,
            y: *mut ::std::os::raw::c_void,
            yType: cudaDataType,
            incy: i64,
            executiontype: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasSaxpy_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSaxpy_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t>,
    pub cublasDaxpy_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDaxpy_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t>,
    pub cublasCaxpy_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCaxpy_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    pub cublasZaxpy_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZaxpy_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    pub cublasCopyEx: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCopyEx_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64) -> cublasStatus_t>,
    pub cublasScopy_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasScopy_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t>,
    pub cublasDcopy_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDcopy_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t>,
    pub cublasCcopy_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCcopy_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    pub cublasZcopy_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZcopy_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    pub cublasSswap_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSswap_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t>,
    pub cublasDswap_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDswap_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t>,
    pub cublasCswap_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCswap_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    pub cublasZswap_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZswap_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    pub cublasSwapEx: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSwapEx_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64) -> cublasStatus_t>,
    pub cublasIsamax_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIsamax_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasIdamax_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIdamax_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasIcamax_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIcamax_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasIzamax_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIzamax_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasIamaxEx: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIamaxEx_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasIsamin_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIsamin_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasIdamin_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIdamin_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasIcamin_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIcamin_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasIzamin_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIzamin_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasIaminEx: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasIaminEx_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut i64) -> cublasStatus_t>,
    pub cublasAsumEx: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t>,
    pub cublasAsumEx_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t>,
    pub cublasSasum_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>,
    pub cublasSasum_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut f32) -> cublasStatus_t>,
    pub cublasDasum_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>,
    pub cublasDasum_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut f64) -> cublasStatus_t>,
    pub cublasScasum_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>,
    pub cublasScasum_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut f32) -> cublasStatus_t>,
    pub cublasDzasum_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>,
    pub cublasDzasum_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut f64) -> cublasStatus_t>,
    pub cublasSrot_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int, c: *const f32, s: *const f32) -> cublasStatus_t>,
    pub cublasSrot_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64, c: *const f32, s: *const f32) -> cublasStatus_t>,
    pub cublasDrot_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int, c: *const f64, s: *const f64) -> cublasStatus_t>,
    pub cublasDrot_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64, c: *const f64, s: *const f64) -> cublasStatus_t>,
    pub cublasCrot_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int, c: *const f32, s: *const cuComplex) -> cublasStatus_t>,
    pub cublasCrot_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64, c: *const f32, s: *const cuComplex) -> cublasStatus_t>,
    pub cublasCsrot_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int, c: *const f32, s: *const f32) -> cublasStatus_t>,
    pub cublasCsrot_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64, c: *const f32, s: *const f32) -> cublasStatus_t>,
    pub cublasZrot_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int, c: *const f64, s: *const cuDoubleComplex) -> cublasStatus_t>,
    pub cublasZrot_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64, c: *const f64, s: *const cuDoubleComplex) -> cublasStatus_t>,
    pub cublasZdrot_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int, c: *const f64, s: *const f64) -> cublasStatus_t>,
    pub cublasZdrot_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64, c: *const f64, s: *const f64) -> cublasStatus_t>,
    pub cublasRotEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: ::std::os::raw::c_int,
            x: *mut ::std::os::raw::c_void,
            xType: cudaDataType,
            incx: ::std::os::raw::c_int,
            y: *mut ::std::os::raw::c_void,
            yType: cudaDataType,
            incy: ::std::os::raw::c_int,
            c: *const ::std::os::raw::c_void,
            s: *const ::std::os::raw::c_void,
            csType: cudaDataType,
            executiontype: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasRotEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: i64,
            x: *mut ::std::os::raw::c_void,
            xType: cudaDataType,
            incx: i64,
            y: *mut ::std::os::raw::c_void,
            yType: cudaDataType,
            incy: i64,
            c: *const ::std::os::raw::c_void,
            s: *const ::std::os::raw::c_void,
            csType: cudaDataType,
            executiontype: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasSrotg_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) -> cublasStatus_t>,
    pub cublasDrotg_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) -> cublasStatus_t>,
    pub cublasCrotg_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut cuComplex, b: *mut cuComplex, c: *mut f32, s: *mut cuComplex) -> cublasStatus_t>,
    pub cublasZrotg_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut cuDoubleComplex, b: *mut cuDoubleComplex, c: *mut f64, s: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasRotgEx: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut ::std::os::raw::c_void, b: *mut ::std::os::raw::c_void, abType: cudaDataType, c: *mut ::std::os::raw::c_void, s: *mut ::std::os::raw::c_void, csType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t>,
    pub cublasSrotm_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int, param: *const f32) -> cublasStatus_t>,
    pub cublasSrotm_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64, param: *const f32) -> cublasStatus_t>,
    pub cublasDrotm_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int, param: *const f64) -> cublasStatus_t>,
    pub cublasDrotm_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64, param: *const f64) -> cublasStatus_t>,
    pub cublasRotmEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: ::std::os::raw::c_int,
            x: *mut ::std::os::raw::c_void,
            xType: cudaDataType,
            incx: ::std::os::raw::c_int,
            y: *mut ::std::os::raw::c_void,
            yType: cudaDataType,
            incy: ::std::os::raw::c_int,
            param: *const ::std::os::raw::c_void,
            paramType: cudaDataType,
            executiontype: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasRotmEx_64: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64, param: *const ::std::os::raw::c_void, paramType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t,
    >,
    pub cublasSrotmg_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, d1: *mut f32, d2: *mut f32, x1: *mut f32, y1: *const f32, param: *mut f32) -> cublasStatus_t>,
    pub cublasDrotmg_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, d1: *mut f64, d2: *mut f64, x1: *mut f64, y1: *const f64, param: *mut f64) -> cublasStatus_t>,
    pub cublasRotmgEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            d1: *mut ::std::os::raw::c_void,
            d1Type: cudaDataType,
            d2: *mut ::std::os::raw::c_void,
            d2Type: cudaDataType,
            x1: *mut ::std::os::raw::c_void,
            x1Type: cudaDataType,
            y1: *const ::std::os::raw::c_void,
            y1Type: cudaDataType,
            param: *mut ::std::os::raw::c_void,
            paramType: cudaDataType,
            executiontype: cudaDataType,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            x: *const f32,
            incx: ::std::os::raw::c_int,
            beta: *const f32,
            y: *mut f32,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>,
    pub cublasDgemv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            x: *const f64,
            incx: ::std::os::raw::c_int,
            beta: *const f64,
            y: *mut f64,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>,
    pub cublasCgemv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuComplex,
            y: *mut cuComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    pub cublasZgemv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            y: *mut cuDoubleComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemv_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    pub cublasSgbmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            kl: ::std::os::raw::c_int,
            ku: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            x: *const f32,
            incx: ::std::os::raw::c_int,
            beta: *const f32,
            y: *mut f32,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>,
    pub cublasDgbmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            kl: ::std::os::raw::c_int,
            ku: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            x: *const f64,
            incx: ::std::os::raw::c_int,
            beta: *const f64,
            y: *mut f64,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>,
    pub cublasCgbmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            kl: ::std::os::raw::c_int,
            ku: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuComplex,
            y: *mut cuComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    pub cublasZgbmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            kl: ::std::os::raw::c_int,
            ku: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            y: *mut cuDoubleComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgbmv_v2_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: i64,
            n: i64,
            kl: i64,
            ku: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            x: *const cuDoubleComplex,
            incx: i64,
            beta: *const cuDoubleComplex,
            y: *mut cuDoubleComplex,
            incy: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasStrmv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasStrmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t>,
    pub cublasDtrmv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDtrmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t>,
    pub cublasCtrmv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCtrmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t>,
    pub cublasZtrmv_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZtrmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>,
    pub cublasStbmv_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasStbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t>,
    pub cublasDtbmv_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDtbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t>,
    pub cublasCtbmv_v2: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasCtbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t>,
    pub cublasZtbmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            x: *mut cuDoubleComplex,
            incx: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZtbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>,
    pub cublasStpmv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasStpmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t>,
    pub cublasDtpmv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDtpmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t>,
    pub cublasCtpmv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCtpmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t>,
    pub cublasZtpmv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZtpmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>,
    pub cublasStrsv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasStrsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t>,
    pub cublasDtrsv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDtrsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t>,
    pub cublasCtrsv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCtrsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t>,
    pub cublasZtrsv_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZtrsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>,
    pub cublasStpsv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasStpsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t>,
    pub cublasDtpsv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDtpsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t>,
    pub cublasCtpsv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCtpsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t>,
    pub cublasZtpsv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZtpsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>,
    pub cublasStbsv_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasStbsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t>,
    pub cublasDtbsv_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDtbsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t>,
    pub cublasCtbsv_v2: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasCtbsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t>,
    pub cublasZtbsv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            x: *mut cuDoubleComplex,
            incx: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZtbsv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>,
    pub cublasSsymv_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, A: *const f32, lda: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, beta: *const f32, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSsymv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>,
    pub cublasDsymv_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, A: *const f64, lda: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, beta: *const f64, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDsymv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>,
    pub cublasCsymv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuComplex,
            y: *mut cuComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCsymv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    pub cublasZsymv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            y: *mut cuDoubleComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZsymv_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    pub cublasChemv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuComplex,
            y: *mut cuComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasChemv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    pub cublasZhemv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            y: *mut cuDoubleComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZhemv_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    pub cublasSsbmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            x: *const f32,
            incx: ::std::os::raw::c_int,
            beta: *const f32,
            y: *mut f32,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSsbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>,
    pub cublasDsbmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            x: *const f64,
            incx: ::std::os::raw::c_int,
            beta: *const f64,
            y: *mut f64,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDsbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>,
    pub cublasChbmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuComplex,
            y: *mut cuComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasChbmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    pub cublasZhbmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            y: *mut cuDoubleComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZhbmv_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    pub cublasSspmv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, AP: *const f32, x: *const f32, incx: ::std::os::raw::c_int, beta: *const f32, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSspmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, AP: *const f32, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>,
    pub cublasDspmv_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, AP: *const f64, x: *const f64, incx: ::std::os::raw::c_int, beta: *const f64, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDspmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, AP: *const f64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>,
    pub cublasChpmv_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, AP: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, beta: *const cuComplex, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasChpmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, AP: *const cuComplex, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    pub cublasZhpmv_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            AP: *const cuDoubleComplex,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            y: *mut cuDoubleComplex,
            incy: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZhpmv_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, AP: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    pub cublasSger_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSger_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, A: *mut f32, lda: i64) -> cublasStatus_t>,
    pub cublasDger_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDger_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, A: *mut f64, lda: i64) -> cublasStatus_t>,
    pub cublasCgeru_v2: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasCgeru_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>,
    pub cublasCgerc_v2: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasCgerc_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>,
    pub cublasZgeru_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            y: *const cuDoubleComplex,
            incy: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgeru_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>,
    pub cublasZgerc_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            y: *const cuDoubleComplex,
            incy: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgerc_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>,
    pub cublasSsyr_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSsyr_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, A: *mut f32, lda: i64) -> cublasStatus_t>,
    pub cublasDsyr_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDsyr_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, A: *mut f64, lda: i64) -> cublasStatus_t>,
    pub cublasCsyr_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCsyr_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>,
    pub cublasZsyr_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZsyr_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>,
    pub cublasCher_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const cuComplex, incx: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCher_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const cuComplex, incx: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>,
    pub cublasZher_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZher_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const cuDoubleComplex, incx: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>,
    pub cublasSspr_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t>,
    pub cublasSspr_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, AP: *mut f32) -> cublasStatus_t>,
    pub cublasDspr_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t>,
    pub cublasDspr_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, AP: *mut f64) -> cublasStatus_t>,
    pub cublasChpr_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const cuComplex, incx: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t>,
    pub cublasChpr_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const cuComplex, incx: i64, AP: *mut cuComplex) -> cublasStatus_t>,
    pub cublasZhpr_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasZhpr_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const cuDoubleComplex, incx: i64, AP: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasSsyr2_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSsyr2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, A: *mut f32, lda: i64) -> cublasStatus_t>,
    pub cublasDsyr2_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDsyr2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, A: *mut f64, lda: i64) -> cublasStatus_t>,
    pub cublasCsyr2_v2: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasCsyr2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>,
    pub cublasZsyr2_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            y: *const cuDoubleComplex,
            incy: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZsyr2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>,
    pub cublasCher2_v2: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasCher2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>,
    pub cublasZher2_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            y: *const cuDoubleComplex,
            incy: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZher2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>,
    pub cublasSspr2_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t>,
    pub cublasSspr2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, AP: *mut f32) -> cublasStatus_t>,
    pub cublasDspr2_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t>,
    pub cublasDspr2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, AP: *mut f64) -> cublasStatus_t>,
    pub cublasChpr2_v2: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t>,
    pub cublasChpr2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, AP: *mut cuComplex) -> cublasStatus_t>,
    pub cublasZhpr2_v2:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasZhpr2_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, AP: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasSgemvBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f32,
            Aarray: *const *const f32,
            lda: ::std::os::raw::c_int,
            xarray: *const *const f32,
            incx: ::std::os::raw::c_int,
            beta: *const f32,
            yarray: *const *mut f32,
            incy: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemvBatched_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f32, Aarray: *const *const f32, lda: i64, xarray: *const *const f32, incx: i64, beta: *const f32, yarray: *const *mut f32, incy: i64, batchCount: i64) -> cublasStatus_t>,
    pub cublasDgemvBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f64,
            Aarray: *const *const f64,
            lda: ::std::os::raw::c_int,
            xarray: *const *const f64,
            incx: ::std::os::raw::c_int,
            beta: *const f64,
            yarray: *const *mut f64,
            incy: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemvBatched_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f64, Aarray: *const *const f64, lda: i64, xarray: *const *const f64, incx: i64, beta: *const f64, yarray: *const *mut f64, incy: i64, batchCount: i64) -> cublasStatus_t>,
    pub cublasCgemvBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            Aarray: *const *const cuComplex,
            lda: ::std::os::raw::c_int,
            xarray: *const *const cuComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuComplex,
            yarray: *const *mut cuComplex,
            incy: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemvBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: i64,
            n: i64,
            alpha: *const cuComplex,
            Aarray: *const *const cuComplex,
            lda: i64,
            xarray: *const *const cuComplex,
            incx: i64,
            beta: *const cuComplex,
            yarray: *const *mut cuComplex,
            incy: i64,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemvBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            Aarray: *const *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            xarray: *const *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            yarray: *const *mut cuDoubleComplex,
            incy: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemvBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: i64,
            n: i64,
            alpha: *const cuDoubleComplex,
            Aarray: *const *const cuDoubleComplex,
            lda: i64,
            xarray: *const *const cuDoubleComplex,
            incx: i64,
            beta: *const cuDoubleComplex,
            yarray: *const *mut cuDoubleComplex,
            incy: i64,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemvStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            x: *const f32,
            incx: ::std::os::raw::c_int,
            stridex: ::std::os::raw::c_longlong,
            beta: *const f32,
            y: *mut f32,
            incy: ::std::os::raw::c_int,
            stridey: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemvStridedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: i64,
            n: i64,
            alpha: *const f32,
            A: *const f32,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            x: *const f32,
            incx: i64,
            stridex: ::std::os::raw::c_longlong,
            beta: *const f32,
            y: *mut f32,
            incy: i64,
            stridey: ::std::os::raw::c_longlong,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemvStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            x: *const f64,
            incx: ::std::os::raw::c_int,
            stridex: ::std::os::raw::c_longlong,
            beta: *const f64,
            y: *mut f64,
            incy: ::std::os::raw::c_int,
            stridey: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemvStridedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: i64,
            n: i64,
            alpha: *const f64,
            A: *const f64,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            x: *const f64,
            incx: i64,
            stridex: ::std::os::raw::c_longlong,
            beta: *const f64,
            y: *mut f64,
            incy: i64,
            stridey: ::std::os::raw::c_longlong,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemvStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            x: *const cuComplex,
            incx: ::std::os::raw::c_int,
            stridex: ::std::os::raw::c_longlong,
            beta: *const cuComplex,
            y: *mut cuComplex,
            incy: ::std::os::raw::c_int,
            stridey: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemvStridedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: i64,
            n: i64,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            x: *const cuComplex,
            incx: i64,
            stridex: ::std::os::raw::c_longlong,
            beta: *const cuComplex,
            y: *mut cuComplex,
            incy: i64,
            stridey: ::std::os::raw::c_longlong,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemvStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            stridex: ::std::os::raw::c_longlong,
            beta: *const cuDoubleComplex,
            y: *mut cuDoubleComplex,
            incy: ::std::os::raw::c_int,
            stridey: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemvStridedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: i64,
            n: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            x: *const cuDoubleComplex,
            incx: i64,
            stridex: ::std::os::raw::c_longlong,
            beta: *const cuDoubleComplex,
            y: *mut cuDoubleComplex,
            incy: i64,
            stridey: ::std::os::raw::c_longlong,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut f32,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemm_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    pub cublasDgemm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            beta: *const f64,
            C: *mut f64,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemm_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    pub cublasCgemm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemm_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasCgemm3m: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemm3m_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasCgemm3mEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemm3mEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuComplex,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: i64,
            beta: *const cuComplex,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemm_v2_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            B: *const cuDoubleComplex,
            ldb: i64,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemm3m: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemm3m_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            B: *const cuDoubleComplex,
            ldb: i64,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemmEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemmEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const f32,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: i64,
            beta: *const f32,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasGemmEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const ::std::os::raw::c_void,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: ::std::os::raw::c_int,
            beta: *const ::std::os::raw::c_void,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
            computeType: cublasComputeType_t,
            algo: cublasGemmAlgo_t,
        ) -> cublasStatus_t,
    >,
    pub cublasGemmEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const ::std::os::raw::c_void,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: i64,
            beta: *const ::std::os::raw::c_void,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
            computeType: cublasComputeType_t,
            algo: cublasGemmAlgo_t,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemmEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemmEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuComplex,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: i64,
            beta: *const cuComplex,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasSsyrk_v2: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, alpha: *const f32, A: *const f32, lda: ::std::os::raw::c_int, beta: *const f32, C: *mut f32, ldc: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasSsyrk_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    pub cublasDsyrk_v2: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, alpha: *const f64, A: *const f64, lda: ::std::os::raw::c_int, beta: *const f64, C: *mut f64, ldc: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasDsyrk_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    pub cublasCsyrk_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCsyrk_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZsyrk_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZsyrk_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, beta: *const cuDoubleComplex, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasCsyrkEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCsyrkEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: i64,
            k: i64,
            alpha: *const cuComplex,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            beta: *const cuComplex,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCsyrk3mEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCsyrk3mEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: i64,
            k: i64,
            alpha: *const cuComplex,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            beta: *const cuComplex,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCherk_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCherk_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const cuComplex, lda: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZherk_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            beta: *const f64,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZherk_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const cuDoubleComplex, lda: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasCherkEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCherkEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: i64,
            k: i64,
            alpha: *const f32,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            beta: *const f32,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCherk3mEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCherk3mEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: i64,
            k: i64,
            alpha: *const f32,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            beta: *const f32,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasSsyr2k_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut f32,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSsyr2k_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    pub cublasDsyr2k_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            beta: *const f64,
            C: *mut f64,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDsyr2k_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    pub cublasCsyr2k_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCsyr2k_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZsyr2k_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZsyr2k_v2_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: i64,
            k: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            B: *const cuDoubleComplex,
            ldb: i64,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCher2k_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCher2k_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZher2k_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const f64,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZher2k_v2_64: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, B: *const cuDoubleComplex, ldb: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t,
    >,
    pub cublasSsyrkx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut f32,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSsyrkx_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    pub cublasDsyrkx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            beta: *const f64,
            C: *mut f64,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDsyrkx_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    pub cublasCsyrkx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCsyrkx_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZsyrkx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZsyrkx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: i64,
            k: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            B: *const cuDoubleComplex,
            ldb: i64,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCherkx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCherkx_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZherkx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const f64,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZherkx_64: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, B: *const cuDoubleComplex, ldb: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t,
    >,
    pub cublasSsymm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            beta: *const f32,
            C: *mut f32,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSsymm_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    pub cublasDsymm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            beta: *const f64,
            C: *mut f64,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDsymm_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    pub cublasCsymm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCsymm_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZsymm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZsymm_v2_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            m: i64,
            n: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            B: *const cuDoubleComplex,
            ldb: i64,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasChemm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasChemm_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZhemm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZhemm_v2_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            m: i64,
            n: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            B: *const cuDoubleComplex,
            ldb: i64,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasStrsm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            B: *mut f32,
            ldb: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasStrsm_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *mut f32, ldb: i64) -> cublasStatus_t>,
    pub cublasDtrsm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            B: *mut f64,
            ldb: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDtrsm_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *mut f64, ldb: i64) -> cublasStatus_t>,
    pub cublasCtrsm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *mut cuComplex,
            ldb: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCtrsm_v2_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *mut cuComplex, ldb: i64) -> cublasStatus_t>,
    pub cublasZtrsm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *mut cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZtrsm_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, B: *mut cuDoubleComplex, ldb: i64) -> cublasStatus_t>,
    pub cublasStrmm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            C: *mut f32,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasStrmm_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    pub cublasDtrmm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            C: *mut f64,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDtrmm_v2_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    pub cublasCtrmm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCtrmm_v2_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: i64,
            n: i64,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: i64,
            B: *const cuComplex,
            ldb: i64,
            C: *mut cuComplex,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasZtrmm_v2: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZtrmm_v2_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: i64,
            n: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            B: *const cuDoubleComplex,
            ldb: i64,
            C: *mut cuDoubleComplex,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemmBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            Aarray: *const *const f32,
            lda: ::std::os::raw::c_int,
            Barray: *const *const f32,
            ldb: ::std::os::raw::c_int,
            beta: *const f32,
            Carray: *const *mut f32,
            ldc: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemmBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const f32,
            Aarray: *const *const f32,
            lda: i64,
            Barray: *const *const f32,
            ldb: i64,
            beta: *const f32,
            Carray: *const *mut f32,
            ldc: i64,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemmBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f64,
            Aarray: *const *const f64,
            lda: ::std::os::raw::c_int,
            Barray: *const *const f64,
            ldb: ::std::os::raw::c_int,
            beta: *const f64,
            Carray: *const *mut f64,
            ldc: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemmBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const f64,
            Aarray: *const *const f64,
            lda: i64,
            Barray: *const *const f64,
            ldb: i64,
            beta: *const f64,
            Carray: *const *mut f64,
            ldc: i64,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemmBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            Aarray: *const *const cuComplex,
            lda: ::std::os::raw::c_int,
            Barray: *const *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            Carray: *const *mut cuComplex,
            ldc: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemmBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuComplex,
            Aarray: *const *const cuComplex,
            lda: i64,
            Barray: *const *const cuComplex,
            ldb: i64,
            beta: *const cuComplex,
            Carray: *const *mut cuComplex,
            ldc: i64,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemm3mBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            Aarray: *const *const cuComplex,
            lda: ::std::os::raw::c_int,
            Barray: *const *const cuComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuComplex,
            Carray: *const *mut cuComplex,
            ldc: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemm3mBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuComplex,
            Aarray: *const *const cuComplex,
            lda: i64,
            Barray: *const *const cuComplex,
            ldb: i64,
            beta: *const cuComplex,
            Carray: *const *mut cuComplex,
            ldc: i64,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemmBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            Aarray: *const *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            Barray: *const *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            Carray: *const *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemmBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuDoubleComplex,
            Aarray: *const *const cuDoubleComplex,
            lda: i64,
            Barray: *const *const cuDoubleComplex,
            ldb: i64,
            beta: *const cuDoubleComplex,
            Carray: *const *mut cuDoubleComplex,
            ldc: i64,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemmStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            strideB: ::std::os::raw::c_longlong,
            beta: *const f32,
            C: *mut f32,
            ldc: ::std::os::raw::c_int,
            strideC: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemmStridedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const f32,
            A: *const f32,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            B: *const f32,
            ldb: i64,
            strideB: ::std::os::raw::c_longlong,
            beta: *const f32,
            C: *mut f32,
            ldc: i64,
            strideC: ::std::os::raw::c_longlong,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemmStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            strideB: ::std::os::raw::c_longlong,
            beta: *const f64,
            C: *mut f64,
            ldc: ::std::os::raw::c_int,
            strideC: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemmStridedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const f64,
            A: *const f64,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            B: *const f64,
            ldb: i64,
            strideB: ::std::os::raw::c_longlong,
            beta: *const f64,
            C: *mut f64,
            ldc: i64,
            strideC: ::std::os::raw::c_longlong,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemmStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            strideB: ::std::os::raw::c_longlong,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
            strideC: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemmStridedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            B: *const cuComplex,
            ldb: i64,
            strideB: ::std::os::raw::c_longlong,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: i64,
            strideC: ::std::os::raw::c_longlong,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemm3mStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            strideB: ::std::os::raw::c_longlong,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
            strideC: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgemm3mStridedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            B: *const cuComplex,
            ldb: i64,
            strideB: ::std::os::raw::c_longlong,
            beta: *const cuComplex,
            C: *mut cuComplex,
            ldc: i64,
            strideC: ::std::os::raw::c_longlong,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemmStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            strideB: ::std::os::raw::c_longlong,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
            strideC: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgemmStridedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            B: *const cuDoubleComplex,
            ldb: i64,
            strideB: ::std::os::raw::c_longlong,
            beta: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: i64,
            strideC: ::std::os::raw::c_longlong,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasGemmBatchedEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const ::std::os::raw::c_void,
            Aarray: *const *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            Barray: *const *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: ::std::os::raw::c_int,
            beta: *const ::std::os::raw::c_void,
            Carray: *const *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
            computeType: cublasComputeType_t,
            algo: cublasGemmAlgo_t,
        ) -> cublasStatus_t,
    >,
    pub cublasGemmBatchedEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const ::std::os::raw::c_void,
            Aarray: *const *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            Barray: *const *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: i64,
            beta: *const ::std::os::raw::c_void,
            Carray: *const *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
            batchCount: i64,
            computeType: cublasComputeType_t,
            algo: cublasGemmAlgo_t,
        ) -> cublasStatus_t,
    >,
    pub cublasGemmStridedBatchedEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            alpha: *const ::std::os::raw::c_void,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: ::std::os::raw::c_int,
            strideB: ::std::os::raw::c_longlong,
            beta: *const ::std::os::raw::c_void,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: ::std::os::raw::c_int,
            strideC: ::std::os::raw::c_longlong,
            batchCount: ::std::os::raw::c_int,
            computeType: cublasComputeType_t,
            algo: cublasGemmAlgo_t,
        ) -> cublasStatus_t,
    >,
    pub cublasGemmStridedBatchedEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            k: i64,
            alpha: *const ::std::os::raw::c_void,
            A: *const ::std::os::raw::c_void,
            Atype: cudaDataType,
            lda: i64,
            strideA: ::std::os::raw::c_longlong,
            B: *const ::std::os::raw::c_void,
            Btype: cudaDataType,
            ldb: i64,
            strideB: ::std::os::raw::c_longlong,
            beta: *const ::std::os::raw::c_void,
            C: *mut ::std::os::raw::c_void,
            Ctype: cudaDataType,
            ldc: i64,
            strideC: ::std::os::raw::c_longlong,
            batchCount: i64,
            computeType: cublasComputeType_t,
            algo: cublasGemmAlgo_t,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemmGroupedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa_array: *const cublasOperation_t,
            transb_array: *const cublasOperation_t,
            m_array: *const ::std::os::raw::c_int,
            n_array: *const ::std::os::raw::c_int,
            k_array: *const ::std::os::raw::c_int,
            alpha_array: *const f32,
            Aarray: *const *const f32,
            lda_array: *const ::std::os::raw::c_int,
            Barray: *const *const f32,
            ldb_array: *const ::std::os::raw::c_int,
            beta_array: *const f32,
            Carray: *const *mut f32,
            ldc_array: *const ::std::os::raw::c_int,
            group_count: ::std::os::raw::c_int,
            group_size: *const ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgemmGroupedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa_array: *const cublasOperation_t,
            transb_array: *const cublasOperation_t,
            m_array: *const i64,
            n_array: *const i64,
            k_array: *const i64,
            alpha_array: *const f32,
            Aarray: *const *const f32,
            lda_array: *const i64,
            Barray: *const *const f32,
            ldb_array: *const i64,
            beta_array: *const f32,
            Carray: *const *mut f32,
            ldc_array: *const i64,
            group_count: i64,
            group_size: *const i64,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemmGroupedBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa_array: *const cublasOperation_t,
            transb_array: *const cublasOperation_t,
            m_array: *const ::std::os::raw::c_int,
            n_array: *const ::std::os::raw::c_int,
            k_array: *const ::std::os::raw::c_int,
            alpha_array: *const f64,
            Aarray: *const *const f64,
            lda_array: *const ::std::os::raw::c_int,
            Barray: *const *const f64,
            ldb_array: *const ::std::os::raw::c_int,
            beta_array: *const f64,
            Carray: *const *mut f64,
            ldc_array: *const ::std::os::raw::c_int,
            group_count: ::std::os::raw::c_int,
            group_size: *const ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgemmGroupedBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa_array: *const cublasOperation_t,
            transb_array: *const cublasOperation_t,
            m_array: *const i64,
            n_array: *const i64,
            k_array: *const i64,
            alpha_array: *const f64,
            Aarray: *const *const f64,
            lda_array: *const i64,
            Barray: *const *const f64,
            ldb_array: *const i64,
            beta_array: *const f64,
            Carray: *const *mut f64,
            ldc_array: *const i64,
            group_count: i64,
            group_size: *const i64,
        ) -> cublasStatus_t,
    >,
    pub cublasGemmGroupedBatchedEx: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa_array: *const cublasOperation_t,
            transb_array: *const cublasOperation_t,
            m_array: *const ::std::os::raw::c_int,
            n_array: *const ::std::os::raw::c_int,
            k_array: *const ::std::os::raw::c_int,
            alpha_array: *const ::std::os::raw::c_void,
            Aarray: *const *const ::std::os::raw::c_void,
            Atype: cudaDataType_t,
            lda_array: *const ::std::os::raw::c_int,
            Barray: *const *const ::std::os::raw::c_void,
            Btype: cudaDataType_t,
            ldb_array: *const ::std::os::raw::c_int,
            beta_array: *const ::std::os::raw::c_void,
            Carray: *const *mut ::std::os::raw::c_void,
            Ctype: cudaDataType_t,
            ldc_array: *const ::std::os::raw::c_int,
            group_count: ::std::os::raw::c_int,
            group_size: *const ::std::os::raw::c_int,
            computeType: cublasComputeType_t,
        ) -> cublasStatus_t,
    >,
    pub cublasGemmGroupedBatchedEx_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa_array: *const cublasOperation_t,
            transb_array: *const cublasOperation_t,
            m_array: *const i64,
            n_array: *const i64,
            k_array: *const i64,
            alpha_array: *const ::std::os::raw::c_void,
            Aarray: *const *const ::std::os::raw::c_void,
            Atype: cudaDataType_t,
            lda_array: *const i64,
            Barray: *const *const ::std::os::raw::c_void,
            Btype: cudaDataType_t,
            ldb_array: *const i64,
            beta_array: *const ::std::os::raw::c_void,
            Carray: *const *mut ::std::os::raw::c_void,
            Ctype: cudaDataType_t,
            ldc_array: *const i64,
            group_count: i64,
            group_size: *const i64,
            computeType: cublasComputeType_t,
        ) -> cublasStatus_t,
    >,
    pub cublasSgeam: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            beta: *const f32,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            C: *mut f32,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgeam_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, beta: *const f32, B: *const f32, ldb: i64, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    pub cublasDgeam: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            beta: *const f64,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            C: *mut f64,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgeam_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, beta: *const f64, B: *const f64, ldb: i64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    pub cublasCgeam: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            beta: *const cuComplex,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgeam_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, beta: *const cuComplex, B: *const cuComplex, ldb: i64, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZgeam: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            beta: *const cuDoubleComplex,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgeam_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            m: i64,
            n: i64,
            alpha: *const cuDoubleComplex,
            A: *const cuDoubleComplex,
            lda: i64,
            beta: *const cuDoubleComplex,
            B: *const cuDoubleComplex,
            ldb: i64,
            C: *mut cuDoubleComplex,
            ldc: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasStrsmBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f32,
            A: *const *const f32,
            lda: ::std::os::raw::c_int,
            B: *const *mut f32,
            ldb: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasStrsmBatched_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const *const f32, lda: i64, B: *const *mut f32, ldb: i64, batchCount: i64) -> cublasStatus_t>,
    pub cublasDtrsmBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const f64,
            A: *const *const f64,
            lda: ::std::os::raw::c_int,
            B: *const *mut f64,
            ldb: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDtrsmBatched_64:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const *const f64, lda: i64, B: *const *mut f64, ldb: i64, batchCount: i64) -> cublasStatus_t>,
    pub cublasCtrsmBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuComplex,
            A: *const *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const *mut cuComplex,
            ldb: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCtrsmBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: i64,
            n: i64,
            alpha: *const cuComplex,
            A: *const *const cuComplex,
            lda: i64,
            B: *const *mut cuComplex,
            ldb: i64,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasZtrsmBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            alpha: *const cuDoubleComplex,
            A: *const *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const *mut cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            batchCount: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZtrsmBatched_64: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            diag: cublasDiagType_t,
            m: i64,
            n: i64,
            alpha: *const cuDoubleComplex,
            A: *const *const cuDoubleComplex,
            lda: i64,
            B: *const *mut cuDoubleComplex,
            ldb: i64,
            batchCount: i64,
        ) -> cublasStatus_t,
    >,
    pub cublasSdgmm: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, C: *mut f32, ldc: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSdgmm_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const f32, lda: i64, x: *const f32, incx: i64, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    pub cublasDdgmm: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, C: *mut f64, ldc: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDdgmm_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const f64, lda: i64, x: *const f64, incx: i64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    pub cublasCdgmm: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, C: *mut cuComplex, ldc: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasCdgmm_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasZdgmm: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            mode: cublasSideMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            x: *const cuDoubleComplex,
            incx: ::std::os::raw::c_int,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZdgmm_64: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t>,
    pub cublasSmatinvBatched: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f32, lda: ::std::os::raw::c_int, Ainv: *const *mut f32, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDmatinvBatched: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f64, lda: ::std::os::raw::c_int, Ainv: *const *mut f64, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCmatinvBatched:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const cuComplex, lda: ::std::os::raw::c_int, Ainv: *const *mut cuComplex, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZmatinvBatched: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const cuDoubleComplex, lda: ::std::os::raw::c_int, Ainv: *const *mut cuDoubleComplex, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasSgeqrfBatched: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut f32, lda: ::std::os::raw::c_int, TauArray: *const *mut f32, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDgeqrfBatched: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut f64, lda: ::std::os::raw::c_int, TauArray: *const *mut f64, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCgeqrfBatched:
        Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut cuComplex, lda: ::std::os::raw::c_int, TauArray: *const *mut cuComplex, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZgeqrfBatched: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut cuDoubleComplex, lda: ::std::os::raw::c_int, TauArray: *const *mut cuDoubleComplex, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasSgelsBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            Aarray: *const *mut f32,
            lda: ::std::os::raw::c_int,
            Carray: *const *mut f32,
            ldc: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            devInfoArray: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgelsBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            Aarray: *const *mut f64,
            lda: ::std::os::raw::c_int,
            Carray: *const *mut f64,
            ldc: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            devInfoArray: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgelsBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            Aarray: *const *mut cuComplex,
            lda: ::std::os::raw::c_int,
            Carray: *const *mut cuComplex,
            ldc: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            devInfoArray: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgelsBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            Aarray: *const *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            Carray: *const *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            devInfoArray: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasStpttr: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const f32, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDtpttr: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const f64, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCtpttr: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const cuComplex, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZtpttr: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasStrttp: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t>,
    pub cublasDtrttp: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t>,
    pub cublasCtrttp: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t>,
    pub cublasZtrttp: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t>,
    pub cublasSgetrfBatched: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut f32, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasDgetrfBatched: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut f64, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasCgetrfBatched: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut cuComplex, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasZgetrfBatched: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut cuDoubleComplex, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    pub cublasSgetriBatched: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f32, lda: ::std::os::raw::c_int, P: *const ::std::os::raw::c_int, C: *const *mut f32, ldc: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasDgetriBatched: Option<
        unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f64, lda: ::std::os::raw::c_int, P: *const ::std::os::raw::c_int, C: *const *mut f64, ldc: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t,
    >,
    pub cublasCgetriBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: ::std::os::raw::c_int,
            A: *const *const cuComplex,
            lda: ::std::os::raw::c_int,
            P: *const ::std::os::raw::c_int,
            C: *const *mut cuComplex,
            ldc: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgetriBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            n: ::std::os::raw::c_int,
            A: *const *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            P: *const ::std::os::raw::c_int,
            C: *const *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasSgetrsBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            Aarray: *const *const f32,
            lda: ::std::os::raw::c_int,
            devIpiv: *const ::std::os::raw::c_int,
            Barray: *const *mut f32,
            ldb: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasDgetrsBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            Aarray: *const *const f64,
            lda: ::std::os::raw::c_int,
            devIpiv: *const ::std::os::raw::c_int,
            Barray: *const *mut f64,
            ldb: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasCgetrsBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            Aarray: *const *const cuComplex,
            lda: ::std::os::raw::c_int,
            devIpiv: *const ::std::os::raw::c_int,
            Barray: *const *mut cuComplex,
            ldb: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasZgetrsBatched: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            Aarray: *const *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            devIpiv: *const ::std::os::raw::c_int,
            Barray: *const *mut cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
    >,
    pub cublasUint8gemmBias: Option<
        unsafe extern "C" fn(
            handle: cublasHandle_t,
            transa: cublasOperation_t,
            transb: cublasOperation_t,
            transc: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const ::std::os::raw::c_uchar,
            A_bias: ::std::os::raw::c_int,
            lda: ::std::os::raw::c_int,
            B: *const ::std::os::raw::c_uchar,
            B_bias: ::std::os::raw::c_int,
            ldb: ::std::os::raw::c_int,
            C: *mut ::std::os::raw::c_uchar,
            C_bias: ::std::os::raw::c_int,
            ldc: ::std::os::raw::c_int,
            C_mult: ::std::os::raw::c_int,
            C_shift: ::std::os::raw::c_int,
        ) -> cublasStatus_t,
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
pub unsafe extern "C" fn cublasCreate_v2(handle: *mut cublasHandle_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCreate_v2 {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCreate_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDestroy_v2(handle: cublasHandle_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDestroy_v2 {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDestroy_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetVersion_v2(handle: cublasHandle_t, version: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetVersion_v2 {
        Some(____func) => unsafe { ____func(handle, version) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetVersion_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetProperty(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetProperty {
        Some(____func) => unsafe { ____func(type_, value) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetProperty"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetCudartVersion() -> usize {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetCudartVersion {
        Some(____func) => unsafe { ____func() },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetCudartVersion"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetWorkspace_v2(handle: cublasHandle_t, workspace: *mut ::std::os::raw::c_void, workspaceSizeInBytes: usize) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetWorkspace_v2 {
        Some(____func) => unsafe { ____func(handle, workspace, workspaceSizeInBytes) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetWorkspace_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetStream_v2(handle: cublasHandle_t, streamId: cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetStream_v2 {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetStream_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetStream_v2(handle: cublasHandle_t, streamId: *mut cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetStream_v2 {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetStream_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetPointerMode_v2(handle: cublasHandle_t, mode: *mut cublasPointerMode_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetPointerMode_v2 {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetPointerMode_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetPointerMode_v2(handle: cublasHandle_t, mode: cublasPointerMode_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetPointerMode_v2 {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetPointerMode_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetAtomicsMode(handle: cublasHandle_t, mode: *mut cublasAtomicsMode_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetAtomicsMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetAtomicsMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetAtomicsMode(handle: cublasHandle_t, mode: cublasAtomicsMode_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetAtomicsMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetAtomicsMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetMathMode(handle: cublasHandle_t, mode: *mut cublasMath_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetMathMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetMathMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetMathMode(handle: cublasHandle_t, mode: cublasMath_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetMathMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetMathMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetSmCountTarget(handle: cublasHandle_t, smCountTarget: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetSmCountTarget {
        Some(____func) => unsafe { ____func(handle, smCountTarget) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetSmCountTarget"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetSmCountTarget(handle: cublasHandle_t, smCountTarget: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetSmCountTarget {
        Some(____func) => unsafe { ____func(handle, smCountTarget) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetSmCountTarget"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetEmulationStrategy(handle: cublasHandle_t, emulationStrategy: *mut cublasEmulationStrategy_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetEmulationStrategy {
        Some(____func) => unsafe { ____func(handle, emulationStrategy) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGetEmulationStrategy"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetEmulationStrategy(handle: cublasHandle_t, emulationStrategy: cublasEmulationStrategy_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetEmulationStrategy {
        Some(____func) => unsafe { ____func(handle, emulationStrategy) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSetEmulationStrategy"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetEmulationSpecialValuesSupport(handle: cublasHandle_t, mask: *mut cudaEmulationSpecialValuesSupport) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetEmulationSpecialValuesSupport {
        Some(____func) => unsafe { ____func(handle, mask) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGetEmulationSpecialValuesSupport"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetEmulationSpecialValuesSupport(handle: cublasHandle_t, mask: cudaEmulationSpecialValuesSupport) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetEmulationSpecialValuesSupport {
        Some(____func) => unsafe { ____func(handle, mask) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSetEmulationSpecialValuesSupport"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetFixedPointEmulationMantissaControl(handle: cublasHandle_t, mantissaControl: *mut cudaEmulationMantissaControl) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetFixedPointEmulationMantissaControl {
        Some(____func) => unsafe { ____func(handle, mantissaControl) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGetFixedPointEmulationMantissaControl"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetFixedPointEmulationMantissaControl(handle: cublasHandle_t, mantissaControl: cudaEmulationMantissaControl) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetFixedPointEmulationMantissaControl {
        Some(____func) => unsafe { ____func(handle, mantissaControl) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSetFixedPointEmulationMantissaControl"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetFixedPointEmulationMaxMantissaBitCount(handle: cublasHandle_t, maxMantissaBitCount: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetFixedPointEmulationMaxMantissaBitCount {
        Some(____func) => unsafe { ____func(handle, maxMantissaBitCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGetFixedPointEmulationMaxMantissaBitCount"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetFixedPointEmulationMaxMantissaBitCount(handle: cublasHandle_t, maxMantissaBitCount: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetFixedPointEmulationMaxMantissaBitCount {
        Some(____func) => unsafe { ____func(handle, maxMantissaBitCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSetFixedPointEmulationMaxMantissaBitCount"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetFixedPointEmulationMantissaBitOffset(handle: cublasHandle_t, mantissaBitOffset: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetFixedPointEmulationMantissaBitOffset {
        Some(____func) => unsafe { ____func(handle, mantissaBitOffset) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGetFixedPointEmulationMantissaBitOffset"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetFixedPointEmulationMantissaBitOffset(handle: cublasHandle_t, mantissaBitOffset: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetFixedPointEmulationMantissaBitOffset {
        Some(____func) => unsafe { ____func(handle, mantissaBitOffset) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSetFixedPointEmulationMantissaBitOffset"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetFixedPointEmulationMantissaBitCountPointer(handle: cublasHandle_t, mantissaBitCount: *mut *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetFixedPointEmulationMantissaBitCountPointer {
        Some(____func) => unsafe { ____func(handle, mantissaBitCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGetFixedPointEmulationMantissaBitCountPointer"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetFixedPointEmulationMantissaBitCountPointer(handle: cublasHandle_t, mantissaBitCount: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetFixedPointEmulationMantissaBitCountPointer {
        Some(____func) => unsafe { ____func(handle, mantissaBitCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSetFixedPointEmulationMantissaBitCountPointer"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetStatusName(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetStatusName {
        Some(____func) => unsafe { ____func(status) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetStatusName"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetStatusString(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetStatusString {
        Some(____func) => unsafe { ____func(status) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetStatusString"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasLoggerConfigure(logIsOn: ::std::os::raw::c_int, logToStdOut: ::std::os::raw::c_int, logToStdErr: ::std::os::raw::c_int, logFileName: *const ::std::os::raw::c_char) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasLoggerConfigure {
        Some(____func) => unsafe { ____func(logIsOn, logToStdOut, logToStdErr, logFileName) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasLoggerConfigure"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetLoggerCallback(userCallback: cublasLogCallback) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetLoggerCallback {
        Some(____func) => unsafe { ____func(userCallback) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetLoggerCallback"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetLoggerCallback(userCallback: *mut cublasLogCallback) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetLoggerCallback {
        Some(____func) => unsafe { ____func(userCallback) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetLoggerCallback"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetVector(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, devicePtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetVector {
        Some(____func) => unsafe { ____func(n, elemSize, x, incx, devicePtr, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetVector"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetVector_64(n: i64, elemSize: i64, x: *const ::std::os::raw::c_void, incx: i64, devicePtr: *mut ::std::os::raw::c_void, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetVector_64 {
        Some(____func) => unsafe { ____func(n, elemSize, x, incx, devicePtr, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetVector_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetVector(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetVector {
        Some(____func) => unsafe { ____func(n, elemSize, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetVector"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetVector_64(n: i64, elemSize: i64, x: *const ::std::os::raw::c_void, incx: i64, y: *mut ::std::os::raw::c_void, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetVector_64 {
        Some(____func) => unsafe { ____func(n, elemSize, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetVector_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetMatrix(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetMatrix {
        Some(____func) => unsafe { ____func(rows, cols, elemSize, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetMatrix"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetMatrix_64(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetMatrix_64 {
        Some(____func) => unsafe { ____func(rows, cols, elemSize, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetMatrix_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetMatrix(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetMatrix {
        Some(____func) => unsafe { ____func(rows, cols, elemSize, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetMatrix"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetMatrix_64(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetMatrix_64 {
        Some(____func) => unsafe { ____func(rows, cols, elemSize, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetMatrix_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetVectorAsync(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, hostPtr: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, devicePtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetVectorAsync {
        Some(____func) => unsafe { ____func(n, elemSize, hostPtr, incx, devicePtr, incy, stream) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetVectorAsync"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetVectorAsync_64(n: i64, elemSize: i64, hostPtr: *const ::std::os::raw::c_void, incx: i64, devicePtr: *mut ::std::os::raw::c_void, incy: i64, stream: cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetVectorAsync_64 {
        Some(____func) => unsafe { ____func(n, elemSize, hostPtr, incx, devicePtr, incy, stream) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetVectorAsync_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetVectorAsync(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, devicePtr: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, hostPtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetVectorAsync {
        Some(____func) => unsafe { ____func(n, elemSize, devicePtr, incx, hostPtr, incy, stream) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetVectorAsync"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetVectorAsync_64(n: i64, elemSize: i64, devicePtr: *const ::std::os::raw::c_void, incx: i64, hostPtr: *mut ::std::os::raw::c_void, incy: i64, stream: cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetVectorAsync_64 {
        Some(____func) => unsafe { ____func(n, elemSize, devicePtr, incx, hostPtr, incy, stream) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetVectorAsync_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetMatrixAsync(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetMatrixAsync {
        Some(____func) => unsafe { ____func(rows, cols, elemSize, A, lda, B, ldb, stream) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetMatrixAsync"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSetMatrixAsync_64(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64, stream: cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSetMatrixAsync_64 {
        Some(____func) => unsafe { ____func(rows, cols, elemSize, A, lda, B, ldb, stream) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSetMatrixAsync_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetMatrixAsync(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetMatrixAsync {
        Some(____func) => unsafe { ____func(rows, cols, elemSize, A, lda, B, ldb, stream) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetMatrixAsync"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGetMatrixAsync_64(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64, stream: cudaStream_t) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGetMatrixAsync_64 {
        Some(____func) => unsafe { ____func(rows, cols, elemSize, A, lda, B, ldb, stream) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGetMatrixAsync_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasXerbla(srName: *const ::std::os::raw::c_char, info: ::std::os::raw::c_int) {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasXerbla {
        Some(____func) => unsafe { ____func(srName, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasXerbla"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasNrm2Ex(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executionType: cudaDataType) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasNrm2Ex {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, result, resultType, executionType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasNrm2Ex"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasNrm2Ex_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executionType: cudaDataType) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasNrm2Ex_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, result, resultType, executionType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasNrm2Ex_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSnrm2_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSnrm2_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSnrm2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSnrm2_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSnrm2_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSnrm2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDnrm2_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDnrm2_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDnrm2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDnrm2_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDnrm2_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDnrm2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasScnrm2_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasScnrm2_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasScnrm2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasScnrm2_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasScnrm2_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasScnrm2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDznrm2_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDznrm2_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDznrm2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDznrm2_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDznrm2_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDznrm2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDotEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *const ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDotEx {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy, result, resultType, executionType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDotEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDotEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *const ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDotEx_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy, result, resultType, executionType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDotEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDotcEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *const ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDotcEx {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy, result, resultType, executionType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDotcEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDotcEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *const ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDotcEx_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy, result, resultType, executionType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDotcEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSdot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSdot_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSdot_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSdot_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, y: *const f32, incy: i64, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSdot_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSdot_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDdot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDdot_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDdot_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDdot_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, y: *const f64, incy: i64, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDdot_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDdot_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCdotu_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, result: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCdotu_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCdotu_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCdotu_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, result: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCdotu_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCdotu_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCdotc_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, result: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCdotc_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCdotc_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCdotc_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, result: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCdotc_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCdotc_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdotu_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, result: *mut cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdotu_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdotu_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdotu_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, result: *mut cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdotu_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdotu_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdotc_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, result: *mut cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdotc_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdotc_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdotc_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, result: *mut cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdotc_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdotc_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasScalEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const ::std::os::raw::c_void, alphaType: cudaDataType, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, executionType: cudaDataType) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasScalEx {
        Some(____func) => unsafe { ____func(handle, n, alpha, alphaType, x, xType, incx, executionType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasScalEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasScalEx_64(handle: cublasHandle_t, n: i64, alpha: *const ::std::os::raw::c_void, alphaType: cudaDataType, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, executionType: cudaDataType) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasScalEx_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, alphaType, x, xType, incx, executionType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasScalEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSscal_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSscal_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSscal_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSscal_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDscal_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDscal_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDscal_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDscal_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCscal_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCscal_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCscal_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCscal_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsscal_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsscal_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *mut cuComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsscal_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsscal_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZscal_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZscal_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZscal_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZscal_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdscal_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdscal_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdscal_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdscal_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdscal_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdscal_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasAxpyEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const ::std::os::raw::c_void,
    alphaType: cudaDataType,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasAxpyEx {
        Some(____func) => unsafe { ____func(handle, n, alpha, alphaType, x, xType, incx, y, yType, incy, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasAxpyEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasAxpyEx_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const ::std::os::raw::c_void,
    alphaType: cudaDataType,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasAxpyEx_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, alphaType, x, xType, incx, y, yType, incy, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasAxpyEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSaxpy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSaxpy_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSaxpy_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSaxpy_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSaxpy_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSaxpy_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDaxpy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDaxpy_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDaxpy_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDaxpy_v2_64(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDaxpy_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDaxpy_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCaxpy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCaxpy_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCaxpy_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCaxpy_v2_64(handle: cublasHandle_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCaxpy_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCaxpy_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZaxpy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZaxpy_v2 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZaxpy_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZaxpy_v2_64(handle: cublasHandle_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZaxpy_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, alpha, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZaxpy_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCopyEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCopyEx {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCopyEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCopyEx_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCopyEx_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCopyEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasScopy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasScopy_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasScopy_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasScopy_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasScopy_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasScopy_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDcopy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDcopy_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDcopy_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDcopy_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDcopy_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDcopy_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCcopy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCcopy_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCcopy_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCcopy_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCcopy_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCcopy_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZcopy_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZcopy_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZcopy_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZcopy_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZcopy_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZcopy_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSswap_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSswap_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSswap_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSswap_v2_64(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSswap_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSswap_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDswap_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDswap_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDswap_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDswap_v2_64(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDswap_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDswap_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCswap_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCswap_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCswap_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCswap_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCswap_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCswap_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZswap_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZswap_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZswap_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZswap_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZswap_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZswap_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSwapEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSwapEx {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSwapEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSwapEx_64(handle: cublasHandle_t, n: i64, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSwapEx_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSwapEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIsamax_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIsamax_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIsamax_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIsamax_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIsamax_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIsamax_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIdamax_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIdamax_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIdamax_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIdamax_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIdamax_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIdamax_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIcamax_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIcamax_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIcamax_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIcamax_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIcamax_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIcamax_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIzamax_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIzamax_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIzamax_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIzamax_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIzamax_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIzamax_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIamaxEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIamaxEx {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIamaxEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIamaxEx_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIamaxEx_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIamaxEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIsamin_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIsamin_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIsamin_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIsamin_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIsamin_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIsamin_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIdamin_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIdamin_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIdamin_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIdamin_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIdamin_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIdamin_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIcamin_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIcamin_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIcamin_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIcamin_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIcamin_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIcamin_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIzamin_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIzamin_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIzamin_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIzamin_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIzamin_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIzamin_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIaminEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIaminEx {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIaminEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasIaminEx_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasIaminEx_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasIaminEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasAsumEx(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasAsumEx {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, result, resultType, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasAsumEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasAsumEx_64(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasAsumEx_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, result, resultType, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasAsumEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSasum_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSasum_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSasum_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSasum_v2_64(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSasum_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSasum_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDasum_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDasum_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDasum_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDasum_v2_64(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDasum_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDasum_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasScasum_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasScasum_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasScasum_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasScasum_v2_64(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasScasum_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasScasum_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDzasum_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDzasum_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDzasum_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDzasum_v2_64(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDzasum_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, result) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDzasum_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int, c: *const f32, s: *const f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSrot_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSrot_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64, c: *const f32, s: *const f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSrot_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSrot_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int, c: *const f64, s: *const f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDrot_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDrot_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64, c: *const f64, s: *const f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDrot_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDrot_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int, c: *const f32, s: *const cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCrot_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCrot_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64, c: *const f32, s: *const cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCrot_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCrot_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int, c: *const f32, s: *const f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsrot_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsrot_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64, c: *const f32, s: *const f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsrot_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsrot_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int, c: *const f64, s: *const cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZrot_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZrot_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64, c: *const f64, s: *const cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZrot_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZrot_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdrot_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int, c: *const f64, s: *const f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdrot_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdrot_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdrot_v2_64(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64, c: *const f64, s: *const f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdrot_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdrot_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasRotEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    c: *const ::std::os::raw::c_void,
    s: *const ::std::os::raw::c_void,
    csType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasRotEx {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy, c, s, csType, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasRotEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasRotEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    c: *const ::std::os::raw::c_void,
    s: *const ::std::os::raw::c_void,
    csType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasRotEx_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy, c, s, csType, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasRotEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSrotg_v2(handle: cublasHandle_t, a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSrotg_v2 {
        Some(____func) => unsafe { ____func(handle, a, b, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSrotg_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDrotg_v2(handle: cublasHandle_t, a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDrotg_v2 {
        Some(____func) => unsafe { ____func(handle, a, b, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDrotg_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCrotg_v2(handle: cublasHandle_t, a: *mut cuComplex, b: *mut cuComplex, c: *mut f32, s: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCrotg_v2 {
        Some(____func) => unsafe { ____func(handle, a, b, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCrotg_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZrotg_v2(handle: cublasHandle_t, a: *mut cuDoubleComplex, b: *mut cuDoubleComplex, c: *mut f64, s: *mut cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZrotg_v2 {
        Some(____func) => unsafe { ____func(handle, a, b, c, s) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZrotg_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasRotgEx(handle: cublasHandle_t, a: *mut ::std::os::raw::c_void, b: *mut ::std::os::raw::c_void, abType: cudaDataType, c: *mut ::std::os::raw::c_void, s: *mut ::std::os::raw::c_void, csType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasRotgEx {
        Some(____func) => unsafe { ____func(handle, a, b, abType, c, s, csType, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasRotgEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSrotm_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int, param: *const f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSrotm_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, param) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSrotm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSrotm_v2_64(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64, param: *const f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSrotm_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, param) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSrotm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDrotm_v2(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int, param: *const f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDrotm_v2 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, param) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDrotm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDrotm_v2_64(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64, param: *const f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDrotm_v2_64 {
        Some(____func) => unsafe { ____func(handle, n, x, incx, y, incy, param) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDrotm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasRotmEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    param: *const ::std::os::raw::c_void,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasRotmEx {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy, param, paramType, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasRotmEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasRotmEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    param: *const ::std::os::raw::c_void,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasRotmEx_64 {
        Some(____func) => unsafe { ____func(handle, n, x, xType, incx, y, yType, incy, param, paramType, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasRotmEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSrotmg_v2(handle: cublasHandle_t, d1: *mut f32, d2: *mut f32, x1: *mut f32, y1: *const f32, param: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSrotmg_v2 {
        Some(____func) => unsafe { ____func(handle, d1, d2, x1, y1, param) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSrotmg_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDrotmg_v2(handle: cublasHandle_t, d1: *mut f64, d2: *mut f64, x1: *mut f64, y1: *const f64, param: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDrotmg_v2 {
        Some(____func) => unsafe { ____func(handle, d1, d2, x1, y1, param) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDrotmg_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasRotmgEx(
    handle: cublasHandle_t,
    d1: *mut ::std::os::raw::c_void,
    d1Type: cudaDataType,
    d2: *mut ::std::os::raw::c_void,
    d2Type: cudaDataType,
    x1: *mut ::std::os::raw::c_void,
    x1Type: cudaDataType,
    y1: *const ::std::os::raw::c_void,
    y1Type: cudaDataType,
    param: *mut ::std::os::raw::c_void,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasRotmgEx {
        Some(____func) => unsafe { ____func(handle, d1, d1Type, d2, d2Type, x1, x1Type, y1, y1Type, param, paramType, executiontype) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasRotmgEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemv_v2 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemv_v2_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemv_v2 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgemv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemv_v2_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgemv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemv_v2 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemv_v2_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemv_v2 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemv_v2_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgbmv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kl: ::std::os::raw::c_int,
    ku: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgbmv_v2 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, kl, ku, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgbmv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, kl, ku, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgbmv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kl: ::std::os::raw::c_int,
    ku: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgbmv_v2 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, kl, ku, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgbmv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, kl, ku, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgbmv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kl: ::std::os::raw::c_int,
    ku: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgbmv_v2 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, kl, ku, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgbmv_v2_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, kl, ku, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgbmv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kl: ::std::os::raw::c_int,
    ku: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgbmv_v2 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, kl, ku, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgbmv_v2_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, kl, ku, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStbmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStbmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtbmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtbmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtbmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtbmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStpmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStpmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStpmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStpmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStpmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtpmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtpmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtpmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtpmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtpmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtpmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtpmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtpmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtpmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtpmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtpmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtpmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtpmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtpmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtpmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStpsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStpsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStpsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStpsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStpsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStpsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtpsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtpsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtpsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtpsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtpsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtpsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtpsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtpsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtpsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtpsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtpsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtpsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtpsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtpsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtpsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtpsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtpsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, AP, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtpsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStbsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStbsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStbsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStbsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStbsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStbsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtbsv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtbsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtbsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtbsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtbsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtbsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtbsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtbsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtbsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtbsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtbsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtbsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtbsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtbsv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtbsv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtbsv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtbsv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, diag, n, k, A, lda, x, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtbsv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsymv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, A: *const f32, lda: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, beta: *const f32, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsymv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsymv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsymv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsymv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsymv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsymv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, A: *const f64, lda: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, beta: *const f64, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsymv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsymv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsymv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsymv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsymv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsymv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsymv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsymv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsymv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsymv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsymv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsymv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsymv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsymv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsymv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsymv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsymv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChemv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChemv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChemv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChemv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChemv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChemv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhemv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhemv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhemv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhemv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhemv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhemv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsbmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, k, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, k, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsbmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, k, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, k, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChbmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, k, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, k, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhbmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, k, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhbmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhbmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhbmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, k, alpha, A, lda, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhbmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSspmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, AP: *const f32, x: *const f32, incx: ::std::os::raw::c_int, beta: *const f32, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSspmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, AP, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSspmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSspmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, AP: *const f32, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSspmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, AP, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSspmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDspmv_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, AP: *const f64, x: *const f64, incx: ::std::os::raw::c_int, beta: *const f64, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDspmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, AP, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDspmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDspmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, AP: *const f64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDspmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, AP, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDspmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChpmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    AP: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChpmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, AP, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChpmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, AP: *const cuComplex, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChpmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, AP, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChpmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhpmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    AP: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhpmv_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, AP, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhpmv_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhpmv_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, AP: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhpmv_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, AP, x, incx, beta, y, incy) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhpmv_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSger_v2(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSger_v2 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSger_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSger_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, A: *mut f32, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSger_v2_64 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSger_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDger_v2(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDger_v2 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDger_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDger_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, A: *mut f64, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDger_v2_64 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDger_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgeru_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgeru_v2 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgeru_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgeru_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgeru_v2_64 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgeru_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgerc_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgerc_v2 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgerc_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgerc_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgerc_v2_64 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgerc_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgeru_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgeru_v2 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgeru_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgeru_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgeru_v2_64 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgeru_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgerc_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgerc_v2 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgerc_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgerc_v2_64(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgerc_v2_64 {
        Some(____func) => unsafe { ____func(handle, m, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgerc_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyr_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyr_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, A: *mut f32, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyr_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyr_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyr_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyr_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, A: *mut f64, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyr_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyr_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyr_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyr_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyr_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyr_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyr_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyr_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyr_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyr_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCher_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const cuComplex, incx: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCher_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCher_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCher_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const cuComplex, incx: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCher_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCher_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZher_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZher_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZher_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZher_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const cuDoubleComplex, incx: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZher_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZher_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSspr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSspr_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSspr_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSspr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, AP: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSspr_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSspr_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDspr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDspr_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDspr_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDspr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, AP: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDspr_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDspr_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChpr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const cuComplex, incx: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChpr_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChpr_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChpr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const cuComplex, incx: i64, AP: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChpr_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChpr_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhpr_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhpr_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhpr_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhpr_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const cuDoubleComplex, incx: i64, AP: *mut cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhpr_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhpr_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyr2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyr2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, A: *mut f32, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyr2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyr2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyr2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyr2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, A: *mut f64, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyr2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyr2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyr2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyr2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyr2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyr2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyr2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyr2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyr2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyr2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCher2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCher2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCher2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCher2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCher2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCher2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZher2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZher2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZher2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZher2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZher2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZher2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSspr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSspr2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSspr2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSspr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, AP: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSspr2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSspr2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDspr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDspr2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDspr2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDspr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, AP: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDspr2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDspr2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChpr2_v2(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChpr2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChpr2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChpr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, AP: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChpr2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChpr2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhpr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    AP: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhpr2_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhpr2_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhpr2_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, AP: *mut cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhpr2_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, n, alpha, x, incx, y, incy, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhpr2_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemvBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    Aarray: *const *const f32,
    lda: ::std::os::raw::c_int,
    xarray: *const *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    yarray: *const *mut f32,
    incy: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemvBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, Aarray, lda, xarray, incx, beta, yarray, incy, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemvBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemvBatched_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f32, Aarray: *const *const f32, lda: i64, xarray: *const *const f32, incx: i64, beta: *const f32, yarray: *const *mut f32, incy: i64, batchCount: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemvBatched_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, Aarray, lda, xarray, incx, beta, yarray, incy, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemvBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemvBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    Aarray: *const *const f64,
    lda: ::std::os::raw::c_int,
    xarray: *const *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    yarray: *const *mut f64,
    incy: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemvBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, Aarray, lda, xarray, incx, beta, yarray, incy, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgemvBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemvBatched_64(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f64, Aarray: *const *const f64, lda: i64, xarray: *const *const f64, incx: i64, beta: *const f64, yarray: *const *mut f64, incy: i64, batchCount: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemvBatched_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, Aarray, lda, xarray, incx, beta, yarray, incy, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgemvBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemvBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    xarray: *const *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    yarray: *const *mut cuComplex,
    incy: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemvBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, Aarray, lda, xarray, incx, beta, yarray, incy, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemvBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemvBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: i64,
    xarray: *const *const cuComplex,
    incx: i64,
    beta: *const cuComplex,
    yarray: *const *mut cuComplex,
    incy: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemvBatched_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, Aarray, lda, xarray, incx, beta, yarray, incy, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemvBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemvBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    Aarray: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    xarray: *const *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    yarray: *const *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemvBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, Aarray, lda, xarray, incx, beta, yarray, incy, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemvBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemvBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    Aarray: *const *const cuDoubleComplex,
    lda: i64,
    xarray: *const *const cuDoubleComplex,
    incx: i64,
    beta: *const cuDoubleComplex,
    yarray: *const *mut cuDoubleComplex,
    incy: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemvBatched_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, Aarray, lda, xarray, incx, beta, yarray, incy, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemvBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemvStridedBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    stridex: ::std::os::raw::c_longlong,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
    stridey: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemvStridedBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, strideA, x, incx, stridex, beta, y, incy, stridey, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSgemvStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemvStridedBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    x: *const f32,
    incx: i64,
    stridex: ::std::os::raw::c_longlong,
    beta: *const f32,
    y: *mut f32,
    incy: i64,
    stridey: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemvStridedBatched_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, strideA, x, incx, stridex, beta, y, incy, stridey, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSgemvStridedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemvStridedBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    stridex: ::std::os::raw::c_longlong,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
    stridey: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemvStridedBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, strideA, x, incx, stridex, beta, y, incy, stridey, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasDgemvStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemvStridedBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    x: *const f64,
    incx: i64,
    stridex: ::std::os::raw::c_longlong,
    beta: *const f64,
    y: *mut f64,
    incy: i64,
    stridey: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemvStridedBatched_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, strideA, x, incx, stridex, beta, y, incy, stridey, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasDgemvStridedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemvStridedBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    stridex: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
    stridey: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemvStridedBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, strideA, x, incx, stridex, beta, y, incy, stridey, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasCgemvStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemvStridedBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    x: *const cuComplex,
    incx: i64,
    stridex: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: i64,
    stridey: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemvStridedBatched_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, strideA, x, incx, stridex, beta, y, incy, stridey, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasCgemvStridedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemvStridedBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    stridex: ::std::os::raw::c_longlong,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    stridey: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemvStridedBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, strideA, x, incx, stridex, beta, y, incy, stridey, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasZgemvStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemvStridedBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    x: *const cuDoubleComplex,
    incx: i64,
    stridex: ::std::os::raw::c_longlong,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: i64,
    stridey: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemvStridedBatched_64 {
        Some(____func) => unsafe { ____func(handle, trans, m, n, alpha, A, lda, strideA, x, incx, stridex, beta, y, incy, stridey, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasZgemvStridedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemm_v2(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemm_v2 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemm_v2_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemm_v2_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemm_v2(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemm_v2 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgemm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemm_v2_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemm_v2_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgemm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm_v2(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm_v2 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm_v2_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm_v2_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm3m(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm3m {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemm3m"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm3m_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm3m_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemm3m_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm3mEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm3mEx {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, B, Btype, ldb, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemm3mEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm3mEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm3mEx_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, B, Btype, ldb, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemm3mEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemm_v2(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemm_v2 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemm_v2_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemm_v2_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemm3m(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemm3m {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemm3m"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemm3m_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemm3m_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemm3m_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemmEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemmEx {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, B, Btype, ldb, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemmEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemmEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemmEx_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, B, Btype, ldb, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemmEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGemmEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const ::std::os::raw::c_void,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGemmEx {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, B, Btype, ldb, beta, C, Ctype, ldc, computeType, algo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGemmEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGemmEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const ::std::os::raw::c_void,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGemmEx_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, B, Btype, ldb, beta, C, Ctype, ldc, computeType, algo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGemmEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemmEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemmEx {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, B, Btype, ldb, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemmEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemmEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemmEx_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, B, Btype, ldb, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemmEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyrk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyrk_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyrk_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyrk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyrk_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyrk_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyrk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyrk_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyrk_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyrk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyrk_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyrk_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyrk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyrk_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyrk_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyrk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyrk_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyrk_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyrk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyrk_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyrk_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyrk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, beta: *const cuDoubleComplex, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyrk_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyrk_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyrkEx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyrkEx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, Atype, lda, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyrkEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyrkEx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyrkEx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, Atype, lda, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyrkEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyrk3mEx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyrk3mEx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, Atype, lda, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyrk3mEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyrk3mEx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyrk3mEx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, Atype, lda, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyrk3mEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCherk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCherk_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCherk_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCherk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const cuComplex, lda: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCherk_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCherk_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZherk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZherk_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZherk_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZherk_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const cuDoubleComplex, lda: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZherk_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZherk_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCherkEx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCherkEx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, Atype, lda, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCherkEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCherkEx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCherkEx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, Atype, lda, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCherkEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCherk3mEx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCherk3mEx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, Atype, lda, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCherk3mEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCherk3mEx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCherk3mEx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, Atype, lda, beta, C, Ctype, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCherk3mEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyr2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyr2k_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyr2k_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyr2k_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyr2k_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyr2k_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyr2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyr2k_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyr2k_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyr2k_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyr2k_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyr2k_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyr2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyr2k_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyr2k_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyr2k_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyr2k_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyr2k_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyr2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyr2k_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyr2k_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyr2k_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyr2k_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyr2k_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCher2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCher2k_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCher2k_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCher2k_v2_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCher2k_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCher2k_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZher2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZher2k_v2 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZher2k_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZher2k_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZher2k_v2_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZher2k_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyrkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyrkx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyrkx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsyrkx_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsyrkx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsyrkx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyrkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyrkx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyrkx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsyrkx_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsyrkx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsyrkx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyrkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyrkx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyrkx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsyrkx_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsyrkx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsyrkx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyrkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyrkx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyrkx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsyrkx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsyrkx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsyrkx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCherkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCherkx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCherkx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCherkx_64(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCherkx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCherkx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZherkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZherkx {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZherkx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZherkx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZherkx_64 {
        Some(____func) => unsafe { ____func(handle, uplo, trans, n, k, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZherkx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsymm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsymm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsymm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSsymm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSsymm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSsymm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsymm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsymm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsymm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDsymm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDsymm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDsymm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsymm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsymm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsymm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCsymm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCsymm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCsymm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsymm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsymm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsymm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZsymm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZsymm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZsymm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChemm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChemm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChemm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasChemm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasChemm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasChemm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhemm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhemm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhemm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZhemm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZhemm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, m, n, alpha, A, lda, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZhemm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrsm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrsm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrsm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrsm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *mut f32, ldb: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrsm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrsm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrsm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrsm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrsm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrsm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *mut f64, ldb: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrsm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrsm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrsm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrsm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrsm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrsm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *mut cuComplex, ldb: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrsm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrsm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrsm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrsm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrsm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrsm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *mut cuDoubleComplex,
    ldb: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrsm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrsm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrmm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrmm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrmm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrmm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, C: *mut f32, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrmm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrmm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrmm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrmm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrmm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrmm_v2_64(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, C: *mut f64, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrmm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrmm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrmm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrmm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrmm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrmm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrmm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrmm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrmm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrmm_v2 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrmm_v2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrmm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrmm_v2_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrmm_v2_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemmBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    Aarray: *const *const f32,
    lda: ::std::os::raw::c_int,
    Barray: *const *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    Carray: *const *mut f32,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemmBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemmBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemmBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f32,
    Aarray: *const *const f32,
    lda: i64,
    Barray: *const *const f32,
    ldb: i64,
    beta: *const f32,
    Carray: *const *mut f32,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemmBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgemmBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemmBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    Aarray: *const *const f64,
    lda: ::std::os::raw::c_int,
    Barray: *const *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    Carray: *const *mut f64,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemmBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgemmBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemmBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f64,
    Aarray: *const *const f64,
    lda: i64,
    Barray: *const *const f64,
    ldb: i64,
    beta: *const f64,
    Carray: *const *mut f64,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemmBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgemmBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemmBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    Barray: *const *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    Carray: *const *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemmBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemmBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemmBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: i64,
    Barray: *const *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    Carray: *const *mut cuComplex,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemmBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemmBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm3mBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    Barray: *const *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    Carray: *const *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm3mBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemm3mBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm3mBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: i64,
    Barray: *const *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    Carray: *const *mut cuComplex,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm3mBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgemm3mBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemmBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    Aarray: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Barray: *const *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    Carray: *const *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemmBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemmBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemmBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    Aarray: *const *const cuDoubleComplex,
    lda: i64,
    Barray: *const *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    Carray: *const *mut cuDoubleComplex,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemmBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, lda, Barray, ldb, beta, Carray, ldc, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgemmBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemmStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemmStridedBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSgemmStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemmStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const f32,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const f32,
    C: *mut f32,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemmStridedBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSgemmStridedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemmStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemmStridedBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasDgemmStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemmStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const f64,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const f64,
    C: *mut f64,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemmStridedBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasDgemmStridedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemmStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemmStridedBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasCgemmStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemmStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuComplex,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemmStridedBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasCgemmStridedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm3mStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm3mStridedBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasCgemm3mStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgemm3mStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuComplex,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgemm3mStridedBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasCgemm3mStridedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemmStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemmStridedBatched {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasZgemmStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgemmStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuDoubleComplex,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgemmStridedBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, lda, strideA, B, ldb, strideB, beta, C, ldc, strideC, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasZgemmStridedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGemmBatchedEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const ::std::os::raw::c_void,
    Aarray: *const *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    Barray: *const *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const ::std::os::raw::c_void,
    Carray: *const *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGemmBatchedEx {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, Atype, lda, Barray, Btype, ldb, beta, Carray, Ctype, ldc, batchCount, computeType, algo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGemmBatchedEx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGemmBatchedEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const ::std::os::raw::c_void,
    Aarray: *const *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    Barray: *const *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const ::std::os::raw::c_void,
    Carray: *const *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
    batchCount: i64,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGemmBatchedEx_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, Aarray, Atype, lda, Barray, Btype, ldb, beta, Carray, Ctype, ldc, batchCount, computeType, algo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasGemmBatchedEx_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGemmStridedBatchedEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const ::std::os::raw::c_void,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGemmStridedBatchedEx {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, strideA, B, Btype, ldb, strideB, beta, C, Ctype, ldc, strideC, batchCount, computeType, algo) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGemmStridedBatchedEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGemmStridedBatchedEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const ::std::os::raw::c_void,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGemmStridedBatchedEx_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, k, alpha, A, Atype, lda, strideA, B, Btype, ldb, strideB, beta, C, Ctype, ldc, strideC, batchCount, computeType, algo) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGemmStridedBatchedEx_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemmGroupedBatched(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const ::std::os::raw::c_int,
    n_array: *const ::std::os::raw::c_int,
    k_array: *const ::std::os::raw::c_int,
    alpha_array: *const f32,
    Aarray: *const *const f32,
    lda_array: *const ::std::os::raw::c_int,
    Barray: *const *const f32,
    ldb_array: *const ::std::os::raw::c_int,
    beta_array: *const f32,
    Carray: *const *mut f32,
    ldc_array: *const ::std::os::raw::c_int,
    group_count: ::std::os::raw::c_int,
    group_size: *const ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemmGroupedBatched {
        Some(____func) => unsafe { ____func(handle, transa_array, transb_array, m_array, n_array, k_array, alpha_array, Aarray, lda_array, Barray, ldb_array, beta_array, Carray, ldc_array, group_count, group_size) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSgemmGroupedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgemmGroupedBatched_64(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const i64,
    n_array: *const i64,
    k_array: *const i64,
    alpha_array: *const f32,
    Aarray: *const *const f32,
    lda_array: *const i64,
    Barray: *const *const f32,
    ldb_array: *const i64,
    beta_array: *const f32,
    Carray: *const *mut f32,
    ldc_array: *const i64,
    group_count: i64,
    group_size: *const i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgemmGroupedBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa_array, transb_array, m_array, n_array, k_array, alpha_array, Aarray, lda_array, Barray, ldb_array, beta_array, Carray, ldc_array, group_count, group_size) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasSgemmGroupedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemmGroupedBatched(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const ::std::os::raw::c_int,
    n_array: *const ::std::os::raw::c_int,
    k_array: *const ::std::os::raw::c_int,
    alpha_array: *const f64,
    Aarray: *const *const f64,
    lda_array: *const ::std::os::raw::c_int,
    Barray: *const *const f64,
    ldb_array: *const ::std::os::raw::c_int,
    beta_array: *const f64,
    Carray: *const *mut f64,
    ldc_array: *const ::std::os::raw::c_int,
    group_count: ::std::os::raw::c_int,
    group_size: *const ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemmGroupedBatched {
        Some(____func) => unsafe { ____func(handle, transa_array, transb_array, m_array, n_array, k_array, alpha_array, Aarray, lda_array, Barray, ldb_array, beta_array, Carray, ldc_array, group_count, group_size) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasDgemmGroupedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgemmGroupedBatched_64(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const i64,
    n_array: *const i64,
    k_array: *const i64,
    alpha_array: *const f64,
    Aarray: *const *const f64,
    lda_array: *const i64,
    Barray: *const *const f64,
    ldb_array: *const i64,
    beta_array: *const f64,
    Carray: *const *mut f64,
    ldc_array: *const i64,
    group_count: i64,
    group_size: *const i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgemmGroupedBatched_64 {
        Some(____func) => unsafe { ____func(handle, transa_array, transb_array, m_array, n_array, k_array, alpha_array, Aarray, lda_array, Barray, ldb_array, beta_array, Carray, ldc_array, group_count, group_size) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasDgemmGroupedBatched_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGemmGroupedBatchedEx(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const ::std::os::raw::c_int,
    n_array: *const ::std::os::raw::c_int,
    k_array: *const ::std::os::raw::c_int,
    alpha_array: *const ::std::os::raw::c_void,
    Aarray: *const *const ::std::os::raw::c_void,
    Atype: cudaDataType_t,
    lda_array: *const ::std::os::raw::c_int,
    Barray: *const *const ::std::os::raw::c_void,
    Btype: cudaDataType_t,
    ldb_array: *const ::std::os::raw::c_int,
    beta_array: *const ::std::os::raw::c_void,
    Carray: *const *mut ::std::os::raw::c_void,
    Ctype: cudaDataType_t,
    ldc_array: *const ::std::os::raw::c_int,
    group_count: ::std::os::raw::c_int,
    group_size: *const ::std::os::raw::c_int,
    computeType: cublasComputeType_t,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGemmGroupedBatchedEx {
        Some(____func) => unsafe {
            ____func(
                handle,
                transa_array,
                transb_array,
                m_array,
                n_array,
                k_array,
                alpha_array,
                Aarray,
                Atype,
                lda_array,
                Barray,
                Btype,
                ldb_array,
                beta_array,
                Carray,
                Ctype,
                ldc_array,
                group_count,
                group_size,
                computeType,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGemmGroupedBatchedEx"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasGemmGroupedBatchedEx_64(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const i64,
    n_array: *const i64,
    k_array: *const i64,
    alpha_array: *const ::std::os::raw::c_void,
    Aarray: *const *const ::std::os::raw::c_void,
    Atype: cudaDataType_t,
    lda_array: *const i64,
    Barray: *const *const ::std::os::raw::c_void,
    Btype: cudaDataType_t,
    ldb_array: *const i64,
    beta_array: *const ::std::os::raw::c_void,
    Carray: *const *mut ::std::os::raw::c_void,
    Ctype: cudaDataType_t,
    ldc_array: *const i64,
    group_count: i64,
    group_size: *const i64,
    computeType: cublasComputeType_t,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasGemmGroupedBatchedEx_64 {
        Some(____func) => unsafe {
            ____func(
                handle,
                transa_array,
                transb_array,
                m_array,
                n_array,
                k_array,
                alpha_array,
                Aarray,
                Atype,
                lda_array,
                Barray,
                Btype,
                ldb_array,
                beta_array,
                Carray,
                Ctype,
                ldc_array,
                group_count,
                group_size,
                computeType,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cublasGemmGroupedBatchedEx_64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgeam(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgeam {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, alpha, A, lda, beta, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgeam"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgeam_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, beta: *const f32, B: *const f32, ldb: i64, C: *mut f32, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgeam_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, alpha, A, lda, beta, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgeam_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgeam(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    beta: *const f64,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgeam {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, alpha, A, lda, beta, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgeam"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgeam_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, beta: *const f64, B: *const f64, ldb: i64, C: *mut f64, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgeam_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, alpha, A, lda, beta, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgeam_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgeam(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    beta: *const cuComplex,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgeam {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, alpha, A, lda, beta, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgeam"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgeam_64(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, beta: *const cuComplex, B: *const cuComplex, ldb: i64, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgeam_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, alpha, A, lda, beta, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgeam_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgeam(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgeam {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, alpha, A, lda, beta, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgeam"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgeam_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    beta: *const cuDoubleComplex,
    B: *const cuDoubleComplex,
    ldb: i64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgeam_64 {
        Some(____func) => unsafe { ____func(handle, transa, transb, m, n, alpha, A, lda, beta, B, ldb, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgeam_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrsmBatched(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const *const f32,
    lda: ::std::os::raw::c_int,
    B: *const *mut f32,
    ldb: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrsmBatched {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrsmBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrsmBatched_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    A: *const *const f32,
    lda: i64,
    B: *const *mut f32,
    ldb: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrsmBatched_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrsmBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrsmBatched(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const *const f64,
    lda: ::std::os::raw::c_int,
    B: *const *mut f64,
    ldb: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrsmBatched {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrsmBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrsmBatched_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    A: *const *const f64,
    lda: i64,
    B: *const *mut f64,
    ldb: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrsmBatched_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrsmBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrsmBatched(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrsmBatched {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrsmBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrsmBatched_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const *const cuComplex,
    lda: i64,
    B: *const *mut cuComplex,
    ldb: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrsmBatched_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrsmBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrsmBatched(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrsmBatched {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrsmBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrsmBatched_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const *const cuDoubleComplex,
    lda: i64,
    B: *const *mut cuDoubleComplex,
    ldb: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrsmBatched_64 {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, diag, m, n, alpha, A, lda, B, ldb, batchCount) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrsmBatched_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSdgmm(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, C: *mut f32, ldc: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSdgmm {
        Some(____func) => unsafe { ____func(handle, mode, m, n, A, lda, x, incx, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSdgmm"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSdgmm_64(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const f32, lda: i64, x: *const f32, incx: i64, C: *mut f32, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSdgmm_64 {
        Some(____func) => unsafe { ____func(handle, mode, m, n, A, lda, x, incx, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSdgmm_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDdgmm(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, C: *mut f64, ldc: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDdgmm {
        Some(____func) => unsafe { ____func(handle, mode, m, n, A, lda, x, incx, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDdgmm"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDdgmm_64(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const f64, lda: i64, x: *const f64, incx: i64, C: *mut f64, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDdgmm_64 {
        Some(____func) => unsafe { ____func(handle, mode, m, n, A, lda, x, incx, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDdgmm_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCdgmm(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCdgmm {
        Some(____func) => unsafe { ____func(handle, mode, m, n, A, lda, x, incx, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCdgmm"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCdgmm_64(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, C: *mut cuComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCdgmm_64 {
        Some(____func) => unsafe { ____func(handle, mode, m, n, A, lda, x, incx, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCdgmm_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdgmm(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdgmm {
        Some(____func) => unsafe { ____func(handle, mode, m, n, A, lda, x, incx, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdgmm"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZdgmm_64(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZdgmm_64 {
        Some(____func) => unsafe { ____func(handle, mode, m, n, A, lda, x, incx, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZdgmm_64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSmatinvBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f32, lda: ::std::os::raw::c_int, Ainv: *const *mut f32, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSmatinvBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, Ainv, lda_inv, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSmatinvBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDmatinvBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f64, lda: ::std::os::raw::c_int, Ainv: *const *mut f64, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDmatinvBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, Ainv, lda_inv, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDmatinvBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCmatinvBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const cuComplex, lda: ::std::os::raw::c_int, Ainv: *const *mut cuComplex, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCmatinvBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, Ainv, lda_inv, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCmatinvBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZmatinvBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Ainv: *const *mut cuDoubleComplex,
    lda_inv: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZmatinvBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, Ainv, lda_inv, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZmatinvBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgeqrfBatched(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut f32, lda: ::std::os::raw::c_int, TauArray: *const *mut f32, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgeqrfBatched {
        Some(____func) => unsafe { ____func(handle, m, n, Aarray, lda, TauArray, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgeqrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgeqrfBatched(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut f64, lda: ::std::os::raw::c_int, TauArray: *const *mut f64, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgeqrfBatched {
        Some(____func) => unsafe { ____func(handle, m, n, Aarray, lda, TauArray, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgeqrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgeqrfBatched(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut cuComplex, lda: ::std::os::raw::c_int, TauArray: *const *mut cuComplex, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgeqrfBatched {
        Some(____func) => unsafe { ____func(handle, m, n, Aarray, lda, TauArray, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgeqrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgeqrfBatched(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    Aarray: *const *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    TauArray: *const *mut cuDoubleComplex,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgeqrfBatched {
        Some(____func) => unsafe { ____func(handle, m, n, Aarray, lda, TauArray, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgeqrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgelsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *mut f32,
    lda: ::std::os::raw::c_int,
    Carray: *const *mut f32,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    devInfoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgelsBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, nrhs, Aarray, lda, Carray, ldc, info, devInfoArray, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgelsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgelsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *mut f64,
    lda: ::std::os::raw::c_int,
    Carray: *const *mut f64,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    devInfoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgelsBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, nrhs, Aarray, lda, Carray, ldc, info, devInfoArray, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgelsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgelsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *mut cuComplex,
    lda: ::std::os::raw::c_int,
    Carray: *const *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    devInfoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgelsBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, nrhs, Aarray, lda, Carray, ldc, info, devInfoArray, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgelsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgelsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Carray: *const *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    devInfoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgelsBatched {
        Some(____func) => unsafe { ____func(handle, trans, m, n, nrhs, Aarray, lda, Carray, ldc, info, devInfoArray, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgelsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStpttr(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const f32, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStpttr {
        Some(____func) => unsafe { ____func(handle, uplo, n, AP, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStpttr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtpttr(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const f64, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtpttr {
        Some(____func) => unsafe { ____func(handle, uplo, n, AP, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtpttr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtpttr(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const cuComplex, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtpttr {
        Some(____func) => unsafe { ____func(handle, uplo, n, AP, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtpttr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtpttr(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtpttr {
        Some(____func) => unsafe { ____func(handle, uplo, n, AP, A, lda) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtpttr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasStrttp(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasStrttp {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasStrttp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDtrttp(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDtrttp {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDtrttp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCtrttp(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCtrttp {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCtrttp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZtrttp(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZtrttp {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, AP) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZtrttp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgetrfBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut f32, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgetrfBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, P, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgetrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgetrfBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut f64, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgetrfBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, P, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgetrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgetrfBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut cuComplex, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgetrfBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, P, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgetrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgetrfBatched(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut cuDoubleComplex, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgetrfBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, P, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgetrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgetriBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const f32,
    lda: ::std::os::raw::c_int,
    P: *const ::std::os::raw::c_int,
    C: *const *mut f32,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgetriBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, P, C, ldc, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgetriBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgetriBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const f64,
    lda: ::std::os::raw::c_int,
    P: *const ::std::os::raw::c_int,
    C: *const *mut f64,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgetriBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, P, C, ldc, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgetriBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgetriBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    P: *const ::std::os::raw::c_int,
    C: *const *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgetriBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, P, C, ldc, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgetriBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgetriBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    P: *const ::std::os::raw::c_int,
    C: *const *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgetriBatched {
        Some(____func) => unsafe { ____func(handle, n, A, lda, P, C, ldc, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgetriBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasSgetrsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *const f32,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    Barray: *const *mut f32,
    ldb: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasSgetrsBatched {
        Some(____func) => unsafe { ____func(handle, trans, n, nrhs, Aarray, lda, devIpiv, Barray, ldb, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasSgetrsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasDgetrsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *const f64,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    Barray: *const *mut f64,
    ldb: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasDgetrsBatched {
        Some(____func) => unsafe { ____func(handle, trans, n, nrhs, Aarray, lda, devIpiv, Barray, ldb, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasDgetrsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasCgetrsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    Barray: *const *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasCgetrsBatched {
        Some(____func) => unsafe { ____func(handle, trans, n, nrhs, Aarray, lda, devIpiv, Barray, ldb, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasCgetrsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasZgetrsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    Barray: *const *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasZgetrsBatched {
        Some(____func) => unsafe { ____func(handle, trans, n, nrhs, Aarray, lda, devIpiv, Barray, ldb, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasZgetrsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cublasUint8gemmBias(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    transc: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const ::std::os::raw::c_uchar,
    A_bias: ::std::os::raw::c_int,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_uchar,
    B_bias: ::std::os::raw::c_int,
    ldb: ::std::os::raw::c_int,
    C: *mut ::std::os::raw::c_uchar,
    C_bias: ::std::os::raw::c_int,
    ldc: ::std::os::raw::c_int,
    C_mult: ::std::os::raw::c_int,
    C_shift: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cublasUint8gemmBias {
        Some(____func) => unsafe { ____func(handle, transa, transb, transc, m, n, k, A, A_bias, lda, B, B_bias, ldb, C, C_bias, ldc, C_mult, C_shift) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cublasUint8gemmBias"),
    }
}
#[cfg(feature = "runtime-link")]
pub unsafe fn load_dynamic_bindings(lib: *mut std::ffi::c_void, get_proc_addr: unsafe fn(*mut std::ffi::c_void, *const u8) -> *mut std::ffi::c_void) {
    let bindings = unsafe {
        Box::new(DynamicBindings {
            cublasCreate_v2: {
                let p = get_proc_addr(lib, b"cublasCreate_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDestroy_v2: {
                let p = get_proc_addr(lib, b"cublasDestroy_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetVersion_v2: {
                let p = get_proc_addr(lib, b"cublasGetVersion_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetProperty: {
                let p = get_proc_addr(lib, b"cublasGetProperty\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetCudartVersion: {
                let p = get_proc_addr(lib, b"cublasGetCudartVersion\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetWorkspace_v2: {
                let p = get_proc_addr(lib, b"cublasSetWorkspace_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetStream_v2: {
                let p = get_proc_addr(lib, b"cublasSetStream_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetStream_v2: {
                let p = get_proc_addr(lib, b"cublasGetStream_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetPointerMode_v2: {
                let p = get_proc_addr(lib, b"cublasGetPointerMode_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetPointerMode_v2: {
                let p = get_proc_addr(lib, b"cublasSetPointerMode_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetAtomicsMode: {
                let p = get_proc_addr(lib, b"cublasGetAtomicsMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetAtomicsMode: {
                let p = get_proc_addr(lib, b"cublasSetAtomicsMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetMathMode: {
                let p = get_proc_addr(lib, b"cublasGetMathMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetMathMode: {
                let p = get_proc_addr(lib, b"cublasSetMathMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetSmCountTarget: {
                let p = get_proc_addr(lib, b"cublasGetSmCountTarget\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetSmCountTarget: {
                let p = get_proc_addr(lib, b"cublasSetSmCountTarget\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetEmulationStrategy: {
                let p = get_proc_addr(lib, b"cublasGetEmulationStrategy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetEmulationStrategy: {
                let p = get_proc_addr(lib, b"cublasSetEmulationStrategy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetEmulationSpecialValuesSupport: {
                let p = get_proc_addr(lib, b"cublasGetEmulationSpecialValuesSupport\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetEmulationSpecialValuesSupport: {
                let p = get_proc_addr(lib, b"cublasSetEmulationSpecialValuesSupport\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetFixedPointEmulationMantissaControl: {
                let p = get_proc_addr(lib, b"cublasGetFixedPointEmulationMantissaControl\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetFixedPointEmulationMantissaControl: {
                let p = get_proc_addr(lib, b"cublasSetFixedPointEmulationMantissaControl\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetFixedPointEmulationMaxMantissaBitCount: {
                let p = get_proc_addr(lib, b"cublasGetFixedPointEmulationMaxMantissaBitCount\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetFixedPointEmulationMaxMantissaBitCount: {
                let p = get_proc_addr(lib, b"cublasSetFixedPointEmulationMaxMantissaBitCount\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetFixedPointEmulationMantissaBitOffset: {
                let p = get_proc_addr(lib, b"cublasGetFixedPointEmulationMantissaBitOffset\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetFixedPointEmulationMantissaBitOffset: {
                let p = get_proc_addr(lib, b"cublasSetFixedPointEmulationMantissaBitOffset\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetFixedPointEmulationMantissaBitCountPointer: {
                let p = get_proc_addr(lib, b"cublasGetFixedPointEmulationMantissaBitCountPointer\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetFixedPointEmulationMantissaBitCountPointer: {
                let p = get_proc_addr(lib, b"cublasSetFixedPointEmulationMantissaBitCountPointer\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetStatusName: {
                let p = get_proc_addr(lib, b"cublasGetStatusName\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetStatusString: {
                let p = get_proc_addr(lib, b"cublasGetStatusString\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasLoggerConfigure: {
                let p = get_proc_addr(lib, b"cublasLoggerConfigure\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetLoggerCallback: {
                let p = get_proc_addr(lib, b"cublasSetLoggerCallback\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetLoggerCallback: {
                let p = get_proc_addr(lib, b"cublasGetLoggerCallback\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetVector: {
                let p = get_proc_addr(lib, b"cublasSetVector\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetVector_64: {
                let p = get_proc_addr(lib, b"cublasSetVector_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetVector: {
                let p = get_proc_addr(lib, b"cublasGetVector\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetVector_64: {
                let p = get_proc_addr(lib, b"cublasGetVector_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetMatrix: {
                let p = get_proc_addr(lib, b"cublasSetMatrix\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetMatrix_64: {
                let p = get_proc_addr(lib, b"cublasSetMatrix_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetMatrix: {
                let p = get_proc_addr(lib, b"cublasGetMatrix\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetMatrix_64: {
                let p = get_proc_addr(lib, b"cublasGetMatrix_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetVectorAsync: {
                let p = get_proc_addr(lib, b"cublasSetVectorAsync\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetVectorAsync_64: {
                let p = get_proc_addr(lib, b"cublasSetVectorAsync_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetVectorAsync: {
                let p = get_proc_addr(lib, b"cublasGetVectorAsync\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetVectorAsync_64: {
                let p = get_proc_addr(lib, b"cublasGetVectorAsync_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetMatrixAsync: {
                let p = get_proc_addr(lib, b"cublasSetMatrixAsync\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSetMatrixAsync_64: {
                let p = get_proc_addr(lib, b"cublasSetMatrixAsync_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetMatrixAsync: {
                let p = get_proc_addr(lib, b"cublasGetMatrixAsync\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGetMatrixAsync_64: {
                let p = get_proc_addr(lib, b"cublasGetMatrixAsync_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasXerbla: {
                let p = get_proc_addr(lib, b"cublasXerbla\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasNrm2Ex: {
                let p = get_proc_addr(lib, b"cublasNrm2Ex\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasNrm2Ex_64: {
                let p = get_proc_addr(lib, b"cublasNrm2Ex_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSnrm2_v2: {
                let p = get_proc_addr(lib, b"cublasSnrm2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSnrm2_v2_64: {
                let p = get_proc_addr(lib, b"cublasSnrm2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDnrm2_v2: {
                let p = get_proc_addr(lib, b"cublasDnrm2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDnrm2_v2_64: {
                let p = get_proc_addr(lib, b"cublasDnrm2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasScnrm2_v2: {
                let p = get_proc_addr(lib, b"cublasScnrm2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasScnrm2_v2_64: {
                let p = get_proc_addr(lib, b"cublasScnrm2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDznrm2_v2: {
                let p = get_proc_addr(lib, b"cublasDznrm2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDznrm2_v2_64: {
                let p = get_proc_addr(lib, b"cublasDznrm2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDotEx: {
                let p = get_proc_addr(lib, b"cublasDotEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDotEx_64: {
                let p = get_proc_addr(lib, b"cublasDotEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDotcEx: {
                let p = get_proc_addr(lib, b"cublasDotcEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDotcEx_64: {
                let p = get_proc_addr(lib, b"cublasDotcEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSdot_v2: {
                let p = get_proc_addr(lib, b"cublasSdot_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSdot_v2_64: {
                let p = get_proc_addr(lib, b"cublasSdot_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDdot_v2: {
                let p = get_proc_addr(lib, b"cublasDdot_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDdot_v2_64: {
                let p = get_proc_addr(lib, b"cublasDdot_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCdotu_v2: {
                let p = get_proc_addr(lib, b"cublasCdotu_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCdotu_v2_64: {
                let p = get_proc_addr(lib, b"cublasCdotu_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCdotc_v2: {
                let p = get_proc_addr(lib, b"cublasCdotc_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCdotc_v2_64: {
                let p = get_proc_addr(lib, b"cublasCdotc_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdotu_v2: {
                let p = get_proc_addr(lib, b"cublasZdotu_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdotu_v2_64: {
                let p = get_proc_addr(lib, b"cublasZdotu_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdotc_v2: {
                let p = get_proc_addr(lib, b"cublasZdotc_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdotc_v2_64: {
                let p = get_proc_addr(lib, b"cublasZdotc_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasScalEx: {
                let p = get_proc_addr(lib, b"cublasScalEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasScalEx_64: {
                let p = get_proc_addr(lib, b"cublasScalEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSscal_v2: {
                let p = get_proc_addr(lib, b"cublasSscal_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSscal_v2_64: {
                let p = get_proc_addr(lib, b"cublasSscal_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDscal_v2: {
                let p = get_proc_addr(lib, b"cublasDscal_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDscal_v2_64: {
                let p = get_proc_addr(lib, b"cublasDscal_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCscal_v2: {
                let p = get_proc_addr(lib, b"cublasCscal_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCscal_v2_64: {
                let p = get_proc_addr(lib, b"cublasCscal_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsscal_v2: {
                let p = get_proc_addr(lib, b"cublasCsscal_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsscal_v2_64: {
                let p = get_proc_addr(lib, b"cublasCsscal_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZscal_v2: {
                let p = get_proc_addr(lib, b"cublasZscal_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZscal_v2_64: {
                let p = get_proc_addr(lib, b"cublasZscal_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdscal_v2: {
                let p = get_proc_addr(lib, b"cublasZdscal_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdscal_v2_64: {
                let p = get_proc_addr(lib, b"cublasZdscal_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasAxpyEx: {
                let p = get_proc_addr(lib, b"cublasAxpyEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasAxpyEx_64: {
                let p = get_proc_addr(lib, b"cublasAxpyEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSaxpy_v2: {
                let p = get_proc_addr(lib, b"cublasSaxpy_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSaxpy_v2_64: {
                let p = get_proc_addr(lib, b"cublasSaxpy_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDaxpy_v2: {
                let p = get_proc_addr(lib, b"cublasDaxpy_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDaxpy_v2_64: {
                let p = get_proc_addr(lib, b"cublasDaxpy_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCaxpy_v2: {
                let p = get_proc_addr(lib, b"cublasCaxpy_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCaxpy_v2_64: {
                let p = get_proc_addr(lib, b"cublasCaxpy_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZaxpy_v2: {
                let p = get_proc_addr(lib, b"cublasZaxpy_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZaxpy_v2_64: {
                let p = get_proc_addr(lib, b"cublasZaxpy_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCopyEx: {
                let p = get_proc_addr(lib, b"cublasCopyEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCopyEx_64: {
                let p = get_proc_addr(lib, b"cublasCopyEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasScopy_v2: {
                let p = get_proc_addr(lib, b"cublasScopy_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasScopy_v2_64: {
                let p = get_proc_addr(lib, b"cublasScopy_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDcopy_v2: {
                let p = get_proc_addr(lib, b"cublasDcopy_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDcopy_v2_64: {
                let p = get_proc_addr(lib, b"cublasDcopy_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCcopy_v2: {
                let p = get_proc_addr(lib, b"cublasCcopy_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCcopy_v2_64: {
                let p = get_proc_addr(lib, b"cublasCcopy_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZcopy_v2: {
                let p = get_proc_addr(lib, b"cublasZcopy_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZcopy_v2_64: {
                let p = get_proc_addr(lib, b"cublasZcopy_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSswap_v2: {
                let p = get_proc_addr(lib, b"cublasSswap_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSswap_v2_64: {
                let p = get_proc_addr(lib, b"cublasSswap_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDswap_v2: {
                let p = get_proc_addr(lib, b"cublasDswap_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDswap_v2_64: {
                let p = get_proc_addr(lib, b"cublasDswap_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCswap_v2: {
                let p = get_proc_addr(lib, b"cublasCswap_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCswap_v2_64: {
                let p = get_proc_addr(lib, b"cublasCswap_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZswap_v2: {
                let p = get_proc_addr(lib, b"cublasZswap_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZswap_v2_64: {
                let p = get_proc_addr(lib, b"cublasZswap_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSwapEx: {
                let p = get_proc_addr(lib, b"cublasSwapEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSwapEx_64: {
                let p = get_proc_addr(lib, b"cublasSwapEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIsamax_v2: {
                let p = get_proc_addr(lib, b"cublasIsamax_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIsamax_v2_64: {
                let p = get_proc_addr(lib, b"cublasIsamax_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIdamax_v2: {
                let p = get_proc_addr(lib, b"cublasIdamax_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIdamax_v2_64: {
                let p = get_proc_addr(lib, b"cublasIdamax_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIcamax_v2: {
                let p = get_proc_addr(lib, b"cublasIcamax_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIcamax_v2_64: {
                let p = get_proc_addr(lib, b"cublasIcamax_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIzamax_v2: {
                let p = get_proc_addr(lib, b"cublasIzamax_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIzamax_v2_64: {
                let p = get_proc_addr(lib, b"cublasIzamax_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIamaxEx: {
                let p = get_proc_addr(lib, b"cublasIamaxEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIamaxEx_64: {
                let p = get_proc_addr(lib, b"cublasIamaxEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIsamin_v2: {
                let p = get_proc_addr(lib, b"cublasIsamin_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIsamin_v2_64: {
                let p = get_proc_addr(lib, b"cublasIsamin_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIdamin_v2: {
                let p = get_proc_addr(lib, b"cublasIdamin_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIdamin_v2_64: {
                let p = get_proc_addr(lib, b"cublasIdamin_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIcamin_v2: {
                let p = get_proc_addr(lib, b"cublasIcamin_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIcamin_v2_64: {
                let p = get_proc_addr(lib, b"cublasIcamin_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIzamin_v2: {
                let p = get_proc_addr(lib, b"cublasIzamin_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIzamin_v2_64: {
                let p = get_proc_addr(lib, b"cublasIzamin_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIaminEx: {
                let p = get_proc_addr(lib, b"cublasIaminEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasIaminEx_64: {
                let p = get_proc_addr(lib, b"cublasIaminEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasAsumEx: {
                let p = get_proc_addr(lib, b"cublasAsumEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasAsumEx_64: {
                let p = get_proc_addr(lib, b"cublasAsumEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSasum_v2: {
                let p = get_proc_addr(lib, b"cublasSasum_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSasum_v2_64: {
                let p = get_proc_addr(lib, b"cublasSasum_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDasum_v2: {
                let p = get_proc_addr(lib, b"cublasDasum_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDasum_v2_64: {
                let p = get_proc_addr(lib, b"cublasDasum_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasScasum_v2: {
                let p = get_proc_addr(lib, b"cublasScasum_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasScasum_v2_64: {
                let p = get_proc_addr(lib, b"cublasScasum_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDzasum_v2: {
                let p = get_proc_addr(lib, b"cublasDzasum_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDzasum_v2_64: {
                let p = get_proc_addr(lib, b"cublasDzasum_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSrot_v2: {
                let p = get_proc_addr(lib, b"cublasSrot_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSrot_v2_64: {
                let p = get_proc_addr(lib, b"cublasSrot_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDrot_v2: {
                let p = get_proc_addr(lib, b"cublasDrot_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDrot_v2_64: {
                let p = get_proc_addr(lib, b"cublasDrot_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCrot_v2: {
                let p = get_proc_addr(lib, b"cublasCrot_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCrot_v2_64: {
                let p = get_proc_addr(lib, b"cublasCrot_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsrot_v2: {
                let p = get_proc_addr(lib, b"cublasCsrot_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsrot_v2_64: {
                let p = get_proc_addr(lib, b"cublasCsrot_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZrot_v2: {
                let p = get_proc_addr(lib, b"cublasZrot_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZrot_v2_64: {
                let p = get_proc_addr(lib, b"cublasZrot_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdrot_v2: {
                let p = get_proc_addr(lib, b"cublasZdrot_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdrot_v2_64: {
                let p = get_proc_addr(lib, b"cublasZdrot_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasRotEx: {
                let p = get_proc_addr(lib, b"cublasRotEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasRotEx_64: {
                let p = get_proc_addr(lib, b"cublasRotEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSrotg_v2: {
                let p = get_proc_addr(lib, b"cublasSrotg_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDrotg_v2: {
                let p = get_proc_addr(lib, b"cublasDrotg_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCrotg_v2: {
                let p = get_proc_addr(lib, b"cublasCrotg_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZrotg_v2: {
                let p = get_proc_addr(lib, b"cublasZrotg_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasRotgEx: {
                let p = get_proc_addr(lib, b"cublasRotgEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSrotm_v2: {
                let p = get_proc_addr(lib, b"cublasSrotm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSrotm_v2_64: {
                let p = get_proc_addr(lib, b"cublasSrotm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDrotm_v2: {
                let p = get_proc_addr(lib, b"cublasDrotm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDrotm_v2_64: {
                let p = get_proc_addr(lib, b"cublasDrotm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasRotmEx: {
                let p = get_proc_addr(lib, b"cublasRotmEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasRotmEx_64: {
                let p = get_proc_addr(lib, b"cublasRotmEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSrotmg_v2: {
                let p = get_proc_addr(lib, b"cublasSrotmg_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDrotmg_v2: {
                let p = get_proc_addr(lib, b"cublasDrotmg_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasRotmgEx: {
                let p = get_proc_addr(lib, b"cublasRotmgEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemv_v2: {
                let p = get_proc_addr(lib, b"cublasSgemv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemv_v2_64: {
                let p = get_proc_addr(lib, b"cublasSgemv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemv_v2: {
                let p = get_proc_addr(lib, b"cublasDgemv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDgemv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemv_v2: {
                let p = get_proc_addr(lib, b"cublasCgemv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemv_v2_64: {
                let p = get_proc_addr(lib, b"cublasCgemv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemv_v2: {
                let p = get_proc_addr(lib, b"cublasZgemv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZgemv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgbmv_v2: {
                let p = get_proc_addr(lib, b"cublasSgbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasSgbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgbmv_v2: {
                let p = get_proc_addr(lib, b"cublasDgbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDgbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgbmv_v2: {
                let p = get_proc_addr(lib, b"cublasCgbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasCgbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgbmv_v2: {
                let p = get_proc_addr(lib, b"cublasZgbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZgbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrmv_v2: {
                let p = get_proc_addr(lib, b"cublasStrmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasStrmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrmv_v2: {
                let p = get_proc_addr(lib, b"cublasDtrmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDtrmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrmv_v2: {
                let p = get_proc_addr(lib, b"cublasCtrmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasCtrmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrmv_v2: {
                let p = get_proc_addr(lib, b"cublasZtrmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZtrmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStbmv_v2: {
                let p = get_proc_addr(lib, b"cublasStbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasStbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtbmv_v2: {
                let p = get_proc_addr(lib, b"cublasDtbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDtbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtbmv_v2: {
                let p = get_proc_addr(lib, b"cublasCtbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasCtbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtbmv_v2: {
                let p = get_proc_addr(lib, b"cublasZtbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZtbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStpmv_v2: {
                let p = get_proc_addr(lib, b"cublasStpmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStpmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasStpmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtpmv_v2: {
                let p = get_proc_addr(lib, b"cublasDtpmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtpmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDtpmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtpmv_v2: {
                let p = get_proc_addr(lib, b"cublasCtpmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtpmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasCtpmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtpmv_v2: {
                let p = get_proc_addr(lib, b"cublasZtpmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtpmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZtpmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrsv_v2: {
                let p = get_proc_addr(lib, b"cublasStrsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasStrsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrsv_v2: {
                let p = get_proc_addr(lib, b"cublasDtrsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDtrsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrsv_v2: {
                let p = get_proc_addr(lib, b"cublasCtrsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasCtrsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrsv_v2: {
                let p = get_proc_addr(lib, b"cublasZtrsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZtrsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStpsv_v2: {
                let p = get_proc_addr(lib, b"cublasStpsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStpsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasStpsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtpsv_v2: {
                let p = get_proc_addr(lib, b"cublasDtpsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtpsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDtpsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtpsv_v2: {
                let p = get_proc_addr(lib, b"cublasCtpsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtpsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasCtpsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtpsv_v2: {
                let p = get_proc_addr(lib, b"cublasZtpsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtpsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZtpsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStbsv_v2: {
                let p = get_proc_addr(lib, b"cublasStbsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStbsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasStbsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtbsv_v2: {
                let p = get_proc_addr(lib, b"cublasDtbsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtbsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDtbsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtbsv_v2: {
                let p = get_proc_addr(lib, b"cublasCtbsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtbsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasCtbsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtbsv_v2: {
                let p = get_proc_addr(lib, b"cublasZtbsv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtbsv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZtbsv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsymv_v2: {
                let p = get_proc_addr(lib, b"cublasSsymv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsymv_v2_64: {
                let p = get_proc_addr(lib, b"cublasSsymv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsymv_v2: {
                let p = get_proc_addr(lib, b"cublasDsymv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsymv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDsymv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsymv_v2: {
                let p = get_proc_addr(lib, b"cublasCsymv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsymv_v2_64: {
                let p = get_proc_addr(lib, b"cublasCsymv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsymv_v2: {
                let p = get_proc_addr(lib, b"cublasZsymv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsymv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZsymv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChemv_v2: {
                let p = get_proc_addr(lib, b"cublasChemv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChemv_v2_64: {
                let p = get_proc_addr(lib, b"cublasChemv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhemv_v2: {
                let p = get_proc_addr(lib, b"cublasZhemv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhemv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZhemv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsbmv_v2: {
                let p = get_proc_addr(lib, b"cublasSsbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasSsbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsbmv_v2: {
                let p = get_proc_addr(lib, b"cublasDsbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDsbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChbmv_v2: {
                let p = get_proc_addr(lib, b"cublasChbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasChbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhbmv_v2: {
                let p = get_proc_addr(lib, b"cublasZhbmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhbmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZhbmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSspmv_v2: {
                let p = get_proc_addr(lib, b"cublasSspmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSspmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasSspmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDspmv_v2: {
                let p = get_proc_addr(lib, b"cublasDspmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDspmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasDspmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChpmv_v2: {
                let p = get_proc_addr(lib, b"cublasChpmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChpmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasChpmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhpmv_v2: {
                let p = get_proc_addr(lib, b"cublasZhpmv_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhpmv_v2_64: {
                let p = get_proc_addr(lib, b"cublasZhpmv_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSger_v2: {
                let p = get_proc_addr(lib, b"cublasSger_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSger_v2_64: {
                let p = get_proc_addr(lib, b"cublasSger_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDger_v2: {
                let p = get_proc_addr(lib, b"cublasDger_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDger_v2_64: {
                let p = get_proc_addr(lib, b"cublasDger_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgeru_v2: {
                let p = get_proc_addr(lib, b"cublasCgeru_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgeru_v2_64: {
                let p = get_proc_addr(lib, b"cublasCgeru_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgerc_v2: {
                let p = get_proc_addr(lib, b"cublasCgerc_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgerc_v2_64: {
                let p = get_proc_addr(lib, b"cublasCgerc_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgeru_v2: {
                let p = get_proc_addr(lib, b"cublasZgeru_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgeru_v2_64: {
                let p = get_proc_addr(lib, b"cublasZgeru_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgerc_v2: {
                let p = get_proc_addr(lib, b"cublasZgerc_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgerc_v2_64: {
                let p = get_proc_addr(lib, b"cublasZgerc_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyr_v2: {
                let p = get_proc_addr(lib, b"cublasSsyr_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyr_v2_64: {
                let p = get_proc_addr(lib, b"cublasSsyr_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyr_v2: {
                let p = get_proc_addr(lib, b"cublasDsyr_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyr_v2_64: {
                let p = get_proc_addr(lib, b"cublasDsyr_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyr_v2: {
                let p = get_proc_addr(lib, b"cublasCsyr_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyr_v2_64: {
                let p = get_proc_addr(lib, b"cublasCsyr_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyr_v2: {
                let p = get_proc_addr(lib, b"cublasZsyr_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyr_v2_64: {
                let p = get_proc_addr(lib, b"cublasZsyr_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCher_v2: {
                let p = get_proc_addr(lib, b"cublasCher_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCher_v2_64: {
                let p = get_proc_addr(lib, b"cublasCher_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZher_v2: {
                let p = get_proc_addr(lib, b"cublasZher_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZher_v2_64: {
                let p = get_proc_addr(lib, b"cublasZher_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSspr_v2: {
                let p = get_proc_addr(lib, b"cublasSspr_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSspr_v2_64: {
                let p = get_proc_addr(lib, b"cublasSspr_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDspr_v2: {
                let p = get_proc_addr(lib, b"cublasDspr_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDspr_v2_64: {
                let p = get_proc_addr(lib, b"cublasDspr_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChpr_v2: {
                let p = get_proc_addr(lib, b"cublasChpr_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChpr_v2_64: {
                let p = get_proc_addr(lib, b"cublasChpr_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhpr_v2: {
                let p = get_proc_addr(lib, b"cublasZhpr_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhpr_v2_64: {
                let p = get_proc_addr(lib, b"cublasZhpr_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyr2_v2: {
                let p = get_proc_addr(lib, b"cublasSsyr2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyr2_v2_64: {
                let p = get_proc_addr(lib, b"cublasSsyr2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyr2_v2: {
                let p = get_proc_addr(lib, b"cublasDsyr2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyr2_v2_64: {
                let p = get_proc_addr(lib, b"cublasDsyr2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyr2_v2: {
                let p = get_proc_addr(lib, b"cublasCsyr2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyr2_v2_64: {
                let p = get_proc_addr(lib, b"cublasCsyr2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyr2_v2: {
                let p = get_proc_addr(lib, b"cublasZsyr2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyr2_v2_64: {
                let p = get_proc_addr(lib, b"cublasZsyr2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCher2_v2: {
                let p = get_proc_addr(lib, b"cublasCher2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCher2_v2_64: {
                let p = get_proc_addr(lib, b"cublasCher2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZher2_v2: {
                let p = get_proc_addr(lib, b"cublasZher2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZher2_v2_64: {
                let p = get_proc_addr(lib, b"cublasZher2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSspr2_v2: {
                let p = get_proc_addr(lib, b"cublasSspr2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSspr2_v2_64: {
                let p = get_proc_addr(lib, b"cublasSspr2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDspr2_v2: {
                let p = get_proc_addr(lib, b"cublasDspr2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDspr2_v2_64: {
                let p = get_proc_addr(lib, b"cublasDspr2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChpr2_v2: {
                let p = get_proc_addr(lib, b"cublasChpr2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChpr2_v2_64: {
                let p = get_proc_addr(lib, b"cublasChpr2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhpr2_v2: {
                let p = get_proc_addr(lib, b"cublasZhpr2_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhpr2_v2_64: {
                let p = get_proc_addr(lib, b"cublasZhpr2_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemvBatched: {
                let p = get_proc_addr(lib, b"cublasSgemvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemvBatched_64: {
                let p = get_proc_addr(lib, b"cublasSgemvBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemvBatched: {
                let p = get_proc_addr(lib, b"cublasDgemvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemvBatched_64: {
                let p = get_proc_addr(lib, b"cublasDgemvBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemvBatched: {
                let p = get_proc_addr(lib, b"cublasCgemvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemvBatched_64: {
                let p = get_proc_addr(lib, b"cublasCgemvBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemvBatched: {
                let p = get_proc_addr(lib, b"cublasZgemvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemvBatched_64: {
                let p = get_proc_addr(lib, b"cublasZgemvBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemvStridedBatched: {
                let p = get_proc_addr(lib, b"cublasSgemvStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemvStridedBatched_64: {
                let p = get_proc_addr(lib, b"cublasSgemvStridedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemvStridedBatched: {
                let p = get_proc_addr(lib, b"cublasDgemvStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemvStridedBatched_64: {
                let p = get_proc_addr(lib, b"cublasDgemvStridedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemvStridedBatched: {
                let p = get_proc_addr(lib, b"cublasCgemvStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemvStridedBatched_64: {
                let p = get_proc_addr(lib, b"cublasCgemvStridedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemvStridedBatched: {
                let p = get_proc_addr(lib, b"cublasZgemvStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemvStridedBatched_64: {
                let p = get_proc_addr(lib, b"cublasZgemvStridedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemm_v2: {
                let p = get_proc_addr(lib, b"cublasSgemm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemm_v2_64: {
                let p = get_proc_addr(lib, b"cublasSgemm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemm_v2: {
                let p = get_proc_addr(lib, b"cublasDgemm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemm_v2_64: {
                let p = get_proc_addr(lib, b"cublasDgemm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm_v2: {
                let p = get_proc_addr(lib, b"cublasCgemm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm_v2_64: {
                let p = get_proc_addr(lib, b"cublasCgemm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm3m: {
                let p = get_proc_addr(lib, b"cublasCgemm3m\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm3m_64: {
                let p = get_proc_addr(lib, b"cublasCgemm3m_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm3mEx: {
                let p = get_proc_addr(lib, b"cublasCgemm3mEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm3mEx_64: {
                let p = get_proc_addr(lib, b"cublasCgemm3mEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemm_v2: {
                let p = get_proc_addr(lib, b"cublasZgemm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemm_v2_64: {
                let p = get_proc_addr(lib, b"cublasZgemm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemm3m: {
                let p = get_proc_addr(lib, b"cublasZgemm3m\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemm3m_64: {
                let p = get_proc_addr(lib, b"cublasZgemm3m_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemmEx: {
                let p = get_proc_addr(lib, b"cublasSgemmEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemmEx_64: {
                let p = get_proc_addr(lib, b"cublasSgemmEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGemmEx: {
                let p = get_proc_addr(lib, b"cublasGemmEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGemmEx_64: {
                let p = get_proc_addr(lib, b"cublasGemmEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemmEx: {
                let p = get_proc_addr(lib, b"cublasCgemmEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemmEx_64: {
                let p = get_proc_addr(lib, b"cublasCgemmEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyrk_v2: {
                let p = get_proc_addr(lib, b"cublasSsyrk_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyrk_v2_64: {
                let p = get_proc_addr(lib, b"cublasSsyrk_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyrk_v2: {
                let p = get_proc_addr(lib, b"cublasDsyrk_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyrk_v2_64: {
                let p = get_proc_addr(lib, b"cublasDsyrk_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyrk_v2: {
                let p = get_proc_addr(lib, b"cublasCsyrk_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyrk_v2_64: {
                let p = get_proc_addr(lib, b"cublasCsyrk_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyrk_v2: {
                let p = get_proc_addr(lib, b"cublasZsyrk_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyrk_v2_64: {
                let p = get_proc_addr(lib, b"cublasZsyrk_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyrkEx: {
                let p = get_proc_addr(lib, b"cublasCsyrkEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyrkEx_64: {
                let p = get_proc_addr(lib, b"cublasCsyrkEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyrk3mEx: {
                let p = get_proc_addr(lib, b"cublasCsyrk3mEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyrk3mEx_64: {
                let p = get_proc_addr(lib, b"cublasCsyrk3mEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCherk_v2: {
                let p = get_proc_addr(lib, b"cublasCherk_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCherk_v2_64: {
                let p = get_proc_addr(lib, b"cublasCherk_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZherk_v2: {
                let p = get_proc_addr(lib, b"cublasZherk_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZherk_v2_64: {
                let p = get_proc_addr(lib, b"cublasZherk_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCherkEx: {
                let p = get_proc_addr(lib, b"cublasCherkEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCherkEx_64: {
                let p = get_proc_addr(lib, b"cublasCherkEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCherk3mEx: {
                let p = get_proc_addr(lib, b"cublasCherk3mEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCherk3mEx_64: {
                let p = get_proc_addr(lib, b"cublasCherk3mEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyr2k_v2: {
                let p = get_proc_addr(lib, b"cublasSsyr2k_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyr2k_v2_64: {
                let p = get_proc_addr(lib, b"cublasSsyr2k_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyr2k_v2: {
                let p = get_proc_addr(lib, b"cublasDsyr2k_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyr2k_v2_64: {
                let p = get_proc_addr(lib, b"cublasDsyr2k_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyr2k_v2: {
                let p = get_proc_addr(lib, b"cublasCsyr2k_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyr2k_v2_64: {
                let p = get_proc_addr(lib, b"cublasCsyr2k_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyr2k_v2: {
                let p = get_proc_addr(lib, b"cublasZsyr2k_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyr2k_v2_64: {
                let p = get_proc_addr(lib, b"cublasZsyr2k_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCher2k_v2: {
                let p = get_proc_addr(lib, b"cublasCher2k_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCher2k_v2_64: {
                let p = get_proc_addr(lib, b"cublasCher2k_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZher2k_v2: {
                let p = get_proc_addr(lib, b"cublasZher2k_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZher2k_v2_64: {
                let p = get_proc_addr(lib, b"cublasZher2k_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyrkx: {
                let p = get_proc_addr(lib, b"cublasSsyrkx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsyrkx_64: {
                let p = get_proc_addr(lib, b"cublasSsyrkx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyrkx: {
                let p = get_proc_addr(lib, b"cublasDsyrkx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsyrkx_64: {
                let p = get_proc_addr(lib, b"cublasDsyrkx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyrkx: {
                let p = get_proc_addr(lib, b"cublasCsyrkx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsyrkx_64: {
                let p = get_proc_addr(lib, b"cublasCsyrkx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyrkx: {
                let p = get_proc_addr(lib, b"cublasZsyrkx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsyrkx_64: {
                let p = get_proc_addr(lib, b"cublasZsyrkx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCherkx: {
                let p = get_proc_addr(lib, b"cublasCherkx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCherkx_64: {
                let p = get_proc_addr(lib, b"cublasCherkx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZherkx: {
                let p = get_proc_addr(lib, b"cublasZherkx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZherkx_64: {
                let p = get_proc_addr(lib, b"cublasZherkx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsymm_v2: {
                let p = get_proc_addr(lib, b"cublasSsymm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSsymm_v2_64: {
                let p = get_proc_addr(lib, b"cublasSsymm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsymm_v2: {
                let p = get_proc_addr(lib, b"cublasDsymm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDsymm_v2_64: {
                let p = get_proc_addr(lib, b"cublasDsymm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsymm_v2: {
                let p = get_proc_addr(lib, b"cublasCsymm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCsymm_v2_64: {
                let p = get_proc_addr(lib, b"cublasCsymm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsymm_v2: {
                let p = get_proc_addr(lib, b"cublasZsymm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZsymm_v2_64: {
                let p = get_proc_addr(lib, b"cublasZsymm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChemm_v2: {
                let p = get_proc_addr(lib, b"cublasChemm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasChemm_v2_64: {
                let p = get_proc_addr(lib, b"cublasChemm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhemm_v2: {
                let p = get_proc_addr(lib, b"cublasZhemm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZhemm_v2_64: {
                let p = get_proc_addr(lib, b"cublasZhemm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrsm_v2: {
                let p = get_proc_addr(lib, b"cublasStrsm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrsm_v2_64: {
                let p = get_proc_addr(lib, b"cublasStrsm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrsm_v2: {
                let p = get_proc_addr(lib, b"cublasDtrsm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrsm_v2_64: {
                let p = get_proc_addr(lib, b"cublasDtrsm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrsm_v2: {
                let p = get_proc_addr(lib, b"cublasCtrsm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrsm_v2_64: {
                let p = get_proc_addr(lib, b"cublasCtrsm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrsm_v2: {
                let p = get_proc_addr(lib, b"cublasZtrsm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrsm_v2_64: {
                let p = get_proc_addr(lib, b"cublasZtrsm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrmm_v2: {
                let p = get_proc_addr(lib, b"cublasStrmm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrmm_v2_64: {
                let p = get_proc_addr(lib, b"cublasStrmm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrmm_v2: {
                let p = get_proc_addr(lib, b"cublasDtrmm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrmm_v2_64: {
                let p = get_proc_addr(lib, b"cublasDtrmm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrmm_v2: {
                let p = get_proc_addr(lib, b"cublasCtrmm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrmm_v2_64: {
                let p = get_proc_addr(lib, b"cublasCtrmm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrmm_v2: {
                let p = get_proc_addr(lib, b"cublasZtrmm_v2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrmm_v2_64: {
                let p = get_proc_addr(lib, b"cublasZtrmm_v2_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemmBatched: {
                let p = get_proc_addr(lib, b"cublasSgemmBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemmBatched_64: {
                let p = get_proc_addr(lib, b"cublasSgemmBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemmBatched: {
                let p = get_proc_addr(lib, b"cublasDgemmBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemmBatched_64: {
                let p = get_proc_addr(lib, b"cublasDgemmBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemmBatched: {
                let p = get_proc_addr(lib, b"cublasCgemmBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemmBatched_64: {
                let p = get_proc_addr(lib, b"cublasCgemmBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm3mBatched: {
                let p = get_proc_addr(lib, b"cublasCgemm3mBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm3mBatched_64: {
                let p = get_proc_addr(lib, b"cublasCgemm3mBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemmBatched: {
                let p = get_proc_addr(lib, b"cublasZgemmBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemmBatched_64: {
                let p = get_proc_addr(lib, b"cublasZgemmBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemmStridedBatched: {
                let p = get_proc_addr(lib, b"cublasSgemmStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemmStridedBatched_64: {
                let p = get_proc_addr(lib, b"cublasSgemmStridedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemmStridedBatched: {
                let p = get_proc_addr(lib, b"cublasDgemmStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemmStridedBatched_64: {
                let p = get_proc_addr(lib, b"cublasDgemmStridedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemmStridedBatched: {
                let p = get_proc_addr(lib, b"cublasCgemmStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemmStridedBatched_64: {
                let p = get_proc_addr(lib, b"cublasCgemmStridedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm3mStridedBatched: {
                let p = get_proc_addr(lib, b"cublasCgemm3mStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgemm3mStridedBatched_64: {
                let p = get_proc_addr(lib, b"cublasCgemm3mStridedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemmStridedBatched: {
                let p = get_proc_addr(lib, b"cublasZgemmStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgemmStridedBatched_64: {
                let p = get_proc_addr(lib, b"cublasZgemmStridedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGemmBatchedEx: {
                let p = get_proc_addr(lib, b"cublasGemmBatchedEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGemmBatchedEx_64: {
                let p = get_proc_addr(lib, b"cublasGemmBatchedEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGemmStridedBatchedEx: {
                let p = get_proc_addr(lib, b"cublasGemmStridedBatchedEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGemmStridedBatchedEx_64: {
                let p = get_proc_addr(lib, b"cublasGemmStridedBatchedEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemmGroupedBatched: {
                let p = get_proc_addr(lib, b"cublasSgemmGroupedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgemmGroupedBatched_64: {
                let p = get_proc_addr(lib, b"cublasSgemmGroupedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemmGroupedBatched: {
                let p = get_proc_addr(lib, b"cublasDgemmGroupedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgemmGroupedBatched_64: {
                let p = get_proc_addr(lib, b"cublasDgemmGroupedBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGemmGroupedBatchedEx: {
                let p = get_proc_addr(lib, b"cublasGemmGroupedBatchedEx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasGemmGroupedBatchedEx_64: {
                let p = get_proc_addr(lib, b"cublasGemmGroupedBatchedEx_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgeam: {
                let p = get_proc_addr(lib, b"cublasSgeam\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgeam_64: {
                let p = get_proc_addr(lib, b"cublasSgeam_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgeam: {
                let p = get_proc_addr(lib, b"cublasDgeam\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgeam_64: {
                let p = get_proc_addr(lib, b"cublasDgeam_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgeam: {
                let p = get_proc_addr(lib, b"cublasCgeam\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgeam_64: {
                let p = get_proc_addr(lib, b"cublasCgeam_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgeam: {
                let p = get_proc_addr(lib, b"cublasZgeam\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgeam_64: {
                let p = get_proc_addr(lib, b"cublasZgeam_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrsmBatched: {
                let p = get_proc_addr(lib, b"cublasStrsmBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrsmBatched_64: {
                let p = get_proc_addr(lib, b"cublasStrsmBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrsmBatched: {
                let p = get_proc_addr(lib, b"cublasDtrsmBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrsmBatched_64: {
                let p = get_proc_addr(lib, b"cublasDtrsmBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrsmBatched: {
                let p = get_proc_addr(lib, b"cublasCtrsmBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrsmBatched_64: {
                let p = get_proc_addr(lib, b"cublasCtrsmBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrsmBatched: {
                let p = get_proc_addr(lib, b"cublasZtrsmBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrsmBatched_64: {
                let p = get_proc_addr(lib, b"cublasZtrsmBatched_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSdgmm: {
                let p = get_proc_addr(lib, b"cublasSdgmm\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSdgmm_64: {
                let p = get_proc_addr(lib, b"cublasSdgmm_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDdgmm: {
                let p = get_proc_addr(lib, b"cublasDdgmm\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDdgmm_64: {
                let p = get_proc_addr(lib, b"cublasDdgmm_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCdgmm: {
                let p = get_proc_addr(lib, b"cublasCdgmm\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCdgmm_64: {
                let p = get_proc_addr(lib, b"cublasCdgmm_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdgmm: {
                let p = get_proc_addr(lib, b"cublasZdgmm\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZdgmm_64: {
                let p = get_proc_addr(lib, b"cublasZdgmm_64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSmatinvBatched: {
                let p = get_proc_addr(lib, b"cublasSmatinvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDmatinvBatched: {
                let p = get_proc_addr(lib, b"cublasDmatinvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCmatinvBatched: {
                let p = get_proc_addr(lib, b"cublasCmatinvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZmatinvBatched: {
                let p = get_proc_addr(lib, b"cublasZmatinvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgeqrfBatched: {
                let p = get_proc_addr(lib, b"cublasSgeqrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgeqrfBatched: {
                let p = get_proc_addr(lib, b"cublasDgeqrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgeqrfBatched: {
                let p = get_proc_addr(lib, b"cublasCgeqrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgeqrfBatched: {
                let p = get_proc_addr(lib, b"cublasZgeqrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgelsBatched: {
                let p = get_proc_addr(lib, b"cublasSgelsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgelsBatched: {
                let p = get_proc_addr(lib, b"cublasDgelsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgelsBatched: {
                let p = get_proc_addr(lib, b"cublasCgelsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgelsBatched: {
                let p = get_proc_addr(lib, b"cublasZgelsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStpttr: {
                let p = get_proc_addr(lib, b"cublasStpttr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtpttr: {
                let p = get_proc_addr(lib, b"cublasDtpttr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtpttr: {
                let p = get_proc_addr(lib, b"cublasCtpttr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtpttr: {
                let p = get_proc_addr(lib, b"cublasZtpttr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasStrttp: {
                let p = get_proc_addr(lib, b"cublasStrttp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDtrttp: {
                let p = get_proc_addr(lib, b"cublasDtrttp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCtrttp: {
                let p = get_proc_addr(lib, b"cublasCtrttp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZtrttp: {
                let p = get_proc_addr(lib, b"cublasZtrttp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgetrfBatched: {
                let p = get_proc_addr(lib, b"cublasSgetrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgetrfBatched: {
                let p = get_proc_addr(lib, b"cublasDgetrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgetrfBatched: {
                let p = get_proc_addr(lib, b"cublasCgetrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgetrfBatched: {
                let p = get_proc_addr(lib, b"cublasZgetrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgetriBatched: {
                let p = get_proc_addr(lib, b"cublasSgetriBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgetriBatched: {
                let p = get_proc_addr(lib, b"cublasDgetriBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgetriBatched: {
                let p = get_proc_addr(lib, b"cublasCgetriBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgetriBatched: {
                let p = get_proc_addr(lib, b"cublasZgetriBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasSgetrsBatched: {
                let p = get_proc_addr(lib, b"cublasSgetrsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasDgetrsBatched: {
                let p = get_proc_addr(lib, b"cublasDgetrsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasCgetrsBatched: {
                let p = get_proc_addr(lib, b"cublasCgetrsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasZgetrsBatched: {
                let p = get_proc_addr(lib, b"cublasZgetrsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cublasUint8gemmBias: {
                let p = get_proc_addr(lib, b"cublasUint8gemmBias\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
        })
    };
    DYNAMIC_BINDINGS.set(bindings).ok();
}
unsafe impl Send for float2 {}
unsafe impl Sync for float2 {}
unsafe impl Send for double2 {}
unsafe impl Sync for double2 {}
unsafe impl Send for CUstream_st {}
unsafe impl Sync for CUstream_st {}
unsafe impl Send for libraryPropertyType_t {}
unsafe impl Sync for libraryPropertyType_t {}
unsafe impl Send for cublasStatus_t {}
unsafe impl Sync for cublasStatus_t {}
unsafe impl Send for cublasFillMode_t {}
unsafe impl Sync for cublasFillMode_t {}
unsafe impl Send for cublasDiagType_t {}
unsafe impl Sync for cublasDiagType_t {}
unsafe impl Send for cublasSideMode_t {}
unsafe impl Sync for cublasSideMode_t {}
unsafe impl Send for cublasOperation_t {}
unsafe impl Sync for cublasOperation_t {}
unsafe impl Send for cublasPointerMode_t {}
unsafe impl Sync for cublasPointerMode_t {}
unsafe impl Send for cublasAtomicsMode_t {}
unsafe impl Sync for cublasAtomicsMode_t {}
unsafe impl Send for cublasGemmAlgo_t {}
unsafe impl Sync for cublasGemmAlgo_t {}
unsafe impl Send for cublasMath_t {}
unsafe impl Sync for cublasMath_t {}
unsafe impl Send for cublasComputeType_t {}
unsafe impl Sync for cublasComputeType_t {}
unsafe impl Send for cublasEmulationStrategy_t {}
unsafe impl Sync for cublasEmulationStrategy_t {}
unsafe impl Send for cublasContext {}
unsafe impl Sync for cublasContext {}
impl std::fmt::Display for cublasStatus_t {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for cublasStatus_t {}
