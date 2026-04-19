#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
pub use crate::sys::CUBLAS_VER_BUILD;
pub use crate::sys::CUBLAS_VER_MAJOR;
pub use crate::sys::CUBLAS_VER_MINOR;
pub use crate::sys::CUBLAS_VER_PATCH;
pub use crate::sys::CUBLAS_VERSION;
pub use crate::sys::cublasAtomicsMode_t;
pub use crate::sys::cublasComputeType_t;
pub use crate::sys::cublasContext;
pub use crate::sys::cublasDiagType_t;
pub use crate::sys::cublasEmulationStrategy_t;
pub use crate::sys::cublasFillMode_t;
pub use crate::sys::cublasGemmAlgo_t;
pub use crate::sys::cublasHandle_t;
pub use crate::sys::cublasLogCallback;
pub use crate::sys::cublasMath_t;
pub use crate::sys::cublasOperation_t;
pub use crate::sys::cublasPointerMode_t;
pub use crate::sys::cublasSideMode_t;
pub use crate::sys::cublasStatus_t as CudaTargetStatus;
pub use crate::sys::cublasStatus_t;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart;
#[allow(unused_imports)]
use cuda_libs_cudart::sys::*;
#[allow(unused_imports)]
use cuda_libs_cudart::types;
#[cfg(feature = "runtime-link")]
impl crate::sys::DynamicBindings {
    pub fn cublasCreate_v2(mut self, val: Option<unsafe extern "C" fn(*mut cublasHandle_t) -> cublasStatus_t>) -> Self {
        self.cublasCreate_v2 = val;
        self
    }
    pub fn cublasDestroy_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t) -> cublasStatus_t>) -> Self {
        self.cublasDestroy_v2 = val;
        self
    }
    pub fn cublasGetVersion_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetVersion_v2 = val;
        self
    }
    pub fn cublasGetProperty(mut self, val: Option<unsafe extern "C" fn(libraryPropertyType, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetProperty = val;
        self
    }
    pub fn cublasGetCudartVersion(mut self, val: Option<unsafe extern "C" fn() -> usize>) -> Self {
        self.cublasGetCudartVersion = val;
        self
    }
    pub fn cublasSetWorkspace_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut ::std::os::raw::c_void, usize) -> cublasStatus_t>) -> Self {
        self.cublasSetWorkspace_v2 = val;
        self
    }
    pub fn cublasSetStream_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasSetStream_v2 = val;
        self
    }
    pub fn cublasGetStream_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasGetStream_v2 = val;
        self
    }
    pub fn cublasGetPointerMode_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut cublasPointerMode_t) -> cublasStatus_t>) -> Self {
        self.cublasGetPointerMode_v2 = val;
        self
    }
    pub fn cublasSetPointerMode_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasPointerMode_t) -> cublasStatus_t>) -> Self {
        self.cublasSetPointerMode_v2 = val;
        self
    }
    pub fn cublasGetAtomicsMode(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut cublasAtomicsMode_t) -> cublasStatus_t>) -> Self {
        self.cublasGetAtomicsMode = val;
        self
    }
    pub fn cublasSetAtomicsMode(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasAtomicsMode_t) -> cublasStatus_t>) -> Self {
        self.cublasSetAtomicsMode = val;
        self
    }
    pub fn cublasGetMathMode(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut cublasMath_t) -> cublasStatus_t>) -> Self {
        self.cublasGetMathMode = val;
        self
    }
    pub fn cublasSetMathMode(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasMath_t) -> cublasStatus_t>) -> Self {
        self.cublasSetMathMode = val;
        self
    }
    pub fn cublasGetSmCountTarget(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetSmCountTarget = val;
        self
    }
    pub fn cublasSetSmCountTarget(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetSmCountTarget = val;
        self
    }
    pub fn cublasGetEmulationStrategy(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut cublasEmulationStrategy_t) -> cublasStatus_t>) -> Self {
        self.cublasGetEmulationStrategy = val;
        self
    }
    pub fn cublasSetEmulationStrategy(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasEmulationStrategy_t) -> cublasStatus_t>) -> Self {
        self.cublasSetEmulationStrategy = val;
        self
    }
    pub fn cublasGetEmulationSpecialValuesSupport(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut cudaEmulationSpecialValuesSupport) -> cublasStatus_t>) -> Self {
        self.cublasGetEmulationSpecialValuesSupport = val;
        self
    }
    pub fn cublasSetEmulationSpecialValuesSupport(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cudaEmulationSpecialValuesSupport) -> cublasStatus_t>) -> Self {
        self.cublasSetEmulationSpecialValuesSupport = val;
        self
    }
    pub fn cublasGetFixedPointEmulationMantissaControl(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut cudaEmulationMantissaControl) -> cublasStatus_t>) -> Self {
        self.cublasGetFixedPointEmulationMantissaControl = val;
        self
    }
    pub fn cublasSetFixedPointEmulationMantissaControl(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cudaEmulationMantissaControl) -> cublasStatus_t>) -> Self {
        self.cublasSetFixedPointEmulationMantissaControl = val;
        self
    }
    pub fn cublasGetFixedPointEmulationMaxMantissaBitCount(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetFixedPointEmulationMaxMantissaBitCount = val;
        self
    }
    pub fn cublasSetFixedPointEmulationMaxMantissaBitCount(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetFixedPointEmulationMaxMantissaBitCount = val;
        self
    }
    pub fn cublasGetFixedPointEmulationMantissaBitOffset(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetFixedPointEmulationMantissaBitOffset = val;
        self
    }
    pub fn cublasSetFixedPointEmulationMantissaBitOffset(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetFixedPointEmulationMantissaBitOffset = val;
        self
    }
    pub fn cublasGetFixedPointEmulationMantissaBitCountPointer(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetFixedPointEmulationMantissaBitCountPointer = val;
        self
    }
    pub fn cublasSetFixedPointEmulationMantissaBitCountPointer(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetFixedPointEmulationMantissaBitCountPointer = val;
        self
    }
    pub fn cublasGetStatusName(mut self, val: Option<unsafe extern "C" fn(cublasStatus_t) -> *const ::std::os::raw::c_char>) -> Self {
        self.cublasGetStatusName = val;
        self
    }
    pub fn cublasGetStatusString(mut self, val: Option<unsafe extern "C" fn(cublasStatus_t) -> *const ::std::os::raw::c_char>) -> Self {
        self.cublasGetStatusString = val;
        self
    }
    pub fn cublasLoggerConfigure(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_char) -> cublasStatus_t>) -> Self {
        self.cublasLoggerConfigure = val;
        self
    }
    pub fn cublasSetLoggerCallback(mut self, val: Option<unsafe extern "C" fn(cublasLogCallback) -> cublasStatus_t>) -> Self {
        self.cublasSetLoggerCallback = val;
        self
    }
    pub fn cublasGetLoggerCallback(mut self, val: Option<unsafe extern "C" fn(*mut cublasLogCallback) -> cublasStatus_t>) -> Self {
        self.cublasGetLoggerCallback = val;
        self
    }
    pub fn cublasSetVector(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_void, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetVector = val;
        self
    }
    pub fn cublasSetVector_64(mut self, val: Option<unsafe extern "C" fn(i64, i64, *const ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_void, i64) -> cublasStatus_t>) -> Self {
        self.cublasSetVector_64 = val;
        self
    }
    pub fn cublasGetVector(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_void, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetVector = val;
        self
    }
    pub fn cublasGetVector_64(mut self, val: Option<unsafe extern "C" fn(i64, i64, *const ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_void, i64) -> cublasStatus_t>) -> Self {
        self.cublasGetVector_64 = val;
        self
    }
    pub fn cublasSetMatrix(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_void, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetMatrix = val;
        self
    }
    pub fn cublasSetMatrix_64(mut self, val: Option<unsafe extern "C" fn(i64, i64, i64, *const ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_void, i64) -> cublasStatus_t>) -> Self {
        self.cublasSetMatrix_64 = val;
        self
    }
    pub fn cublasGetMatrix(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_void, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetMatrix = val;
        self
    }
    pub fn cublasGetMatrix_64(mut self, val: Option<unsafe extern "C" fn(i64, i64, i64, *const ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_void, i64) -> cublasStatus_t>) -> Self {
        self.cublasGetMatrix_64 = val;
        self
    }
    pub fn cublasSetVectorAsync(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_void, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, ::std::os::raw::c_int, cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasSetVectorAsync = val;
        self
    }
    pub fn cublasSetVectorAsync_64(mut self, val: Option<unsafe extern "C" fn(i64, i64, *const ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_void, i64, cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasSetVectorAsync_64 = val;
        self
    }
    pub fn cublasGetVectorAsync(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_void, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, ::std::os::raw::c_int, cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasGetVectorAsync = val;
        self
    }
    pub fn cublasGetVectorAsync_64(mut self, val: Option<unsafe extern "C" fn(i64, i64, *const ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_void, i64, cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasGetVectorAsync_64 = val;
        self
    }
    pub fn cublasSetMatrixAsync(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_void, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, ::std::os::raw::c_int, cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasSetMatrixAsync = val;
        self
    }
    pub fn cublasSetMatrixAsync_64(mut self, val: Option<unsafe extern "C" fn(i64, i64, i64, *const ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_void, i64, cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasSetMatrixAsync_64 = val;
        self
    }
    pub fn cublasGetMatrixAsync(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_void, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, ::std::os::raw::c_int, cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasGetMatrixAsync = val;
        self
    }
    pub fn cublasGetMatrixAsync_64(mut self, val: Option<unsafe extern "C" fn(i64, i64, i64, *const ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_void, i64, cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasGetMatrixAsync_64 = val;
        self
    }
    pub fn cublasXerbla(mut self, val: Option<unsafe extern "C" fn(*const ::std::os::raw::c_char, ::std::os::raw::c_int)>) -> Self {
        self.cublasXerbla = val;
        self
    }
    pub fn cublasNrm2Ex(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasNrm2Ex = val;
        self
    }
    pub fn cublasNrm2Ex_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasNrm2Ex_64 = val;
        self
    }
    pub fn cublasSnrm2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSnrm2_v2 = val;
        self
    }
    pub fn cublasSnrm2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f32, i64, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSnrm2_v2_64 = val;
        self
    }
    pub fn cublasDnrm2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDnrm2_v2 = val;
        self
    }
    pub fn cublasDnrm2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f64, i64, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDnrm2_v2_64 = val;
        self
    }
    pub fn cublasScnrm2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasScnrm2_v2 = val;
        self
    }
    pub fn cublasScnrm2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuComplex, i64, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasScnrm2_v2_64 = val;
        self
    }
    pub fn cublasDznrm2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDznrm2_v2 = val;
        self
    }
    pub fn cublasDznrm2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuDoubleComplex, i64, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDznrm2_v2_64 = val;
        self
    }
    pub fn cublasDotEx(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDotEx = val;
        self
    }
    pub fn cublasDotEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasDotEx_64 = val;
        self
    }
    pub fn cublasDotcEx(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDotcEx = val;
        self
    }
    pub fn cublasDotcEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasDotcEx_64 = val;
        self
    }
    pub fn cublasSdot_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSdot_v2 = val;
        self
    }
    pub fn cublasSdot_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f32, i64, *const f32, i64, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSdot_v2_64 = val;
        self
    }
    pub fn cublasDdot_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDdot_v2 = val;
        self
    }
    pub fn cublasDdot_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f64, i64, *const f64, i64, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDdot_v2_64 = val;
        self
    }
    pub fn cublasCdotu_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCdotu_v2 = val;
        self
    }
    pub fn cublasCdotu_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuComplex, i64, *const cuComplex, i64, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCdotu_v2_64 = val;
        self
    }
    pub fn cublasCdotc_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCdotc_v2 = val;
        self
    }
    pub fn cublasCdotc_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuComplex, i64, *const cuComplex, i64, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCdotc_v2_64 = val;
        self
    }
    pub fn cublasZdotu_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZdotu_v2 = val;
        self
    }
    pub fn cublasZdotu_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZdotu_v2_64 = val;
        self
    }
    pub fn cublasZdotc_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZdotc_v2 = val;
        self
    }
    pub fn cublasZdotc_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZdotc_v2_64 = val;
        self
    }
    pub fn cublasScalEx(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, *mut ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasScalEx = val;
        self
    }
    pub fn cublasScalEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const ::std::os::raw::c_void, cudaDataType, *mut ::std::os::raw::c_void, cudaDataType, i64, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasScalEx_64 = val;
        self
    }
    pub fn cublasSscal_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSscal_v2 = val;
        self
    }
    pub fn cublasSscal_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSscal_v2_64 = val;
        self
    }
    pub fn cublasDscal_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDscal_v2 = val;
        self
    }
    pub fn cublasDscal_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDscal_v2_64 = val;
        self
    }
    pub fn cublasCscal_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCscal_v2 = val;
        self
    }
    pub fn cublasCscal_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCscal_v2_64 = val;
        self
    }
    pub fn cublasCsscal_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f32, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCsscal_v2 = val;
        self
    }
    pub fn cublasCsscal_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f32, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsscal_v2_64 = val;
        self
    }
    pub fn cublasZscal_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZscal_v2 = val;
        self
    }
    pub fn cublasZscal_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZscal_v2_64 = val;
        self
    }
    pub fn cublasZdscal_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f64, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZdscal_v2 = val;
        self
    }
    pub fn cublasZdscal_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZdscal_v2_64 = val;
        self
    }
    pub fn cublasAxpyEx(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, cudaDataType) -> cublasStatus_t>,
    ) -> Self {
        self.cublasAxpyEx = val;
        self
    }
    pub fn cublasAxpyEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const ::std::os::raw::c_void, cudaDataType, *const ::std::os::raw::c_void, cudaDataType, i64, *mut ::std::os::raw::c_void, cudaDataType, i64, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasAxpyEx_64 = val;
        self
    }
    pub fn cublasSaxpy_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSaxpy_v2 = val;
        self
    }
    pub fn cublasSaxpy_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f32, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSaxpy_v2_64 = val;
        self
    }
    pub fn cublasDaxpy_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDaxpy_v2 = val;
        self
    }
    pub fn cublasDaxpy_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDaxpy_v2_64 = val;
        self
    }
    pub fn cublasCaxpy_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCaxpy_v2 = val;
        self
    }
    pub fn cublasCaxpy_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuComplex, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCaxpy_v2_64 = val;
        self
    }
    pub fn cublasZaxpy_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZaxpy_v2 = val;
        self
    }
    pub fn cublasZaxpy_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZaxpy_v2_64 = val;
        self
    }
    pub fn cublasCopyEx(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCopyEx = val;
        self
    }
    pub fn cublasCopyEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *mut ::std::os::raw::c_void, cudaDataType, i64) -> cublasStatus_t>) -> Self {
        self.cublasCopyEx_64 = val;
        self
    }
    pub fn cublasScopy_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasScopy_v2 = val;
        self
    }
    pub fn cublasScopy_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasScopy_v2_64 = val;
        self
    }
    pub fn cublasDcopy_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDcopy_v2 = val;
        self
    }
    pub fn cublasDcopy_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDcopy_v2_64 = val;
        self
    }
    pub fn cublasCcopy_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCcopy_v2 = val;
        self
    }
    pub fn cublasCcopy_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCcopy_v2_64 = val;
        self
    }
    pub fn cublasZcopy_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZcopy_v2 = val;
        self
    }
    pub fn cublasZcopy_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZcopy_v2_64 = val;
        self
    }
    pub fn cublasSswap_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSswap_v2 = val;
        self
    }
    pub fn cublasSswap_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSswap_v2_64 = val;
        self
    }
    pub fn cublasDswap_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDswap_v2 = val;
        self
    }
    pub fn cublasDswap_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDswap_v2_64 = val;
        self
    }
    pub fn cublasCswap_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCswap_v2 = val;
        self
    }
    pub fn cublasCswap_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCswap_v2_64 = val;
        self
    }
    pub fn cublasZswap_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZswap_v2 = val;
        self
    }
    pub fn cublasZswap_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZswap_v2_64 = val;
        self
    }
    pub fn cublasSwapEx(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSwapEx = val;
        self
    }
    pub fn cublasSwapEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut ::std::os::raw::c_void, cudaDataType, i64, *mut ::std::os::raw::c_void, cudaDataType, i64) -> cublasStatus_t>) -> Self {
        self.cublasSwapEx_64 = val;
        self
    }
    pub fn cublasIsamax_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIsamax_v2 = val;
        self
    }
    pub fn cublasIsamax_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f32, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIsamax_v2_64 = val;
        self
    }
    pub fn cublasIdamax_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIdamax_v2 = val;
        self
    }
    pub fn cublasIdamax_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f64, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIdamax_v2_64 = val;
        self
    }
    pub fn cublasIcamax_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIcamax_v2 = val;
        self
    }
    pub fn cublasIcamax_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuComplex, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIcamax_v2_64 = val;
        self
    }
    pub fn cublasIzamax_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIzamax_v2 = val;
        self
    }
    pub fn cublasIzamax_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuDoubleComplex, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIzamax_v2_64 = val;
        self
    }
    pub fn cublasIamaxEx(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIamaxEx = val;
        self
    }
    pub fn cublasIamaxEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIamaxEx_64 = val;
        self
    }
    pub fn cublasIsamin_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIsamin_v2 = val;
        self
    }
    pub fn cublasIsamin_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f32, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIsamin_v2_64 = val;
        self
    }
    pub fn cublasIdamin_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIdamin_v2 = val;
        self
    }
    pub fn cublasIdamin_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f64, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIdamin_v2_64 = val;
        self
    }
    pub fn cublasIcamin_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIcamin_v2 = val;
        self
    }
    pub fn cublasIcamin_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuComplex, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIcamin_v2_64 = val;
        self
    }
    pub fn cublasIzamin_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIzamin_v2 = val;
        self
    }
    pub fn cublasIzamin_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuDoubleComplex, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIzamin_v2_64 = val;
        self
    }
    pub fn cublasIaminEx(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIaminEx = val;
        self
    }
    pub fn cublasIaminEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIaminEx_64 = val;
        self
    }
    pub fn cublasAsumEx(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasAsumEx = val;
        self
    }
    pub fn cublasAsumEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasAsumEx_64 = val;
        self
    }
    pub fn cublasSasum_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSasum_v2 = val;
        self
    }
    pub fn cublasSasum_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f32, i64, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSasum_v2_64 = val;
        self
    }
    pub fn cublasDasum_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDasum_v2 = val;
        self
    }
    pub fn cublasDasum_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const f64, i64, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDasum_v2_64 = val;
        self
    }
    pub fn cublasScasum_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasScasum_v2 = val;
        self
    }
    pub fn cublasScasum_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuComplex, i64, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasScasum_v2_64 = val;
        self
    }
    pub fn cublasDzasum_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDzasum_v2 = val;
        self
    }
    pub fn cublasDzasum_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *const cuDoubleComplex, i64, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDzasum_v2_64 = val;
        self
    }
    pub fn cublasSrot_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *const f32, *const f32) -> cublasStatus_t>) -> Self {
        self.cublasSrot_v2 = val;
        self
    }
    pub fn cublasSrot_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut f32, i64, *mut f32, i64, *const f32, *const f32) -> cublasStatus_t>) -> Self {
        self.cublasSrot_v2_64 = val;
        self
    }
    pub fn cublasDrot_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *const f64, *const f64) -> cublasStatus_t>) -> Self {
        self.cublasDrot_v2 = val;
        self
    }
    pub fn cublasDrot_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut f64, i64, *mut f64, i64, *const f64, *const f64) -> cublasStatus_t>) -> Self {
        self.cublasDrot_v2_64 = val;
        self
    }
    pub fn cublasCrot_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *const f32, *const cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCrot_v2 = val;
        self
    }
    pub fn cublasCrot_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut cuComplex, i64, *mut cuComplex, i64, *const f32, *const cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCrot_v2_64 = val;
        self
    }
    pub fn cublasCsrot_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *const f32, *const f32) -> cublasStatus_t>) -> Self {
        self.cublasCsrot_v2 = val;
        self
    }
    pub fn cublasCsrot_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut cuComplex, i64, *mut cuComplex, i64, *const f32, *const f32) -> cublasStatus_t>) -> Self {
        self.cublasCsrot_v2_64 = val;
        self
    }
    pub fn cublasZrot_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *const f64, *const cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZrot_v2 = val;
        self
    }
    pub fn cublasZrot_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut cuDoubleComplex, i64, *mut cuDoubleComplex, i64, *const f64, *const cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZrot_v2_64 = val;
        self
    }
    pub fn cublasZdrot_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *const f64, *const f64) -> cublasStatus_t>) -> Self {
        self.cublasZdrot_v2 = val;
        self
    }
    pub fn cublasZdrot_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut cuDoubleComplex, i64, *mut cuDoubleComplex, i64, *const f64, *const f64) -> cublasStatus_t>) -> Self {
        self.cublasZdrot_v2_64 = val;
        self
    }
    pub fn cublasRotEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                *const ::std::os::raw::c_void,
                cudaDataType,
                cudaDataType,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasRotEx = val;
        self
    }
    pub fn cublasRotEx_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut ::std::os::raw::c_void, cudaDataType, i64, *mut ::std::os::raw::c_void, cudaDataType, i64, *const ::std::os::raw::c_void, *const ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>,
    ) -> Self {
        self.cublasRotEx_64 = val;
        self
    }
    pub fn cublasSrotg_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut f32, *mut f32, *mut f32, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSrotg_v2 = val;
        self
    }
    pub fn cublasDrotg_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut f64, *mut f64, *mut f64, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDrotg_v2 = val;
        self
    }
    pub fn cublasCrotg_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut cuComplex, *mut cuComplex, *mut f32, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCrotg_v2 = val;
        self
    }
    pub fn cublasZrotg_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut cuDoubleComplex, *mut cuDoubleComplex, *mut f64, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZrotg_v2 = val;
        self
    }
    pub fn cublasRotgEx(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void, cudaDataType, *mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasRotgEx = val;
        self
    }
    pub fn cublasSrotm_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *const f32) -> cublasStatus_t>) -> Self {
        self.cublasSrotm_v2 = val;
        self
    }
    pub fn cublasSrotm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut f32, i64, *mut f32, i64, *const f32) -> cublasStatus_t>) -> Self {
        self.cublasSrotm_v2_64 = val;
        self
    }
    pub fn cublasDrotm_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *const f64) -> cublasStatus_t>) -> Self {
        self.cublasDrotm_v2 = val;
        self
    }
    pub fn cublasDrotm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut f64, i64, *mut f64, i64, *const f64) -> cublasStatus_t>) -> Self {
        self.cublasDrotm_v2_64 = val;
        self
    }
    pub fn cublasRotmEx(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *mut ::std::os::raw::c_void, cudaDataType, ::std::os::raw::c_int, *const ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>,
    ) -> Self {
        self.cublasRotmEx = val;
        self
    }
    pub fn cublasRotmEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, *mut ::std::os::raw::c_void, cudaDataType, i64, *mut ::std::os::raw::c_void, cudaDataType, i64, *const ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasRotmEx_64 = val;
        self
    }
    pub fn cublasSrotmg_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut f32, *mut f32, *mut f32, *const f32, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSrotmg_v2 = val;
        self
    }
    pub fn cublasDrotmg_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, *mut f64, *mut f64, *mut f64, *const f64, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDrotmg_v2 = val;
        self
    }
    pub fn cublasRotmgEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(cublasHandle_t, *mut ::std::os::raw::c_void, cudaDataType, *mut ::std::os::raw::c_void, cudaDataType, *mut ::std::os::raw::c_void, cudaDataType, *const ::std::os::raw::c_void, cudaDataType, *mut ::std::os::raw::c_void, cudaDataType, cudaDataType) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasRotmgEx = val;
        self
    }
    pub fn cublasSgemv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgemv_v2 = val;
        self
    }
    pub fn cublasSgemv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const f32, *const f32, i64, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSgemv_v2_64 = val;
        self
    }
    pub fn cublasDgemv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgemv_v2 = val;
        self
    }
    pub fn cublasDgemv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const f64, *const f64, i64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDgemv_v2_64 = val;
        self
    }
    pub fn cublasCgemv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgemv_v2 = val;
        self
    }
    pub fn cublasCgemv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCgemv_v2_64 = val;
        self
    }
    pub fn cublasZgemv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgemv_v2 = val;
        self
    }
    pub fn cublasZgemv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZgemv_v2_64 = val;
        self
    }
    pub fn cublasSgbmv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                *mut f32,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgbmv_v2 = val;
        self
    }
    pub fn cublasSgbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, i64, i64, *const f32, *const f32, i64, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSgbmv_v2_64 = val;
        self
    }
    pub fn cublasDgbmv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                *mut f64,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgbmv_v2 = val;
        self
    }
    pub fn cublasDgbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, i64, i64, *const f64, *const f64, i64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDgbmv_v2_64 = val;
        self
    }
    pub fn cublasCgbmv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgbmv_v2 = val;
        self
    }
    pub fn cublasCgbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCgbmv_v2_64 = val;
        self
    }
    pub fn cublasZgbmv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgbmv_v2 = val;
        self
    }
    pub fn cublasZgbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZgbmv_v2_64 = val;
        self
    }
    pub fn cublasStrmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStrmv_v2 = val;
        self
    }
    pub fn cublasStrmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasStrmv_v2_64 = val;
        self
    }
    pub fn cublasDtrmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtrmv_v2 = val;
        self
    }
    pub fn cublasDtrmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDtrmv_v2_64 = val;
        self
    }
    pub fn cublasCtrmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtrmv_v2 = val;
        self
    }
    pub fn cublasCtrmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCtrmv_v2_64 = val;
        self
    }
    pub fn cublasZtrmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZtrmv_v2 = val;
        self
    }
    pub fn cublasZtrmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZtrmv_v2_64 = val;
        self
    }
    pub fn cublasStbmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStbmv_v2 = val;
        self
    }
    pub fn cublasStbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasStbmv_v2_64 = val;
        self
    }
    pub fn cublasDtbmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtbmv_v2 = val;
        self
    }
    pub fn cublasDtbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDtbmv_v2_64 = val;
        self
    }
    pub fn cublasCtbmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtbmv_v2 = val;
        self
    }
    pub fn cublasCtbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCtbmv_v2_64 = val;
        self
    }
    pub fn cublasZtbmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZtbmv_v2 = val;
        self
    }
    pub fn cublasZtbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZtbmv_v2_64 = val;
        self
    }
    pub fn cublasStpmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStpmv_v2 = val;
        self
    }
    pub fn cublasStpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasStpmv_v2_64 = val;
        self
    }
    pub fn cublasDtpmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtpmv_v2 = val;
        self
    }
    pub fn cublasDtpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDtpmv_v2_64 = val;
        self
    }
    pub fn cublasCtpmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtpmv_v2 = val;
        self
    }
    pub fn cublasCtpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCtpmv_v2_64 = val;
        self
    }
    pub fn cublasZtpmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZtpmv_v2 = val;
        self
    }
    pub fn cublasZtpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZtpmv_v2_64 = val;
        self
    }
    pub fn cublasStrsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStrsv_v2 = val;
        self
    }
    pub fn cublasStrsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasStrsv_v2_64 = val;
        self
    }
    pub fn cublasDtrsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtrsv_v2 = val;
        self
    }
    pub fn cublasDtrsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDtrsv_v2_64 = val;
        self
    }
    pub fn cublasCtrsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtrsv_v2 = val;
        self
    }
    pub fn cublasCtrsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCtrsv_v2_64 = val;
        self
    }
    pub fn cublasZtrsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZtrsv_v2 = val;
        self
    }
    pub fn cublasZtrsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZtrsv_v2_64 = val;
        self
    }
    pub fn cublasStpsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStpsv_v2 = val;
        self
    }
    pub fn cublasStpsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasStpsv_v2_64 = val;
        self
    }
    pub fn cublasDtpsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtpsv_v2 = val;
        self
    }
    pub fn cublasDtpsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDtpsv_v2_64 = val;
        self
    }
    pub fn cublasCtpsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtpsv_v2 = val;
        self
    }
    pub fn cublasCtpsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCtpsv_v2_64 = val;
        self
    }
    pub fn cublasZtpsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZtpsv_v2 = val;
        self
    }
    pub fn cublasZtpsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZtpsv_v2_64 = val;
        self
    }
    pub fn cublasStbsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStbsv_v2 = val;
        self
    }
    pub fn cublasStbsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasStbsv_v2_64 = val;
        self
    }
    pub fn cublasDtbsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtbsv_v2 = val;
        self
    }
    pub fn cublasDtbsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDtbsv_v2_64 = val;
        self
    }
    pub fn cublasCtbsv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtbsv_v2 = val;
        self
    }
    pub fn cublasCtbsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCtbsv_v2_64 = val;
        self
    }
    pub fn cublasZtbsv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZtbsv_v2 = val;
        self
    }
    pub fn cublasZtbsv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZtbsv_v2_64 = val;
        self
    }
    pub fn cublasSsymv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSsymv_v2 = val;
        self
    }
    pub fn cublasSsymv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f32, *const f32, i64, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSsymv_v2_64 = val;
        self
    }
    pub fn cublasDsymv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDsymv_v2 = val;
        self
    }
    pub fn cublasDsymv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f64, *const f64, i64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDsymv_v2_64 = val;
        self
    }
    pub fn cublasCsymv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCsymv_v2 = val;
        self
    }
    pub fn cublasCsymv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsymv_v2_64 = val;
        self
    }
    pub fn cublasZsymv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZsymv_v2 = val;
        self
    }
    pub fn cublasZsymv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZsymv_v2_64 = val;
        self
    }
    pub fn cublasChemv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasChemv_v2 = val;
        self
    }
    pub fn cublasChemv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasChemv_v2_64 = val;
        self
    }
    pub fn cublasZhemv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZhemv_v2 = val;
        self
    }
    pub fn cublasZhemv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZhemv_v2_64 = val;
        self
    }
    pub fn cublasSsbmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSsbmv_v2 = val;
        self
    }
    pub fn cublasSsbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, i64, *const f32, *const f32, i64, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSsbmv_v2_64 = val;
        self
    }
    pub fn cublasDsbmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDsbmv_v2 = val;
        self
    }
    pub fn cublasDsbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, i64, *const f64, *const f64, i64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDsbmv_v2_64 = val;
        self
    }
    pub fn cublasChbmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasChbmv_v2 = val;
        self
    }
    pub fn cublasChbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasChbmv_v2_64 = val;
        self
    }
    pub fn cublasZhbmv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZhbmv_v2 = val;
        self
    }
    pub fn cublasZhbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZhbmv_v2_64 = val;
        self
    }
    pub fn cublasSspmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, *const f32, *const f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSspmv_v2 = val;
        self
    }
    pub fn cublasSspmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f32, *const f32, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSspmv_v2_64 = val;
        self
    }
    pub fn cublasDspmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, *const f64, *const f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDspmv_v2 = val;
        self
    }
    pub fn cublasDspmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f64, *const f64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDspmv_v2_64 = val;
        self
    }
    pub fn cublasChpmv_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasChpmv_v2 = val;
        self
    }
    pub fn cublasChpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuComplex, *const cuComplex, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasChpmv_v2_64 = val;
        self
    }
    pub fn cublasZhpmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZhpmv_v2 = val;
        self
    }
    pub fn cublasZhpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuDoubleComplex, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZhpmv_v2_64 = val;
        self
    }
    pub fn cublasSger_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSger_v2 = val;
        self
    }
    pub fn cublasSger_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, i64, *const f32, *const f32, i64, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSger_v2_64 = val;
        self
    }
    pub fn cublasDger_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDger_v2 = val;
        self
    }
    pub fn cublasDger_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, i64, *const f64, *const f64, i64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDger_v2_64 = val;
        self
    }
    pub fn cublasCgeru_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCgeru_v2 = val;
        self
    }
    pub fn cublasCgeru_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCgeru_v2_64 = val;
        self
    }
    pub fn cublasCgerc_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCgerc_v2 = val;
        self
    }
    pub fn cublasCgerc_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCgerc_v2_64 = val;
        self
    }
    pub fn cublasZgeru_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZgeru_v2 = val;
        self
    }
    pub fn cublasZgeru_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZgeru_v2_64 = val;
        self
    }
    pub fn cublasZgerc_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZgerc_v2 = val;
        self
    }
    pub fn cublasZgerc_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZgerc_v2_64 = val;
        self
    }
    pub fn cublasSsyr_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSsyr_v2 = val;
        self
    }
    pub fn cublasSsyr_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f32, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSsyr_v2_64 = val;
        self
    }
    pub fn cublasDsyr_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDsyr_v2 = val;
        self
    }
    pub fn cublasDsyr_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDsyr_v2_64 = val;
        self
    }
    pub fn cublasCsyr_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCsyr_v2 = val;
        self
    }
    pub fn cublasCsyr_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuComplex, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyr_v2_64 = val;
        self
    }
    pub fn cublasZsyr_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZsyr_v2 = val;
        self
    }
    pub fn cublasZsyr_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZsyr_v2_64 = val;
        self
    }
    pub fn cublasCher_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCher_v2 = val;
        self
    }
    pub fn cublasCher_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f32, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCher_v2_64 = val;
        self
    }
    pub fn cublasZher_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZher_v2 = val;
        self
    }
    pub fn cublasZher_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZher_v2_64 = val;
        self
    }
    pub fn cublasSspr_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSspr_v2 = val;
        self
    }
    pub fn cublasSspr_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f32, *const f32, i64, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSspr_v2_64 = val;
        self
    }
    pub fn cublasDspr_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDspr_v2 = val;
        self
    }
    pub fn cublasDspr_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f64, *const f64, i64, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDspr_v2_64 = val;
        self
    }
    pub fn cublasChpr_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasChpr_v2 = val;
        self
    }
    pub fn cublasChpr_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f32, *const cuComplex, i64, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasChpr_v2_64 = val;
        self
    }
    pub fn cublasZhpr_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZhpr_v2 = val;
        self
    }
    pub fn cublasZhpr_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f64, *const cuDoubleComplex, i64, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZhpr_v2_64 = val;
        self
    }
    pub fn cublasSsyr2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSsyr2_v2 = val;
        self
    }
    pub fn cublasSsyr2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f32, *const f32, i64, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSsyr2_v2_64 = val;
        self
    }
    pub fn cublasDsyr2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDsyr2_v2 = val;
        self
    }
    pub fn cublasDsyr2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f64, *const f64, i64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDsyr2_v2_64 = val;
        self
    }
    pub fn cublasCsyr2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCsyr2_v2 = val;
        self
    }
    pub fn cublasCsyr2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyr2_v2_64 = val;
        self
    }
    pub fn cublasZsyr2_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZsyr2_v2 = val;
        self
    }
    pub fn cublasZsyr2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZsyr2_v2_64 = val;
        self
    }
    pub fn cublasCher2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCher2_v2 = val;
        self
    }
    pub fn cublasCher2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCher2_v2_64 = val;
        self
    }
    pub fn cublasZher2_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZher2_v2 = val;
        self
    }
    pub fn cublasZher2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZher2_v2_64 = val;
        self
    }
    pub fn cublasSspr2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSspr2_v2 = val;
        self
    }
    pub fn cublasSspr2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f32, *const f32, i64, *const f32, i64, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSspr2_v2_64 = val;
        self
    }
    pub fn cublasDspr2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDspr2_v2 = val;
        self
    }
    pub fn cublasDspr2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const f64, *const f64, i64, *const f64, i64, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDspr2_v2_64 = val;
        self
    }
    pub fn cublasChpr2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasChpr2_v2 = val;
        self
    }
    pub fn cublasChpr2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasChpr2_v2_64 = val;
        self
    }
    pub fn cublasZhpr2_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZhpr2_v2 = val;
        self
    }
    pub fn cublasZhpr2_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZhpr2_v2_64 = val;
        self
    }
    pub fn cublasSgemvBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const *const f32,
                ::std::os::raw::c_int,
                *const *const f32,
                ::std::os::raw::c_int,
                *const f32,
                *const *mut f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgemvBatched = val;
        self
    }
    pub fn cublasSgemvBatched_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const f32, *const *const f32, i64, *const *const f32, i64, *const f32, *const *mut f32, i64, i64) -> cublasStatus_t>) -> Self {
        self.cublasSgemvBatched_64 = val;
        self
    }
    pub fn cublasDgemvBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                *const *const f64,
                ::std::os::raw::c_int,
                *const *const f64,
                ::std::os::raw::c_int,
                *const f64,
                *const *mut f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgemvBatched = val;
        self
    }
    pub fn cublasDgemvBatched_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const f64, *const *const f64, i64, *const *const f64, i64, *const f64, *const *mut f64, i64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDgemvBatched_64 = val;
        self
    }
    pub fn cublasCgemvBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const *const cuComplex,
                ::std::os::raw::c_int,
                *const *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const *mut cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemvBatched = val;
        self
    }
    pub fn cublasCgemvBatched_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const cuComplex, *const *const cuComplex, i64, *const *const cuComplex, i64, *const cuComplex, *const *mut cuComplex, i64, i64) -> cublasStatus_t>) -> Self {
        self.cublasCgemvBatched_64 = val;
        self
    }
    pub fn cublasZgemvBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgemvBatched = val;
        self
    }
    pub fn cublasZgemvBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const cuDoubleComplex, *const *const cuDoubleComplex, i64, *const *const cuDoubleComplex, i64, *const cuDoubleComplex, *const *mut cuDoubleComplex, i64, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZgemvBatched_64 = val;
        self
    }
    pub fn cublasSgemvStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f32,
                *mut f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgemvStridedBatched = val;
        self
    }
    pub fn cublasSgemvStridedBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const f32, *const f32, i64, ::std::os::raw::c_longlong, *const f32, i64, ::std::os::raw::c_longlong, *const f32, *mut f32, i64, ::std::os::raw::c_longlong, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgemvStridedBatched_64 = val;
        self
    }
    pub fn cublasDgemvStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                *const f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f64,
                *mut f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgemvStridedBatched = val;
        self
    }
    pub fn cublasDgemvStridedBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const f64, *const f64, i64, ::std::os::raw::c_longlong, *const f64, i64, ::std::os::raw::c_longlong, *const f64, *mut f64, i64, ::std::os::raw::c_longlong, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgemvStridedBatched_64 = val;
        self
    }
    pub fn cublasCgemvStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemvStridedBatched = val;
        self
    }
    pub fn cublasCgemvStridedBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, i64, i64, *const cuComplex, *const cuComplex, i64, ::std::os::raw::c_longlong, *const cuComplex, i64, ::std::os::raw::c_longlong, *const cuComplex, *mut cuComplex, i64, ::std::os::raw::c_longlong, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgemvStridedBatched_64 = val;
        self
    }
    pub fn cublasZgemvStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgemvStridedBatched = val;
        self
    }
    pub fn cublasZgemvStridedBatched_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                i64,
                i64,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                i64,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                i64,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                i64,
                ::std::os::raw::c_longlong,
                i64,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgemvStridedBatched_64 = val;
        self
    }
    pub fn cublasSgemm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                *mut f32,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgemm_v2 = val;
        self
    }
    pub fn cublasSgemm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const f32, *const f32, i64, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSgemm_v2_64 = val;
        self
    }
    pub fn cublasDgemm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                *mut f64,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgemm_v2 = val;
        self
    }
    pub fn cublasDgemm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const f64, *const f64, i64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDgemm_v2_64 = val;
        self
    }
    pub fn cublasCgemm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemm_v2 = val;
        self
    }
    pub fn cublasCgemm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCgemm_v2_64 = val;
        self
    }
    pub fn cublasCgemm3m(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemm3m = val;
        self
    }
    pub fn cublasCgemm3m_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCgemm3m_64 = val;
        self
    }
    pub fn cublasCgemm3mEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemm3mEx = val;
        self
    }
    pub fn cublasCgemm3mEx_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                i64,
                i64,
                i64,
                *const cuComplex,
                *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                *const cuComplex,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                i64,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemm3mEx_64 = val;
        self
    }
    pub fn cublasZgemm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgemm_v2 = val;
        self
    }
    pub fn cublasZgemm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZgemm_v2_64 = val;
        self
    }
    pub fn cublasZgemm3m(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgemm3m = val;
        self
    }
    pub fn cublasZgemm3m_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZgemm3m_64 = val;
        self
    }
    pub fn cublasSgemmEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const f32,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgemmEx = val;
        self
    }
    pub fn cublasSgemmEx_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const f32, *const ::std::os::raw::c_void, cudaDataType, i64, *const ::std::os::raw::c_void, cudaDataType, i64, *const f32, *mut ::std::os::raw::c_void, cudaDataType, i64) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgemmEx_64 = val;
        self
    }
    pub fn cublasGemmEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                cublasComputeType_t,
                cublasGemmAlgo_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasGemmEx = val;
        self
    }
    pub fn cublasGemmEx_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                i64,
                i64,
                i64,
                *const ::std::os::raw::c_void,
                *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                *const ::std::os::raw::c_void,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                i64,
                cublasComputeType_t,
                cublasGemmAlgo_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasGemmEx_64 = val;
        self
    }
    pub fn cublasCgemmEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemmEx = val;
        self
    }
    pub fn cublasCgemmEx_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                i64,
                i64,
                i64,
                *const cuComplex,
                *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                *const cuComplex,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                i64,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemmEx_64 = val;
        self
    }
    pub fn cublasSsyrk_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSsyrk_v2 = val;
        self
    }
    pub fn cublasSsyrk_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f32, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSsyrk_v2_64 = val;
        self
    }
    pub fn cublasDsyrk_v2(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDsyrk_v2 = val;
        self
    }
    pub fn cublasDsyrk_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDsyrk_v2_64 = val;
        self
    }
    pub fn cublasCsyrk_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCsyrk_v2 = val;
        self
    }
    pub fn cublasCsyrk_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyrk_v2_64 = val;
        self
    }
    pub fn cublasZsyrk_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZsyrk_v2 = val;
        self
    }
    pub fn cublasZsyrk_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZsyrk_v2_64 = val;
        self
    }
    pub fn cublasCsyrkEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCsyrkEx = val;
        self
    }
    pub fn cublasCsyrkEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuComplex, *const ::std::os::raw::c_void, cudaDataType, i64, *const cuComplex, *mut ::std::os::raw::c_void, cudaDataType, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyrkEx_64 = val;
        self
    }
    pub fn cublasCsyrk3mEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCsyrk3mEx = val;
        self
    }
    pub fn cublasCsyrk3mEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuComplex, *const ::std::os::raw::c_void, cudaDataType, i64, *const cuComplex, *mut ::std::os::raw::c_void, cudaDataType, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyrk3mEx_64 = val;
        self
    }
    pub fn cublasCherk_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const cuComplex, ::std::os::raw::c_int, *const f32, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCherk_v2 = val;
        self
    }
    pub fn cublasCherk_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f32, *const cuComplex, i64, *const f32, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCherk_v2_64 = val;
        self
    }
    pub fn cublasZherk_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const cuDoubleComplex, ::std::os::raw::c_int, *const f64, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZherk_v2 = val;
        self
    }
    pub fn cublasZherk_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f64, *const cuDoubleComplex, i64, *const f64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZherk_v2_64 = val;
        self
    }
    pub fn cublasCherkEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const f32,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCherkEx = val;
        self
    }
    pub fn cublasCherkEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f32, *const ::std::os::raw::c_void, cudaDataType, i64, *const f32, *mut ::std::os::raw::c_void, cudaDataType, i64) -> cublasStatus_t>) -> Self {
        self.cublasCherkEx_64 = val;
        self
    }
    pub fn cublasCherk3mEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const f32,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCherk3mEx = val;
        self
    }
    pub fn cublasCherk3mEx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f32, *const ::std::os::raw::c_void, cudaDataType, i64, *const f32, *mut ::std::os::raw::c_void, cudaDataType, i64) -> cublasStatus_t>) -> Self {
        self.cublasCherk3mEx_64 = val;
        self
    }
    pub fn cublasSsyr2k_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSsyr2k_v2 = val;
        self
    }
    pub fn cublasSsyr2k_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f32, *const f32, i64, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSsyr2k_v2_64 = val;
        self
    }
    pub fn cublasDsyr2k_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDsyr2k_v2 = val;
        self
    }
    pub fn cublasDsyr2k_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f64, *const f64, i64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDsyr2k_v2_64 = val;
        self
    }
    pub fn cublasCsyr2k_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCsyr2k_v2 = val;
        self
    }
    pub fn cublasCsyr2k_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyr2k_v2_64 = val;
        self
    }
    pub fn cublasZsyr2k_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZsyr2k_v2 = val;
        self
    }
    pub fn cublasZsyr2k_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZsyr2k_v2_64 = val;
        self
    }
    pub fn cublasCher2k_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const f32,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCher2k_v2 = val;
        self
    }
    pub fn cublasCher2k_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const f32, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCher2k_v2_64 = val;
        self
    }
    pub fn cublasZher2k_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const f64,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZher2k_v2 = val;
        self
    }
    pub fn cublasZher2k_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const f64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZher2k_v2_64 = val;
        self
    }
    pub fn cublasSsyrkx(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSsyrkx = val;
        self
    }
    pub fn cublasSsyrkx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f32, *const f32, i64, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSsyrkx_64 = val;
        self
    }
    pub fn cublasDsyrkx(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDsyrkx = val;
        self
    }
    pub fn cublasDsyrkx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const f64, *const f64, i64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDsyrkx_64 = val;
        self
    }
    pub fn cublasCsyrkx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCsyrkx = val;
        self
    }
    pub fn cublasCsyrkx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyrkx_64 = val;
        self
    }
    pub fn cublasZsyrkx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZsyrkx = val;
        self
    }
    pub fn cublasZsyrkx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZsyrkx_64 = val;
        self
    }
    pub fn cublasCherkx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const f32,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCherkx = val;
        self
    }
    pub fn cublasCherkx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const f32, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCherkx_64 = val;
        self
    }
    pub fn cublasZherkx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const f64,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZherkx = val;
        self
    }
    pub fn cublasZherkx_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, cublasOperation_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const f64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZherkx_64 = val;
        self
    }
    pub fn cublasSsymm_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSsymm_v2 = val;
        self
    }
    pub fn cublasSsymm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, i64, i64, *const f32, *const f32, i64, *const f32, i64, *const f32, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSsymm_v2_64 = val;
        self
    }
    pub fn cublasDsymm_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDsymm_v2 = val;
        self
    }
    pub fn cublasDsymm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, i64, i64, *const f64, *const f64, i64, *const f64, i64, *const f64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDsymm_v2_64 = val;
        self
    }
    pub fn cublasCsymm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCsymm_v2 = val;
        self
    }
    pub fn cublasCsymm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCsymm_v2_64 = val;
        self
    }
    pub fn cublasZsymm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZsymm_v2 = val;
        self
    }
    pub fn cublasZsymm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZsymm_v2_64 = val;
        self
    }
    pub fn cublasChemm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasChemm_v2 = val;
        self
    }
    pub fn cublasChemm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *const cuComplex, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasChemm_v2_64 = val;
        self
    }
    pub fn cublasZhemm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZhemm_v2 = val;
        self
    }
    pub fn cublasZhemm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZhemm_v2_64 = val;
        self
    }
    pub fn cublasStrsm_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasStrsm_v2 = val;
        self
    }
    pub fn cublasStrsm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f32, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasStrsm_v2_64 = val;
        self
    }
    pub fn cublasDtrsm_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDtrsm_v2 = val;
        self
    }
    pub fn cublasDtrsm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDtrsm_v2_64 = val;
        self
    }
    pub fn cublasCtrsm_v2(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCtrsm_v2 = val;
        self
    }
    pub fn cublasCtrsm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuComplex, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCtrsm_v2_64 = val;
        self
    }
    pub fn cublasZtrsm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                cublasDiagType_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZtrsm_v2 = val;
        self
    }
    pub fn cublasZtrsm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZtrsm_v2_64 = val;
        self
    }
    pub fn cublasStrmm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                cublasDiagType_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasStrmm_v2 = val;
        self
    }
    pub fn cublasStrmm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f32, *const f32, i64, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasStrmm_v2_64 = val;
        self
    }
    pub fn cublasDtrmm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                cublasDiagType_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDtrmm_v2 = val;
        self
    }
    pub fn cublasDtrmm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f64, *const f64, i64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDtrmm_v2_64 = val;
        self
    }
    pub fn cublasCtrmm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                cublasDiagType_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCtrmm_v2 = val;
        self
    }
    pub fn cublasCtrmm_v2_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCtrmm_v2_64 = val;
        self
    }
    pub fn cublasZtrmm_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                cublasDiagType_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZtrmm_v2 = val;
        self
    }
    pub fn cublasZtrmm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZtrmm_v2_64 = val;
        self
    }
    pub fn cublasSgemmBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const *const f32,
                ::std::os::raw::c_int,
                *const *const f32,
                ::std::os::raw::c_int,
                *const f32,
                *const *mut f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgemmBatched = val;
        self
    }
    pub fn cublasSgemmBatched_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const f32, *const *const f32, i64, *const *const f32, i64, *const f32, *const *mut f32, i64, i64) -> cublasStatus_t>) -> Self {
        self.cublasSgemmBatched_64 = val;
        self
    }
    pub fn cublasDgemmBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                *const *const f64,
                ::std::os::raw::c_int,
                *const *const f64,
                ::std::os::raw::c_int,
                *const f64,
                *const *mut f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgemmBatched = val;
        self
    }
    pub fn cublasDgemmBatched_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const f64, *const *const f64, i64, *const *const f64, i64, *const f64, *const *mut f64, i64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDgemmBatched_64 = val;
        self
    }
    pub fn cublasCgemmBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const *const cuComplex,
                ::std::os::raw::c_int,
                *const *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const *mut cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemmBatched = val;
        self
    }
    pub fn cublasCgemmBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const cuComplex, *const *const cuComplex, i64, *const *const cuComplex, i64, *const cuComplex, *const *mut cuComplex, i64, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgemmBatched_64 = val;
        self
    }
    pub fn cublasCgemm3mBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const *const cuComplex,
                ::std::os::raw::c_int,
                *const *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const *mut cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemm3mBatched = val;
        self
    }
    pub fn cublasCgemm3mBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const cuComplex, *const *const cuComplex, i64, *const *const cuComplex, i64, *const cuComplex, *const *mut cuComplex, i64, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgemm3mBatched_64 = val;
        self
    }
    pub fn cublasZgemmBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgemmBatched = val;
        self
    }
    pub fn cublasZgemmBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const cuDoubleComplex, *const *const cuDoubleComplex, i64, *const *const cuDoubleComplex, i64, *const cuDoubleComplex, *const *mut cuDoubleComplex, i64, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZgemmBatched_64 = val;
        self
    }
    pub fn cublasSgemmStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f32,
                *mut f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgemmStridedBatched = val;
        self
    }
    pub fn cublasSgemmStridedBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const f32, *const f32, i64, ::std::os::raw::c_longlong, *const f32, i64, ::std::os::raw::c_longlong, *const f32, *mut f32, i64, ::std::os::raw::c_longlong, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgemmStridedBatched_64 = val;
        self
    }
    pub fn cublasDgemmStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                *const f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f64,
                *mut f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgemmStridedBatched = val;
        self
    }
    pub fn cublasDgemmStridedBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, i64, *const f64, *const f64, i64, ::std::os::raw::c_longlong, *const f64, i64, ::std::os::raw::c_longlong, *const f64, *mut f64, i64, ::std::os::raw::c_longlong, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgemmStridedBatched_64 = val;
        self
    }
    pub fn cublasCgemmStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemmStridedBatched = val;
        self
    }
    pub fn cublasCgemmStridedBatched_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                i64,
                i64,
                i64,
                *const cuComplex,
                *const cuComplex,
                i64,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                i64,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                *mut cuComplex,
                i64,
                ::std::os::raw::c_longlong,
                i64,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemmStridedBatched_64 = val;
        self
    }
    pub fn cublasCgemm3mStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemm3mStridedBatched = val;
        self
    }
    pub fn cublasCgemm3mStridedBatched_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                i64,
                i64,
                i64,
                *const cuComplex,
                *const cuComplex,
                i64,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                i64,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                *mut cuComplex,
                i64,
                ::std::os::raw::c_longlong,
                i64,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemm3mStridedBatched_64 = val;
        self
    }
    pub fn cublasZgemmStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgemmStridedBatched = val;
        self
    }
    pub fn cublasZgemmStridedBatched_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                i64,
                i64,
                i64,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                i64,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                i64,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                i64,
                ::std::os::raw::c_longlong,
                i64,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgemmStridedBatched_64 = val;
        self
    }
    pub fn cublasGemmBatchedEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                *const *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                *const *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cublasComputeType_t,
                cublasGemmAlgo_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasGemmBatchedEx = val;
        self
    }
    pub fn cublasGemmBatchedEx_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                i64,
                i64,
                i64,
                *const ::std::os::raw::c_void,
                *const *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                *const *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                *const ::std::os::raw::c_void,
                *const *mut ::std::os::raw::c_void,
                cudaDataType,
                i64,
                i64,
                cublasComputeType_t,
                cublasGemmAlgo_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasGemmBatchedEx_64 = val;
        self
    }
    pub fn cublasGemmStridedBatchedEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const ::std::os::raw::c_void,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                ::std::os::raw::c_int,
                cublasComputeType_t,
                cublasGemmAlgo_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasGemmStridedBatchedEx = val;
        self
    }
    pub fn cublasGemmStridedBatchedEx_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                i64,
                i64,
                i64,
                *const ::std::os::raw::c_void,
                *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                ::std::os::raw::c_longlong,
                *const ::std::os::raw::c_void,
                cudaDataType,
                i64,
                ::std::os::raw::c_longlong,
                *const ::std::os::raw::c_void,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                i64,
                ::std::os::raw::c_longlong,
                i64,
                cublasComputeType_t,
                cublasGemmAlgo_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasGemmStridedBatchedEx_64 = val;
        self
    }
    pub fn cublasSgemmGroupedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                *const cublasOperation_t,
                *const cublasOperation_t,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const f32,
                *const *const f32,
                *const ::std::os::raw::c_int,
                *const *const f32,
                *const ::std::os::raw::c_int,
                *const f32,
                *const *mut f32,
                *const ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgemmGroupedBatched = val;
        self
    }
    pub fn cublasSgemmGroupedBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, *const cublasOperation_t, *const cublasOperation_t, *const i64, *const i64, *const i64, *const f32, *const *const f32, *const i64, *const *const f32, *const i64, *const f32, *const *mut f32, *const i64, i64, *const i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgemmGroupedBatched_64 = val;
        self
    }
    pub fn cublasDgemmGroupedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                *const cublasOperation_t,
                *const cublasOperation_t,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const f64,
                *const *const f64,
                *const ::std::os::raw::c_int,
                *const *const f64,
                *const ::std::os::raw::c_int,
                *const f64,
                *const *mut f64,
                *const ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgemmGroupedBatched = val;
        self
    }
    pub fn cublasDgemmGroupedBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, *const cublasOperation_t, *const cublasOperation_t, *const i64, *const i64, *const i64, *const f64, *const *const f64, *const i64, *const *const f64, *const i64, *const f64, *const *mut f64, *const i64, i64, *const i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgemmGroupedBatched_64 = val;
        self
    }
    pub fn cublasGemmGroupedBatchedEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                *const cublasOperation_t,
                *const cublasOperation_t,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                *const *const ::std::os::raw::c_void,
                cudaDataType_t,
                *const ::std::os::raw::c_int,
                *const *const ::std::os::raw::c_void,
                cudaDataType_t,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_void,
                *const *mut ::std::os::raw::c_void,
                cudaDataType_t,
                *const ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                cublasComputeType_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasGemmGroupedBatchedEx = val;
        self
    }
    pub fn cublasGemmGroupedBatchedEx_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                *const cublasOperation_t,
                *const cublasOperation_t,
                *const i64,
                *const i64,
                *const i64,
                *const ::std::os::raw::c_void,
                *const *const ::std::os::raw::c_void,
                cudaDataType_t,
                *const i64,
                *const *const ::std::os::raw::c_void,
                cudaDataType_t,
                *const i64,
                *const ::std::os::raw::c_void,
                *const *mut ::std::os::raw::c_void,
                cudaDataType_t,
                *const i64,
                i64,
                *const i64,
                cublasComputeType_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasGemmGroupedBatchedEx_64 = val;
        self
    }
    pub fn cublasSgeam(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgeam = val;
        self
    }
    pub fn cublasSgeam_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, *const f32, *const f32, i64, *const f32, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSgeam_64 = val;
        self
    }
    pub fn cublasDgeam(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgeam = val;
        self
    }
    pub fn cublasDgeam_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, *const f64, *const f64, i64, *const f64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDgeam_64 = val;
        self
    }
    pub fn cublasCgeam(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgeam = val;
        self
    }
    pub fn cublasCgeam_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, *const cuComplex, *const cuComplex, i64, *const cuComplex, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCgeam_64 = val;
        self
    }
    pub fn cublasZgeam(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgeam = val;
        self
    }
    pub fn cublasZgeam_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, cublasOperation_t, i64, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *const cuDoubleComplex, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZgeam_64 = val;
        self
    }
    pub fn cublasStrsmBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                cublasDiagType_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                *const *const f32,
                ::std::os::raw::c_int,
                *const *mut f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasStrsmBatched = val;
        self
    }
    pub fn cublasStrsmBatched_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f32, *const *const f32, i64, *const *mut f32, i64, i64) -> cublasStatus_t>) -> Self {
        self.cublasStrsmBatched_64 = val;
        self
    }
    pub fn cublasDtrsmBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                cublasDiagType_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                *const *const f64,
                ::std::os::raw::c_int,
                *const *mut f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDtrsmBatched = val;
        self
    }
    pub fn cublasDtrsmBatched_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const f64, *const *const f64, i64, *const *mut f64, i64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDtrsmBatched_64 = val;
        self
    }
    pub fn cublasCtrsmBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                cublasDiagType_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const *const cuComplex,
                ::std::os::raw::c_int,
                *const *mut cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCtrsmBatched = val;
        self
    }
    pub fn cublasCtrsmBatched_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuComplex, *const *const cuComplex, i64, *const *mut cuComplex, i64, i64) -> cublasStatus_t>) -> Self {
        self.cublasCtrsmBatched_64 = val;
        self
    }
    pub fn cublasZtrsmBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                cublasDiagType_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZtrsmBatched = val;
        self
    }
    pub fn cublasZtrsmBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, cublasDiagType_t, i64, i64, *const cuDoubleComplex, *const *const cuDoubleComplex, i64, *const *mut cuDoubleComplex, i64, i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZtrsmBatched_64 = val;
        self
    }
    pub fn cublasSdgmm(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSdgmm = val;
        self
    }
    pub fn cublasSdgmm_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, i64, i64, *const f32, i64, *const f32, i64, *mut f32, i64) -> cublasStatus_t>) -> Self {
        self.cublasSdgmm_64 = val;
        self
    }
    pub fn cublasDdgmm(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDdgmm = val;
        self
    }
    pub fn cublasDdgmm_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, i64, i64, *const f64, i64, *const f64, i64, *mut f64, i64) -> cublasStatus_t>) -> Self {
        self.cublasDdgmm_64 = val;
        self
    }
    pub fn cublasCdgmm(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCdgmm = val;
        self
    }
    pub fn cublasCdgmm_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, i64, i64, *const cuComplex, i64, *const cuComplex, i64, *mut cuComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasCdgmm_64 = val;
        self
    }
    pub fn cublasZdgmm(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZdgmm = val;
        self
    }
    pub fn cublasZdgmm_64(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasSideMode_t, i64, i64, *const cuDoubleComplex, i64, *const cuDoubleComplex, i64, *mut cuDoubleComplex, i64) -> cublasStatus_t>) -> Self {
        self.cublasZdgmm_64 = val;
        self
    }
    pub fn cublasSmatinvBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *const f32, ::std::os::raw::c_int, *const *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSmatinvBatched = val;
        self
    }
    pub fn cublasDmatinvBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *const f64, ::std::os::raw::c_int, *const *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDmatinvBatched = val;
        self
    }
    pub fn cublasCmatinvBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *const cuComplex, ::std::os::raw::c_int, *const *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCmatinvBatched = val;
        self
    }
    pub fn cublasZmatinvBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *const cuDoubleComplex, ::std::os::raw::c_int, *const *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZmatinvBatched = val;
        self
    }
    pub fn cublasSgeqrfBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const *mut f32, ::std::os::raw::c_int, *const *mut f32, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSgeqrfBatched = val;
        self
    }
    pub fn cublasDgeqrfBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const *mut f64, ::std::os::raw::c_int, *const *mut f64, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDgeqrfBatched = val;
        self
    }
    pub fn cublasCgeqrfBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const *mut cuComplex, ::std::os::raw::c_int, *const *mut cuComplex, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCgeqrfBatched = val;
        self
    }
    pub fn cublasZgeqrfBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const *mut cuDoubleComplex, ::std::os::raw::c_int, *const *mut cuDoubleComplex, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZgeqrfBatched = val;
        self
    }
    pub fn cublasSgelsBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const *mut f32,
                ::std::os::raw::c_int,
                *const *mut f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgelsBatched = val;
        self
    }
    pub fn cublasDgelsBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const *mut f64,
                ::std::os::raw::c_int,
                *const *mut f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgelsBatched = val;
        self
    }
    pub fn cublasCgelsBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const *mut cuComplex,
                ::std::os::raw::c_int,
                *const *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgelsBatched = val;
        self
    }
    pub fn cublasZgelsBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *const *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgelsBatched = val;
        self
    }
    pub fn cublasStpttr(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStpttr = val;
        self
    }
    pub fn cublasDtpttr(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtpttr = val;
        self
    }
    pub fn cublasCtpttr(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtpttr = val;
        self
    }
    pub fn cublasZtpttr(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZtpttr = val;
        self
    }
    pub fn cublasStrttp(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasStrttp = val;
        self
    }
    pub fn cublasDtrttp(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDtrttp = val;
        self
    }
    pub fn cublasCtrttp(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCtrttp = val;
        self
    }
    pub fn cublasZtrttp(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZtrttp = val;
        self
    }
    pub fn cublasSgetrfBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSgetrfBatched = val;
        self
    }
    pub fn cublasDgetrfBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDgetrfBatched = val;
        self
    }
    pub fn cublasCgetrfBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCgetrfBatched = val;
        self
    }
    pub fn cublasZgetrfBatched(mut self, val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZgetrfBatched = val;
        self
    }
    pub fn cublasSgetriBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *const f32, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgetriBatched = val;
        self
    }
    pub fn cublasDgetriBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *const f64, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgetriBatched = val;
        self
    }
    pub fn cublasCgetriBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *const cuComplex, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgetriBatched = val;
        self
    }
    pub fn cublasZgetriBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cublasHandle_t, ::std::os::raw::c_int, *const *const cuDoubleComplex, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZgetriBatched = val;
        self
    }
    pub fn cublasSgetrsBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const *const f32, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgetrsBatched = val;
        self
    }
    pub fn cublasDgetrsBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(cublasHandle_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const *const f64, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgetrsBatched = val;
        self
    }
    pub fn cublasCgetrsBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const *const cuComplex,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgetrsBatched = val;
        self
    }
    pub fn cublasZgetrsBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgetrsBatched = val;
        self
    }
    pub fn cublasUint8gemmBias(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasHandle_t,
                cublasOperation_t,
                cublasOperation_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_uchar,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const ::std::os::raw::c_uchar,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_uchar,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasUint8gemmBias = val;
        self
    }
}
pub unsafe fn cublasCreate_v2() -> Result<cublasHandle_t, crate::sys::cublasStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cublasHandle_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasCreate_v2(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cublasHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasDestroy_v2(handle: cublasHandle_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDestroy_v2(handle) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetVersion_v2(handle: cublasHandle_t) -> Result<i32, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetVersion_v2(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasGetProperty(type_: libraryPropertyType) -> Result<i32, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasGetCudartVersion() -> usize {
    unsafe { crate::sys::cublasGetCudartVersion() }
}
pub unsafe fn cublasSetWorkspace_v2<T0: types::CudaAsMutPtr>(handle: cublasHandle_t, mut workspace: T0, workspaceSizeInBytes: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetWorkspace_v2(handle, workspace.as_mut_ptr() as *mut _, workspaceSizeInBytes) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetStream_v2(handle: cublasHandle_t, streamId: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetStream_v2(handle, streamId) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetStream_v2(handle: cublasHandle_t) -> Result<cudaStream_t, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetStream_v2(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaStream_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasGetPointerMode_v2(handle: cublasHandle_t) -> Result<cublasPointerMode_t, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cublasPointerMode_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetPointerMode_v2(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cublasPointerMode_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetPointerMode_v2(handle: cublasHandle_t, mode: cublasPointerMode_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetPointerMode_v2(handle, mode) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetAtomicsMode(handle: cublasHandle_t) -> Result<cublasAtomicsMode_t, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cublasAtomicsMode_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetAtomicsMode(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cublasAtomicsMode_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetAtomicsMode(handle: cublasHandle_t, mode: cublasAtomicsMode_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetAtomicsMode(handle, mode) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetMathMode(handle: cublasHandle_t) -> Result<cublasMath_t, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cublasMath_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetMathMode(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cublasMath_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetMathMode(handle: cublasHandle_t, mode: cublasMath_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetMathMode(handle, mode) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetSmCountTarget(handle: cublasHandle_t) -> Result<i32, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetSmCountTarget(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetSmCountTarget(handle: cublasHandle_t, smCountTarget: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetSmCountTarget(handle, smCountTarget as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetEmulationStrategy(handle: cublasHandle_t) -> Result<cublasEmulationStrategy_t, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cublasEmulationStrategy_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetEmulationStrategy(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cublasEmulationStrategy_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetEmulationStrategy(handle: cublasHandle_t, emulationStrategy: cublasEmulationStrategy_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetEmulationStrategy(handle, emulationStrategy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetEmulationSpecialValuesSupport(handle: cublasHandle_t) -> Result<cudaEmulationSpecialValuesSupport, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaEmulationSpecialValuesSupport> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetEmulationSpecialValuesSupport(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaEmulationSpecialValuesSupport) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetEmulationSpecialValuesSupport(handle: cublasHandle_t, mask: cudaEmulationSpecialValuesSupport) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetEmulationSpecialValuesSupport(handle, mask) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetFixedPointEmulationMantissaControl(handle: cublasHandle_t) -> Result<cudaEmulationMantissaControl, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaEmulationMantissaControl> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetFixedPointEmulationMantissaControl(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaEmulationMantissaControl) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetFixedPointEmulationMantissaControl(handle: cublasHandle_t, mantissaControl: cudaEmulationMantissaControl) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetFixedPointEmulationMantissaControl(handle, mantissaControl) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetFixedPointEmulationMaxMantissaBitCount(handle: cublasHandle_t) -> Result<i32, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetFixedPointEmulationMaxMantissaBitCount(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetFixedPointEmulationMaxMantissaBitCount(handle: cublasHandle_t, maxMantissaBitCount: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetFixedPointEmulationMaxMantissaBitCount(handle, maxMantissaBitCount as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetFixedPointEmulationMantissaBitOffset(handle: cublasHandle_t) -> Result<i32, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetFixedPointEmulationMantissaBitOffset(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetFixedPointEmulationMantissaBitOffset(handle: cublasHandle_t, mantissaBitOffset: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetFixedPointEmulationMantissaBitOffset(handle, mantissaBitOffset as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetFixedPointEmulationMantissaBitCountPointer(handle: cublasHandle_t) -> Result<*mut ::std::os::raw::c_int, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<*mut ::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetFixedPointEmulationMantissaBitCountPointer(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as *mut ::std::os::raw::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetFixedPointEmulationMantissaBitCountPointer<T0: types::CudaAsMutPtr>(handle: cublasHandle_t, mut mantissaBitCount: T0) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetFixedPointEmulationMantissaBitCountPointer(handle, mantissaBitCount.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetStatusName(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cublasGetStatusName(status) }
}
pub unsafe fn cublasGetStatusString(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cublasGetStatusString(status) }
}
pub unsafe fn cublasLoggerConfigure<T0: types::CudaAsPtr>(logIsOn: i32, logToStdOut: i32, logToStdErr: i32, logFileName: T0) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLoggerConfigure(logIsOn as _, logToStdOut as _, logToStdErr as _, logFileName.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetLoggerCallback(userCallback: cublasLogCallback) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetLoggerCallback(userCallback) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetLoggerCallback() -> Result<cublasLogCallback, crate::sys::cublasStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cublasLogCallback> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasGetLoggerCallback(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cublasLogCallback) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasSetVector<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(n: i32, elemSize: i32, x: T0, incx: i32, mut devicePtr: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetVector(n as _, elemSize as _, x.as_const_ptr() as *const _, incx as _, devicePtr.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetVector_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(n: i64, elemSize: i64, x: T0, incx: i64, mut devicePtr: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetVector_64(n, elemSize, x.as_const_ptr() as *const _, incx, devicePtr.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetVector<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(n: i32, elemSize: i32, x: T0, incx: i32, mut y: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetVector(n as _, elemSize as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetVector_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(n: i64, elemSize: i64, x: T0, incx: i64, mut y: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetVector_64(n, elemSize, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetMatrix<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(rows: i32, cols: i32, elemSize: i32, A: T0, lda: i32, mut B: T1, ldb: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetMatrix(rows as _, cols as _, elemSize as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetMatrix_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(rows: i64, cols: i64, elemSize: i64, A: T0, lda: i64, mut B: T1, ldb: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetMatrix_64(rows, cols, elemSize, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetMatrix<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(rows: i32, cols: i32, elemSize: i32, A: T0, lda: i32, mut B: T1, ldb: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetMatrix(rows as _, cols as _, elemSize as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetMatrix_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(rows: i64, cols: i64, elemSize: i64, A: T0, lda: i64, mut B: T1, ldb: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetMatrix_64(rows, cols, elemSize, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetVectorAsync<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(n: i32, elemSize: i32, hostPtr: T0, incx: i32, mut devicePtr: T1, incy: i32, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetVectorAsync(n as _, elemSize as _, hostPtr.as_const_ptr() as *const _, incx as _, devicePtr.as_mut_ptr() as *mut _, incy as _, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetVectorAsync_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(n: i64, elemSize: i64, hostPtr: T0, incx: i64, mut devicePtr: T1, incy: i64, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetVectorAsync_64(n, elemSize, hostPtr.as_const_ptr() as *const _, incx, devicePtr.as_mut_ptr() as *mut _, incy, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetVectorAsync<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(n: i32, elemSize: i32, devicePtr: T0, incx: i32, mut hostPtr: T1, incy: i32, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetVectorAsync(n as _, elemSize as _, devicePtr.as_const_ptr() as *const _, incx as _, hostPtr.as_mut_ptr() as *mut _, incy as _, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetVectorAsync_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(n: i64, elemSize: i64, devicePtr: T0, incx: i64, mut hostPtr: T1, incy: i64, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetVectorAsync_64(n, elemSize, devicePtr.as_const_ptr() as *const _, incx, hostPtr.as_mut_ptr() as *mut _, incy, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetMatrixAsync<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(rows: i32, cols: i32, elemSize: i32, A: T0, lda: i32, mut B: T1, ldb: i32, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetMatrixAsync(rows as _, cols as _, elemSize as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetMatrixAsync_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(rows: i64, cols: i64, elemSize: i64, A: T0, lda: i64, mut B: T1, ldb: i64, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetMatrixAsync_64(rows, cols, elemSize, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetMatrixAsync<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(rows: i32, cols: i32, elemSize: i32, A: T0, lda: i32, mut B: T1, ldb: i32, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetMatrixAsync(rows as _, cols as _, elemSize as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetMatrixAsync_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(rows: i64, cols: i64, elemSize: i64, A: T0, lda: i64, mut B: T1, ldb: i64, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetMatrixAsync_64(rows, cols, elemSize, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasXerbla<T0: types::CudaAsPtr>(srName: T0, info: i32) -> () {
    unsafe { crate::sys::cublasXerbla(srName.as_const_ptr() as *const _, info as _) }
}
pub unsafe fn cublasNrm2Ex<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, xType: cudaDataType, incx: i32, mut result: T1, resultType: cudaDataType, executionType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasNrm2Ex(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasNrm2Ex_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, xType: cudaDataType, incx: i64, mut result: T1, resultType: cudaDataType, executionType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasNrm2Ex_64(handle, n, x.as_const_ptr() as *const _, xType, incx, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSnrm2_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSnrm2_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSnrm2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSnrm2_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDnrm2_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDnrm2_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDnrm2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDnrm2_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScnrm2_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScnrm2_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScnrm2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScnrm2_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDznrm2_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDznrm2_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDznrm2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDznrm2_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDotEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    n: i32,
    x: T0,
    xType: cudaDataType,
    incx: i32,
    y: T1,
    yType: cudaDataType,
    incy: i32,
    mut result: T2,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDotEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, y.as_const_ptr() as *const _, yType, incy as _, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDotEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    n: i64,
    x: T0,
    xType: cudaDataType,
    incx: i64,
    y: T1,
    yType: cudaDataType,
    incy: i64,
    mut result: T2,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDotEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, y.as_const_ptr() as *const _, yType, incy, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDotcEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    n: i32,
    x: T0,
    xType: cudaDataType,
    incx: i32,
    y: T1,
    yType: cudaDataType,
    incy: i32,
    mut result: T2,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDotcEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, y.as_const_ptr() as *const _, yType, incy as _, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDotcEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    n: i64,
    x: T0,
    xType: cudaDataType,
    incx: i64,
    y: T1,
    yType: cudaDataType,
    incy: i64,
    mut result: T2,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDotcEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, y.as_const_ptr() as *const _, yType, incy, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSdot_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, y: T1, incy: i32, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSdot_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSdot_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, y: T1, incy: i64, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSdot_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDdot_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, y: T1, incy: i32, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDdot_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDdot_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, y: T1, incy: i64, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDdot_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdotu_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, y: T1, incy: i32, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdotu_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdotu_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, y: T1, incy: i64, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdotu_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdotc_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, y: T1, incy: i32, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdotc_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdotc_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, y: T1, incy: i64, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdotc_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdotu_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, y: T1, incy: i32, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdotu_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdotu_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, y: T1, incy: i64, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdotu_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdotc_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, y: T1, incy: i32, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdotc_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdotc_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, y: T1, incy: i64, mut result: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdotc_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScalEx<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, alphaType: cudaDataType, mut x: T1, xType: cudaDataType, incx: i32, executionType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScalEx(handle, n as _, alpha.as_const_ptr() as *const _, alphaType, x.as_mut_ptr() as *mut _, xType, incx as _, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScalEx_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, alphaType: cudaDataType, mut x: T1, xType: cudaDataType, incx: i64, executionType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScalEx_64(handle, n, alpha.as_const_ptr() as *const _, alphaType, x.as_mut_ptr() as *mut _, xType, incx, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSscal_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSscal_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDscal_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDscal_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCscal_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCscal_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsscal_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsscal_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZscal_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZscal_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdscal_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdscal_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasAxpyEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    n: i32,
    alpha: T0,
    alphaType: cudaDataType,
    x: T1,
    xType: cudaDataType,
    incx: i32,
    mut y: T2,
    yType: cudaDataType,
    incy: i32,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasAxpyEx(handle, n as _, alpha.as_const_ptr() as *const _, alphaType, x.as_const_ptr() as *const _, xType, incx as _, y.as_mut_ptr() as *mut _, yType, incy as _, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasAxpyEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    n: i64,
    alpha: T0,
    alphaType: cudaDataType,
    x: T1,
    xType: cudaDataType,
    incx: i64,
    mut y: T2,
    yType: cudaDataType,
    incy: i64,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasAxpyEx_64(handle, n, alpha.as_const_ptr() as *const _, alphaType, x.as_const_ptr() as *const _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSaxpy_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, x: T1, incx: i32, mut y: T2, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSaxpy_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSaxpy_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, x: T1, incx: i64, mut y: T2, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSaxpy_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDaxpy_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, x: T1, incx: i32, mut y: T2, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDaxpy_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDaxpy_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, x: T1, incx: i64, mut y: T2, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDaxpy_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCaxpy_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, x: T1, incx: i32, mut y: T2, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCaxpy_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCaxpy_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, x: T1, incx: i64, mut y: T2, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCaxpy_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZaxpy_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, alpha: T0, x: T1, incx: i32, mut y: T2, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZaxpy_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZaxpy_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, alpha: T0, x: T1, incx: i64, mut y: T2, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZaxpy_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCopyEx<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, xType: cudaDataType, incx: i32, mut y: T1, yType: cudaDataType, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCopyEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, y.as_mut_ptr() as *mut _, yType, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCopyEx_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, xType: cudaDataType, incx: i64, mut y: T1, yType: cudaDataType, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCopyEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScopy_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut y: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScopy_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScopy_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut y: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScopy_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDcopy_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut y: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDcopy_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDcopy_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut y: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDcopy_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCcopy_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut y: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCcopy_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCcopy_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut y: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCcopy_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZcopy_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut y: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZcopy_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZcopy_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut y: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZcopy_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSswap_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSswap_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSswap_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSswap_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDswap_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDswap_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDswap_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDswap_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCswap_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCswap_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCswap_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCswap_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZswap_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZswap_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZswap_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZswap_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSwapEx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, mut x: T0, xType: cudaDataType, incx: i32, mut y: T1, yType: cudaDataType, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSwapEx(handle, n as _, x.as_mut_ptr() as *mut _, xType, incx as _, y.as_mut_ptr() as *mut _, yType, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSwapEx_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, mut x: T0, xType: cudaDataType, incx: i64, mut y: T1, yType: cudaDataType, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSwapEx_64(handle, n, x.as_mut_ptr() as *mut _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIsamax_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIsamax_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIsamax_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIsamax_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIdamax_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIdamax_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIdamax_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIdamax_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIcamax_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIcamax_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIcamax_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIcamax_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIzamax_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIzamax_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIzamax_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIzamax_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIamaxEx<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, xType: cudaDataType, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIamaxEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIamaxEx_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, xType: cudaDataType, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIamaxEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIsamin_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIsamin_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIsamin_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIsamin_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIdamin_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIdamin_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIdamin_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIdamin_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIcamin_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIcamin_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIcamin_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIcamin_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIzamin_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIzamin_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIzamin_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIzamin_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIaminEx<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, xType: cudaDataType, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIaminEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIaminEx_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, xType: cudaDataType, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIaminEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasAsumEx<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, xType: cudaDataType, incx: i32, mut result: T1, resultType: cudaDataType, executiontype: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasAsumEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, result.as_mut_ptr() as *mut _, resultType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasAsumEx_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, xType: cudaDataType, incx: i64, mut result: T1, resultType: cudaDataType, executiontype: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasAsumEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, result.as_mut_ptr() as *mut _, resultType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSasum_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSasum_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSasum_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSasum_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDasum_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDasum_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDasum_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDasum_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScasum_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScasum_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScasum_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScasum_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDzasum_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, x: T0, incx: i32, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDzasum_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDzasum_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i64, x: T0, incx: i64, mut result: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDzasum_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrot_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrot_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrot_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrot_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCrot_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCrot_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsrot_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsrot_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZrot_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZrot_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdrot_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdrot_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64, c: T2, s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotEx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i32,
    mut x: T0,
    xType: cudaDataType,
    incx: i32,
    mut y: T1,
    yType: cudaDataType,
    incy: i32,
    c: T2,
    s: T3,
    csType: cudaDataType,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasRotEx(
            handle,
            n as _,
            x.as_mut_ptr() as *mut _,
            xType,
            incx as _,
            y.as_mut_ptr() as *mut _,
            yType,
            incy as _,
            c.as_const_ptr() as *const _,
            s.as_const_ptr() as *const _,
            csType,
            executiontype,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotEx_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i64,
    mut x: T0,
    xType: cudaDataType,
    incx: i64,
    mut y: T1,
    yType: cudaDataType,
    incy: i64,
    c: T2,
    s: T3,
    csType: cudaDataType,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasRotEx_64(handle, n, x.as_mut_ptr() as *mut _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _, csType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrotg_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, mut a: T0, mut b: T1, mut c: T2, mut s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrotg_v2(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrotg_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, mut a: T0, mut b: T1, mut c: T2, mut s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrotg_v2(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCrotg_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, mut a: T0, mut b: T1, mut c: T2, mut s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCrotg_v2(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZrotg_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, mut a: T0, mut b: T1, mut c: T2, mut s: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZrotg_v2(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotgEx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    mut a: T0,
    mut b: T1,
    abType: cudaDataType,
    mut c: T2,
    mut s: T3,
    csType: cudaDataType,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasRotgEx(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, abType, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _, csType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrotm_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32, param: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrotm_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, param.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrotm_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64, param: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrotm_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, param.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrotm_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T0, incx: i32, mut y: T1, incy: i32, param: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrotm_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, param.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrotm_v2_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T0, incx: i64, mut y: T1, incy: i64, param: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrotm_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, param.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotmEx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i32,
    mut x: T0,
    xType: cudaDataType,
    incx: i32,
    mut y: T1,
    yType: cudaDataType,
    incy: i32,
    param: T2,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasRotmEx(handle, n as _, x.as_mut_ptr() as *mut _, xType, incx as _, y.as_mut_ptr() as *mut _, yType, incy as _, param.as_const_ptr() as *const _, paramType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotmEx_64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i64,
    mut x: T0,
    xType: cudaDataType,
    incx: i64,
    mut y: T1,
    yType: cudaDataType,
    incy: i64,
    param: T2,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasRotmEx_64(handle, n, x.as_mut_ptr() as *mut _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy, param.as_const_ptr() as *const _, paramType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrotmg_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(handle: cublasHandle_t, mut d1: T0, mut d2: T1, mut x1: T2, y1: T3, mut param: T4) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrotmg_v2(handle, d1.as_mut_ptr() as *mut _, d2.as_mut_ptr() as *mut _, x1.as_mut_ptr() as *mut _, y1.as_const_ptr() as *const _, param.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrotmg_v2<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(handle: cublasHandle_t, mut d1: T0, mut d2: T1, mut x1: T2, y1: T3, mut param: T4) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrotmg_v2(handle, d1.as_mut_ptr() as *mut _, d2.as_mut_ptr() as *mut _, x1.as_mut_ptr() as *mut _, y1.as_const_ptr() as *const _, param.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotmgEx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    mut d1: T0,
    d1Type: cudaDataType,
    mut d2: T1,
    d2Type: cudaDataType,
    mut x1: T2,
    x1Type: cudaDataType,
    y1: T3,
    y1Type: cudaDataType,
    mut param: T4,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasRotmgEx(
            handle,
            d1.as_mut_ptr() as *mut _,
            d1Type,
            d2.as_mut_ptr() as *mut _,
            d2Type,
            x1.as_mut_ptr() as *mut _,
            x1Type,
            y1.as_const_ptr() as *const _,
            y1Type,
            param.as_mut_ptr() as *mut _,
            paramType,
            executiontype,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemv_v2(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemv_v2_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemv_v2(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemv_v2_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemv_v2(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemv_v2_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemv_v2(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemv_v2_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgbmv_v2(
            handle,
            trans,
            m as _,
            n as _,
            kl as _,
            ku as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgbmv_v2_64(
            handle,
            trans,
            m,
            n,
            kl,
            ku,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgbmv_v2(
            handle,
            trans,
            m as _,
            n as _,
            kl as _,
            ku as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgbmv_v2_64(
            handle,
            trans,
            m,
            n,
            kl,
            ku,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgbmv_v2(
            handle,
            trans,
            m as _,
            n as _,
            kl as _,
            ku as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgbmv_v2_64(
            handle,
            trans,
            m,
            n,
            kl,
            ku,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgbmv_v2(
            handle,
            trans,
            m as _,
            n as _,
            kl as _,
            ku as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgbmv_v2_64(
            handle,
            trans,
            m,
            n,
            kl,
            ku,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrmv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrmv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrmv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrmv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrmv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrmv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrmv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrmv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStbmv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStbmv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtbmv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtbmv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtbmv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtbmv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtbmv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtbmv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStpmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpmv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStpmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpmv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpmv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpmv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpmv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpmv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpmv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpmv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStpsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpsv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStpsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpsv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpsv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpsv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpsv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpsv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T0, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpsv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T0, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpsv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStbsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStbsv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStbsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStbsv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtbsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtbsv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtbsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtbsv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtbsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtbsv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtbsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtbsv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtbsv_v2<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T0, lda: i32, mut x: T1, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtbsv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtbsv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T0, lda: i64, mut x: T1, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtbsv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsymv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsymv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsymv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsymv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsymv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsymv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsymv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsymv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsymv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsymv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsymv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsymv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsymv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZsymv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsymv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZsymv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChemv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasChemv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChemv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasChemv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhemv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZhemv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhemv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZhemv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsbmv_v2(
            handle,
            uplo,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsbmv_v2_64(
            handle,
            uplo,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsbmv_v2(
            handle,
            uplo,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsbmv_v2_64(
            handle,
            uplo,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasChbmv_v2(
            handle,
            uplo,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasChbmv_v2_64(
            handle,
            uplo,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhbmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZhbmv_v2(
            handle,
            uplo,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhbmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZhbmv_v2_64(
            handle,
            uplo,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    AP: T1,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSspmv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            AP.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    AP: T1,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSspmv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            AP.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    AP: T1,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDspmv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            AP.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    AP: T1,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDspmv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            AP.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    AP: T1,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasChpmv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            AP.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    AP: T1,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasChpmv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            AP.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpmv_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T0,
    AP: T1,
    x: T2,
    incx: i32,
    beta: T3,
    mut y: T4,
    incy: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZhpmv_v2(
            handle,
            uplo,
            n as _,
            alpha.as_const_ptr() as *const _,
            AP.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpmv_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T0,
    AP: T1,
    x: T2,
    incx: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZhpmv_v2_64(
            handle,
            uplo,
            n,
            alpha.as_const_ptr() as *const _,
            AP.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSger_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSger_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSger_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSger_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDger_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDger_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDger_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDger_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgeru_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgeru_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgeru_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgeru_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgerc_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgerc_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgerc_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgerc_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgeru_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgeru_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgeru_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgeru_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgerc_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgerc_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgerc_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgerc_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut A: T2, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut A: T2, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut A: T2, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut A: T2, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut A: T2, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut A: T2, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut A: T2, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut A: T2, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut A: T2, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCher_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut A: T2, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCher_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut A: T2, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZher_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut A: T2, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZher_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspr_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut AP: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSspr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspr_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut AP: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSspr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspr_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut AP: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDspr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspr_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut AP: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDspr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpr_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut AP: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasChpr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpr_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut AP: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasChpr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpr_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, mut AP: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZhpr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpr_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, mut AP: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZhpr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCher2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCher2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut A: T3, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZher2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut A: T3, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZher2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspr2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut AP: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSspr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspr2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut AP: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSspr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspr2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut AP: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDspr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspr2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut AP: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDspr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpr2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut AP: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasChpr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpr2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut AP: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasChpr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpr2_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T0, x: T1, incx: i32, y: T2, incy: i32, mut AP: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZhpr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpr2_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T0, x: T1, incx: i64, y: T2, incy: i64, mut AP: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZhpr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    Aarray: T1,
    lda: i32,
    xarray: T2,
    incx: i32,
    beta: T3,
    yarray: T4,
    incy: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemvBatched(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            xarray.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            yarray.as_const_ptr() as *const _,
            incy as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemvBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    Aarray: T1,
    lda: i64,
    xarray: T2,
    incx: i64,
    beta: T3,
    yarray: T4,
    incy: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemvBatched_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda,
            xarray.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            yarray.as_const_ptr() as *const _,
            incy,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    Aarray: T1,
    lda: i32,
    xarray: T2,
    incx: i32,
    beta: T3,
    yarray: T4,
    incy: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemvBatched(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            xarray.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            yarray.as_const_ptr() as *const _,
            incy as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemvBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    Aarray: T1,
    lda: i64,
    xarray: T2,
    incx: i64,
    beta: T3,
    yarray: T4,
    incy: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemvBatched_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda,
            xarray.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            yarray.as_const_ptr() as *const _,
            incy,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    Aarray: T1,
    lda: i32,
    xarray: T2,
    incx: i32,
    beta: T3,
    yarray: T4,
    incy: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemvBatched(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            xarray.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            yarray.as_const_ptr() as *const _,
            incy as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemvBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    Aarray: T1,
    lda: i64,
    xarray: T2,
    incx: i64,
    beta: T3,
    yarray: T4,
    incy: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemvBatched_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda,
            xarray.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            yarray.as_const_ptr() as *const _,
            incy,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    Aarray: T1,
    lda: i32,
    xarray: T2,
    incx: i32,
    beta: T3,
    yarray: T4,
    incy: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemvBatched(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            xarray.as_const_ptr() as *const _,
            incx as _,
            beta.as_const_ptr() as *const _,
            yarray.as_const_ptr() as *const _,
            incy as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemvBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    Aarray: T1,
    lda: i64,
    xarray: T2,
    incx: i64,
    beta: T3,
    yarray: T4,
    incy: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemvBatched_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda,
            xarray.as_const_ptr() as *const _,
            incx,
            beta.as_const_ptr() as *const _,
            yarray.as_const_ptr() as *const _,
            incy,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemvStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    strideA: i64,
    x: T2,
    incx: i32,
    stridex: i64,
    beta: T3,
    mut y: T4,
    incy: i32,
    stridey: i64,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemvStridedBatched(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            x.as_const_ptr() as *const _,
            incx as _,
            stridex as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
            stridey as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemvStridedBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    strideA: i64,
    x: T2,
    incx: i64,
    stridex: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
    stridey: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemvStridedBatched_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            strideA as _,
            x.as_const_ptr() as *const _,
            incx,
            stridex as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
            stridey as _,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemvStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    strideA: i64,
    x: T2,
    incx: i32,
    stridex: i64,
    beta: T3,
    mut y: T4,
    incy: i32,
    stridey: i64,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemvStridedBatched(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            x.as_const_ptr() as *const _,
            incx as _,
            stridex as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
            stridey as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemvStridedBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    strideA: i64,
    x: T2,
    incx: i64,
    stridex: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
    stridey: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemvStridedBatched_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            strideA as _,
            x.as_const_ptr() as *const _,
            incx,
            stridex as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
            stridey as _,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemvStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    strideA: i64,
    x: T2,
    incx: i32,
    stridex: i64,
    beta: T3,
    mut y: T4,
    incy: i32,
    stridey: i64,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemvStridedBatched(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            x.as_const_ptr() as *const _,
            incx as _,
            stridex as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
            stridey as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemvStridedBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    strideA: i64,
    x: T2,
    incx: i64,
    stridex: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
    stridey: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemvStridedBatched_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            strideA as _,
            x.as_const_ptr() as *const _,
            incx,
            stridex as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
            stridey as _,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemvStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    strideA: i64,
    x: T2,
    incx: i32,
    stridex: i64,
    beta: T3,
    mut y: T4,
    incy: i32,
    stridey: i64,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemvStridedBatched(
            handle,
            trans,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            x.as_const_ptr() as *const _,
            incx as _,
            stridex as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy as _,
            stridey as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemvStridedBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    strideA: i64,
    x: T2,
    incx: i64,
    stridex: i64,
    beta: T3,
    mut y: T4,
    incy: i64,
    stridey: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemvStridedBatched_64(
            handle,
            trans,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            strideA as _,
            x.as_const_ptr() as *const _,
            incx,
            stridex as _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            incy,
            stridey as _,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemm_v2(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemm_v2_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemm_v2(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemm_v2_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm_v2(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm_v2_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm3m<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm3m(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm3m_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm3m_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm3mEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i32,
    B: T2,
    Btype: cudaDataType,
    ldb: i32,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm3mEx(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda as _,
            B.as_const_ptr() as *const _,
            Btype,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm3mEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i64,
    B: T2,
    Btype: cudaDataType,
    ldb: i64,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm3mEx_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda,
            B.as_const_ptr() as *const _,
            Btype,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemm_v2(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemm_v2_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemm3m<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemm3m(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemm3m_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemm3m_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemmEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i32,
    B: T2,
    Btype: cudaDataType,
    ldb: i32,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemmEx(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda as _,
            B.as_const_ptr() as *const _,
            Btype,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemmEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i64,
    B: T2,
    Btype: cudaDataType,
    ldb: i64,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemmEx_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda,
            B.as_const_ptr() as *const _,
            Btype,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGemmEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i32,
    B: T2,
    Btype: cudaDataType,
    ldb: i32,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i32,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGemmEx(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda as _,
            B.as_const_ptr() as *const _,
            Btype,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc as _,
            computeType,
            algo,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGemmEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i64,
    B: T2,
    Btype: cudaDataType,
    ldb: i64,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i64,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGemmEx_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda,
            B.as_const_ptr() as *const _,
            Btype,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc,
            computeType,
            algo,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemmEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i32,
    B: T2,
    Btype: cudaDataType,
    ldb: i32,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemmEx(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda as _,
            B.as_const_ptr() as *const _,
            Btype,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemmEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i64,
    B: T2,
    Btype: cudaDataType,
    ldb: i64,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemmEx_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda,
            B.as_const_ptr() as *const _,
            Btype,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyrk_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyrk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyrk_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyrk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyrk_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyrk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyrk_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyrk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrk_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyrk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrk_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyrk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyrk_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyrk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyrk_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyrk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrkEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i32,
    beta: T2,
    mut C: T3,
    Ctype: cudaDataType,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsyrkEx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrkEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i64,
    beta: T2,
    mut C: T3,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyrkEx_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, Atype, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, Ctype, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrk3mEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i32,
    beta: T2,
    mut C: T3,
    Ctype: cudaDataType,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsyrk3mEx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrk3mEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i64,
    beta: T2,
    mut C: T3,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyrk3mEx_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, Atype, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, Ctype, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherk_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCherk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherk_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCherk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZherk_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZherk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZherk_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZherk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherkEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i32,
    beta: T2,
    mut C: T3,
    Ctype: cudaDataType,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCherkEx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherkEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i64,
    beta: T2,
    mut C: T3,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCherkEx_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, Atype, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, Ctype, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherk3mEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i32,
    beta: T2,
    mut C: T3,
    Ctype: cudaDataType,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCherk3mEx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherk3mEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i64,
    beta: T2,
    mut C: T3,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCherk3mEx_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, Atype, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, Ctype, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr2k_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsyr2k_v2(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr2k_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsyr2k_v2_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr2k_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsyr2k_v2(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr2k_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsyr2k_v2_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr2k_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsyr2k_v2(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr2k_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsyr2k_v2_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr2k_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZsyr2k_v2(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr2k_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZsyr2k_v2_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher2k_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCher2k_v2(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher2k_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCher2k_v2_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher2k_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZher2k_v2(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher2k_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZher2k_v2_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyrkx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsyrkx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyrkx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsyrkx_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyrkx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsyrkx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyrkx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsyrkx_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrkx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsyrkx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrkx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsyrkx_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyrkx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZsyrkx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyrkx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZsyrkx_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherkx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCherkx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherkx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCherkx_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZherkx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZherkx(
            handle,
            uplo,
            trans,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZherkx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZherkx_64(
            handle,
            uplo,
            trans,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsymm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsymm_v2(
            handle,
            side,
            uplo,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsymm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSsymm_v2_64(
            handle,
            side,
            uplo,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsymm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsymm_v2(
            handle,
            side,
            uplo,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsymm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDsymm_v2_64(
            handle,
            side,
            uplo,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsymm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsymm_v2(
            handle,
            side,
            uplo,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsymm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCsymm_v2_64(
            handle,
            side,
            uplo,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsymm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZsymm_v2(
            handle,
            side,
            uplo,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsymm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZsymm_v2_64(
            handle,
            side,
            uplo,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChemm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasChemm_v2(
            handle,
            side,
            uplo,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChemm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasChemm_v2_64(
            handle,
            side,
            uplo,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhemm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    beta: T3,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZhemm_v2(
            handle,
            side,
            uplo,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhemm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZhemm_v2_64(
            handle,
            side,
            uplo,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            B.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    mut B: T2,
    ldb: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsm_v2(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    mut B: T2,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    mut B: T2,
    ldb: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsm_v2(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    mut B: T2,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    mut B: T2,
    ldb: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsm_v2(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    mut B: T2,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    mut B: T2,
    ldb: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsm_v2(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    mut B: T2,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrmm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasStrmm_v2(
            handle,
            side,
            uplo,
            trans,
            diag,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrmm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrmm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrmm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDtrmm_v2(
            handle,
            side,
            uplo,
            trans,
            diag,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrmm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrmm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrmm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCtrmm_v2(
            handle,
            side,
            uplo,
            trans,
            diag,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrmm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrmm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrmm_v2<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    mut C: T3,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZtrmm_v2(
            handle,
            side,
            uplo,
            trans,
            diag,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrmm_v2_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    mut C: T3,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrmm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemmBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    Aarray: T1,
    lda: i32,
    Barray: T2,
    ldb: i32,
    beta: T3,
    Carray: T4,
    ldc: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemmBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            Barray.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemmBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    Aarray: T1,
    lda: i64,
    Barray: T2,
    ldb: i64,
    beta: T3,
    Carray: T4,
    ldc: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemmBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda,
            Barray.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemmBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    Aarray: T1,
    lda: i32,
    Barray: T2,
    ldb: i32,
    beta: T3,
    Carray: T4,
    ldc: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemmBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            Barray.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemmBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    Aarray: T1,
    lda: i64,
    Barray: T2,
    ldb: i64,
    beta: T3,
    Carray: T4,
    ldc: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemmBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda,
            Barray.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemmBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    Aarray: T1,
    lda: i32,
    Barray: T2,
    ldb: i32,
    beta: T3,
    Carray: T4,
    ldc: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemmBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            Barray.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemmBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    Aarray: T1,
    lda: i64,
    Barray: T2,
    ldb: i64,
    beta: T3,
    Carray: T4,
    ldc: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemmBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda,
            Barray.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm3mBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    Aarray: T1,
    lda: i32,
    Barray: T2,
    ldb: i32,
    beta: T3,
    Carray: T4,
    ldc: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm3mBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            Barray.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm3mBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    Aarray: T1,
    lda: i64,
    Barray: T2,
    ldb: i64,
    beta: T3,
    Carray: T4,
    ldc: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm3mBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda,
            Barray.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemmBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    Aarray: T1,
    lda: i32,
    Barray: T2,
    ldb: i32,
    beta: T3,
    Carray: T4,
    ldc: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemmBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            Barray.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemmBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    Aarray: T1,
    lda: i64,
    Barray: T2,
    ldb: i64,
    beta: T3,
    Carray: T4,
    ldc: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemmBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda,
            Barray.as_const_ptr() as *const _,
            ldb,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemmStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    strideA: i64,
    B: T2,
    ldb: i32,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i32,
    strideC: i64,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemmStridedBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            strideC as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemmStridedBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    strideA: i64,
    B: T2,
    ldb: i64,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
    strideC: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemmStridedBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
            strideC as _,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemmStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    strideA: i64,
    B: T2,
    ldb: i32,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i32,
    strideC: i64,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemmStridedBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            strideC as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemmStridedBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    strideA: i64,
    B: T2,
    ldb: i64,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
    strideC: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemmStridedBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
            strideC as _,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemmStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    strideA: i64,
    B: T2,
    ldb: i32,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i32,
    strideC: i64,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemmStridedBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            strideC as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemmStridedBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    strideA: i64,
    B: T2,
    ldb: i64,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
    strideC: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemmStridedBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
            strideC as _,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm3mStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    strideA: i64,
    B: T2,
    ldb: i32,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i32,
    strideC: i64,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm3mStridedBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            strideC as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgemm3mStridedBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    strideA: i64,
    B: T2,
    ldb: i64,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
    strideC: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgemm3mStridedBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
            strideC as _,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemmStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    strideA: i64,
    B: T2,
    ldb: i32,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i32,
    strideC: i64,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemmStridedBatched(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            strideC as _,
            batchCount as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgemmStridedBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    strideA: i64,
    B: T2,
    ldb: i64,
    strideB: i64,
    beta: T3,
    mut C: T4,
    ldc: i64,
    strideC: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgemmStridedBatched_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            strideA as _,
            B.as_const_ptr() as *const _,
            ldb,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc,
            strideC as _,
            batchCount,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGemmBatchedEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    Aarray: T1,
    Atype: cudaDataType,
    lda: i32,
    Barray: T2,
    Btype: cudaDataType,
    ldb: i32,
    beta: T3,
    Carray: T4,
    Ctype: cudaDataType,
    ldc: i32,
    batchCount: i32,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGemmBatchedEx(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            Atype,
            lda as _,
            Barray.as_const_ptr() as *const _,
            Btype,
            ldb as _,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            Ctype,
            ldc as _,
            batchCount as _,
            computeType,
            algo,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGemmBatchedEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    Aarray: T1,
    Atype: cudaDataType,
    lda: i64,
    Barray: T2,
    Btype: cudaDataType,
    ldb: i64,
    beta: T3,
    Carray: T4,
    Ctype: cudaDataType,
    ldc: i64,
    batchCount: i64,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGemmBatchedEx_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            Atype,
            lda,
            Barray.as_const_ptr() as *const _,
            Btype,
            ldb,
            beta.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            Ctype,
            ldc,
            batchCount,
            computeType,
            algo,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGemmStridedBatchedEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i32,
    strideA: i64,
    B: T2,
    Btype: cudaDataType,
    ldb: i32,
    strideB: i64,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i32,
    strideC: i64,
    batchCount: i32,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGemmStridedBatchedEx(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            k as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda as _,
            strideA as _,
            B.as_const_ptr() as *const _,
            Btype,
            ldb as _,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc as _,
            strideC as _,
            batchCount as _,
            computeType,
            algo,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGemmStridedBatchedEx_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T0,
    A: T1,
    Atype: cudaDataType,
    lda: i64,
    strideA: i64,
    B: T2,
    Btype: cudaDataType,
    ldb: i64,
    strideB: i64,
    beta: T3,
    mut C: T4,
    Ctype: cudaDataType,
    ldc: i64,
    strideC: i64,
    batchCount: i64,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGemmStridedBatchedEx_64(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Atype,
            lda,
            strideA as _,
            B.as_const_ptr() as *const _,
            Btype,
            ldb,
            strideB as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            Ctype,
            ldc,
            strideC as _,
            batchCount,
            computeType,
            algo,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemmGroupedBatched<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsPtr,
    T8: types::CudaAsPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsPtr,
    T12: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T0,
    transb_array: T1,
    m_array: T2,
    n_array: T3,
    k_array: T4,
    alpha_array: T5,
    Aarray: T6,
    lda_array: T7,
    Barray: T8,
    ldb_array: T9,
    beta_array: T10,
    Carray: T11,
    ldc_array: T12,
    group_count: i32,
    group_size: T13,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemmGroupedBatched(
            handle,
            transa_array.as_const_ptr() as *const _,
            transb_array.as_const_ptr() as *const _,
            m_array.as_const_ptr() as *const _,
            n_array.as_const_ptr() as *const _,
            k_array.as_const_ptr() as *const _,
            alpha_array.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda_array.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            ldb_array.as_const_ptr() as *const _,
            beta_array.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc_array.as_const_ptr() as *const _,
            group_count as _,
            group_size.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemmGroupedBatched_64<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsPtr,
    T8: types::CudaAsPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsPtr,
    T12: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T0,
    transb_array: T1,
    m_array: T2,
    n_array: T3,
    k_array: T4,
    alpha_array: T5,
    Aarray: T6,
    lda_array: T7,
    Barray: T8,
    ldb_array: T9,
    beta_array: T10,
    Carray: T11,
    ldc_array: T12,
    group_count: i64,
    group_size: T13,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgemmGroupedBatched_64(
            handle,
            transa_array.as_const_ptr() as *const _,
            transb_array.as_const_ptr() as *const _,
            m_array.as_const_ptr() as *const _,
            n_array.as_const_ptr() as *const _,
            k_array.as_const_ptr() as *const _,
            alpha_array.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda_array.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            ldb_array.as_const_ptr() as *const _,
            beta_array.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc_array.as_const_ptr() as *const _,
            group_count,
            group_size.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemmGroupedBatched<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsPtr,
    T8: types::CudaAsPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsPtr,
    T12: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T0,
    transb_array: T1,
    m_array: T2,
    n_array: T3,
    k_array: T4,
    alpha_array: T5,
    Aarray: T6,
    lda_array: T7,
    Barray: T8,
    ldb_array: T9,
    beta_array: T10,
    Carray: T11,
    ldc_array: T12,
    group_count: i32,
    group_size: T13,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemmGroupedBatched(
            handle,
            transa_array.as_const_ptr() as *const _,
            transb_array.as_const_ptr() as *const _,
            m_array.as_const_ptr() as *const _,
            n_array.as_const_ptr() as *const _,
            k_array.as_const_ptr() as *const _,
            alpha_array.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda_array.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            ldb_array.as_const_ptr() as *const _,
            beta_array.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc_array.as_const_ptr() as *const _,
            group_count as _,
            group_size.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgemmGroupedBatched_64<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsPtr,
    T8: types::CudaAsPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsPtr,
    T12: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T0,
    transb_array: T1,
    m_array: T2,
    n_array: T3,
    k_array: T4,
    alpha_array: T5,
    Aarray: T6,
    lda_array: T7,
    Barray: T8,
    ldb_array: T9,
    beta_array: T10,
    Carray: T11,
    ldc_array: T12,
    group_count: i64,
    group_size: T13,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgemmGroupedBatched_64(
            handle,
            transa_array.as_const_ptr() as *const _,
            transb_array.as_const_ptr() as *const _,
            m_array.as_const_ptr() as *const _,
            n_array.as_const_ptr() as *const _,
            k_array.as_const_ptr() as *const _,
            alpha_array.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            lda_array.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            ldb_array.as_const_ptr() as *const _,
            beta_array.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            ldc_array.as_const_ptr() as *const _,
            group_count,
            group_size.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGemmGroupedBatchedEx<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsPtr,
    T8: types::CudaAsPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsPtr,
    T12: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T0,
    transb_array: T1,
    m_array: T2,
    n_array: T3,
    k_array: T4,
    alpha_array: T5,
    Aarray: T6,
    Atype: cudaDataType_t,
    lda_array: T7,
    Barray: T8,
    Btype: cudaDataType_t,
    ldb_array: T9,
    beta_array: T10,
    Carray: T11,
    Ctype: cudaDataType_t,
    ldc_array: T12,
    group_count: i32,
    group_size: T13,
    computeType: cublasComputeType_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGemmGroupedBatchedEx(
            handle,
            transa_array.as_const_ptr() as *const _,
            transb_array.as_const_ptr() as *const _,
            m_array.as_const_ptr() as *const _,
            n_array.as_const_ptr() as *const _,
            k_array.as_const_ptr() as *const _,
            alpha_array.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            Atype,
            lda_array.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            Btype,
            ldb_array.as_const_ptr() as *const _,
            beta_array.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            Ctype,
            ldc_array.as_const_ptr() as *const _,
            group_count as _,
            group_size.as_const_ptr() as *const _,
            computeType,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGemmGroupedBatchedEx_64<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsPtr,
    T8: types::CudaAsPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsPtr,
    T12: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T0,
    transb_array: T1,
    m_array: T2,
    n_array: T3,
    k_array: T4,
    alpha_array: T5,
    Aarray: T6,
    Atype: cudaDataType_t,
    lda_array: T7,
    Barray: T8,
    Btype: cudaDataType_t,
    ldb_array: T9,
    beta_array: T10,
    Carray: T11,
    Ctype: cudaDataType_t,
    ldc_array: T12,
    group_count: i64,
    group_size: T13,
    computeType: cublasComputeType_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGemmGroupedBatchedEx_64(
            handle,
            transa_array.as_const_ptr() as *const _,
            transb_array.as_const_ptr() as *const _,
            m_array.as_const_ptr() as *const _,
            n_array.as_const_ptr() as *const _,
            k_array.as_const_ptr() as *const _,
            alpha_array.as_const_ptr() as *const _,
            Aarray.as_const_ptr() as *const _,
            Atype,
            lda_array.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            Btype,
            ldb_array.as_const_ptr() as *const _,
            beta_array.as_const_ptr() as *const _,
            Carray.as_const_ptr() as *const _,
            Ctype,
            ldc_array.as_const_ptr() as *const _,
            group_count,
            group_size.as_const_ptr() as *const _,
            computeType,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgeam<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    B: T3,
    ldb: i32,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgeam(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            beta.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgeam_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    B: T3,
    ldb: i64,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgeam_64(
            handle,
            transa,
            transb,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            beta.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgeam<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    B: T3,
    ldb: i32,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgeam(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            beta.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgeam_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    B: T3,
    ldb: i64,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgeam_64(
            handle,
            transa,
            transb,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            beta.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgeam<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    B: T3,
    ldb: i32,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgeam(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            beta.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgeam_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    B: T3,
    ldb: i64,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgeam_64(
            handle,
            transa,
            transb,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            beta.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgeam<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    beta: T2,
    B: T3,
    ldb: i32,
    mut C: T4,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgeam(
            handle,
            transa,
            transb,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            beta.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgeam_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    beta: T2,
    B: T3,
    ldb: i64,
    mut C: T4,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgeam_64(
            handle,
            transa,
            transb,
            m,
            n,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda,
            beta.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb,
            C.as_mut_ptr() as *mut _,
            ldc,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsmBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsmBatched(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, batchCount as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsmBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsmBatched_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, batchCount) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsmBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsmBatched(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, batchCount as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsmBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsmBatched_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, batchCount) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsmBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsmBatched(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, batchCount as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsmBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsmBatched_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, batchCount) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsmBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T0,
    A: T1,
    lda: i32,
    B: T2,
    ldb: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsmBatched(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, batchCount as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsmBatched_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T0,
    A: T1,
    lda: i64,
    B: T2,
    ldb: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsmBatched_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, batchCount) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSdgmm<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i32, n: i32, A: T0, lda: i32, x: T1, incx: i32, mut C: T2, ldc: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSdgmm(handle, mode, m as _, n as _, A.as_const_ptr() as *const _, lda as _, x.as_const_ptr() as *const _, incx as _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSdgmm_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: T0, lda: i64, x: T1, incx: i64, mut C: T2, ldc: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSdgmm_64(handle, mode, m, n, A.as_const_ptr() as *const _, lda, x.as_const_ptr() as *const _, incx, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDdgmm<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i32, n: i32, A: T0, lda: i32, x: T1, incx: i32, mut C: T2, ldc: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDdgmm(handle, mode, m as _, n as _, A.as_const_ptr() as *const _, lda as _, x.as_const_ptr() as *const _, incx as _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDdgmm_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: T0, lda: i64, x: T1, incx: i64, mut C: T2, ldc: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDdgmm_64(handle, mode, m, n, A.as_const_ptr() as *const _, lda, x.as_const_ptr() as *const _, incx, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdgmm<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i32, n: i32, A: T0, lda: i32, x: T1, incx: i32, mut C: T2, ldc: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdgmm(handle, mode, m as _, n as _, A.as_const_ptr() as *const _, lda as _, x.as_const_ptr() as *const _, incx as _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdgmm_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: T0, lda: i64, x: T1, incx: i64, mut C: T2, ldc: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdgmm_64(handle, mode, m, n, A.as_const_ptr() as *const _, lda, x.as_const_ptr() as *const _, incx, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdgmm<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i32, n: i32, A: T0, lda: i32, x: T1, incx: i32, mut C: T2, ldc: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdgmm(handle, mode, m as _, n as _, A.as_const_ptr() as *const _, lda as _, x.as_const_ptr() as *const _, incx as _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdgmm_64<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: T0, lda: i64, x: T1, incx: i64, mut C: T2, ldc: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdgmm_64(handle, mode, m, n, A.as_const_ptr() as *const _, lda, x.as_const_ptr() as *const _, incx, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSmatinvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, Ainv: T1, lda_inv: i32, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSmatinvBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, Ainv.as_const_ptr() as *const _, lda_inv as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDmatinvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, Ainv: T1, lda_inv: i32, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDmatinvBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, Ainv.as_const_ptr() as *const _, lda_inv as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCmatinvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, Ainv: T1, lda_inv: i32, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCmatinvBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, Ainv.as_const_ptr() as *const _, lda_inv as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZmatinvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, Ainv: T1, lda_inv: i32, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZmatinvBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, Ainv.as_const_ptr() as *const _, lda_inv as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgeqrfBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, Aarray: T0, lda: i32, TauArray: T1, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSgeqrfBatched(handle, m as _, n as _, Aarray.as_const_ptr() as *const _, lda as _, TauArray.as_const_ptr() as *const _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgeqrfBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, Aarray: T0, lda: i32, TauArray: T1, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDgeqrfBatched(handle, m as _, n as _, Aarray.as_const_ptr() as *const _, lda as _, TauArray.as_const_ptr() as *const _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgeqrfBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, Aarray: T0, lda: i32, TauArray: T1, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgeqrfBatched(handle, m as _, n as _, Aarray.as_const_ptr() as *const _, lda as _, TauArray.as_const_ptr() as *const _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgeqrfBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, m: i32, n: i32, Aarray: T0, lda: i32, TauArray: T1, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgeqrfBatched(handle, m as _, n as _, Aarray.as_const_ptr() as *const _, lda as _, TauArray.as_const_ptr() as *const _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgelsBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    nrhs: i32,
    Aarray: T0,
    lda: i32,
    Carray: T1,
    ldc: i32,
    mut info: T2,
    mut devInfoArray: T3,
    batchSize: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgelsBatched(
            handle,
            trans,
            m as _,
            n as _,
            nrhs as _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            Carray.as_const_ptr() as *const _,
            ldc as _,
            info.as_mut_ptr() as *mut _,
            devInfoArray.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgelsBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    nrhs: i32,
    Aarray: T0,
    lda: i32,
    Carray: T1,
    ldc: i32,
    mut info: T2,
    mut devInfoArray: T3,
    batchSize: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgelsBatched(
            handle,
            trans,
            m as _,
            n as _,
            nrhs as _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            Carray.as_const_ptr() as *const _,
            ldc as _,
            info.as_mut_ptr() as *mut _,
            devInfoArray.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgelsBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    nrhs: i32,
    Aarray: T0,
    lda: i32,
    Carray: T1,
    ldc: i32,
    mut info: T2,
    mut devInfoArray: T3,
    batchSize: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgelsBatched(
            handle,
            trans,
            m as _,
            n as _,
            nrhs as _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            Carray.as_const_ptr() as *const _,
            ldc as _,
            info.as_mut_ptr() as *mut _,
            devInfoArray.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgelsBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    nrhs: i32,
    Aarray: T0,
    lda: i32,
    Carray: T1,
    ldc: i32,
    mut info: T2,
    mut devInfoArray: T3,
    batchSize: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgelsBatched(
            handle,
            trans,
            m as _,
            n as _,
            nrhs as _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            Carray.as_const_ptr() as *const _,
            ldc as _,
            info.as_mut_ptr() as *mut _,
            devInfoArray.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStpttr<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, AP: T0, mut A: T1, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpttr(handle, uplo, n as _, AP.as_const_ptr() as *const _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpttr<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, AP: T0, mut A: T1, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpttr(handle, uplo, n as _, AP.as_const_ptr() as *const _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpttr<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, AP: T0, mut A: T1, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpttr(handle, uplo, n as _, AP.as_const_ptr() as *const _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpttr<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, AP: T0, mut A: T1, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpttr(handle, uplo, n as _, AP.as_const_ptr() as *const _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrttp<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, mut AP: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrttp(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrttp<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, mut AP: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrttp(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrttp<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, mut AP: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrttp(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrttp<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, mut AP: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrttp(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgetrfBatched<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, mut P: T1, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSgetrfBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_mut_ptr() as *mut _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgetrfBatched<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, mut P: T1, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDgetrfBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_mut_ptr() as *mut _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgetrfBatched<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, mut P: T1, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgetrfBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_mut_ptr() as *mut _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgetrfBatched<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, mut P: T1, mut info: T2, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgetrfBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_mut_ptr() as *mut _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgetriBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, P: T1, C: T2, ldc: i32, mut info: T3, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSgetriBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_const_ptr() as *const _, C.as_const_ptr() as *const _, ldc as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgetriBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, P: T1, C: T2, ldc: i32, mut info: T3, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDgetriBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_const_ptr() as *const _, C.as_const_ptr() as *const _, ldc as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgetriBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, P: T1, C: T2, ldc: i32, mut info: T3, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgetriBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_const_ptr() as *const _, C.as_const_ptr() as *const _, ldc as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgetriBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cublasHandle_t, n: i32, A: T0, lda: i32, P: T1, C: T2, ldc: i32, mut info: T3, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgetriBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_const_ptr() as *const _, C.as_const_ptr() as *const _, ldc as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgetrsBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    Aarray: T0,
    lda: i32,
    devIpiv: T1,
    Barray: T2,
    ldb: i32,
    mut info: T3,
    batchSize: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSgetrsBatched(
            handle,
            trans,
            n as _,
            nrhs as _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            devIpiv.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            ldb as _,
            info.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgetrsBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    Aarray: T0,
    lda: i32,
    devIpiv: T1,
    Barray: T2,
    ldb: i32,
    mut info: T3,
    batchSize: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasDgetrsBatched(
            handle,
            trans,
            n as _,
            nrhs as _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            devIpiv.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            ldb as _,
            info.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgetrsBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    Aarray: T0,
    lda: i32,
    devIpiv: T1,
    Barray: T2,
    ldb: i32,
    mut info: T3,
    batchSize: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasCgetrsBatched(
            handle,
            trans,
            n as _,
            nrhs as _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            devIpiv.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            ldb as _,
            info.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgetrsBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    Aarray: T0,
    lda: i32,
    devIpiv: T1,
    Barray: T2,
    ldb: i32,
    mut info: T3,
    batchSize: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasZgetrsBatched(
            handle,
            trans,
            n as _,
            nrhs as _,
            Aarray.as_const_ptr() as *const _,
            lda as _,
            devIpiv.as_const_ptr() as *const _,
            Barray.as_const_ptr() as *const _,
            ldb as _,
            info.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasUint8gemmBias<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    transc: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T0,
    A_bias: i32,
    lda: i32,
    B: T1,
    B_bias: i32,
    ldb: i32,
    mut C: T2,
    C_bias: i32,
    ldc: i32,
    C_mult: i32,
    C_shift: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasUint8gemmBias(
            handle,
            transa,
            transb,
            transc,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            A_bias as _,
            lda as _,
            B.as_const_ptr() as *const _,
            B_bias as _,
            ldb as _,
            C.as_mut_ptr() as *mut _,
            C_bias as _,
            ldc as _,
            C_mult as _,
            C_shift as _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
