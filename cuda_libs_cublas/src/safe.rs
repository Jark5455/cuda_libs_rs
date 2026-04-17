pub use crate::sys::cublasStatus_t as CudaTargetStatus;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart;
#[allow(unused_imports)]
use cuda_libs_cudart::sys::*;
#[allow(unused_imports)]
use cuda_libs_cudart::types;
#[cfg(feature = "runtime-link")]
impl crate::sys::float2 {
    pub fn x(mut self, val: f32) -> Self {
        self.x = val;
        self
    }
    pub fn y(mut self, val: f32) -> Self {
        self.y = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::double2 {
    pub fn x(mut self, val: f64) -> Self {
        self.x = val;
        self
    }
    pub fn y(mut self, val: f64) -> Self {
        self.y = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::DynamicBindings {
    pub fn cublasCreate_v2(mut self, val: Option<unsafe extern "C" fn(handle: *mut cublasHandle_t) -> cublasStatus_t>) -> Self {
        self.cublasCreate_v2 = val;
        self
    }
    pub fn cublasDestroy_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t) -> cublasStatus_t>) -> Self {
        self.cublasDestroy_v2 = val;
        self
    }
    pub fn cublasGetVersion_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, version: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetVersion_v2 = val;
        self
    }
    pub fn cublasGetProperty(mut self, val: Option<unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetProperty = val;
        self
    }
    pub fn cublasGetCudartVersion(mut self, val: Option<unsafe extern "C" fn() -> usize>) -> Self {
        self.cublasGetCudartVersion = val;
        self
    }
    pub fn cublasSetWorkspace_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, workspace: *mut ::std::os::raw::c_void, workspaceSizeInBytes: usize) -> cublasStatus_t>) -> Self {
        self.cublasSetWorkspace_v2 = val;
        self
    }
    pub fn cublasSetStream_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, streamId: cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasSetStream_v2 = val;
        self
    }
    pub fn cublasGetStream_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, streamId: *mut cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasGetStream_v2 = val;
        self
    }
    pub fn cublasGetPointerMode_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: *mut cublasPointerMode_t) -> cublasStatus_t>) -> Self {
        self.cublasGetPointerMode_v2 = val;
        self
    }
    pub fn cublasSetPointerMode_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasPointerMode_t) -> cublasStatus_t>) -> Self {
        self.cublasSetPointerMode_v2 = val;
        self
    }
    pub fn cublasGetAtomicsMode(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: *mut cublasAtomicsMode_t) -> cublasStatus_t>) -> Self {
        self.cublasGetAtomicsMode = val;
        self
    }
    pub fn cublasSetAtomicsMode(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasAtomicsMode_t) -> cublasStatus_t>) -> Self {
        self.cublasSetAtomicsMode = val;
        self
    }
    pub fn cublasGetMathMode(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: *mut cublasMath_t) -> cublasStatus_t>) -> Self {
        self.cublasGetMathMode = val;
        self
    }
    pub fn cublasSetMathMode(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasMath_t) -> cublasStatus_t>) -> Self {
        self.cublasSetMathMode = val;
        self
    }
    pub fn cublasGetSmCountTarget(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, smCountTarget: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetSmCountTarget = val;
        self
    }
    pub fn cublasSetSmCountTarget(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, smCountTarget: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetSmCountTarget = val;
        self
    }
    pub fn cublasGetEmulationStrategy(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, emulationStrategy: *mut cublasEmulationStrategy_t) -> cublasStatus_t>) -> Self {
        self.cublasGetEmulationStrategy = val;
        self
    }
    pub fn cublasSetEmulationStrategy(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, emulationStrategy: cublasEmulationStrategy_t) -> cublasStatus_t>) -> Self {
        self.cublasSetEmulationStrategy = val;
        self
    }
    pub fn cublasGetEmulationSpecialValuesSupport(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mask: *mut cudaEmulationSpecialValuesSupport) -> cublasStatus_t>) -> Self {
        self.cublasGetEmulationSpecialValuesSupport = val;
        self
    }
    pub fn cublasSetEmulationSpecialValuesSupport(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mask: cudaEmulationSpecialValuesSupport) -> cublasStatus_t>) -> Self {
        self.cublasSetEmulationSpecialValuesSupport = val;
        self
    }
    pub fn cublasGetFixedPointEmulationMantissaControl(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaControl: *mut cudaEmulationMantissaControl) -> cublasStatus_t>) -> Self {
        self.cublasGetFixedPointEmulationMantissaControl = val;
        self
    }
    pub fn cublasSetFixedPointEmulationMantissaControl(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaControl: cudaEmulationMantissaControl) -> cublasStatus_t>) -> Self {
        self.cublasSetFixedPointEmulationMantissaControl = val;
        self
    }
    pub fn cublasGetFixedPointEmulationMaxMantissaBitCount(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, maxMantissaBitCount: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetFixedPointEmulationMaxMantissaBitCount = val;
        self
    }
    pub fn cublasSetFixedPointEmulationMaxMantissaBitCount(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, maxMantissaBitCount: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetFixedPointEmulationMaxMantissaBitCount = val;
        self
    }
    pub fn cublasGetFixedPointEmulationMantissaBitOffset(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaBitOffset: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetFixedPointEmulationMantissaBitOffset = val;
        self
    }
    pub fn cublasSetFixedPointEmulationMantissaBitOffset(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaBitOffset: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetFixedPointEmulationMantissaBitOffset = val;
        self
    }
    pub fn cublasGetFixedPointEmulationMantissaBitCountPointer(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaBitCount: *mut *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetFixedPointEmulationMantissaBitCountPointer = val;
        self
    }
    pub fn cublasSetFixedPointEmulationMantissaBitCountPointer(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mantissaBitCount: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetFixedPointEmulationMantissaBitCountPointer = val;
        self
    }
    pub fn cublasGetStatusName(mut self, val: Option<unsafe extern "C" fn(status: cublasStatus_t) -> *const ::std::os::raw::c_char>) -> Self {
        self.cublasGetStatusName = val;
        self
    }
    pub fn cublasGetStatusString(mut self, val: Option<unsafe extern "C" fn(status: cublasStatus_t) -> *const ::std::os::raw::c_char>) -> Self {
        self.cublasGetStatusString = val;
        self
    }
    pub fn cublasLoggerConfigure(mut self, val: Option<unsafe extern "C" fn(logIsOn: ::std::os::raw::c_int, logToStdOut: ::std::os::raw::c_int, logToStdErr: ::std::os::raw::c_int, logFileName: *const ::std::os::raw::c_char) -> cublasStatus_t>) -> Self {
        self.cublasLoggerConfigure = val;
        self
    }
    pub fn cublasSetLoggerCallback(mut self, val: Option<unsafe extern "C" fn(userCallback: cublasLogCallback) -> cublasStatus_t>) -> Self {
        self.cublasSetLoggerCallback = val;
        self
    }
    pub fn cublasGetLoggerCallback(mut self, val: Option<unsafe extern "C" fn(userCallback: *mut cublasLogCallback) -> cublasStatus_t>) -> Self {
        self.cublasGetLoggerCallback = val;
        self
    }
    pub fn cublasSetVector(mut self, val: Option<unsafe extern "C" fn(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, devicePtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSetVector = val;
        self
    }
    pub fn cublasSetVector_64(mut self, val: Option<unsafe extern "C" fn(n: i64, elemSize: i64, x: *const ::std::os::raw::c_void, incx: i64, devicePtr: *mut ::std::os::raw::c_void, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasSetVector_64 = val;
        self
    }
    pub fn cublasGetVector(mut self, val: Option<unsafe extern "C" fn(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasGetVector = val;
        self
    }
    pub fn cublasGetVector_64(mut self, val: Option<unsafe extern "C" fn(n: i64, elemSize: i64, x: *const ::std::os::raw::c_void, incx: i64, y: *mut ::std::os::raw::c_void, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasGetVector_64 = val;
        self
    }
    pub fn cublasSetMatrix(
        mut self,
        val: Option<unsafe extern "C" fn(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSetMatrix = val;
        self
    }
    pub fn cublasSetMatrix_64(mut self, val: Option<unsafe extern "C" fn(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64) -> cublasStatus_t>) -> Self {
        self.cublasSetMatrix_64 = val;
        self
    }
    pub fn cublasGetMatrix(
        mut self,
        val: Option<unsafe extern "C" fn(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasGetMatrix = val;
        self
    }
    pub fn cublasGetMatrix_64(mut self, val: Option<unsafe extern "C" fn(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64) -> cublasStatus_t>) -> Self {
        self.cublasGetMatrix_64 = val;
        self
    }
    pub fn cublasSetVectorAsync(
        mut self,
        val: Option<unsafe extern "C" fn(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, hostPtr: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, devicePtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSetVectorAsync = val;
        self
    }
    pub fn cublasSetVectorAsync_64(mut self, val: Option<unsafe extern "C" fn(n: i64, elemSize: i64, hostPtr: *const ::std::os::raw::c_void, incx: i64, devicePtr: *mut ::std::os::raw::c_void, incy: i64, stream: cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasSetVectorAsync_64 = val;
        self
    }
    pub fn cublasGetVectorAsync(
        mut self,
        val: Option<unsafe extern "C" fn(n: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, devicePtr: *const ::std::os::raw::c_void, incx: ::std::os::raw::c_int, hostPtr: *mut ::std::os::raw::c_void, incy: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t>,
    ) -> Self {
        self.cublasGetVectorAsync = val;
        self
    }
    pub fn cublasGetVectorAsync_64(mut self, val: Option<unsafe extern "C" fn(n: i64, elemSize: i64, devicePtr: *const ::std::os::raw::c_void, incx: i64, hostPtr: *mut ::std::os::raw::c_void, incy: i64, stream: cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasGetVectorAsync_64 = val;
        self
    }
    pub fn cublasSetMatrixAsync(
        mut self,
        val: Option<unsafe extern "C" fn(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSetMatrixAsync = val;
        self
    }
    pub fn cublasSetMatrixAsync_64(mut self, val: Option<unsafe extern "C" fn(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64, stream: cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasSetMatrixAsync_64 = val;
        self
    }
    pub fn cublasGetMatrixAsync(
        mut self,
        val: Option<unsafe extern "C" fn(rows: ::std::os::raw::c_int, cols: ::std::os::raw::c_int, elemSize: ::std::os::raw::c_int, A: *const ::std::os::raw::c_void, lda: ::std::os::raw::c_int, B: *mut ::std::os::raw::c_void, ldb: ::std::os::raw::c_int, stream: cudaStream_t) -> cublasStatus_t>,
    ) -> Self {
        self.cublasGetMatrixAsync = val;
        self
    }
    pub fn cublasGetMatrixAsync_64(mut self, val: Option<unsafe extern "C" fn(rows: i64, cols: i64, elemSize: i64, A: *const ::std::os::raw::c_void, lda: i64, B: *mut ::std::os::raw::c_void, ldb: i64, stream: cudaStream_t) -> cublasStatus_t>) -> Self {
        self.cublasGetMatrixAsync_64 = val;
        self
    }
    pub fn cublasXerbla(mut self, val: Option<unsafe extern "C" fn(srName: *const ::std::os::raw::c_char, info: ::std::os::raw::c_int)>) -> Self {
        self.cublasXerbla = val;
        self
    }
    pub fn cublasNrm2Ex(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executionType: cudaDataType) -> cublasStatus_t>,
    ) -> Self {
        self.cublasNrm2Ex = val;
        self
    }
    pub fn cublasNrm2Ex_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executionType: cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasNrm2Ex_64 = val;
        self
    }
    pub fn cublasSnrm2_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSnrm2_v2 = val;
        self
    }
    pub fn cublasSnrm2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSnrm2_v2_64 = val;
        self
    }
    pub fn cublasDnrm2_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDnrm2_v2 = val;
        self
    }
    pub fn cublasDnrm2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDnrm2_v2_64 = val;
        self
    }
    pub fn cublasScnrm2_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasScnrm2_v2 = val;
        self
    }
    pub fn cublasScnrm2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasScnrm2_v2_64 = val;
        self
    }
    pub fn cublasDznrm2_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDznrm2_v2 = val;
        self
    }
    pub fn cublasDznrm2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDznrm2_v2_64 = val;
        self
    }
    pub fn cublasDotEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDotEx = val;
        self
    }
    pub fn cublasDotEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDotEx_64 = val;
        self
    }
    pub fn cublasDotcEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDotcEx = val;
        self
    }
    pub fn cublasDotcEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDotcEx_64 = val;
        self
    }
    pub fn cublasSdot_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSdot_v2 = val;
        self
    }
    pub fn cublasSdot_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, y: *const f32, incy: i64, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSdot_v2_64 = val;
        self
    }
    pub fn cublasDdot_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDdot_v2 = val;
        self
    }
    pub fn cublasDdot_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, y: *const f64, incy: i64, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDdot_v2_64 = val;
        self
    }
    pub fn cublasCdotu_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, result: *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCdotu_v2 = val;
        self
    }
    pub fn cublasCdotu_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, result: *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCdotu_v2_64 = val;
        self
    }
    pub fn cublasCdotc_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, result: *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCdotc_v2 = val;
        self
    }
    pub fn cublasCdotc_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, result: *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCdotc_v2_64 = val;
        self
    }
    pub fn cublasZdotu_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, result: *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZdotu_v2 = val;
        self
    }
    pub fn cublasZdotu_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, result: *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZdotu_v2_64 = val;
        self
    }
    pub fn cublasZdotc_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, result: *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZdotc_v2 = val;
        self
    }
    pub fn cublasZdotc_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, result: *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZdotc_v2_64 = val;
        self
    }
    pub fn cublasScalEx(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const ::std::os::raw::c_void, alphaType: cudaDataType, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, executionType: cudaDataType) -> cublasStatus_t>,
    ) -> Self {
        self.cublasScalEx = val;
        self
    }
    pub fn cublasScalEx_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const ::std::os::raw::c_void, alphaType: cudaDataType, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, executionType: cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasScalEx_64 = val;
        self
    }
    pub fn cublasSscal_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSscal_v2 = val;
        self
    }
    pub fn cublasSscal_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasSscal_v2_64 = val;
        self
    }
    pub fn cublasDscal_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDscal_v2 = val;
        self
    }
    pub fn cublasDscal_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasDscal_v2_64 = val;
        self
    }
    pub fn cublasCscal_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCscal_v2 = val;
        self
    }
    pub fn cublasCscal_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasCscal_v2_64 = val;
        self
    }
    pub fn cublasCsscal_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCsscal_v2 = val;
        self
    }
    pub fn cublasCsscal_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *mut cuComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasCsscal_v2_64 = val;
        self
    }
    pub fn cublasZscal_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZscal_v2 = val;
        self
    }
    pub fn cublasZscal_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasZscal_v2_64 = val;
        self
    }
    pub fn cublasZdscal_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZdscal_v2 = val;
        self
    }
    pub fn cublasZdscal_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasZdscal_v2_64 = val;
        self
    }
    pub fn cublasAxpyEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasAxpyEx = val;
        self
    }
    pub fn cublasAxpyEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasAxpyEx_64 = val;
        self
    }
    pub fn cublasSaxpy_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSaxpy_v2 = val;
        self
    }
    pub fn cublasSaxpy_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasSaxpy_v2_64 = val;
        self
    }
    pub fn cublasDaxpy_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDaxpy_v2 = val;
        self
    }
    pub fn cublasDaxpy_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasDaxpy_v2_64 = val;
        self
    }
    pub fn cublasCaxpy_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCaxpy_v2 = val;
        self
    }
    pub fn cublasCaxpy_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasCaxpy_v2_64 = val;
        self
    }
    pub fn cublasZaxpy_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZaxpy_v2 = val;
        self
    }
    pub fn cublasZaxpy_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasZaxpy_v2_64 = val;
        self
    }
    pub fn cublasCopyEx(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCopyEx = val;
        self
    }
    pub fn cublasCopyEx_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasCopyEx_64 = val;
        self
    }
    pub fn cublasScopy_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasScopy_v2 = val;
        self
    }
    pub fn cublasScopy_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasScopy_v2_64 = val;
        self
    }
    pub fn cublasDcopy_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDcopy_v2 = val;
        self
    }
    pub fn cublasDcopy_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasDcopy_v2_64 = val;
        self
    }
    pub fn cublasCcopy_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCcopy_v2 = val;
        self
    }
    pub fn cublasCcopy_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasCcopy_v2_64 = val;
        self
    }
    pub fn cublasZcopy_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZcopy_v2 = val;
        self
    }
    pub fn cublasZcopy_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasZcopy_v2_64 = val;
        self
    }
    pub fn cublasSswap_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSswap_v2 = val;
        self
    }
    pub fn cublasSswap_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasSswap_v2_64 = val;
        self
    }
    pub fn cublasDswap_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDswap_v2 = val;
        self
    }
    pub fn cublasDswap_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasDswap_v2_64 = val;
        self
    }
    pub fn cublasCswap_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCswap_v2 = val;
        self
    }
    pub fn cublasCswap_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasCswap_v2_64 = val;
        self
    }
    pub fn cublasZswap_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZswap_v2 = val;
        self
    }
    pub fn cublasZswap_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasZswap_v2_64 = val;
        self
    }
    pub fn cublasSwapEx(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSwapEx = val;
        self
    }
    pub fn cublasSwapEx_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut ::std::os::raw::c_void, xType: cudaDataType, incx: i64, y: *mut ::std::os::raw::c_void, yType: cudaDataType, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasSwapEx_64 = val;
        self
    }
    pub fn cublasIsamax_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIsamax_v2 = val;
        self
    }
    pub fn cublasIsamax_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIsamax_v2_64 = val;
        self
    }
    pub fn cublasIdamax_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIdamax_v2 = val;
        self
    }
    pub fn cublasIdamax_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIdamax_v2_64 = val;
        self
    }
    pub fn cublasIcamax_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIcamax_v2 = val;
        self
    }
    pub fn cublasIcamax_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIcamax_v2_64 = val;
        self
    }
    pub fn cublasIzamax_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIzamax_v2 = val;
        self
    }
    pub fn cublasIzamax_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIzamax_v2_64 = val;
        self
    }
    pub fn cublasIamaxEx(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIamaxEx = val;
        self
    }
    pub fn cublasIamaxEx_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIamaxEx_64 = val;
        self
    }
    pub fn cublasIsamin_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIsamin_v2 = val;
        self
    }
    pub fn cublasIsamin_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIsamin_v2_64 = val;
        self
    }
    pub fn cublasIdamin_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIdamin_v2 = val;
        self
    }
    pub fn cublasIdamin_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIdamin_v2_64 = val;
        self
    }
    pub fn cublasIcamin_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIcamin_v2 = val;
        self
    }
    pub fn cublasIcamin_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIcamin_v2_64 = val;
        self
    }
    pub fn cublasIzamin_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIzamin_v2 = val;
        self
    }
    pub fn cublasIzamin_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIzamin_v2_64 = val;
        self
    }
    pub fn cublasIaminEx(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasIaminEx = val;
        self
    }
    pub fn cublasIaminEx_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut i64) -> cublasStatus_t>) -> Self {
        self.cublasIaminEx_64 = val;
        self
    }
    pub fn cublasAsumEx(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t>,
    ) -> Self {
        self.cublasAsumEx = val;
        self
    }
    pub fn cublasAsumEx_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const ::std::os::raw::c_void, xType: cudaDataType, incx: i64, result: *mut ::std::os::raw::c_void, resultType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasAsumEx_64 = val;
        self
    }
    pub fn cublasSasum_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSasum_v2 = val;
        self
    }
    pub fn cublasSasum_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSasum_v2_64 = val;
        self
    }
    pub fn cublasDasum_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDasum_v2 = val;
        self
    }
    pub fn cublasDasum_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const f64, incx: i64, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDasum_v2_64 = val;
        self
    }
    pub fn cublasScasum_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasScasum_v2 = val;
        self
    }
    pub fn cublasScasum_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuComplex, incx: i64, result: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasScasum_v2_64 = val;
        self
    }
    pub fn cublasDzasum_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDzasum_v2 = val;
        self
    }
    pub fn cublasDzasum_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *const cuDoubleComplex, incx: i64, result: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDzasum_v2_64 = val;
        self
    }
    pub fn cublasSrot_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int, c: *const f32, s: *const f32) -> cublasStatus_t>) -> Self {
        self.cublasSrot_v2 = val;
        self
    }
    pub fn cublasSrot_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64, c: *const f32, s: *const f32) -> cublasStatus_t>) -> Self {
        self.cublasSrot_v2_64 = val;
        self
    }
    pub fn cublasDrot_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int, c: *const f64, s: *const f64) -> cublasStatus_t>) -> Self {
        self.cublasDrot_v2 = val;
        self
    }
    pub fn cublasDrot_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64, c: *const f64, s: *const f64) -> cublasStatus_t>) -> Self {
        self.cublasDrot_v2_64 = val;
        self
    }
    pub fn cublasCrot_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int, c: *const f32, s: *const cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCrot_v2 = val;
        self
    }
    pub fn cublasCrot_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64, c: *const f32, s: *const cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCrot_v2_64 = val;
        self
    }
    pub fn cublasCsrot_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int, y: *mut cuComplex, incy: ::std::os::raw::c_int, c: *const f32, s: *const f32) -> cublasStatus_t>) -> Self {
        self.cublasCsrot_v2 = val;
        self
    }
    pub fn cublasCsrot_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuComplex, incx: i64, y: *mut cuComplex, incy: i64, c: *const f32, s: *const f32) -> cublasStatus_t>) -> Self {
        self.cublasCsrot_v2_64 = val;
        self
    }
    pub fn cublasZrot_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int, c: *const f64, s: *const cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZrot_v2 = val;
        self
    }
    pub fn cublasZrot_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64, c: *const f64, s: *const cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZrot_v2_64 = val;
        self
    }
    pub fn cublasZdrot_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int, y: *mut cuDoubleComplex, incy: ::std::os::raw::c_int, c: *const f64, s: *const f64) -> cublasStatus_t>) -> Self {
        self.cublasZdrot_v2 = val;
        self
    }
    pub fn cublasZdrot_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut cuDoubleComplex, incx: i64, y: *mut cuDoubleComplex, incy: i64, c: *const f64, s: *const f64) -> cublasStatus_t>) -> Self {
        self.cublasZdrot_v2_64 = val;
        self
    }
    pub fn cublasRotEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasRotEx = val;
        self
    }
    pub fn cublasRotEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasRotEx_64 = val;
        self
    }
    pub fn cublasSrotg_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSrotg_v2 = val;
        self
    }
    pub fn cublasDrotg_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDrotg_v2 = val;
        self
    }
    pub fn cublasCrotg_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut cuComplex, b: *mut cuComplex, c: *mut f32, s: *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCrotg_v2 = val;
        self
    }
    pub fn cublasZrotg_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut cuDoubleComplex, b: *mut cuDoubleComplex, c: *mut f64, s: *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZrotg_v2 = val;
        self
    }
    pub fn cublasRotgEx(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, a: *mut ::std::os::raw::c_void, b: *mut ::std::os::raw::c_void, abType: cudaDataType, c: *mut ::std::os::raw::c_void, s: *mut ::std::os::raw::c_void, csType: cudaDataType, executiontype: cudaDataType) -> cublasStatus_t>,
    ) -> Self {
        self.cublasRotgEx = val;
        self
    }
    pub fn cublasSrotm_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int, y: *mut f32, incy: ::std::os::raw::c_int, param: *const f32) -> cublasStatus_t>) -> Self {
        self.cublasSrotm_v2 = val;
        self
    }
    pub fn cublasSrotm_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f32, incx: i64, y: *mut f32, incy: i64, param: *const f32) -> cublasStatus_t>) -> Self {
        self.cublasSrotm_v2_64 = val;
        self
    }
    pub fn cublasDrotm_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int, y: *mut f64, incy: ::std::os::raw::c_int, param: *const f64) -> cublasStatus_t>) -> Self {
        self.cublasDrotm_v2 = val;
        self
    }
    pub fn cublasDrotm_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: i64, x: *mut f64, incx: i64, y: *mut f64, incy: i64, param: *const f64) -> cublasStatus_t>) -> Self {
        self.cublasDrotm_v2_64 = val;
        self
    }
    pub fn cublasRotmEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasRotmEx = val;
        self
    }
    pub fn cublasRotmEx_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasRotmEx_64 = val;
        self
    }
    pub fn cublasSrotmg_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, d1: *mut f32, d2: *mut f32, x1: *mut f32, y1: *const f32, param: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSrotmg_v2 = val;
        self
    }
    pub fn cublasDrotmg_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, d1: *mut f64, d2: *mut f64, x1: *mut f64, y1: *const f64, param: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDrotmg_v2 = val;
        self
    }
    pub fn cublasRotmgEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasRotmgEx = val;
        self
    }
    pub fn cublasSgemv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemv_v2 = val;
        self
    }
    pub fn cublasSgemv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasSgemv_v2_64 = val;
        self
    }
    pub fn cublasDgemv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemv_v2 = val;
        self
    }
    pub fn cublasDgemv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasDgemv_v2_64 = val;
        self
    }
    pub fn cublasCgemv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemv_v2 = val;
        self
    }
    pub fn cublasCgemv_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgemv_v2_64 = val;
        self
    }
    pub fn cublasZgemv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemv_v2 = val;
        self
    }
    pub fn cublasZgemv_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZgemv_v2_64 = val;
        self
    }
    pub fn cublasSgbmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgbmv_v2 = val;
        self
    }
    pub fn cublasSgbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasSgbmv_v2_64 = val;
        self
    }
    pub fn cublasDgbmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgbmv_v2 = val;
        self
    }
    pub fn cublasDgbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasDgbmv_v2_64 = val;
        self
    }
    pub fn cublasCgbmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgbmv_v2 = val;
        self
    }
    pub fn cublasCgbmv_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, kl: i64, ku: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgbmv_v2_64 = val;
        self
    }
    pub fn cublasZgbmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgbmv_v2 = val;
        self
    }
    pub fn cublasZgbmv_v2_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgbmv_v2_64 = val;
        self
    }
    pub fn cublasStrmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasStrmv_v2 = val;
        self
    }
    pub fn cublasStrmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasStrmv_v2_64 = val;
        self
    }
    pub fn cublasDtrmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDtrmv_v2 = val;
        self
    }
    pub fn cublasDtrmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasDtrmv_v2_64 = val;
        self
    }
    pub fn cublasCtrmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCtrmv_v2 = val;
        self
    }
    pub fn cublasCtrmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasCtrmv_v2_64 = val;
        self
    }
    pub fn cublasZtrmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZtrmv_v2 = val;
        self
    }
    pub fn cublasZtrmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasZtrmv_v2_64 = val;
        self
    }
    pub fn cublasStbmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasStbmv_v2 = val;
        self
    }
    pub fn cublasStbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasStbmv_v2_64 = val;
        self
    }
    pub fn cublasDtbmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDtbmv_v2 = val;
        self
    }
    pub fn cublasDtbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasDtbmv_v2_64 = val;
        self
    }
    pub fn cublasCtbmv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCtbmv_v2 = val;
        self
    }
    pub fn cublasCtbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasCtbmv_v2_64 = val;
        self
    }
    pub fn cublasZtbmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZtbmv_v2 = val;
        self
    }
    pub fn cublasZtbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasZtbmv_v2_64 = val;
        self
    }
    pub fn cublasStpmv_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStpmv_v2 = val;
        self
    }
    pub fn cublasStpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasStpmv_v2_64 = val;
        self
    }
    pub fn cublasDtpmv_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtpmv_v2 = val;
        self
    }
    pub fn cublasDtpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasDtpmv_v2_64 = val;
        self
    }
    pub fn cublasCtpmv_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtpmv_v2 = val;
        self
    }
    pub fn cublasCtpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasCtpmv_v2_64 = val;
        self
    }
    pub fn cublasZtpmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZtpmv_v2 = val;
        self
    }
    pub fn cublasZtpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasZtpmv_v2_64 = val;
        self
    }
    pub fn cublasStrsv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasStrsv_v2 = val;
        self
    }
    pub fn cublasStrsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasStrsv_v2_64 = val;
        self
    }
    pub fn cublasDtrsv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDtrsv_v2 = val;
        self
    }
    pub fn cublasDtrsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasDtrsv_v2_64 = val;
        self
    }
    pub fn cublasCtrsv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCtrsv_v2 = val;
        self
    }
    pub fn cublasCtrsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasCtrsv_v2_64 = val;
        self
    }
    pub fn cublasZtrsv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZtrsv_v2 = val;
        self
    }
    pub fn cublasZtrsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasZtrsv_v2_64 = val;
        self
    }
    pub fn cublasStpsv_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f32, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStpsv_v2 = val;
        self
    }
    pub fn cublasStpsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f32, x: *mut f32, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasStpsv_v2_64 = val;
        self
    }
    pub fn cublasDtpsv_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const f64, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtpsv_v2 = val;
        self
    }
    pub fn cublasDtpsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const f64, x: *mut f64, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasDtpsv_v2_64 = val;
        self
    }
    pub fn cublasCtpsv_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuComplex, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtpsv_v2 = val;
        self
    }
    pub fn cublasCtpsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuComplex, x: *mut cuComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasCtpsv_v2_64 = val;
        self
    }
    pub fn cublasZtpsv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZtpsv_v2 = val;
        self
    }
    pub fn cublasZtpsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: *const cuDoubleComplex, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasZtpsv_v2_64 = val;
        self
    }
    pub fn cublasStbsv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *mut f32, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasStbsv_v2 = val;
        self
    }
    pub fn cublasStbsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f32, lda: i64, x: *mut f32, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasStbsv_v2_64 = val;
        self
    }
    pub fn cublasDtbsv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *mut f64, incx: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDtbsv_v2 = val;
        self
    }
    pub fn cublasDtbsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const f64, lda: i64, x: *mut f64, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasDtbsv_v2_64 = val;
        self
    }
    pub fn cublasCtbsv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *mut cuComplex, incx: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCtbsv_v2 = val;
        self
    }
    pub fn cublasCtbsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuComplex, lda: i64, x: *mut cuComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasCtbsv_v2_64 = val;
        self
    }
    pub fn cublasZtbsv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZtbsv_v2 = val;
        self
    }
    pub fn cublasZtbsv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: *const cuDoubleComplex, lda: i64, x: *mut cuDoubleComplex, incx: i64) -> cublasStatus_t>) -> Self {
        self.cublasZtbsv_v2_64 = val;
        self
    }
    pub fn cublasSsymv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, A: *const f32, lda: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, beta: *const f32, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSsymv_v2 = val;
        self
    }
    pub fn cublasSsymv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasSsymv_v2_64 = val;
        self
    }
    pub fn cublasDsymv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, A: *const f64, lda: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, beta: *const f64, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDsymv_v2 = val;
        self
    }
    pub fn cublasDsymv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasDsymv_v2_64 = val;
        self
    }
    pub fn cublasCsymv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCsymv_v2 = val;
        self
    }
    pub fn cublasCsymv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasCsymv_v2_64 = val;
        self
    }
    pub fn cublasZsymv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZsymv_v2 = val;
        self
    }
    pub fn cublasZsymv_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZsymv_v2_64 = val;
        self
    }
    pub fn cublasChemv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasChemv_v2 = val;
        self
    }
    pub fn cublasChemv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasChemv_v2_64 = val;
        self
    }
    pub fn cublasZhemv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZhemv_v2 = val;
        self
    }
    pub fn cublasZhemv_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZhemv_v2_64 = val;
        self
    }
    pub fn cublasSsbmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSsbmv_v2 = val;
        self
    }
    pub fn cublasSsbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasSsbmv_v2_64 = val;
        self
    }
    pub fn cublasDsbmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDsbmv_v2 = val;
        self
    }
    pub fn cublasDsbmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasDsbmv_v2_64 = val;
        self
    }
    pub fn cublasChbmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasChbmv_v2 = val;
        self
    }
    pub fn cublasChbmv_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasChbmv_v2_64 = val;
        self
    }
    pub fn cublasZhbmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZhbmv_v2 = val;
        self
    }
    pub fn cublasZhbmv_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZhbmv_v2_64 = val;
        self
    }
    pub fn cublasSspmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, AP: *const f32, x: *const f32, incx: ::std::os::raw::c_int, beta: *const f32, y: *mut f32, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSspmv_v2 = val;
        self
    }
    pub fn cublasSspmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, AP: *const f32, x: *const f32, incx: i64, beta: *const f32, y: *mut f32, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasSspmv_v2_64 = val;
        self
    }
    pub fn cublasDspmv_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, AP: *const f64, x: *const f64, incx: ::std::os::raw::c_int, beta: *const f64, y: *mut f64, incy: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDspmv_v2 = val;
        self
    }
    pub fn cublasDspmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, AP: *const f64, x: *const f64, incx: i64, beta: *const f64, y: *mut f64, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasDspmv_v2_64 = val;
        self
    }
    pub fn cublasChpmv_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, AP: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, beta: *const cuComplex, y: *mut cuComplex, incy: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasChpmv_v2 = val;
        self
    }
    pub fn cublasChpmv_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, AP: *const cuComplex, x: *const cuComplex, incx: i64, beta: *const cuComplex, y: *mut cuComplex, incy: i64) -> cublasStatus_t>) -> Self {
        self.cublasChpmv_v2_64 = val;
        self
    }
    pub fn cublasZhpmv_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZhpmv_v2 = val;
        self
    }
    pub fn cublasZhpmv_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, AP: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, beta: *const cuDoubleComplex, y: *mut cuDoubleComplex, incy: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZhpmv_v2_64 = val;
        self
    }
    pub fn cublasSger_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSger_v2 = val;
        self
    }
    pub fn cublasSger_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, A: *mut f32, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasSger_v2_64 = val;
        self
    }
    pub fn cublasDger_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDger_v2 = val;
        self
    }
    pub fn cublasDger_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, A: *mut f64, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasDger_v2_64 = val;
        self
    }
    pub fn cublasCgeru_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgeru_v2 = val;
        self
    }
    pub fn cublasCgeru_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasCgeru_v2_64 = val;
        self
    }
    pub fn cublasCgerc_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgerc_v2 = val;
        self
    }
    pub fn cublasCgerc_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasCgerc_v2_64 = val;
        self
    }
    pub fn cublasZgeru_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgeru_v2 = val;
        self
    }
    pub fn cublasZgeru_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasZgeru_v2_64 = val;
        self
    }
    pub fn cublasZgerc_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgerc_v2 = val;
        self
    }
    pub fn cublasZgerc_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: i64, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasZgerc_v2_64 = val;
        self
    }
    pub fn cublasSsyr_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSsyr_v2 = val;
        self
    }
    pub fn cublasSsyr_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, A: *mut f32, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasSsyr_v2_64 = val;
        self
    }
    pub fn cublasDsyr_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDsyr_v2 = val;
        self
    }
    pub fn cublasDsyr_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, A: *mut f64, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasDsyr_v2_64 = val;
        self
    }
    pub fn cublasCsyr_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCsyr_v2 = val;
        self
    }
    pub fn cublasCsyr_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyr_v2_64 = val;
        self
    }
    pub fn cublasZsyr_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZsyr_v2 = val;
        self
    }
    pub fn cublasZsyr_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasZsyr_v2_64 = val;
        self
    }
    pub fn cublasCher_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const cuComplex, incx: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCher_v2 = val;
        self
    }
    pub fn cublasCher_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const cuComplex, incx: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasCher_v2_64 = val;
        self
    }
    pub fn cublasZher_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZher_v2 = val;
        self
    }
    pub fn cublasZher_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const cuDoubleComplex, incx: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasZher_v2_64 = val;
        self
    }
    pub fn cublasSspr_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSspr_v2 = val;
        self
    }
    pub fn cublasSspr_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, AP: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSspr_v2_64 = val;
        self
    }
    pub fn cublasDspr_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDspr_v2 = val;
        self
    }
    pub fn cublasDspr_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, AP: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDspr_v2_64 = val;
        self
    }
    pub fn cublasChpr_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const cuComplex, incx: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasChpr_v2 = val;
        self
    }
    pub fn cublasChpr_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const cuComplex, incx: i64, AP: *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasChpr_v2_64 = val;
        self
    }
    pub fn cublasZhpr_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZhpr_v2 = val;
        self
    }
    pub fn cublasZhpr_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const cuDoubleComplex, incx: i64, AP: *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZhpr_v2_64 = val;
        self
    }
    pub fn cublasSsyr2_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSsyr2_v2 = val;
        self
    }
    pub fn cublasSsyr2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, A: *mut f32, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasSsyr2_v2_64 = val;
        self
    }
    pub fn cublasDsyr2_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDsyr2_v2 = val;
        self
    }
    pub fn cublasDsyr2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, A: *mut f64, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasDsyr2_v2_64 = val;
        self
    }
    pub fn cublasCsyr2_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCsyr2_v2 = val;
        self
    }
    pub fn cublasCsyr2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyr2_v2_64 = val;
        self
    }
    pub fn cublasZsyr2_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZsyr2_v2 = val;
        self
    }
    pub fn cublasZsyr2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasZsyr2_v2_64 = val;
        self
    }
    pub fn cublasCher2_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCher2_v2 = val;
        self
    }
    pub fn cublasCher2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, A: *mut cuComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasCher2_v2_64 = val;
        self
    }
    pub fn cublasZher2_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZher2_v2 = val;
        self
    }
    pub fn cublasZher2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, A: *mut cuDoubleComplex, lda: i64) -> cublasStatus_t>) -> Self {
        self.cublasZher2_v2_64 = val;
        self
    }
    pub fn cublasSspr2_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f32, x: *const f32, incx: ::std::os::raw::c_int, y: *const f32, incy: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSspr2_v2 = val;
        self
    }
    pub fn cublasSspr2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f32, x: *const f32, incx: i64, y: *const f32, incy: i64, AP: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasSspr2_v2_64 = val;
        self
    }
    pub fn cublasDspr2_v2(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const f64, x: *const f64, incx: ::std::os::raw::c_int, y: *const f64, incy: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDspr2_v2 = val;
        self
    }
    pub fn cublasDspr2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const f64, x: *const f64, incx: i64, y: *const f64, incy: i64, AP: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDspr2_v2_64 = val;
        self
    }
    pub fn cublasChpr2_v2(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuComplex, x: *const cuComplex, incx: ::std::os::raw::c_int, y: *const cuComplex, incy: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t>,
    ) -> Self {
        self.cublasChpr2_v2 = val;
        self
    }
    pub fn cublasChpr2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuComplex, x: *const cuComplex, incx: i64, y: *const cuComplex, incy: i64, AP: *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasChpr2_v2_64 = val;
        self
    }
    pub fn cublasZhpr2_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: ::std::os::raw::c_int, y: *const cuDoubleComplex, incy: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZhpr2_v2 = val;
        self
    }
    pub fn cublasZhpr2_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: *const cuDoubleComplex, x: *const cuDoubleComplex, incx: i64, y: *const cuDoubleComplex, incy: i64, AP: *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZhpr2_v2_64 = val;
        self
    }
    pub fn cublasSgemvBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemvBatched = val;
        self
    }
    pub fn cublasSgemvBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f32, Aarray: *const *const f32, lda: i64, xarray: *const *const f32, incx: i64, beta: *const f32, yarray: *const *mut f32, incy: i64, batchCount: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgemvBatched_64 = val;
        self
    }
    pub fn cublasDgemvBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemvBatched = val;
        self
    }
    pub fn cublasDgemvBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, trans: cublasOperation_t, m: i64, n: i64, alpha: *const f64, Aarray: *const *const f64, lda: i64, xarray: *const *const f64, incx: i64, beta: *const f64, yarray: *const *mut f64, incy: i64, batchCount: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgemvBatched_64 = val;
        self
    }
    pub fn cublasCgemvBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemvBatched = val;
        self
    }
    pub fn cublasCgemvBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemvBatched_64 = val;
        self
    }
    pub fn cublasZgemvBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemvBatched = val;
        self
    }
    pub fn cublasZgemvBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemvBatched_64 = val;
        self
    }
    pub fn cublasSgemvStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemvStridedBatched = val;
        self
    }
    pub fn cublasSgemvStridedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemvStridedBatched_64 = val;
        self
    }
    pub fn cublasDgemvStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemvStridedBatched = val;
        self
    }
    pub fn cublasDgemvStridedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemvStridedBatched_64 = val;
        self
    }
    pub fn cublasCgemvStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemvStridedBatched = val;
        self
    }
    pub fn cublasCgemvStridedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemvStridedBatched_64 = val;
        self
    }
    pub fn cublasZgemvStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemvStridedBatched = val;
        self
    }
    pub fn cublasZgemvStridedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemvStridedBatched_64 = val;
        self
    }
    pub fn cublasSgemm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemm_v2 = val;
        self
    }
    pub fn cublasSgemm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgemm_v2_64 = val;
        self
    }
    pub fn cublasDgemm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemm_v2 = val;
        self
    }
    pub fn cublasDgemm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgemm_v2_64 = val;
        self
    }
    pub fn cublasCgemm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemm_v2 = val;
        self
    }
    pub fn cublasCgemm_v2_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemm_v2_64 = val;
        self
    }
    pub fn cublasCgemm3m(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemm3m = val;
        self
    }
    pub fn cublasCgemm3m_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCgemm3m_64 = val;
        self
    }
    pub fn cublasCgemm3mEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemm3mEx = val;
        self
    }
    pub fn cublasCgemm3mEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemm3mEx_64 = val;
        self
    }
    pub fn cublasZgemm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemm_v2 = val;
        self
    }
    pub fn cublasZgemm_v2_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemm_v2_64 = val;
        self
    }
    pub fn cublasZgemm3m(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemm3m = val;
        self
    }
    pub fn cublasZgemm3m_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemm3m_64 = val;
        self
    }
    pub fn cublasSgemmEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemmEx = val;
        self
    }
    pub fn cublasSgemmEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemmEx_64 = val;
        self
    }
    pub fn cublasGemmEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasGemmEx = val;
        self
    }
    pub fn cublasGemmEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasGemmEx_64 = val;
        self
    }
    pub fn cublasCgemmEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemmEx = val;
        self
    }
    pub fn cublasCgemmEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemmEx_64 = val;
        self
    }
    pub fn cublasSsyrk_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, alpha: *const f32, A: *const f32, lda: ::std::os::raw::c_int, beta: *const f32, C: *mut f32, ldc: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSsyrk_v2 = val;
        self
    }
    pub fn cublasSsyrk_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasSsyrk_v2_64 = val;
        self
    }
    pub fn cublasDsyrk_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, alpha: *const f64, A: *const f64, lda: ::std::os::raw::c_int, beta: *const f64, C: *mut f64, ldc: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDsyrk_v2 = val;
        self
    }
    pub fn cublasDsyrk_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasDsyrk_v2_64 = val;
        self
    }
    pub fn cublasCsyrk_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCsyrk_v2 = val;
        self
    }
    pub fn cublasCsyrk_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasCsyrk_v2_64 = val;
        self
    }
    pub fn cublasZsyrk_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZsyrk_v2 = val;
        self
    }
    pub fn cublasZsyrk_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, beta: *const cuDoubleComplex, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZsyrk_v2_64 = val;
        self
    }
    pub fn cublasCsyrkEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCsyrkEx = val;
        self
    }
    pub fn cublasCsyrkEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCsyrkEx_64 = val;
        self
    }
    pub fn cublasCsyrk3mEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCsyrk3mEx = val;
        self
    }
    pub fn cublasCsyrk3mEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCsyrk3mEx_64 = val;
        self
    }
    pub fn cublasCherk_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCherk_v2 = val;
        self
    }
    pub fn cublasCherk_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const cuComplex, lda: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasCherk_v2_64 = val;
        self
    }
    pub fn cublasZherk_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZherk_v2 = val;
        self
    }
    pub fn cublasZherk_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const cuDoubleComplex, lda: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasZherk_v2_64 = val;
        self
    }
    pub fn cublasCherkEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCherkEx = val;
        self
    }
    pub fn cublasCherkEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCherkEx_64 = val;
        self
    }
    pub fn cublasCherk3mEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCherk3mEx = val;
        self
    }
    pub fn cublasCherk3mEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCherk3mEx_64 = val;
        self
    }
    pub fn cublasSsyr2k_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSsyr2k_v2 = val;
        self
    }
    pub fn cublasSsyr2k_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSsyr2k_v2_64 = val;
        self
    }
    pub fn cublasDsyr2k_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDsyr2k_v2 = val;
        self
    }
    pub fn cublasDsyr2k_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDsyr2k_v2_64 = val;
        self
    }
    pub fn cublasCsyr2k_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCsyr2k_v2 = val;
        self
    }
    pub fn cublasCsyr2k_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCsyr2k_v2_64 = val;
        self
    }
    pub fn cublasZsyr2k_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZsyr2k_v2 = val;
        self
    }
    pub fn cublasZsyr2k_v2_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZsyr2k_v2_64 = val;
        self
    }
    pub fn cublasCher2k_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCher2k_v2 = val;
        self
    }
    pub fn cublasCher2k_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCher2k_v2_64 = val;
        self
    }
    pub fn cublasZher2k_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZher2k_v2 = val;
        self
    }
    pub fn cublasZher2k_v2_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, B: *const cuDoubleComplex, ldb: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZher2k_v2_64 = val;
        self
    }
    pub fn cublasSsyrkx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSsyrkx = val;
        self
    }
    pub fn cublasSsyrkx_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasSsyrkx_64 = val;
        self
    }
    pub fn cublasDsyrkx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDsyrkx = val;
        self
    }
    pub fn cublasDsyrkx_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasDsyrkx_64 = val;
        self
    }
    pub fn cublasCsyrkx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCsyrkx = val;
        self
    }
    pub fn cublasCsyrkx_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCsyrkx_64 = val;
        self
    }
    pub fn cublasZsyrkx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZsyrkx = val;
        self
    }
    pub fn cublasZsyrkx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZsyrkx_64 = val;
        self
    }
    pub fn cublasCherkx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCherkx = val;
        self
    }
    pub fn cublasCherkx_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const f32, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCherkx_64 = val;
        self
    }
    pub fn cublasZherkx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZherkx = val;
        self
    }
    pub fn cublasZherkx_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: i64, k: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, B: *const cuDoubleComplex, ldb: i64, beta: *const f64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZherkx_64 = val;
        self
    }
    pub fn cublasSsymm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSsymm_v2 = val;
        self
    }
    pub fn cublasSsymm_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, beta: *const f32, C: *mut f32, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasSsymm_v2_64 = val;
        self
    }
    pub fn cublasDsymm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDsymm_v2 = val;
        self
    }
    pub fn cublasDsymm_v2_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, beta: *const f64, C: *mut f64, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasDsymm_v2_64 = val;
        self
    }
    pub fn cublasCsymm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCsymm_v2 = val;
        self
    }
    pub fn cublasCsymm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCsymm_v2_64 = val;
        self
    }
    pub fn cublasZsymm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZsymm_v2 = val;
        self
    }
    pub fn cublasZsymm_v2_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZsymm_v2_64 = val;
        self
    }
    pub fn cublasChemm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasChemm_v2 = val;
        self
    }
    pub fn cublasChemm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *const cuComplex, ldb: i64, beta: *const cuComplex, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasChemm_v2_64 = val;
        self
    }
    pub fn cublasZhemm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZhemm_v2 = val;
        self
    }
    pub fn cublasZhemm_v2_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZhemm_v2_64 = val;
        self
    }
    pub fn cublasStrsm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasStrsm_v2 = val;
        self
    }
    pub fn cublasStrsm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *mut f32, ldb: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasStrsm_v2_64 = val;
        self
    }
    pub fn cublasDtrsm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDtrsm_v2 = val;
        self
    }
    pub fn cublasDtrsm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *mut f64, ldb: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDtrsm_v2_64 = val;
        self
    }
    pub fn cublasCtrsm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCtrsm_v2 = val;
        self
    }
    pub fn cublasCtrsm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, B: *mut cuComplex, ldb: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCtrsm_v2_64 = val;
        self
    }
    pub fn cublasZtrsm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZtrsm_v2 = val;
        self
    }
    pub fn cublasZtrsm_v2_64(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const cuDoubleComplex, A: *const cuDoubleComplex, lda: i64, B: *mut cuDoubleComplex, ldb: i64) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZtrsm_v2_64 = val;
        self
    }
    pub fn cublasStrmm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasStrmm_v2 = val;
        self
    }
    pub fn cublasStrmm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, B: *const f32, ldb: i64, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasStrmm_v2_64 = val;
        self
    }
    pub fn cublasDtrmm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDtrmm_v2 = val;
        self
    }
    pub fn cublasDtrmm_v2_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, B: *const f64, ldb: i64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDtrmm_v2_64 = val;
        self
    }
    pub fn cublasCtrmm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCtrmm_v2 = val;
        self
    }
    pub fn cublasCtrmm_v2_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCtrmm_v2_64 = val;
        self
    }
    pub fn cublasZtrmm_v2(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZtrmm_v2 = val;
        self
    }
    pub fn cublasZtrmm_v2_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZtrmm_v2_64 = val;
        self
    }
    pub fn cublasSgemmBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemmBatched = val;
        self
    }
    pub fn cublasSgemmBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemmBatched_64 = val;
        self
    }
    pub fn cublasDgemmBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemmBatched = val;
        self
    }
    pub fn cublasDgemmBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemmBatched_64 = val;
        self
    }
    pub fn cublasCgemmBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemmBatched = val;
        self
    }
    pub fn cublasCgemmBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemmBatched_64 = val;
        self
    }
    pub fn cublasCgemm3mBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemm3mBatched = val;
        self
    }
    pub fn cublasCgemm3mBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemm3mBatched_64 = val;
        self
    }
    pub fn cublasZgemmBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemmBatched = val;
        self
    }
    pub fn cublasZgemmBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemmBatched_64 = val;
        self
    }
    pub fn cublasSgemmStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemmStridedBatched = val;
        self
    }
    pub fn cublasSgemmStridedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemmStridedBatched_64 = val;
        self
    }
    pub fn cublasDgemmStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemmStridedBatched = val;
        self
    }
    pub fn cublasDgemmStridedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemmStridedBatched_64 = val;
        self
    }
    pub fn cublasCgemmStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemmStridedBatched = val;
        self
    }
    pub fn cublasCgemmStridedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemmStridedBatched_64 = val;
        self
    }
    pub fn cublasCgemm3mStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemm3mStridedBatched = val;
        self
    }
    pub fn cublasCgemm3mStridedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgemm3mStridedBatched_64 = val;
        self
    }
    pub fn cublasZgemmStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemmStridedBatched = val;
        self
    }
    pub fn cublasZgemmStridedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgemmStridedBatched_64 = val;
        self
    }
    pub fn cublasGemmBatchedEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasGemmBatchedEx = val;
        self
    }
    pub fn cublasGemmBatchedEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasGemmBatchedEx_64 = val;
        self
    }
    pub fn cublasGemmStridedBatchedEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasGemmStridedBatchedEx = val;
        self
    }
    pub fn cublasGemmStridedBatchedEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasGemmStridedBatchedEx_64 = val;
        self
    }
    pub fn cublasSgemmGroupedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemmGroupedBatched = val;
        self
    }
    pub fn cublasSgemmGroupedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgemmGroupedBatched_64 = val;
        self
    }
    pub fn cublasDgemmGroupedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemmGroupedBatched = val;
        self
    }
    pub fn cublasDgemmGroupedBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgemmGroupedBatched_64 = val;
        self
    }
    pub fn cublasGemmGroupedBatchedEx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasGemmGroupedBatchedEx = val;
        self
    }
    pub fn cublasGemmGroupedBatchedEx_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasGemmGroupedBatchedEx_64 = val;
        self
    }
    pub fn cublasSgeam(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgeam = val;
        self
    }
    pub fn cublasSgeam_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const f32, A: *const f32, lda: i64, beta: *const f32, B: *const f32, ldb: i64, C: *mut f32, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgeam_64 = val;
        self
    }
    pub fn cublasDgeam(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgeam = val;
        self
    }
    pub fn cublasDgeam_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const f64, A: *const f64, lda: i64, beta: *const f64, B: *const f64, ldb: i64, C: *mut f64, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgeam_64 = val;
        self
    }
    pub fn cublasCgeam(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgeam = val;
        self
    }
    pub fn cublasCgeam_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, alpha: *const cuComplex, A: *const cuComplex, lda: i64, beta: *const cuComplex, B: *const cuComplex, ldb: i64, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgeam_64 = val;
        self
    }
    pub fn cublasZgeam(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgeam = val;
        self
    }
    pub fn cublasZgeam_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgeam_64 = val;
        self
    }
    pub fn cublasStrsmBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasStrsmBatched = val;
        self
    }
    pub fn cublasStrsmBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, A: *const *const f32, lda: i64, B: *const *mut f32, ldb: i64, batchCount: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasStrsmBatched_64 = val;
        self
    }
    pub fn cublasDtrsmBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDtrsmBatched = val;
        self
    }
    pub fn cublasDtrsmBatched_64(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, side: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f64, A: *const *const f64, lda: i64, B: *const *mut f64, ldb: i64, batchCount: i64) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDtrsmBatched_64 = val;
        self
    }
    pub fn cublasCtrsmBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCtrsmBatched = val;
        self
    }
    pub fn cublasCtrsmBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCtrsmBatched_64 = val;
        self
    }
    pub fn cublasZtrsmBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZtrsmBatched = val;
        self
    }
    pub fn cublasZtrsmBatched_64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZtrsmBatched_64 = val;
        self
    }
    pub fn cublasSdgmm(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, x: *const f32, incx: ::std::os::raw::c_int, C: *mut f32, ldc: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSdgmm = val;
        self
    }
    pub fn cublasSdgmm_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const f32, lda: i64, x: *const f32, incx: i64, C: *mut f32, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasSdgmm_64 = val;
        self
    }
    pub fn cublasDdgmm(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, x: *const f64, incx: ::std::os::raw::c_int, C: *mut f64, ldc: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDdgmm = val;
        self
    }
    pub fn cublasDdgmm_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const f64, lda: i64, x: *const f64, incx: i64, C: *mut f64, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasDdgmm_64 = val;
        self
    }
    pub fn cublasCdgmm(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, x: *const cuComplex, incx: ::std::os::raw::c_int, C: *mut cuComplex, ldc: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasCdgmm = val;
        self
    }
    pub fn cublasCdgmm_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const cuComplex, lda: i64, x: *const cuComplex, incx: i64, C: *mut cuComplex, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasCdgmm_64 = val;
        self
    }
    pub fn cublasZdgmm(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZdgmm = val;
        self
    }
    pub fn cublasZdgmm_64(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: *const cuDoubleComplex, lda: i64, x: *const cuDoubleComplex, incx: i64, C: *mut cuDoubleComplex, ldc: i64) -> cublasStatus_t>) -> Self {
        self.cublasZdgmm_64 = val;
        self
    }
    pub fn cublasSmatinvBatched(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f32, lda: ::std::os::raw::c_int, Ainv: *const *mut f32, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSmatinvBatched = val;
        self
    }
    pub fn cublasDmatinvBatched(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f64, lda: ::std::os::raw::c_int, Ainv: *const *mut f64, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDmatinvBatched = val;
        self
    }
    pub fn cublasCmatinvBatched(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const cuComplex, lda: ::std::os::raw::c_int, Ainv: *const *mut cuComplex, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCmatinvBatched = val;
        self
    }
    pub fn cublasZmatinvBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const cuDoubleComplex, lda: ::std::os::raw::c_int, Ainv: *const *mut cuDoubleComplex, lda_inv: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZmatinvBatched = val;
        self
    }
    pub fn cublasSgeqrfBatched(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut f32, lda: ::std::os::raw::c_int, TauArray: *const *mut f32, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasSgeqrfBatched = val;
        self
    }
    pub fn cublasDgeqrfBatched(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut f64, lda: ::std::os::raw::c_int, TauArray: *const *mut f64, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasDgeqrfBatched = val;
        self
    }
    pub fn cublasCgeqrfBatched(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut cuComplex, lda: ::std::os::raw::c_int, TauArray: *const *mut cuComplex, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgeqrfBatched = val;
        self
    }
    pub fn cublasZgeqrfBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Aarray: *const *mut cuDoubleComplex, lda: ::std::os::raw::c_int, TauArray: *const *mut cuDoubleComplex, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasZgeqrfBatched = val;
        self
    }
    pub fn cublasSgelsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgelsBatched = val;
        self
    }
    pub fn cublasDgelsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgelsBatched = val;
        self
    }
    pub fn cublasCgelsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgelsBatched = val;
        self
    }
    pub fn cublasZgelsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgelsBatched = val;
        self
    }
    pub fn cublasStpttr(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const f32, A: *mut f32, lda: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasStpttr = val;
        self
    }
    pub fn cublasDtpttr(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const f64, A: *mut f64, lda: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDtpttr = val;
        self
    }
    pub fn cublasCtpttr(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const cuComplex, A: *mut cuComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasCtpttr = val;
        self
    }
    pub fn cublasZtpttr(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, AP: *const cuDoubleComplex, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasZtpttr = val;
        self
    }
    pub fn cublasStrttp(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, AP: *mut f32) -> cublasStatus_t>) -> Self {
        self.cublasStrttp = val;
        self
    }
    pub fn cublasDtrttp(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, AP: *mut f64) -> cublasStatus_t>) -> Self {
        self.cublasDtrttp = val;
        self
    }
    pub fn cublasCtrttp(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, AP: *mut cuComplex) -> cublasStatus_t>) -> Self {
        self.cublasCtrttp = val;
        self
    }
    pub fn cublasZtrttp(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, AP: *mut cuDoubleComplex) -> cublasStatus_t>) -> Self {
        self.cublasZtrttp = val;
        self
    }
    pub fn cublasSgetrfBatched(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut f32, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasSgetrfBatched = val;
        self
    }
    pub fn cublasDgetrfBatched(mut self, val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut f64, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasDgetrfBatched = val;
        self
    }
    pub fn cublasCgetrfBatched(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut cuComplex, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasCgetrfBatched = val;
        self
    }
    pub fn cublasZgetrfBatched(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *mut cuDoubleComplex, lda: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasZgetrfBatched = val;
        self
    }
    pub fn cublasSgetriBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f32, lda: ::std::os::raw::c_int, P: *const ::std::os::raw::c_int, C: *const *mut f32, ldc: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasSgetriBatched = val;
        self
    }
    pub fn cublasDgetriBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cublasHandle_t, n: ::std::os::raw::c_int, A: *const *const f64, lda: ::std::os::raw::c_int, P: *const ::std::os::raw::c_int, C: *const *mut f64, ldc: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasDgetriBatched = val;
        self
    }
    pub fn cublasCgetriBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgetriBatched = val;
        self
    }
    pub fn cublasZgetriBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgetriBatched = val;
        self
    }
    pub fn cublasSgetrsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasSgetrsBatched = val;
        self
    }
    pub fn cublasDgetrsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasDgetrsBatched = val;
        self
    }
    pub fn cublasCgetrsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasCgetrsBatched = val;
        self
    }
    pub fn cublasZgetrsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cublasZgetrsBatched = val;
        self
    }
    pub fn cublasUint8gemmBias(
        mut self,
        val: Option<
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
pub unsafe fn cublasSetWorkspace_v2<T: types::CudaAsPtr>(handle: cublasHandle_t, mut workspace: T, workspaceSizeInBytes: usize) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasSetFixedPointEmulationMantissaBitCountPointer<T: types::CudaAsPtr>(handle: cublasHandle_t, mut mantissaBitCount: T) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetFixedPointEmulationMantissaBitCountPointer(handle, mantissaBitCount.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetStatusName(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cublasGetStatusName(status) }
}
pub unsafe fn cublasGetStatusString(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cublasGetStatusString(status) }
}
pub unsafe fn cublasLoggerConfigure<T: types::CudaAsPtr>(logIsOn: i32, logToStdOut: i32, logToStdErr: i32, logFileName: T) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasSetVector<T: types::CudaAsPtr, U: types::CudaAsPtr>(n: i32, elemSize: i32, x: T, incx: i32, mut devicePtr: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetVector(n as _, elemSize as _, x.as_const_ptr() as *const _, incx as _, devicePtr.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetVector_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(n: i64, elemSize: i64, x: T, incx: i64, mut devicePtr: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetVector_64(n, elemSize, x.as_const_ptr() as *const _, incx, devicePtr.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetVector<T: types::CudaAsPtr, U: types::CudaAsPtr>(n: i32, elemSize: i32, x: T, incx: i32, mut y: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetVector(n as _, elemSize as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetVector_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(n: i64, elemSize: i64, x: T, incx: i64, mut y: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetVector_64(n, elemSize, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetMatrix<T: types::CudaAsPtr, U: types::CudaAsPtr>(rows: i32, cols: i32, elemSize: i32, A: T, lda: i32, mut B: U, ldb: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetMatrix(rows as _, cols as _, elemSize as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetMatrix_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(rows: i64, cols: i64, elemSize: i64, A: T, lda: i64, mut B: U, ldb: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetMatrix_64(rows, cols, elemSize, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetMatrix<T: types::CudaAsPtr, U: types::CudaAsPtr>(rows: i32, cols: i32, elemSize: i32, A: T, lda: i32, mut B: U, ldb: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetMatrix(rows as _, cols as _, elemSize as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetMatrix_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(rows: i64, cols: i64, elemSize: i64, A: T, lda: i64, mut B: U, ldb: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetMatrix_64(rows, cols, elemSize, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetVectorAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(n: i32, elemSize: i32, hostPtr: T, incx: i32, mut devicePtr: U, incy: i32, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetVectorAsync(n as _, elemSize as _, hostPtr.as_const_ptr() as *const _, incx as _, devicePtr.as_mut_ptr() as *mut _, incy as _, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetVectorAsync_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(n: i64, elemSize: i64, hostPtr: T, incx: i64, mut devicePtr: U, incy: i64, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetVectorAsync_64(n, elemSize, hostPtr.as_const_ptr() as *const _, incx, devicePtr.as_mut_ptr() as *mut _, incy, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetVectorAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(n: i32, elemSize: i32, devicePtr: T, incx: i32, mut hostPtr: U, incy: i32, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetVectorAsync(n as _, elemSize as _, devicePtr.as_const_ptr() as *const _, incx as _, hostPtr.as_mut_ptr() as *mut _, incy as _, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetVectorAsync_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(n: i64, elemSize: i64, devicePtr: T, incx: i64, mut hostPtr: U, incy: i64, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetVectorAsync_64(n, elemSize, devicePtr.as_const_ptr() as *const _, incx, hostPtr.as_mut_ptr() as *mut _, incy, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetMatrixAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(rows: i32, cols: i32, elemSize: i32, A: T, lda: i32, mut B: U, ldb: i32, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetMatrixAsync(rows as _, cols as _, elemSize as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSetMatrixAsync_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(rows: i64, cols: i64, elemSize: i64, A: T, lda: i64, mut B: U, ldb: i64, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetMatrixAsync_64(rows, cols, elemSize, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetMatrixAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(rows: i32, cols: i32, elemSize: i32, A: T, lda: i32, mut B: U, ldb: i32, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetMatrixAsync(rows as _, cols as _, elemSize as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasGetMatrixAsync_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(rows: i64, cols: i64, elemSize: i64, A: T, lda: i64, mut B: U, ldb: i64, stream: cudaStream_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasGetMatrixAsync_64(rows, cols, elemSize, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb, stream) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasXerbla<T: types::CudaAsPtr>(srName: T, info: i32) {
    unsafe { crate::sys::cublasXerbla(srName.as_const_ptr() as *const _, info as _) }
}
pub unsafe fn cublasNrm2Ex<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, xType: cudaDataType, incx: i32, mut result: U, resultType: cudaDataType, executionType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasNrm2Ex(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasNrm2Ex_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, xType: cudaDataType, incx: i64, mut result: U, resultType: cudaDataType, executionType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasNrm2Ex_64(handle, n, x.as_const_ptr() as *const _, xType, incx, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSnrm2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSnrm2_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSnrm2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSnrm2_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDnrm2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDnrm2_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDnrm2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDnrm2_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScnrm2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScnrm2_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScnrm2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScnrm2_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDznrm2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDznrm2_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDznrm2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDznrm2_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDotEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i32,
    x: T,
    xType: cudaDataType,
    incx: i32,
    y: U,
    yType: cudaDataType,
    incy: i32,
    mut result: V,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDotEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, y.as_const_ptr() as *const _, yType, incy as _, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDotEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i64,
    x: T,
    xType: cudaDataType,
    incx: i64,
    y: U,
    yType: cudaDataType,
    incy: i64,
    mut result: V,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDotEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, y.as_const_ptr() as *const _, yType, incy, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDotcEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i32,
    x: T,
    xType: cudaDataType,
    incx: i32,
    y: U,
    yType: cudaDataType,
    incy: i32,
    mut result: V,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDotcEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, y.as_const_ptr() as *const _, yType, incy as _, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDotcEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i64,
    x: T,
    xType: cudaDataType,
    incx: i64,
    y: U,
    yType: cudaDataType,
    incy: i64,
    mut result: V,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDotcEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, y.as_const_ptr() as *const _, yType, incy, result.as_mut_ptr() as *mut _, resultType, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSdot_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, y: U, incy: i32, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSdot_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSdot_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, y: U, incy: i64, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSdot_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDdot_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, y: U, incy: i32, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDdot_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDdot_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, y: U, incy: i64, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDdot_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdotu_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, y: U, incy: i32, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdotu_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdotu_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, y: U, incy: i64, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdotu_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdotc_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, y: U, incy: i32, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdotc_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdotc_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, y: U, incy: i64, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdotc_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdotu_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, y: U, incy: i32, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdotu_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdotu_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, y: U, incy: i64, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdotu_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdotc_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, y: U, incy: i32, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdotc_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdotc_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, y: U, incy: i64, mut result: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdotc_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScalEx<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, alphaType: cudaDataType, mut x: U, xType: cudaDataType, incx: i32, executionType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScalEx(handle, n as _, alpha.as_const_ptr() as *const _, alphaType, x.as_mut_ptr() as *mut _, xType, incx as _, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScalEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, alphaType: cudaDataType, mut x: U, xType: cudaDataType, incx: i64, executionType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScalEx_64(handle, n, alpha.as_const_ptr() as *const _, alphaType, x.as_mut_ptr() as *mut _, xType, incx, executionType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSscal_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSscal_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDscal_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDscal_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCscal_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCscal_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsscal_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsscal_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZscal_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZscal_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdscal_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdscal_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdscal_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdscal_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasAxpyEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i32,
    alpha: T,
    alphaType: cudaDataType,
    x: U,
    xType: cudaDataType,
    incx: i32,
    mut y: V,
    yType: cudaDataType,
    incy: i32,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasAxpyEx(handle, n as _, alpha.as_const_ptr() as *const _, alphaType, x.as_const_ptr() as *const _, xType, incx as _, y.as_mut_ptr() as *mut _, yType, incy as _, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasAxpyEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i64,
    alpha: T,
    alphaType: cudaDataType,
    x: U,
    xType: cudaDataType,
    incx: i64,
    mut y: V,
    yType: cudaDataType,
    incy: i64,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasAxpyEx_64(handle, n, alpha.as_const_ptr() as *const _, alphaType, x.as_const_ptr() as *const _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSaxpy_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, x: U, incx: i32, mut y: V, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSaxpy_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSaxpy_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, x: U, incx: i64, mut y: V, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSaxpy_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDaxpy_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, x: U, incx: i32, mut y: V, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDaxpy_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDaxpy_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, x: U, incx: i64, mut y: V, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDaxpy_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCaxpy_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, x: U, incx: i32, mut y: V, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCaxpy_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCaxpy_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, x: U, incx: i64, mut y: V, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCaxpy_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZaxpy_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, alpha: T, x: U, incx: i32, mut y: V, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZaxpy_v2(handle, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZaxpy_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, alpha: T, x: U, incx: i64, mut y: V, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZaxpy_v2_64(handle, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCopyEx<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, xType: cudaDataType, incx: i32, mut y: U, yType: cudaDataType, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCopyEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, y.as_mut_ptr() as *mut _, yType, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCopyEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, xType: cudaDataType, incx: i64, mut y: U, yType: cudaDataType, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCopyEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScopy_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut y: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScopy_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScopy_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut y: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScopy_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDcopy_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut y: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDcopy_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDcopy_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut y: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDcopy_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCcopy_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut y: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCcopy_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCcopy_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut y: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCcopy_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZcopy_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut y: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZcopy_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZcopy_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut y: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZcopy_v2_64(handle, n, x.as_const_ptr() as *const _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSswap_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSswap_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSswap_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSswap_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDswap_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDswap_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDswap_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDswap_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCswap_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCswap_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCswap_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCswap_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZswap_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZswap_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZswap_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZswap_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSwapEx<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, xType: cudaDataType, incx: i32, mut y: U, yType: cudaDataType, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSwapEx(handle, n as _, x.as_mut_ptr() as *mut _, xType, incx as _, y.as_mut_ptr() as *mut _, yType, incy as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSwapEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, xType: cudaDataType, incx: i64, mut y: U, yType: cudaDataType, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSwapEx_64(handle, n, x.as_mut_ptr() as *mut _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIsamax_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIsamax_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIsamax_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIsamax_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIdamax_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIdamax_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIdamax_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIdamax_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIcamax_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIcamax_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIcamax_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIcamax_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIzamax_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIzamax_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIzamax_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIzamax_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIamaxEx<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, xType: cudaDataType, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIamaxEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIamaxEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, xType: cudaDataType, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIamaxEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIsamin_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIsamin_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIsamin_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIsamin_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIdamin_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIdamin_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIdamin_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIdamin_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIcamin_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIcamin_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIcamin_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIcamin_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIzamin_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIzamin_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIzamin_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIzamin_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIaminEx<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, xType: cudaDataType, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIaminEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasIaminEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, xType: cudaDataType, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasIaminEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasAsumEx<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, xType: cudaDataType, incx: i32, mut result: U, resultType: cudaDataType, executiontype: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasAsumEx(handle, n as _, x.as_const_ptr() as *const _, xType, incx as _, result.as_mut_ptr() as *mut _, resultType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasAsumEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, xType: cudaDataType, incx: i64, mut result: U, resultType: cudaDataType, executiontype: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasAsumEx_64(handle, n, x.as_const_ptr() as *const _, xType, incx, result.as_mut_ptr() as *mut _, resultType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSasum_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSasum_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSasum_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSasum_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDasum_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDasum_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDasum_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDasum_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScasum_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScasum_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasScasum_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasScasum_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDzasum_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, x: T, incx: i32, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDzasum_v2(handle, n as _, x.as_const_ptr() as *const _, incx as _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDzasum_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, x: T, incx: i64, mut result: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDzasum_v2_64(handle, n, x.as_const_ptr() as *const _, incx, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrot_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrot_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrot_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrot_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCrot_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCrot_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsrot_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsrot_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZrot_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZrot_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdrot_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdrot_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdrot_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64, c: V, s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdrot_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i32,
    mut x: T,
    xType: cudaDataType,
    incx: i32,
    mut y: U,
    yType: cudaDataType,
    incy: i32,
    c: V,
    s: W,
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
pub unsafe fn cublasRotEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i64,
    mut x: T,
    xType: cudaDataType,
    incx: i64,
    mut y: U,
    yType: cudaDataType,
    incy: i64,
    c: V,
    s: W,
    csType: cudaDataType,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasRotEx_64(handle, n, x.as_mut_ptr() as *mut _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy, c.as_const_ptr() as *const _, s.as_const_ptr() as *const _, csType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrotg_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, mut a: T, mut b: U, mut c: V, mut s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrotg_v2(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrotg_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, mut a: T, mut b: U, mut c: V, mut s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrotg_v2(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCrotg_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, mut a: T, mut b: U, mut c: V, mut s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCrotg_v2(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZrotg_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, mut a: T, mut b: U, mut c: V, mut s: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZrotg_v2(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotgEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, mut a: T, mut b: U, abType: cudaDataType, mut c: V, mut s: W, csType: cudaDataType, executiontype: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasRotgEx(handle, a.as_mut_ptr() as *mut _, b.as_mut_ptr() as *mut _, abType, c.as_mut_ptr() as *mut _, s.as_mut_ptr() as *mut _, csType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrotm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32, param: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrotm_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, param.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrotm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64, param: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrotm_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, param.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrotm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, mut x: T, incx: i32, mut y: U, incy: i32, param: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrotm_v2(handle, n as _, x.as_mut_ptr() as *mut _, incx as _, y.as_mut_ptr() as *mut _, incy as _, param.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrotm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i64, mut x: T, incx: i64, mut y: U, incy: i64, param: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrotm_v2_64(handle, n, x.as_mut_ptr() as *mut _, incx, y.as_mut_ptr() as *mut _, incy, param.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotmEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i32,
    mut x: T,
    xType: cudaDataType,
    incx: i32,
    mut y: U,
    yType: cudaDataType,
    incy: i32,
    param: V,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasRotmEx(handle, n as _, x.as_mut_ptr() as *mut _, xType, incx as _, y.as_mut_ptr() as *mut _, yType, incy as _, param.as_const_ptr() as *const _, paramType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotmEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    n: i64,
    mut x: T,
    xType: cudaDataType,
    incx: i64,
    mut y: U,
    yType: cudaDataType,
    incy: i64,
    param: V,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasRotmEx_64(handle, n, x.as_mut_ptr() as *mut _, xType, incx, y.as_mut_ptr() as *mut _, yType, incy, param.as_const_ptr() as *const _, paramType, executiontype) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSrotmg_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, mut d1: T, mut d2: U, mut x1: V, y1: W, mut param: X) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSrotmg_v2(handle, d1.as_mut_ptr() as *mut _, d2.as_mut_ptr() as *mut _, x1.as_mut_ptr() as *mut _, y1.as_const_ptr() as *const _, param.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDrotmg_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, mut d1: T, mut d2: U, mut x1: V, y1: W, mut param: X) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDrotmg_v2(handle, d1.as_mut_ptr() as *mut _, d2.as_mut_ptr() as *mut _, x1.as_mut_ptr() as *mut _, y1.as_const_ptr() as *const _, param.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasRotmgEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    mut d1: T,
    d1Type: cudaDataType,
    mut d2: U,
    d2Type: cudaDataType,
    mut x1: V,
    x1Type: cudaDataType,
    y1: W,
    y1Type: cudaDataType,
    mut param: X,
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
pub unsafe fn cublasSgemv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasSgemv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDgemv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDgemv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasCgemv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasCgemv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZgemv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZgemv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasSgbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasSgbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDgbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDgbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasCgbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasCgbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZgbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZgbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasStrmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrmv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrmv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrmv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrmv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrmv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrmv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrmv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrmv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStbmv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStbmv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtbmv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtbmv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtbmv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtbmv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtbmv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtbmv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStpmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpmv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStpmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpmv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpmv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpmv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpmv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpmv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpmv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpmv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsv_v2(handle, uplo, trans, diag, n as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsv_v2_64(handle, uplo, trans, diag, n, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStpsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpsv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStpsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpsv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpsv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpsv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpsv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpsv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, AP: T, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpsv_v2(handle, uplo, trans, diag, n as _, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, AP: T, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpsv_v2_64(handle, uplo, trans, diag, n, AP.as_const_ptr() as *const _, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStbsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStbsv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStbsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStbsv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtbsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtbsv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtbsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtbsv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtbsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtbsv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtbsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtbsv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtbsv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i32, k: i32, A: T, lda: i32, mut x: U, incx: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtbsv_v2(handle, uplo, trans, diag, n as _, k as _, A.as_const_ptr() as *const _, lda as _, x.as_mut_ptr() as *mut _, incx as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtbsv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, n: i64, k: i64, A: T, lda: i64, mut x: U, incx: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtbsv_v2_64(handle, uplo, trans, diag, n, k, A.as_const_ptr() as *const _, lda, x.as_mut_ptr() as *mut _, incx) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsymv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasSsymv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDsymv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDsymv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasCsymv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasCsymv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZsymv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZsymv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasChemv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasChemv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZhemv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZhemv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasSsbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasSsbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDsbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDsbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasChbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasChbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZhbmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    x: V,
    incx: i32,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZhbmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    x: V,
    incx: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasSspmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, AP: U, x: V, incx: i32, beta: W, mut y: X, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasSspmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, AP: U, x: V, incx: i64, beta: W, mut y: X, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasDspmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, AP: U, x: V, incx: i32, beta: W, mut y: X, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasDspmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, AP: U, x: V, incx: i64, beta: W, mut y: X, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasChpmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, AP: U, x: V, incx: i32, beta: W, mut y: X, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasChpmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, AP: U, x: V, incx: i64, beta: W, mut y: X, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasZhpmv_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, AP: U, x: V, incx: i32, beta: W, mut y: X, incy: i32) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasZhpmv_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, AP: U, x: V, incx: i64, beta: W, mut y: X, incy: i64) -> Result<(), crate::sys::cublasStatus_t> {
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
pub unsafe fn cublasSger_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSger_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSger_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSger_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDger_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDger_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDger_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDger_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgeru_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgeru_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgeru_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgeru_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgerc_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgerc_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgerc_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgerc_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgeru_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgeru_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgeru_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgeru_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgerc_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgerc_v2(handle, m as _, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgerc_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, m: i64, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgerc_v2_64(handle, m, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut A: V, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut A: V, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut A: V, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut A: V, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut A: V, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut A: V, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut A: V, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut A: V, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut A: V, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCher_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut A: V, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCher_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut A: V, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZher_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut A: V, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZher_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspr_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut AP: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSspr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspr_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut AP: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSspr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspr_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut AP: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDspr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspr_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut AP: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDspr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpr_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut AP: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasChpr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpr_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut AP: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasChpr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpr_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, mut AP: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZhpr_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpr_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, mut AP: V) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZhpr_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyr2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyr2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyr2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCher2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCher2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCher2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut A: W, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZher2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZher2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut A: W, lda: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZher2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, A.as_mut_ptr() as *mut _, lda) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspr2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut AP: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSspr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSspr2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut AP: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSspr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspr2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut AP: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDspr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDspr2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut AP: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDspr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpr2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut AP: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasChpr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasChpr2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut AP: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasChpr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpr2_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, alpha: T, x: U, incx: i32, y: V, incy: i32, mut AP: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZhpr2_v2(handle, uplo, n as _, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx as _, y.as_const_ptr() as *const _, incy as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZhpr2_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i64, alpha: T, x: U, incx: i64, y: V, incy: i64, mut AP: W) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZhpr2_v2_64(handle, uplo, n, alpha.as_const_ptr() as *const _, x.as_const_ptr() as *const _, incx, y.as_const_ptr() as *const _, incy, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemvBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    Aarray: U,
    lda: i32,
    xarray: V,
    incx: i32,
    beta: W,
    yarray: X,
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
pub unsafe fn cublasSgemvBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    Aarray: U,
    lda: i64,
    xarray: V,
    incx: i64,
    beta: W,
    yarray: X,
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
pub unsafe fn cublasDgemvBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    Aarray: U,
    lda: i32,
    xarray: V,
    incx: i32,
    beta: W,
    yarray: X,
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
pub unsafe fn cublasDgemvBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    Aarray: U,
    lda: i64,
    xarray: V,
    incx: i64,
    beta: W,
    yarray: X,
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
pub unsafe fn cublasCgemvBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    Aarray: U,
    lda: i32,
    xarray: V,
    incx: i32,
    beta: W,
    yarray: X,
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
pub unsafe fn cublasCgemvBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    Aarray: U,
    lda: i64,
    xarray: V,
    incx: i64,
    beta: W,
    yarray: X,
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
pub unsafe fn cublasZgemvBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    Aarray: U,
    lda: i32,
    xarray: V,
    incx: i32,
    beta: W,
    yarray: X,
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
pub unsafe fn cublasZgemvBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    Aarray: U,
    lda: i64,
    xarray: V,
    incx: i64,
    beta: W,
    yarray: X,
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
pub unsafe fn cublasSgemvStridedBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    strideA: i64,
    x: V,
    incx: i32,
    stridex: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasSgemvStridedBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    strideA: i64,
    x: V,
    incx: i64,
    stridex: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDgemvStridedBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    strideA: i64,
    x: V,
    incx: i32,
    stridex: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasDgemvStridedBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    strideA: i64,
    x: V,
    incx: i64,
    stridex: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasCgemvStridedBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    strideA: i64,
    x: V,
    incx: i32,
    stridex: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasCgemvStridedBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    strideA: i64,
    x: V,
    incx: i64,
    stridex: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZgemvStridedBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    strideA: i64,
    x: V,
    incx: i32,
    stridex: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasZgemvStridedBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    strideA: i64,
    x: V,
    incx: i64,
    stridex: i64,
    beta: W,
    mut y: X,
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
pub unsafe fn cublasSgemm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSgemm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDgemm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDgemm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemm3m<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemm3m_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemm3mEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i32,
    B: V,
    Btype: cudaDataType,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemm3mEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i64,
    B: V,
    Btype: cudaDataType,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZgemm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZgemm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZgemm3m<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZgemm3m_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSgemmEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i32,
    B: V,
    Btype: cudaDataType,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSgemmEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i64,
    B: V,
    Btype: cudaDataType,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasGemmEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i32,
    B: V,
    Btype: cudaDataType,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasGemmEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i64,
    B: V,
    Btype: cudaDataType,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemmEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i32,
    B: V,
    Btype: cudaDataType,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemmEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i64,
    B: V,
    Btype: cudaDataType,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSsyrk_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    mut C: W,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyrk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyrk_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSsyrk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyrk_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    mut C: W,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyrk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDsyrk_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDsyrk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrk_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    mut C: W,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyrk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrk_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyrk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyrk_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    mut C: W,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyrk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZsyrk_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZsyrk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrkEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i32,
    beta: V,
    mut C: W,
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
pub unsafe fn cublasCsyrkEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i64,
    beta: V,
    mut C: W,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyrkEx_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, Atype, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, Ctype, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCsyrk3mEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i32,
    beta: V,
    mut C: W,
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
pub unsafe fn cublasCsyrk3mEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i64,
    beta: V,
    mut C: W,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCsyrk3mEx_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, Atype, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, Ctype, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherk_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    mut C: W,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCherk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherk_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCherk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZherk_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    mut C: W,
    ldc: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZherk_v2(handle, uplo, trans, n as _, k as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZherk_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZherk_v2_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherkEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i32,
    beta: V,
    mut C: W,
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
pub unsafe fn cublasCherkEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i64,
    beta: V,
    mut C: W,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCherkEx_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, Atype, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, Ctype, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCherk3mEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i32,
    beta: V,
    mut C: W,
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
pub unsafe fn cublasCherk3mEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i64,
    beta: V,
    mut C: W,
    Ctype: cudaDataType,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCherk3mEx_64(handle, uplo, trans, n, k, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, Atype, lda, beta.as_const_ptr() as *const _, C.as_mut_ptr() as *mut _, Ctype, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSsyr2k_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSsyr2k_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDsyr2k_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDsyr2k_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCsyr2k_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCsyr2k_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZsyr2k_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZsyr2k_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCher2k_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCher2k_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZher2k_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZher2k_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSsyrkx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSsyrkx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDsyrkx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDsyrkx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCsyrkx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCsyrkx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZsyrkx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZsyrkx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCherkx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCherkx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZherkx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZherkx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSsymm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSsymm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDsymm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDsymm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCsymm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCsymm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZsymm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZsymm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasChemm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasChemm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZhemm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZhemm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasStrsm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    mut B: V,
    ldb: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsm_v2(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    mut B: V,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    mut B: V,
    ldb: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsm_v2(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    mut B: V,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    mut B: V,
    ldb: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsm_v2(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    mut B: V,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    mut B: V,
    ldb: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsm_v2(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    mut B: V,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_mut_ptr() as *mut _, ldb) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrmm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    mut C: W,
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
pub unsafe fn cublasStrmm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrmm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrmm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    mut C: W,
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
pub unsafe fn cublasDtrmm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrmm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrmm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    mut C: W,
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
pub unsafe fn cublasCtrmm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrmm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrmm_v2<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    mut C: W,
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
pub unsafe fn cublasZtrmm_v2_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    mut C: W,
    ldc: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrmm_v2_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgemmBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    Aarray: U,
    lda: i32,
    Barray: V,
    ldb: i32,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasSgemmBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    Aarray: U,
    lda: i64,
    Barray: V,
    ldb: i64,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasDgemmBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    Aarray: U,
    lda: i32,
    Barray: V,
    ldb: i32,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasDgemmBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    Aarray: U,
    lda: i64,
    Barray: V,
    ldb: i64,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasCgemmBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    Aarray: U,
    lda: i32,
    Barray: V,
    ldb: i32,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasCgemmBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    Aarray: U,
    lda: i64,
    Barray: V,
    ldb: i64,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasCgemm3mBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    Aarray: U,
    lda: i32,
    Barray: V,
    ldb: i32,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasCgemm3mBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    Aarray: U,
    lda: i64,
    Barray: V,
    ldb: i64,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasZgemmBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    Aarray: U,
    lda: i32,
    Barray: V,
    ldb: i32,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasZgemmBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    Aarray: U,
    lda: i64,
    Barray: V,
    ldb: i64,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasSgemmStridedBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    strideA: i64,
    B: V,
    ldb: i32,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasSgemmStridedBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    strideA: i64,
    B: V,
    ldb: i64,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDgemmStridedBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    strideA: i64,
    B: V,
    ldb: i32,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasDgemmStridedBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    strideA: i64,
    B: V,
    ldb: i64,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemmStridedBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    strideA: i64,
    B: V,
    ldb: i32,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemmStridedBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    strideA: i64,
    B: V,
    ldb: i64,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemm3mStridedBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    strideA: i64,
    B: V,
    ldb: i32,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasCgemm3mStridedBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    strideA: i64,
    B: V,
    ldb: i64,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZgemmStridedBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    lda: i32,
    strideA: i64,
    B: V,
    ldb: i32,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasZgemmStridedBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    lda: i64,
    strideA: i64,
    B: V,
    ldb: i64,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasGemmBatchedEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    Aarray: U,
    Atype: cudaDataType,
    lda: i32,
    Barray: V,
    Btype: cudaDataType,
    ldb: i32,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasGemmBatchedEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    Aarray: U,
    Atype: cudaDataType,
    lda: i64,
    Barray: V,
    Btype: cudaDataType,
    ldb: i64,
    beta: W,
    Carray: X,
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
pub unsafe fn cublasGemmStridedBatchedEx<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i32,
    strideA: i64,
    B: V,
    Btype: cudaDataType,
    ldb: i32,
    strideB: i64,
    beta: W,
    mut C: X,
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
pub unsafe fn cublasGemmStridedBatchedEx_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    A: U,
    Atype: cudaDataType,
    lda: i64,
    strideA: i64,
    B: V,
    Btype: cudaDataType,
    ldb: i64,
    strideB: i64,
    beta: W,
    mut C: X,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
    C: types::CudaAsPtr,
    D: types::CudaAsPtr,
    E: types::CudaAsPtr,
    F: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T,
    transb_array: U,
    m_array: V,
    n_array: W,
    k_array: X,
    alpha_array: Y,
    Aarray: Z,
    lda_array: A,
    Barray: B,
    ldb_array: C,
    beta_array: D,
    Carray: E,
    ldc_array: F,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
    C: types::CudaAsPtr,
    D: types::CudaAsPtr,
    E: types::CudaAsPtr,
    F: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T,
    transb_array: U,
    m_array: V,
    n_array: W,
    k_array: X,
    alpha_array: Y,
    Aarray: Z,
    lda_array: A,
    Barray: B,
    ldb_array: C,
    beta_array: D,
    Carray: E,
    ldc_array: F,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
    C: types::CudaAsPtr,
    D: types::CudaAsPtr,
    E: types::CudaAsPtr,
    F: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T,
    transb_array: U,
    m_array: V,
    n_array: W,
    k_array: X,
    alpha_array: Y,
    Aarray: Z,
    lda_array: A,
    Barray: B,
    ldb_array: C,
    beta_array: D,
    Carray: E,
    ldc_array: F,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
    C: types::CudaAsPtr,
    D: types::CudaAsPtr,
    E: types::CudaAsPtr,
    F: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T,
    transb_array: U,
    m_array: V,
    n_array: W,
    k_array: X,
    alpha_array: Y,
    Aarray: Z,
    lda_array: A,
    Barray: B,
    ldb_array: C,
    beta_array: D,
    Carray: E,
    ldc_array: F,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
    C: types::CudaAsPtr,
    D: types::CudaAsPtr,
    E: types::CudaAsPtr,
    F: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T,
    transb_array: U,
    m_array: V,
    n_array: W,
    k_array: X,
    alpha_array: Y,
    Aarray: Z,
    Atype: cudaDataType_t,
    lda_array: A,
    Barray: B,
    Btype: cudaDataType_t,
    ldb_array: C,
    beta_array: D,
    Carray: E,
    Ctype: cudaDataType_t,
    ldc_array: F,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
    C: types::CudaAsPtr,
    D: types::CudaAsPtr,
    E: types::CudaAsPtr,
    F: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cublasHandle_t,
    transa_array: T,
    transb_array: U,
    m_array: V,
    n_array: W,
    k_array: X,
    alpha_array: Y,
    Aarray: Z,
    Atype: cudaDataType_t,
    lda_array: A,
    Barray: B,
    Btype: cudaDataType_t,
    ldb_array: C,
    beta_array: D,
    Carray: E,
    Ctype: cudaDataType_t,
    ldc_array: F,
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
pub unsafe fn cublasSgeam<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    B: W,
    ldb: i32,
    mut C: X,
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
pub unsafe fn cublasSgeam_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    B: W,
    ldb: i64,
    mut C: X,
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
pub unsafe fn cublasDgeam<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    B: W,
    ldb: i32,
    mut C: X,
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
pub unsafe fn cublasDgeam_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    B: W,
    ldb: i64,
    mut C: X,
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
pub unsafe fn cublasCgeam<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    B: W,
    ldb: i32,
    mut C: X,
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
pub unsafe fn cublasCgeam_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    B: W,
    ldb: i64,
    mut C: X,
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
pub unsafe fn cublasZgeam<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    beta: V,
    B: W,
    ldb: i32,
    mut C: X,
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
pub unsafe fn cublasZgeam_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr, X: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    beta: V,
    B: W,
    ldb: i64,
    mut C: X,
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
pub unsafe fn cublasStrsmBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsmBatched(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, batchCount as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrsmBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrsmBatched_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, batchCount) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsmBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsmBatched(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, batchCount as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrsmBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrsmBatched_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, batchCount) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsmBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsmBatched(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, batchCount as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrsmBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrsmBatched_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, batchCount) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsmBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    B: V,
    ldb: i32,
    batchCount: i32,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsmBatched(handle, side, uplo, trans, diag, m as _, n as _, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, batchCount as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrsmBatched_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: T,
    A: U,
    lda: i64,
    B: V,
    ldb: i64,
    batchCount: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrsmBatched_64(handle, side, uplo, trans, diag, m, n, alpha.as_const_ptr() as *const _, A.as_const_ptr() as *const _, lda, B.as_const_ptr() as *const _, ldb, batchCount) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSdgmm<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i32, n: i32, A: T, lda: i32, x: U, incx: i32, mut C: V, ldc: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSdgmm(handle, mode, m as _, n as _, A.as_const_ptr() as *const _, lda as _, x.as_const_ptr() as *const _, incx as _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSdgmm_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: T, lda: i64, x: U, incx: i64, mut C: V, ldc: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSdgmm_64(handle, mode, m, n, A.as_const_ptr() as *const _, lda, x.as_const_ptr() as *const _, incx, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDdgmm<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i32, n: i32, A: T, lda: i32, x: U, incx: i32, mut C: V, ldc: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDdgmm(handle, mode, m as _, n as _, A.as_const_ptr() as *const _, lda as _, x.as_const_ptr() as *const _, incx as _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDdgmm_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: T, lda: i64, x: U, incx: i64, mut C: V, ldc: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDdgmm_64(handle, mode, m, n, A.as_const_ptr() as *const _, lda, x.as_const_ptr() as *const _, incx, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdgmm<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i32, n: i32, A: T, lda: i32, x: U, incx: i32, mut C: V, ldc: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdgmm(handle, mode, m as _, n as _, A.as_const_ptr() as *const _, lda as _, x.as_const_ptr() as *const _, incx as _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCdgmm_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: T, lda: i64, x: U, incx: i64, mut C: V, ldc: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCdgmm_64(handle, mode, m, n, A.as_const_ptr() as *const _, lda, x.as_const_ptr() as *const _, incx, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdgmm<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i32, n: i32, A: T, lda: i32, x: U, incx: i32, mut C: V, ldc: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdgmm(handle, mode, m as _, n as _, A.as_const_ptr() as *const _, lda as _, x.as_const_ptr() as *const _, incx as _, C.as_mut_ptr() as *mut _, ldc as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZdgmm_64<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, mode: cublasSideMode_t, m: i64, n: i64, A: T, lda: i64, x: U, incx: i64, mut C: V, ldc: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZdgmm_64(handle, mode, m, n, A.as_const_ptr() as *const _, lda, x.as_const_ptr() as *const _, incx, C.as_mut_ptr() as *mut _, ldc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSmatinvBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, Ainv: U, lda_inv: i32, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSmatinvBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, Ainv.as_const_ptr() as *const _, lda_inv as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDmatinvBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, Ainv: U, lda_inv: i32, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDmatinvBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, Ainv.as_const_ptr() as *const _, lda_inv as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCmatinvBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, Ainv: U, lda_inv: i32, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCmatinvBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, Ainv.as_const_ptr() as *const _, lda_inv as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZmatinvBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, Ainv: U, lda_inv: i32, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZmatinvBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, Ainv.as_const_ptr() as *const _, lda_inv as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgeqrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, Aarray: T, lda: i32, TauArray: U, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSgeqrfBatched(handle, m as _, n as _, Aarray.as_const_ptr() as *const _, lda as _, TauArray.as_const_ptr() as *const _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgeqrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, Aarray: T, lda: i32, TauArray: U, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDgeqrfBatched(handle, m as _, n as _, Aarray.as_const_ptr() as *const _, lda as _, TauArray.as_const_ptr() as *const _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgeqrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, Aarray: T, lda: i32, TauArray: U, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgeqrfBatched(handle, m as _, n as _, Aarray.as_const_ptr() as *const _, lda as _, TauArray.as_const_ptr() as *const _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgeqrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, m: i32, n: i32, Aarray: T, lda: i32, TauArray: U, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgeqrfBatched(handle, m as _, n as _, Aarray.as_const_ptr() as *const _, lda as _, TauArray.as_const_ptr() as *const _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgelsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    nrhs: i32,
    Aarray: T,
    lda: i32,
    Carray: U,
    ldc: i32,
    mut info: V,
    mut devInfoArray: W,
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
pub unsafe fn cublasDgelsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    nrhs: i32,
    Aarray: T,
    lda: i32,
    Carray: U,
    ldc: i32,
    mut info: V,
    mut devInfoArray: W,
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
pub unsafe fn cublasCgelsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    nrhs: i32,
    Aarray: T,
    lda: i32,
    Carray: U,
    ldc: i32,
    mut info: V,
    mut devInfoArray: W,
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
pub unsafe fn cublasZgelsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    nrhs: i32,
    Aarray: T,
    lda: i32,
    Carray: U,
    ldc: i32,
    mut info: V,
    mut devInfoArray: W,
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
pub unsafe fn cublasStpttr<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, AP: T, mut A: U, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStpttr(handle, uplo, n as _, AP.as_const_ptr() as *const _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtpttr<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, AP: T, mut A: U, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtpttr(handle, uplo, n as _, AP.as_const_ptr() as *const _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtpttr<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, AP: T, mut A: U, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtpttr(handle, uplo, n as _, AP.as_const_ptr() as *const _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtpttr<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, AP: T, mut A: U, lda: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtpttr(handle, uplo, n as _, AP.as_const_ptr() as *const _, A.as_mut_ptr() as *mut _, lda as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasStrttp<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, A: T, lda: i32, mut AP: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasStrttp(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDtrttp<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, A: T, lda: i32, mut AP: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDtrttp(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCtrttp<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, A: T, lda: i32, mut AP: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCtrttp(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZtrttp<T: types::CudaAsPtr, U: types::CudaAsPtr>(handle: cublasHandle_t, uplo: cublasFillMode_t, n: i32, A: T, lda: i32, mut AP: U) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZtrttp(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, AP.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgetrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, mut P: U, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSgetrfBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_mut_ptr() as *mut _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgetrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, mut P: U, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDgetrfBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_mut_ptr() as *mut _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgetrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, mut P: U, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgetrfBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_mut_ptr() as *mut _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgetrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, mut P: U, mut info: V, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgetrfBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_mut_ptr() as *mut _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgetriBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, P: U, C: V, ldc: i32, mut info: W, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSgetriBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_const_ptr() as *const _, C.as_const_ptr() as *const _, ldc as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasDgetriBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, P: U, C: V, ldc: i32, mut info: W, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasDgetriBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_const_ptr() as *const _, C.as_const_ptr() as *const _, ldc as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasCgetriBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, P: U, C: V, ldc: i32, mut info: W, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasCgetriBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_const_ptr() as *const _, C.as_const_ptr() as *const _, ldc as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasZgetriBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(handle: cublasHandle_t, n: i32, A: T, lda: i32, P: U, C: V, ldc: i32, mut info: W, batchSize: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasZgetriBatched(handle, n as _, A.as_const_ptr() as *const _, lda as _, P.as_const_ptr() as *const _, C.as_const_ptr() as *const _, ldc as _, info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cublasSgetrsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    Aarray: T,
    lda: i32,
    devIpiv: U,
    Barray: V,
    ldb: i32,
    mut info: W,
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
pub unsafe fn cublasDgetrsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    Aarray: T,
    lda: i32,
    devIpiv: U,
    Barray: V,
    ldb: i32,
    mut info: W,
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
pub unsafe fn cublasCgetrsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    Aarray: T,
    lda: i32,
    devIpiv: U,
    Barray: V,
    ldb: i32,
    mut info: W,
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
pub unsafe fn cublasZgetrsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    Aarray: T,
    lda: i32,
    devIpiv: U,
    Barray: V,
    ldb: i32,
    mut info: W,
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
pub unsafe fn cublasUint8gemmBias<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    transc: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    A_bias: i32,
    lda: i32,
    B: U,
    B_bias: i32,
    ldb: i32,
    mut C: V,
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
