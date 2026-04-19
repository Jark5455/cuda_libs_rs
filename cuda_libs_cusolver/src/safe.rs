#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
pub use crate::sys::__off_t;
pub use crate::sys::__off64_t;
pub use crate::sys::__uint64_t;
pub use crate::sys::CUSOLVER_VER_BUILD;
pub use crate::sys::CUSOLVER_VER_MAJOR;
pub use crate::sys::CUSOLVER_VER_MINOR;
pub use crate::sys::CUSOLVER_VER_PATCH;
pub use crate::sys::CUSOLVER_VERSION;
pub use crate::sys::csrqrInfo;
pub use crate::sys::csrqrInfo_t;
pub use crate::sys::cusolver_int_t;
pub use crate::sys::cusolverAlgMode_t;
pub use crate::sys::cusolverDeterministicMode_t;
pub use crate::sys::cusolverDirectMode_t;
pub use crate::sys::cusolverDnContext;
pub use crate::sys::cusolverDnFunction_t;
pub use crate::sys::cusolverDnHandle_t;
pub use crate::sys::cusolverDnIRSInfos;
pub use crate::sys::cusolverDnIRSInfos_t;
pub use crate::sys::cusolverDnIRSParams;
pub use crate::sys::cusolverDnIRSParams_t;
pub use crate::sys::cusolverDnLoggerCallback_t;
pub use crate::sys::cusolverDnParams;
pub use crate::sys::cusolverDnParams_t;
pub use crate::sys::cusolverEigComp_t;
pub use crate::sys::cusolverEigMode_t;
pub use crate::sys::cusolverEigRange_t;
pub use crate::sys::cusolverEigType_t;
pub use crate::sys::cusolverIRSRefinement_t;
pub use crate::sys::cusolverMathMode_t;
pub use crate::sys::cusolverNorm_t;
pub use crate::sys::cusolverPrecType_t;
pub use crate::sys::cusolverSpContext;
pub use crate::sys::cusolverSpHandle_t;
pub use crate::sys::cusolverStatus_t as CudaTargetStatus;
pub use crate::sys::cusolverStatus_t;
pub use crate::sys::cusolverStorevMode_t;
pub use crate::sys::gesvdjInfo;
pub use crate::sys::gesvdjInfo_t;
pub use crate::sys::syevjInfo;
pub use crate::sys::syevjInfo_t;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cublas;
#[allow(unused_imports)]
use cuda_libs_cublas::sys::*;
use cuda_libs_cudart;
#[allow(unused_imports)]
use cuda_libs_cudart::sys::*;
#[allow(unused_imports)]
use cuda_libs_cudart::types;
use cuda_libs_cusparse;
#[allow(unused_imports)]
use cuda_libs_cusparse::sys::*;
#[cfg(feature = "runtime-link")]
impl crate::sys::DynamicBindings {
    pub fn cusolverGetProperty(mut self, val: Option<unsafe extern "C" fn(libraryPropertyType, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverGetProperty = val;
        self
    }
    pub fn cusolverGetVersion(mut self, val: Option<unsafe extern "C" fn(*mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverGetVersion = val;
        self
    }
    pub fn cusolverDnCreate(mut self, val: Option<unsafe extern "C" fn(*mut cusolverDnHandle_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCreate = val;
        self
    }
    pub fn cusolverDnDestroy(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDestroy = val;
        self
    }
    pub fn cusolverDnSetStream(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cudaStream_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSetStream = val;
        self
    }
    pub fn cusolverDnGetStream(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, *mut cudaStream_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnGetStream = val;
        self
    }
    pub fn cusolverDnSetDeterministicMode(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDeterministicMode_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSetDeterministicMode = val;
        self
    }
    pub fn cusolverDnGetDeterministicMode(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, *mut cusolverDeterministicMode_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnGetDeterministicMode = val;
        self
    }
    pub fn cusolverDnSetMathMode(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverMathMode_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSetMathMode = val;
        self
    }
    pub fn cusolverDnGetMathMode(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, *mut cusolverMathMode_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnGetMathMode = val;
        self
    }
    pub fn cusolverDnSetEmulationStrategy(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cudaEmulationStrategy_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSetEmulationStrategy = val;
        self
    }
    pub fn cusolverDnGetEmulationStrategy(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, *mut cudaEmulationStrategy_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnGetEmulationStrategy = val;
        self
    }
    pub fn cusolverDnSetFixedPointEmulationMantissaControl(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cudaEmulationMantissaControl_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSetFixedPointEmulationMantissaControl = val;
        self
    }
    pub fn cusolverDnGetFixedPointEmulationMantissaControl(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, *mut cudaEmulationMantissaControl_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnGetFixedPointEmulationMantissaControl = val;
        self
    }
    pub fn cusolverDnSetFixedPointEmulationMaxMantissaBitCount(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSetFixedPointEmulationMaxMantissaBitCount = val;
        self
    }
    pub fn cusolverDnGetFixedPointEmulationMaxMantissaBitCount(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnGetFixedPointEmulationMaxMantissaBitCount = val;
        self
    }
    pub fn cusolverDnSetFixedPointEmulationMantissaBitOffset(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSetFixedPointEmulationMantissaBitOffset = val;
        self
    }
    pub fn cusolverDnGetFixedPointEmulationMantissaBitOffset(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnGetFixedPointEmulationMantissaBitOffset = val;
        self
    }
    pub fn cusolverDnSetEmulationSpecialValuesSupport(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cudaEmulationSpecialValuesSupport_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSetEmulationSpecialValuesSupport = val;
        self
    }
    pub fn cusolverDnGetEmulationSpecialValuesSupport(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, *mut cudaEmulationSpecialValuesSupport_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnGetEmulationSpecialValuesSupport = val;
        self
    }
    pub fn cusolverDnIRSParamsCreate(mut self, val: Option<unsafe extern "C" fn(*mut cusolverDnIRSParams_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsCreate = val;
        self
    }
    pub fn cusolverDnIRSParamsDestroy(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsDestroy = val;
        self
    }
    pub fn cusolverDnIRSParamsSetRefinementSolver(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t, cusolverIRSRefinement_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsSetRefinementSolver = val;
        self
    }
    pub fn cusolverDnIRSParamsSetSolverMainPrecision(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t, cusolverPrecType_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsSetSolverMainPrecision = val;
        self
    }
    pub fn cusolverDnIRSParamsSetSolverLowestPrecision(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t, cusolverPrecType_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsSetSolverLowestPrecision = val;
        self
    }
    pub fn cusolverDnIRSParamsSetSolverPrecisions(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t, cusolverPrecType_t, cusolverPrecType_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsSetSolverPrecisions = val;
        self
    }
    pub fn cusolverDnIRSParamsSetTol(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t, f64) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsSetTol = val;
        self
    }
    pub fn cusolverDnIRSParamsSetTolInner(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t, f64) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsSetTolInner = val;
        self
    }
    pub fn cusolverDnIRSParamsSetMaxIters(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t, cusolver_int_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsSetMaxIters = val;
        self
    }
    pub fn cusolverDnIRSParamsSetMaxItersInner(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t, cusolver_int_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsSetMaxItersInner = val;
        self
    }
    pub fn cusolverDnIRSParamsGetMaxIters(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t, *mut cusolver_int_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsGetMaxIters = val;
        self
    }
    pub fn cusolverDnIRSParamsEnableFallback(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsEnableFallback = val;
        self
    }
    pub fn cusolverDnIRSParamsDisableFallback(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSParams_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSParamsDisableFallback = val;
        self
    }
    pub fn cusolverDnIRSInfosDestroy(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSInfos_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSInfosDestroy = val;
        self
    }
    pub fn cusolverDnIRSInfosCreate(mut self, val: Option<unsafe extern "C" fn(*mut cusolverDnIRSInfos_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSInfosCreate = val;
        self
    }
    pub fn cusolverDnIRSInfosGetNiters(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSInfos_t, *mut cusolver_int_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSInfosGetNiters = val;
        self
    }
    pub fn cusolverDnIRSInfosGetOuterNiters(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSInfos_t, *mut cusolver_int_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSInfosGetOuterNiters = val;
        self
    }
    pub fn cusolverDnIRSInfosRequestResidual(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSInfos_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSInfosRequestResidual = val;
        self
    }
    pub fn cusolverDnIRSInfosGetResidualHistory(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSInfos_t, *mut *mut ::std::os::raw::c_void) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSInfosGetResidualHistory = val;
        self
    }
    pub fn cusolverDnIRSInfosGetMaxIters(mut self, val: Option<unsafe extern "C" fn(cusolverDnIRSInfos_t, *mut cusolver_int_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSInfosGetMaxIters = val;
        self
    }
    pub fn cusolverDnZZgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZZgesv = val;
        self
    }
    pub fn cusolverDnZCgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZCgesv = val;
        self
    }
    pub fn cusolverDnZKgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZKgesv = val;
        self
    }
    pub fn cusolverDnZEgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZEgesv = val;
        self
    }
    pub fn cusolverDnZYgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZYgesv = val;
        self
    }
    pub fn cusolverDnCCgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCCgesv = val;
        self
    }
    pub fn cusolverDnCEgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCEgesv = val;
        self
    }
    pub fn cusolverDnCKgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCKgesv = val;
        self
    }
    pub fn cusolverDnCYgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCYgesv = val;
        self
    }
    pub fn cusolverDnDDgesv(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDDgesv = val;
        self
    }
    pub fn cusolverDnDSgesv(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDSgesv = val;
        self
    }
    pub fn cusolverDnDHgesv(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDHgesv = val;
        self
    }
    pub fn cusolverDnDBgesv(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDBgesv = val;
        self
    }
    pub fn cusolverDnDXgesv(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDXgesv = val;
        self
    }
    pub fn cusolverDnSSgesv(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSSgesv = val;
        self
    }
    pub fn cusolverDnSHgesv(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSHgesv = val;
        self
    }
    pub fn cusolverDnSBgesv(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSBgesv = val;
        self
    }
    pub fn cusolverDnSXgesv(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSXgesv = val;
        self
    }
    pub fn cusolverDnZZgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZZgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZCgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZCgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZKgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZKgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZEgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZEgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZYgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZYgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnCCgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCCgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnCKgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCKgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnCEgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCEgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnCYgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCYgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDDgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDDgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDSgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDSgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDHgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDHgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDBgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDBgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDXgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDXgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnSSgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSSgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnSHgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSHgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnSBgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSBgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnSXgesv_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSXgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZZgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZZgels = val;
        self
    }
    pub fn cusolverDnZCgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZCgels = val;
        self
    }
    pub fn cusolverDnZKgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZKgels = val;
        self
    }
    pub fn cusolverDnZEgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZEgels = val;
        self
    }
    pub fn cusolverDnZYgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolver_int_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut cuDoubleComplex,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZYgels = val;
        self
    }
    pub fn cusolverDnCCgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCCgels = val;
        self
    }
    pub fn cusolverDnCKgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCKgels = val;
        self
    }
    pub fn cusolverDnCEgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCEgels = val;
        self
    }
    pub fn cusolverDnCYgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCYgels = val;
        self
    }
    pub fn cusolverDnDDgels(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDDgels = val;
        self
    }
    pub fn cusolverDnDSgels(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDSgels = val;
        self
    }
    pub fn cusolverDnDHgels(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDHgels = val;
        self
    }
    pub fn cusolverDnDBgels(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDBgels = val;
        self
    }
    pub fn cusolverDnDXgels(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDXgels = val;
        self
    }
    pub fn cusolverDnSSgels(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSSgels = val;
        self
    }
    pub fn cusolverDnSHgels(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSHgels = val;
        self
    }
    pub fn cusolverDnSBgels(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSBgels = val;
        self
    }
    pub fn cusolverDnSXgels(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, usize, *mut cusolver_int_t, *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSXgels = val;
        self
    }
    pub fn cusolverDnZZgels_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZZgels_bufferSize = val;
        self
    }
    pub fn cusolverDnZCgels_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZCgels_bufferSize = val;
        self
    }
    pub fn cusolverDnZKgels_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZKgels_bufferSize = val;
        self
    }
    pub fn cusolverDnZEgels_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZEgels_bufferSize = val;
        self
    }
    pub fn cusolverDnZYgels_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut cuDoubleComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZYgels_bufferSize = val;
        self
    }
    pub fn cusolverDnCCgels_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCCgels_bufferSize = val;
        self
    }
    pub fn cusolverDnCKgels_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCKgels_bufferSize = val;
        self
    }
    pub fn cusolverDnCEgels_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCEgels_bufferSize = val;
        self
    }
    pub fn cusolverDnCYgels_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut cuComplex, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCYgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDDgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDDgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDSgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDSgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDHgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDHgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDBgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDBgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDXgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut f64, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDXgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSSgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSSgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSHgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSHgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSBgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSBgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSXgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut f32, cusolver_int_t, *mut ::std::os::raw::c_void, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSXgels_bufferSize = val;
        self
    }
    pub fn cusolverDnIRSXgesv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnIRSParams_t,
                cusolverDnIRSInfos_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSXgesv = val;
        self
    }
    pub fn cusolverDnIRSXgesv_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnIRSParams_t, cusolver_int_t, cusolver_int_t, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSXgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnIRSXgels(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnIRSParams_t,
                cusolverDnIRSInfos_t,
                cusolver_int_t,
                cusolver_int_t,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                cusolver_int_t,
                *mut ::std::os::raw::c_void,
                usize,
                *mut cusolver_int_t,
                *mut cusolver_int_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSXgels = val;
        self
    }
    pub fn cusolverDnIRSXgels_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnIRSParams_t, cusolver_int_t, cusolver_int_t, cusolver_int_t, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnIRSXgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSpotrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnDpotrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnCpotrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnZpotrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnSpotrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSpotrf = val;
        self
    }
    pub fn cusolverDnDpotrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDpotrf = val;
        self
    }
    pub fn cusolverDnCpotrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCpotrf = val;
        self
    }
    pub fn cusolverDnZpotrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZpotrf = val;
        self
    }
    pub fn cusolverDnSpotrs(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSpotrs = val;
        self
    }
    pub fn cusolverDnDpotrs(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDpotrs = val;
        self
    }
    pub fn cusolverDnCpotrs(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCpotrs = val;
        self
    }
    pub fn cusolverDnZpotrs(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZpotrs = val;
        self
    }
    pub fn cusolverDnSpotrfBatched(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSpotrfBatched = val;
        self
    }
    pub fn cusolverDnDpotrfBatched(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDpotrfBatched = val;
        self
    }
    pub fn cusolverDnCpotrfBatched(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCpotrfBatched = val;
        self
    }
    pub fn cusolverDnZpotrfBatched(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZpotrfBatched = val;
        self
    }
    pub fn cusolverDnSpotrsBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut *mut f32, ::std::os::raw::c_int, *mut *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSpotrsBatched = val;
        self
    }
    pub fn cusolverDnDpotrsBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut *mut f64, ::std::os::raw::c_int, *mut *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDpotrsBatched = val;
        self
    }
    pub fn cusolverDnCpotrsBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut *mut cuComplex, ::std::os::raw::c_int, *mut *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCpotrsBatched = val;
        self
    }
    pub fn cusolverDnZpotrsBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut *mut cuDoubleComplex, ::std::os::raw::c_int, *mut *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZpotrsBatched = val;
        self
    }
    pub fn cusolverDnSpotri_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSpotri_bufferSize = val;
        self
    }
    pub fn cusolverDnDpotri_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDpotri_bufferSize = val;
        self
    }
    pub fn cusolverDnCpotri_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCpotri_bufferSize = val;
        self
    }
    pub fn cusolverDnZpotri_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZpotri_bufferSize = val;
        self
    }
    pub fn cusolverDnSpotri(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSpotri = val;
        self
    }
    pub fn cusolverDnDpotri(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDpotri = val;
        self
    }
    pub fn cusolverDnCpotri(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCpotri = val;
        self
    }
    pub fn cusolverDnZpotri(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZpotri = val;
        self
    }
    pub fn cusolverDnXtrtri_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, cublasDiagType_t, i64, cudaDataType, *mut ::std::os::raw::c_void, i64, *mut usize, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXtrtri_bufferSize = val;
        self
    }
    pub fn cusolverDnXtrtri(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, cublasDiagType_t, i64, cudaDataType, *mut ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_void, usize, *mut ::std::os::raw::c_void, usize, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXtrtri = val;
        self
    }
    pub fn cusolverDnSlauum_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSlauum_bufferSize = val;
        self
    }
    pub fn cusolverDnDlauum_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDlauum_bufferSize = val;
        self
    }
    pub fn cusolverDnClauum_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnClauum_bufferSize = val;
        self
    }
    pub fn cusolverDnZlauum_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZlauum_bufferSize = val;
        self
    }
    pub fn cusolverDnSlauum(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSlauum = val;
        self
    }
    pub fn cusolverDnDlauum(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDlauum = val;
        self
    }
    pub fn cusolverDnClauum(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnClauum = val;
        self
    }
    pub fn cusolverDnZlauum(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZlauum = val;
        self
    }
    pub fn cusolverDnSgetrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnDgetrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnCgetrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnZgetrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnSgetrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSgetrf = val;
        self
    }
    pub fn cusolverDnDgetrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDgetrf = val;
        self
    }
    pub fn cusolverDnCgetrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut cuComplex, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCgetrf = val;
        self
    }
    pub fn cusolverDnZgetrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZgetrf = val;
        self
    }
    pub fn cusolverDnSlaswp(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSlaswp = val;
        self
    }
    pub fn cusolverDnDlaswp(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDlaswp = val;
        self
    }
    pub fn cusolverDnClaswp(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnClaswp = val;
        self
    }
    pub fn cusolverDnZlaswp(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const ::std::os::raw::c_int, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZlaswp = val;
        self
    }
    pub fn cusolverDnSgetrs(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSgetrs = val;
        self
    }
    pub fn cusolverDnDgetrs(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDgetrs = val;
        self
    }
    pub fn cusolverDnCgetrs(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCgetrs = val;
        self
    }
    pub fn cusolverDnZgetrs(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZgetrs = val;
        self
    }
    pub fn cusolverDnSgeqrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnDgeqrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnCgeqrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnZgeqrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnSgeqrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSgeqrf = val;
        self
    }
    pub fn cusolverDnDgeqrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDgeqrf = val;
        self
    }
    pub fn cusolverDnCgeqrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut cuComplex, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCgeqrf = val;
        self
    }
    pub fn cusolverDnZgeqrf(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZgeqrf = val;
        self
    }
    pub fn cusolverDnSorgqr_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSorgqr_bufferSize = val;
        self
    }
    pub fn cusolverDnDorgqr_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDorgqr_bufferSize = val;
        self
    }
    pub fn cusolverDnCungqr_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCungqr_bufferSize = val;
        self
    }
    pub fn cusolverDnZungqr_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZungqr_bufferSize = val;
        self
    }
    pub fn cusolverDnSorgqr(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSorgqr = val;
        self
    }
    pub fn cusolverDnDorgqr(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDorgqr = val;
        self
    }
    pub fn cusolverDnCungqr(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCungqr = val;
        self
    }
    pub fn cusolverDnZungqr(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZungqr = val;
        self
    }
    pub fn cusolverDnSormqr_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSormqr_bufferSize = val;
        self
    }
    pub fn cusolverDnDormqr_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDormqr_bufferSize = val;
        self
    }
    pub fn cusolverDnCunmqr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCunmqr_bufferSize = val;
        self
    }
    pub fn cusolverDnZunmqr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZunmqr_bufferSize = val;
        self
    }
    pub fn cusolverDnSormqr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSormqr = val;
        self
    }
    pub fn cusolverDnDormqr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDormqr = val;
        self
    }
    pub fn cusolverDnCunmqr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCunmqr = val;
        self
    }
    pub fn cusolverDnZunmqr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZunmqr = val;
        self
    }
    pub fn cusolverDnSsytrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsytrf_bufferSize = val;
        self
    }
    pub fn cusolverDnDsytrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsytrf_bufferSize = val;
        self
    }
    pub fn cusolverDnCsytrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCsytrf_bufferSize = val;
        self
    }
    pub fn cusolverDnZsytrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZsytrf_bufferSize = val;
        self
    }
    pub fn cusolverDnSsytrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsytrf = val;
        self
    }
    pub fn cusolverDnDsytrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsytrf = val;
        self
    }
    pub fn cusolverDnCsytrf(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCsytrf = val;
        self
    }
    pub fn cusolverDnZsytrf(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZsytrf = val;
        self
    }
    pub fn cusolverDnXsytrs_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, i64, i64, cudaDataType, *const ::std::os::raw::c_void, i64, *const i64, cudaDataType, *mut ::std::os::raw::c_void, i64, *mut usize, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXsytrs_bufferSize = val;
        self
    }
    pub fn cusolverDnXsytrs(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasFillMode_t,
                i64,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                *const i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsytrs = val;
        self
    }
    pub fn cusolverDnSsytri_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsytri_bufferSize = val;
        self
    }
    pub fn cusolverDnDsytri_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsytri_bufferSize = val;
        self
    }
    pub fn cusolverDnCsytri_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCsytri_bufferSize = val;
        self
    }
    pub fn cusolverDnZsytri_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZsytri_bufferSize = val;
        self
    }
    pub fn cusolverDnSsytri(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsytri = val;
        self
    }
    pub fn cusolverDnDsytri(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsytri = val;
        self
    }
    pub fn cusolverDnCsytri(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCsytri = val;
        self
    }
    pub fn cusolverDnZsytri(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZsytri = val;
        self
    }
    pub fn cusolverDnSgebrd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSgebrd_bufferSize = val;
        self
    }
    pub fn cusolverDnDgebrd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDgebrd_bufferSize = val;
        self
    }
    pub fn cusolverDnCgebrd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCgebrd_bufferSize = val;
        self
    }
    pub fn cusolverDnZgebrd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZgebrd_bufferSize = val;
        self
    }
    pub fn cusolverDnSgebrd(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, *mut f32, *mut f32, *mut f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSgebrd = val;
        self
    }
    pub fn cusolverDnDgebrd(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, *mut f64, *mut f64, *mut f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDgebrd = val;
        self
    }
    pub fn cusolverDnCgebrd(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut f32, *mut f32, *mut cuComplex, *mut cuComplex, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCgebrd = val;
        self
    }
    pub fn cusolverDnZgebrd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut f64,
                *mut f64,
                *mut cuDoubleComplex,
                *mut cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgebrd = val;
        self
    }
    pub fn cusolverDnSorgbr_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSorgbr_bufferSize = val;
        self
    }
    pub fn cusolverDnDorgbr_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDorgbr_bufferSize = val;
        self
    }
    pub fn cusolverDnCungbr_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCungbr_bufferSize = val;
        self
    }
    pub fn cusolverDnZungbr_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZungbr_bufferSize = val;
        self
    }
    pub fn cusolverDnSorgbr(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSorgbr = val;
        self
    }
    pub fn cusolverDnDorgbr(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDorgbr = val;
        self
    }
    pub fn cusolverDnCungbr(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCungbr = val;
        self
    }
    pub fn cusolverDnZungbr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZungbr = val;
        self
    }
    pub fn cusolverDnSsytrd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *const f32, *const f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsytrd_bufferSize = val;
        self
    }
    pub fn cusolverDnDsytrd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *const f64, *const f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsytrd_bufferSize = val;
        self
    }
    pub fn cusolverDnChetrd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const f32, *const f32, *const cuComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnChetrd_bufferSize = val;
        self
    }
    pub fn cusolverDnZhetrd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const f64, *const f64, *const cuDoubleComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZhetrd_bufferSize = val;
        self
    }
    pub fn cusolverDnSsytrd(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, *mut f32, *mut f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsytrd = val;
        self
    }
    pub fn cusolverDnDsytrd(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, *mut f64, *mut f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsytrd = val;
        self
    }
    pub fn cusolverDnChetrd(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut f32, *mut f32, *mut cuComplex, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnChetrd = val;
        self
    }
    pub fn cusolverDnZhetrd(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut f64, *mut f64, *mut cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZhetrd = val;
        self
    }
    pub fn cusolverDnSorgtr_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSorgtr_bufferSize = val;
        self
    }
    pub fn cusolverDnDorgtr_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDorgtr_bufferSize = val;
        self
    }
    pub fn cusolverDnCungtr_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCungtr_bufferSize = val;
        self
    }
    pub fn cusolverDnZungtr_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZungtr_bufferSize = val;
        self
    }
    pub fn cusolverDnSorgtr(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *const f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSorgtr = val;
        self
    }
    pub fn cusolverDnDorgtr(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *const f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDorgtr = val;
        self
    }
    pub fn cusolverDnCungtr(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *const cuComplex, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCungtr = val;
        self
    }
    pub fn cusolverDnZungtr(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZungtr = val;
        self
    }
    pub fn cusolverDnSormtr_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *const f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSormtr_bufferSize = val;
        self
    }
    pub fn cusolverDnDormtr_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cublasSideMode_t, cublasFillMode_t, cublasOperation_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *const f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDormtr_bufferSize = val;
        self
    }
    pub fn cusolverDnCunmtr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCunmtr_bufferSize = val;
        self
    }
    pub fn cusolverDnZunmtr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZunmtr_bufferSize = val;
        self
    }
    pub fn cusolverDnSormtr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSormtr = val;
        self
    }
    pub fn cusolverDnDormtr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDormtr = val;
        self
    }
    pub fn cusolverDnCunmtr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCunmtr = val;
        self
    }
    pub fn cusolverDnZunmtr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cublasSideMode_t,
                cublasFillMode_t,
                cublasOperation_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZunmtr = val;
        self
    }
    pub fn cusolverDnSgesvd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnDgesvd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnCgesvd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnZgesvd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnSgesvd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgesvd = val;
        self
    }
    pub fn cusolverDnDgesvd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgesvd = val;
        self
    }
    pub fn cusolverDnCgesvd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut f32,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut f32,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgesvd = val;
        self
    }
    pub fn cusolverDnZgesvd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut f64,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut f64,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgesvd = val;
        self
    }
    pub fn cusolverDnSsyevd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsyevd_bufferSize = val;
        self
    }
    pub fn cusolverDnDsyevd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsyevd_bufferSize = val;
        self
    }
    pub fn cusolverDnCheevd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCheevd_bufferSize = val;
        self
    }
    pub fn cusolverDnZheevd_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZheevd_bufferSize = val;
        self
    }
    pub fn cusolverDnSsyevd(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsyevd = val;
        self
    }
    pub fn cusolverDnDsyevd(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsyevd = val;
        self
    }
    pub fn cusolverDnCheevd(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut f32, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCheevd = val;
        self
    }
    pub fn cusolverDnZheevd(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut f64, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZheevd = val;
        self
    }
    pub fn cusolverDnSsyevdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                f32,
                f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *const f32,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsyevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnDsyevdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                f64,
                f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *const f64,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsyevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnCheevdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                f32,
                f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *const f32,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCheevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnZheevdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                f64,
                f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *const f64,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZheevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnSsyevdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                f32,
                f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f32,
                *mut f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsyevdx = val;
        self
    }
    pub fn cusolverDnDsyevdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                f64,
                f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f64,
                *mut f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsyevdx = val;
        self
    }
    pub fn cusolverDnCheevdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                f32,
                f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f32,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCheevdx = val;
        self
    }
    pub fn cusolverDnZheevdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                f64,
                f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f64,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZheevdx = val;
        self
    }
    pub fn cusolverDnSsygvdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                f32,
                f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *const f32,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsygvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnDsygvdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                f64,
                f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *const f64,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsygvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnChegvdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                f32,
                f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *const f32,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnChegvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnZhegvdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                f64,
                f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *const f64,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZhegvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnSsygvdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                f32,
                f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f32,
                *mut f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsygvdx = val;
        self
    }
    pub fn cusolverDnDsygvdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                f64,
                f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f64,
                *mut f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsygvdx = val;
        self
    }
    pub fn cusolverDnChegvdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                f32,
                f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f32,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnChegvdx = val;
        self
    }
    pub fn cusolverDnZhegvdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                f64,
                f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f64,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZhegvdx = val;
        self
    }
    pub fn cusolverDnSsygvd_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSsygvd_bufferSize = val;
        self
    }
    pub fn cusolverDnDsygvd_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDsygvd_bufferSize = val;
        self
    }
    pub fn cusolverDnChegvd_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnChegvd_bufferSize = val;
        self
    }
    pub fn cusolverDnZhegvd_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZhegvd_bufferSize = val;
        self
    }
    pub fn cusolverDnSsygvd(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSsygvd = val;
        self
    }
    pub fn cusolverDnDsygvd(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDsygvd = val;
        self
    }
    pub fn cusolverDnChegvd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut f32,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnChegvd = val;
        self
    }
    pub fn cusolverDnZhegvd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut f64,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZhegvd = val;
        self
    }
    pub fn cusolverDnXsygvd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                cudaDataType,
                *mut usize,
                *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsygvd_bufferSize = val;
        self
    }
    pub fn cusolverDnXsygvd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsygvd = val;
        self
    }
    pub fn cusolverDnXsygvdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                *mut ::std::os::raw::c_void,
                *mut ::std::os::raw::c_void,
                i64,
                i64,
                *mut i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                cudaDataType,
                *mut usize,
                *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsygvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnXsygvdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                *mut ::std::os::raw::c_void,
                *mut ::std::os::raw::c_void,
                i64,
                i64,
                *mut i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsygvdx = val;
        self
    }
    pub fn cusolverDnCreateSyevjInfo(mut self, val: Option<unsafe extern "C" fn(*mut syevjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCreateSyevjInfo = val;
        self
    }
    pub fn cusolverDnDestroySyevjInfo(mut self, val: Option<unsafe extern "C" fn(syevjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDestroySyevjInfo = val;
        self
    }
    pub fn cusolverDnXsyevjSetTolerance(mut self, val: Option<unsafe extern "C" fn(syevjInfo_t, f64) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXsyevjSetTolerance = val;
        self
    }
    pub fn cusolverDnXsyevjSetMaxSweeps(mut self, val: Option<unsafe extern "C" fn(syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXsyevjSetMaxSweeps = val;
        self
    }
    pub fn cusolverDnXsyevjSetSortEig(mut self, val: Option<unsafe extern "C" fn(syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXsyevjSetSortEig = val;
        self
    }
    pub fn cusolverDnXsyevjGetResidual(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, syevjInfo_t, *mut f64) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXsyevjGetResidual = val;
        self
    }
    pub fn cusolverDnXsyevjGetSweeps(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, syevjInfo_t, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXsyevjGetSweeps = val;
        self
    }
    pub fn cusolverDnSsyevjBatched_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int, syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSsyevjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnDsyevjBatched_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int, syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDsyevjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnCheevjBatched_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int, syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCheevjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnZheevjBatched_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int, syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZheevjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnSsyevjBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSsyevjBatched = val;
        self
    }
    pub fn cusolverDnDsyevjBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDsyevjBatched = val;
        self
    }
    pub fn cusolverDnCheevjBatched(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut f32, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCheevjBatched = val;
        self
    }
    pub fn cusolverDnZheevjBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut f64, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, syevjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZheevjBatched = val;
        self
    }
    pub fn cusolverDnSsyevj_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsyevj_bufferSize = val;
        self
    }
    pub fn cusolverDnDsyevj_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsyevj_bufferSize = val;
        self
    }
    pub fn cusolverDnCheevj_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCheevj_bufferSize = val;
        self
    }
    pub fn cusolverDnZheevj_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnZheevj_bufferSize = val;
        self
    }
    pub fn cusolverDnSsyevj(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f32, ::std::os::raw::c_int, *mut f32, *mut f32, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSsyevj = val;
        self
    }
    pub fn cusolverDnDsyevj(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut f64, ::std::os::raw::c_int, *mut f64, *mut f64, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDsyevj = val;
        self
    }
    pub fn cusolverDnCheevj(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuComplex, ::std::os::raw::c_int, *mut f32, *mut cuComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCheevj = val;
        self
    }
    pub fn cusolverDnZheevj(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut f64, *mut cuDoubleComplex, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnZheevj = val;
        self
    }
    pub fn cusolverDnSsygvj_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSsygvj_bufferSize = val;
        self
    }
    pub fn cusolverDnDsygvj_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDsygvj_bufferSize = val;
        self
    }
    pub fn cusolverDnChegvj_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const cuComplex, ::std::os::raw::c_int, *const f32, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnChegvj_bufferSize = val;
        self
    }
    pub fn cusolverDnZhegvj_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverDnHandle_t, cusolverEigType_t, cusolverEigMode_t, cublasFillMode_t, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const cuDoubleComplex, ::std::os::raw::c_int, *const f64, *mut ::std::os::raw::c_int, syevjInfo_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZhegvj_bufferSize = val;
        self
    }
    pub fn cusolverDnSsygvj(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                *mut f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                syevjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsygvj = val;
        self
    }
    pub fn cusolverDnDsygvj(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                *mut f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                syevjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsygvj = val;
        self
    }
    pub fn cusolverDnChegvj(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut f32,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                syevjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnChegvj = val;
        self
    }
    pub fn cusolverDnZhegvj(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigType_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut f64,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                syevjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZhegvj = val;
        self
    }
    pub fn cusolverDnCreateGesvdjInfo(mut self, val: Option<unsafe extern "C" fn(*mut gesvdjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCreateGesvdjInfo = val;
        self
    }
    pub fn cusolverDnDestroyGesvdjInfo(mut self, val: Option<unsafe extern "C" fn(gesvdjInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDestroyGesvdjInfo = val;
        self
    }
    pub fn cusolverDnXgesvdjSetTolerance(mut self, val: Option<unsafe extern "C" fn(gesvdjInfo_t, f64) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXgesvdjSetTolerance = val;
        self
    }
    pub fn cusolverDnXgesvdjSetMaxSweeps(mut self, val: Option<unsafe extern "C" fn(gesvdjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXgesvdjSetMaxSweeps = val;
        self
    }
    pub fn cusolverDnXgesvdjSetSortEig(mut self, val: Option<unsafe extern "C" fn(gesvdjInfo_t, ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXgesvdjSetSortEig = val;
        self
    }
    pub fn cusolverDnXgesvdjGetResidual(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, gesvdjInfo_t, *mut f64) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXgesvdjGetResidual = val;
        self
    }
    pub fn cusolverDnXgesvdjGetSweeps(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, gesvdjInfo_t, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXgesvdjGetSweeps = val;
        self
    }
    pub fn cusolverDnSgesvdjBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgesvdjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnDgesvdjBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgesvdjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnCgesvdjBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const f32,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgesvdjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnZgesvdjBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const f64,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgesvdjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnSgesvdjBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgesvdjBatched = val;
        self
    }
    pub fn cusolverDnDgesvdjBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgesvdjBatched = val;
        self
    }
    pub fn cusolverDnCgesvdjBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut f32,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgesvdjBatched = val;
        self
    }
    pub fn cusolverDnZgesvdjBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut f64,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgesvdjBatched = val;
        self
    }
    pub fn cusolverDnSgesvdj_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                *const f32,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgesvdj_bufferSize = val;
        self
    }
    pub fn cusolverDnDgesvdj_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                *const f64,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgesvdj_bufferSize = val;
        self
    }
    pub fn cusolverDnCgesvdj_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const f32,
                *const cuComplex,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgesvdj_bufferSize = val;
        self
    }
    pub fn cusolverDnZgesvdj_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const f64,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgesvdj_bufferSize = val;
        self
    }
    pub fn cusolverDnSgesvdj(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgesvdj = val;
        self
    }
    pub fn cusolverDnDgesvdj(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgesvdj = val;
        self
    }
    pub fn cusolverDnCgesvdj(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut f32,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgesvdj = val;
        self
    }
    pub fn cusolverDnZgesvdj(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut f64,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                gesvdjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgesvdj = val;
        self
    }
    pub fn cusolverDnSgesvdaStridedBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f32,
                ::std::os::raw::c_longlong,
                *const f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgesvdaStridedBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnDgesvdaStridedBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f64,
                ::std::os::raw::c_longlong,
                *const f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgesvdaStridedBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnCgesvdaStridedBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f32,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgesvdaStridedBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnZgesvdaStridedBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const f64,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut ::std::os::raw::c_int,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgesvdaStridedBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnSgesvdaStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut f32,
                ::std::os::raw::c_longlong,
                *mut f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut f32,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut f32,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgesvdaStridedBatched = val;
        self
    }
    pub fn cusolverDnDgesvdaStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut f64,
                ::std::os::raw::c_longlong,
                *mut f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut f64,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut f64,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgesvdaStridedBatched = val;
        self
    }
    pub fn cusolverDnCgesvdaStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut f32,
                ::std::os::raw::c_longlong,
                *mut cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut cuComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut cuComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgesvdaStridedBatched = val;
        self
    }
    pub fn cusolverDnZgesvdaStridedBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut f64,
                ::std::os::raw::c_longlong,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                ::std::os::raw::c_longlong,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut f64,
                ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgesvdaStridedBatched = val;
        self
    }
    pub fn cusolverDnCreateParams(mut self, val: Option<unsafe extern "C" fn(*mut cusolverDnParams_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnCreateParams = val;
        self
    }
    pub fn cusolverDnDestroyParams(mut self, val: Option<unsafe extern "C" fn(cusolverDnParams_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnDestroyParams = val;
        self
    }
    pub fn cusolverDnSetAdvOptions(mut self, val: Option<unsafe extern "C" fn(cusolverDnParams_t, cusolverDnFunction_t, cusolverAlgMode_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnSetAdvOptions = val;
        self
    }
    pub fn cusolverDnXpotrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, cublasFillMode_t, i64, cudaDataType, *const ::std::os::raw::c_void, i64, cudaDataType, *mut usize, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnXpotrf(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, cublasFillMode_t, i64, cudaDataType, *mut ::std::os::raw::c_void, i64, cudaDataType, *mut ::std::os::raw::c_void, usize, *mut ::std::os::raw::c_void, usize, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXpotrf = val;
        self
    }
    pub fn cusolverDnXpotrs(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, cublasFillMode_t, i64, i64, cudaDataType, *const ::std::os::raw::c_void, i64, cudaDataType, *mut ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXpotrs = val;
        self
    }
    pub fn cusolverDnXgeqrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, i64, i64, cudaDataType, *const ::std::os::raw::c_void, i64, cudaDataType, *const ::std::os::raw::c_void, cudaDataType, *mut usize, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnXgeqrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                i64,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgeqrf = val;
        self
    }
    pub fn cusolverDnXgetrf_bufferSize(mut self, val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, i64, i64, cudaDataType, *const ::std::os::raw::c_void, i64, cudaDataType, *mut usize, *mut usize) -> cusolverStatus_t>) -> Self {
        self.cusolverDnXgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnXgetrf(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, i64, i64, cudaDataType, *mut ::std::os::raw::c_void, i64, *mut i64, cudaDataType, *mut ::std::os::raw::c_void, usize, *mut ::std::os::raw::c_void, usize, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXgetrf = val;
        self
    }
    pub fn cusolverDnXgetrs(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, cublasOperation_t, i64, i64, cudaDataType, *const ::std::os::raw::c_void, i64, *const i64, cudaDataType, *mut ::std::os::raw::c_void, i64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXgetrs = val;
        self
    }
    pub fn cusolverDnXsyevd_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, cusolverEigMode_t, cublasFillMode_t, i64, cudaDataType, *const ::std::os::raw::c_void, i64, cudaDataType, *const ::std::os::raw::c_void, cudaDataType, *mut usize, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXsyevd_bufferSize = val;
        self
    }
    pub fn cusolverDnXsyevd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsyevd = val;
        self
    }
    pub fn cusolverDnXstedc_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, cusolverEigComp_t, i64, cudaDataType, *const ::std::os::raw::c_void, *const ::std::os::raw::c_void, cudaDataType, *const ::std::os::raw::c_void, i64, cudaDataType, *mut usize, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXstedc_bufferSize = val;
        self
    }
    pub fn cusolverDnXstedc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigComp_t,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXstedc = val;
        self
    }
    pub fn cusolverDnXsyevBatched_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, cusolverEigMode_t, cublasFillMode_t, i64, cudaDataType, *const ::std::os::raw::c_void, i64, cudaDataType, *const ::std::os::raw::c_void, cudaDataType, *mut usize, *mut usize, i64) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXsyevBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnXsyevBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigMode_t,
                cublasFillMode_t,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
                i64,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsyevBatched = val;
        self
    }
    pub fn cusolverDnXsyevdx_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                *mut ::std::os::raw::c_void,
                *mut ::std::os::raw::c_void,
                i64,
                i64,
                *mut i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                cudaDataType,
                *mut usize,
                *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsyevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnXsyevdx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigMode_t,
                cusolverEigRange_t,
                cublasFillMode_t,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                *mut ::std::os::raw::c_void,
                *mut ::std::os::raw::c_void,
                i64,
                i64,
                *mut i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsyevdx = val;
        self
    }
    pub fn cusolverDnXgeev_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigMode_t,
                cusolverEigMode_t,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut usize,
                *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgeev_bufferSize = val;
        self
    }
    pub fn cusolverDnXgeev(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigMode_t,
                cusolverEigMode_t,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgeev = val;
        self
    }
    pub fn cusolverDnXgesvd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_schar,
                i64,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut usize,
                *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnXgesvd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_schar,
                i64,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgesvd = val;
        self
    }
    pub fn cusolverDnXgesvdp_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                i64,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut usize,
                *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgesvdp_bufferSize = val;
        self
    }
    pub fn cusolverDnXgesvdp(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverEigMode_t,
                ::std::os::raw::c_int,
                i64,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
                *mut f64,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgesvdp = val;
        self
    }
    pub fn cusolverDnXgesvdr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_schar,
                i64,
                i64,
                i64,
                i64,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut usize,
                *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgesvdr_bufferSize = val;
        self
    }
    pub fn cusolverDnXgesvdr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                ::std::os::raw::c_schar,
                ::std::os::raw::c_schar,
                i64,
                i64,
                i64,
                i64,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgesvdr = val;
        self
    }
    pub fn cusolverDnXlarft_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverDirectMode_t,
                cusolverStorevMode_t,
                i64,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut usize,
                *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXlarft_bufferSize = val;
        self
    }
    pub fn cusolverDnXlarft(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cusolverDirectMode_t,
                cusolverStorevMode_t,
                i64,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *const ::std::os::raw::c_void,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXlarft = val;
        self
    }
    pub fn cusolverDnLoggerSetCallback(mut self, val: Option<unsafe extern "C" fn(cusolverDnLoggerCallback_t) -> cusolverStatus_t>) -> Self {
        self.cusolverDnLoggerSetCallback = val;
        self
    }
    pub fn cusolverDnLoggerSetFile(mut self, val: Option<unsafe extern "C" fn(*mut FILE) -> cusolverStatus_t>) -> Self {
        self.cusolverDnLoggerSetFile = val;
        self
    }
    pub fn cusolverDnLoggerOpenFile(mut self, val: Option<unsafe extern "C" fn(*const ::std::os::raw::c_char) -> cusolverStatus_t>) -> Self {
        self.cusolverDnLoggerOpenFile = val;
        self
    }
    pub fn cusolverDnLoggerSetLevel(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnLoggerSetLevel = val;
        self
    }
    pub fn cusolverDnLoggerSetMask(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverDnLoggerSetMask = val;
        self
    }
    pub fn cusolverDnLoggerForceDisable(mut self, val: Option<unsafe extern "C" fn() -> cusolverStatus_t>) -> Self {
        self.cusolverDnLoggerForceDisable = val;
        self
    }
    pub fn cusolverDnXpolar_bufferSize(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverDnHandle_t, cusolverDnParams_t, cublasFillMode_t, i64, i64, cudaDataType, *const ::std::os::raw::c_void, i64, cudaDataType, *const ::std::os::raw::c_void, i64, cudaDataType, *mut usize, *mut usize) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXpolar_bufferSize = val;
        self
    }
    pub fn cusolverDnXpolar(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverDnHandle_t,
                cusolverDnParams_t,
                cublasFillMode_t,
                i64,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                i64,
                cudaDataType,
                *mut ::std::os::raw::c_void,
                usize,
                *mut ::std::os::raw::c_void,
                usize,
                *mut f64,
                *mut f64,
                *mut f64,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXpolar = val;
        self
    }
    pub fn cusolverSpCreate(mut self, val: Option<unsafe extern "C" fn(*mut cusolverSpHandle_t) -> cusolverStatus_t>) -> Self {
        self.cusolverSpCreate = val;
        self
    }
    pub fn cusolverSpDestroy(mut self, val: Option<unsafe extern "C" fn(cusolverSpHandle_t) -> cusolverStatus_t>) -> Self {
        self.cusolverSpDestroy = val;
        self
    }
    pub fn cusolverSpSetStream(mut self, val: Option<unsafe extern "C" fn(cusolverSpHandle_t, cudaStream_t) -> cusolverStatus_t>) -> Self {
        self.cusolverSpSetStream = val;
        self
    }
    pub fn cusolverSpGetStream(mut self, val: Option<unsafe extern "C" fn(cusolverSpHandle_t, *mut cudaStream_t) -> cusolverStatus_t>) -> Self {
        self.cusolverSpGetStream = val;
        self
    }
    pub fn cusolverSpXcsrissymHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpXcsrissymHost = val;
        self
    }
    pub fn cusolverSpScsrlsvluHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f32, f32, ::std::os::raw::c_int, *mut f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpScsrlsvluHost = val;
        self
    }
    pub fn cusolverSpDcsrlsvluHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f64, f64, ::std::os::raw::c_int, *mut f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDcsrlsvluHost = val;
        self
    }
    pub fn cusolverSpCcsrlsvluHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuComplex,
                f32,
                ::std::os::raw::c_int,
                *mut cuComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsrlsvluHost = val;
        self
    }
    pub fn cusolverSpZcsrlsvluHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuDoubleComplex,
                f64,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsrlsvluHost = val;
        self
    }
    pub fn cusolverSpScsrlsvqr(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f32, f32, ::std::os::raw::c_int, *mut f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpScsrlsvqr = val;
        self
    }
    pub fn cusolverSpDcsrlsvqr(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f64, f64, ::std::os::raw::c_int, *mut f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDcsrlsvqr = val;
        self
    }
    pub fn cusolverSpCcsrlsvqr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuComplex,
                f32,
                ::std::os::raw::c_int,
                *mut cuComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsrlsvqr = val;
        self
    }
    pub fn cusolverSpZcsrlsvqr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuDoubleComplex,
                f64,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsrlsvqr = val;
        self
    }
    pub fn cusolverSpScsrlsvqrHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f32, f32, ::std::os::raw::c_int, *mut f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpScsrlsvqrHost = val;
        self
    }
    pub fn cusolverSpDcsrlsvqrHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f64, f64, ::std::os::raw::c_int, *mut f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDcsrlsvqrHost = val;
        self
    }
    pub fn cusolverSpCcsrlsvqrHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuComplex,
                f32,
                ::std::os::raw::c_int,
                *mut cuComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsrlsvqrHost = val;
        self
    }
    pub fn cusolverSpZcsrlsvqrHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuDoubleComplex,
                f64,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsrlsvqrHost = val;
        self
    }
    pub fn cusolverSpScsrlsvcholHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f32, f32, ::std::os::raw::c_int, *mut f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpScsrlsvcholHost = val;
        self
    }
    pub fn cusolverSpDcsrlsvcholHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f64, f64, ::std::os::raw::c_int, *mut f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDcsrlsvcholHost = val;
        self
    }
    pub fn cusolverSpCcsrlsvcholHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuComplex,
                f32,
                ::std::os::raw::c_int,
                *mut cuComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsrlsvcholHost = val;
        self
    }
    pub fn cusolverSpZcsrlsvcholHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuDoubleComplex,
                f64,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsrlsvcholHost = val;
        self
    }
    pub fn cusolverSpScsrlsvchol(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f32, f32, ::std::os::raw::c_int, *mut f32, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpScsrlsvchol = val;
        self
    }
    pub fn cusolverSpDcsrlsvchol(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const f64, f64, ::std::os::raw::c_int, *mut f64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDcsrlsvchol = val;
        self
    }
    pub fn cusolverSpCcsrlsvchol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuComplex,
                f32,
                ::std::os::raw::c_int,
                *mut cuComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsrlsvchol = val;
        self
    }
    pub fn cusolverSpZcsrlsvchol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuDoubleComplex,
                f64,
                ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsrlsvchol = val;
        self
    }
    pub fn cusolverSpScsrlsqvqrHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const f32,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const f32,
                f32,
                *mut ::std::os::raw::c_int,
                *mut f32,
                *mut ::std::os::raw::c_int,
                *mut f32,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpScsrlsqvqrHost = val;
        self
    }
    pub fn cusolverSpDcsrlsqvqrHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const f64,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const f64,
                f64,
                *mut ::std::os::raw::c_int,
                *mut f64,
                *mut ::std::os::raw::c_int,
                *mut f64,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpDcsrlsqvqrHost = val;
        self
    }
    pub fn cusolverSpCcsrlsqvqrHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuComplex,
                f32,
                *mut ::std::os::raw::c_int,
                *mut cuComplex,
                *mut ::std::os::raw::c_int,
                *mut f32,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsrlsqvqrHost = val;
        self
    }
    pub fn cusolverSpZcsrlsqvqrHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuDoubleComplex,
                f64,
                *mut ::std::os::raw::c_int,
                *mut cuDoubleComplex,
                *mut ::std::os::raw::c_int,
                *mut f64,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsrlsqvqrHost = val;
        self
    }
    pub fn cusolverSpScsreigvsiHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, f32, *const f32, ::std::os::raw::c_int, f32, *mut f32, *mut f32) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpScsreigvsiHost = val;
        self
    }
    pub fn cusolverSpDcsreigvsiHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, f64, *const f64, ::std::os::raw::c_int, f64, *mut f64, *mut f64) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDcsreigvsiHost = val;
        self
    }
    pub fn cusolverSpCcsreigvsiHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                f32,
                *mut cuComplex,
                *mut cuComplex,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsreigvsiHost = val;
        self
    }
    pub fn cusolverSpZcsreigvsiHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                f64,
                *mut cuDoubleComplex,
                *mut cuDoubleComplex,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsreigvsiHost = val;
        self
    }
    pub fn cusolverSpScsreigvsi(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, f32, *const f32, ::std::os::raw::c_int, f32, *mut f32, *mut f32) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpScsreigvsi = val;
        self
    }
    pub fn cusolverSpDcsreigvsi(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, f64, *const f64, ::std::os::raw::c_int, f64, *mut f64, *mut f64) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDcsreigvsi = val;
        self
    }
    pub fn cusolverSpCcsreigvsi(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                cuComplex,
                *const cuComplex,
                ::std::os::raw::c_int,
                f32,
                *mut cuComplex,
                *mut cuComplex,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsreigvsi = val;
        self
    }
    pub fn cusolverSpZcsreigvsi(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                cuDoubleComplex,
                *const cuDoubleComplex,
                ::std::os::raw::c_int,
                f64,
                *mut cuDoubleComplex,
                *mut cuDoubleComplex,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsreigvsi = val;
        self
    }
    pub fn cusolverSpScsreigsHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, cuComplex, cuComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpScsreigsHost = val;
        self
    }
    pub fn cusolverSpDcsreigsHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, cuDoubleComplex, cuDoubleComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDcsreigsHost = val;
        self
    }
    pub fn cusolverSpCcsreigsHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const cuComplex, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, cuComplex, cuComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpCcsreigsHost = val;
        self
    }
    pub fn cusolverSpZcsreigsHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const cuDoubleComplex, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, cuDoubleComplex, cuDoubleComplex, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpZcsreigsHost = val;
        self
    }
    pub fn cusolverSpXcsrsymrcmHost(mut self, val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverSpXcsrsymrcmHost = val;
        self
    }
    pub fn cusolverSpXcsrsymmdqHost(mut self, val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverSpXcsrsymmdqHost = val;
        self
    }
    pub fn cusolverSpXcsrsymamdHost(mut self, val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverSpXcsrsymamdHost = val;
        self
    }
    pub fn cusolverSpXcsrmetisndHost(mut self, val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const i64, *mut ::std::os::raw::c_int) -> cusolverStatus_t>) -> Self {
        self.cusolverSpXcsrmetisndHost = val;
        self
    }
    pub fn cusolverSpScsrzfdHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpScsrzfdHost = val;
        self
    }
    pub fn cusolverSpDcsrzfdHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDcsrzfdHost = val;
        self
    }
    pub fn cusolverSpCcsrzfdHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const cuComplex, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpCcsrzfdHost = val;
        self
    }
    pub fn cusolverSpZcsrzfdHost(
        mut self,
        val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const cuDoubleComplex, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpZcsrzfdHost = val;
        self
    }
    pub fn cusolverSpXcsrperm_bufferSizeHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, *mut usize) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpXcsrperm_bufferSizeHost = val;
        self
    }
    pub fn cusolverSpXcsrpermHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *mut ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *mut ::std::os::raw::c_int,
                *mut ::std::os::raw::c_void,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpXcsrpermHost = val;
        self
    }
    pub fn cusolverSpCreateCsrqrInfo(mut self, val: Option<unsafe extern "C" fn(*mut csrqrInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverSpCreateCsrqrInfo = val;
        self
    }
    pub fn cusolverSpDestroyCsrqrInfo(mut self, val: Option<unsafe extern "C" fn(csrqrInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverSpDestroyCsrqrInfo = val;
        self
    }
    pub fn cusolverSpXcsrqrAnalysisBatched(mut self, val: Option<unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, csrqrInfo_t) -> cusolverStatus_t>) -> Self {
        self.cusolverSpXcsrqrAnalysisBatched = val;
        self
    }
    pub fn cusolverSpScsrqrBufferInfoBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f32, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, ::std::os::raw::c_int, csrqrInfo_t, *mut usize, *mut usize) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpScsrqrBufferInfoBatched = val;
        self
    }
    pub fn cusolverSpDcsrqrBufferInfoBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const f64, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, ::std::os::raw::c_int, csrqrInfo_t, *mut usize, *mut usize) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpDcsrqrBufferInfoBatched = val;
        self
    }
    pub fn cusolverSpCcsrqrBufferInfoBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(cusolverSpHandle_t, ::std::os::raw::c_int, ::std::os::raw::c_int, ::std::os::raw::c_int, cusparseMatDescr_t, *const cuComplex, *const ::std::os::raw::c_int, *const ::std::os::raw::c_int, ::std::os::raw::c_int, csrqrInfo_t, *mut usize, *mut usize) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsrqrBufferInfoBatched = val;
        self
    }
    pub fn cusolverSpZcsrqrBufferInfoBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                csrqrInfo_t,
                *mut usize,
                *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsrqrBufferInfoBatched = val;
        self
    }
    pub fn cusolverSpScsrqrsvBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const f32,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const f32,
                *mut f32,
                ::std::os::raw::c_int,
                csrqrInfo_t,
                *mut ::std::os::raw::c_void,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpScsrqrsvBatched = val;
        self
    }
    pub fn cusolverSpDcsrqrsvBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const f64,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const f64,
                *mut f64,
                ::std::os::raw::c_int,
                csrqrInfo_t,
                *mut ::std::os::raw::c_void,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpDcsrqrsvBatched = val;
        self
    }
    pub fn cusolverSpCcsrqrsvBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuComplex,
                *mut cuComplex,
                ::std::os::raw::c_int,
                csrqrInfo_t,
                *mut ::std::os::raw::c_void,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpCcsrqrsvBatched = val;
        self
    }
    pub fn cusolverSpZcsrqrsvBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cusolverSpHandle_t,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                cusparseMatDescr_t,
                *const cuDoubleComplex,
                *const ::std::os::raw::c_int,
                *const ::std::os::raw::c_int,
                *const cuDoubleComplex,
                *mut cuDoubleComplex,
                ::std::os::raw::c_int,
                csrqrInfo_t,
                *mut ::std::os::raw::c_void,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpZcsrqrsvBatched = val;
        self
    }
}
pub unsafe fn cusolverGetProperty(type_: libraryPropertyType) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverGetVersion() -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnCreate() -> Result<cusolverDnHandle_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverDnHandle_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverDnHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnDestroy(handle: cusolverDnHandle_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDestroy(handle) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSetStream(handle: cusolverDnHandle_t, streamId: cudaStream_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetStream(handle, streamId) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnGetStream(handle: cusolverDnHandle_t) -> Result<cudaStream_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnGetStream(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaStream_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetDeterministicMode(handle: cusolverDnHandle_t, mode: cusolverDeterministicMode_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetDeterministicMode(handle, mode) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnGetDeterministicMode(handle: cusolverDnHandle_t) -> Result<cusolverDeterministicMode_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolverDeterministicMode_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnGetDeterministicMode(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolverDeterministicMode_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetMathMode(handle: cusolverDnHandle_t, mode: cusolverMathMode_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetMathMode(handle, mode) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnGetMathMode(handle: cusolverDnHandle_t) -> Result<cusolverMathMode_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolverMathMode_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnGetMathMode(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolverMathMode_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetEmulationStrategy(handle: cusolverDnHandle_t, strategy: cudaEmulationStrategy_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetEmulationStrategy(handle, strategy) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnGetEmulationStrategy(handle: cusolverDnHandle_t) -> Result<cudaEmulationStrategy_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaEmulationStrategy_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnGetEmulationStrategy(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaEmulationStrategy_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetFixedPointEmulationMantissaControl(handle: cusolverDnHandle_t, control: cudaEmulationMantissaControl_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetFixedPointEmulationMantissaControl(handle, control) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnGetFixedPointEmulationMantissaControl(handle: cusolverDnHandle_t) -> Result<cudaEmulationMantissaControl_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaEmulationMantissaControl_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnGetFixedPointEmulationMantissaControl(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaEmulationMantissaControl_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetFixedPointEmulationMaxMantissaBitCount(handle: cusolverDnHandle_t, mantissaBitCount: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetFixedPointEmulationMaxMantissaBitCount(handle, mantissaBitCount as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnGetFixedPointEmulationMaxMantissaBitCount(handle: cusolverDnHandle_t) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnGetFixedPointEmulationMaxMantissaBitCount(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetFixedPointEmulationMantissaBitOffset(handle: cusolverDnHandle_t, mantissaBitOffset: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetFixedPointEmulationMantissaBitOffset(handle, mantissaBitOffset as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnGetFixedPointEmulationMantissaBitOffset(handle: cusolverDnHandle_t) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnGetFixedPointEmulationMantissaBitOffset(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetEmulationSpecialValuesSupport(handle: cusolverDnHandle_t, mask: cudaEmulationSpecialValuesSupport_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetEmulationSpecialValuesSupport(handle, mask) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnGetEmulationSpecialValuesSupport(handle: cusolverDnHandle_t) -> Result<cudaEmulationSpecialValuesSupport_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaEmulationSpecialValuesSupport_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnGetEmulationSpecialValuesSupport(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaEmulationSpecialValuesSupport_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSParamsCreate() -> Result<cusolverDnIRSParams_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverDnIRSParams_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnIRSParamsCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverDnIRSParams_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSParamsDestroy(params: cusolverDnIRSParams_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsDestroy(params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsSetRefinementSolver(params: cusolverDnIRSParams_t, refinement_solver: cusolverIRSRefinement_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetRefinementSolver(params, refinement_solver) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsSetSolverMainPrecision(params: cusolverDnIRSParams_t, solver_main_precision: cusolverPrecType_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetSolverMainPrecision(params, solver_main_precision) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsSetSolverLowestPrecision(params: cusolverDnIRSParams_t, solver_lowest_precision: cusolverPrecType_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetSolverLowestPrecision(params, solver_lowest_precision) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsSetSolverPrecisions(params: cusolverDnIRSParams_t, solver_main_precision: cusolverPrecType_t, solver_lowest_precision: cusolverPrecType_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetSolverPrecisions(params, solver_main_precision, solver_lowest_precision) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsSetTol(params: cusolverDnIRSParams_t, val: f64) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetTol(params, val) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsSetTolInner(params: cusolverDnIRSParams_t, val: f64) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetTolInner(params, val) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsSetMaxIters(params: cusolverDnIRSParams_t, maxiters: cusolver_int_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetMaxIters(params, maxiters) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsSetMaxItersInner(params: cusolverDnIRSParams_t, maxiters_inner: cusolver_int_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetMaxItersInner(params, maxiters_inner) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsGetMaxIters(params: cusolverDnIRSParams_t) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnIRSParamsGetMaxIters(params, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolver_int_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSParamsEnableFallback(params: cusolverDnIRSParams_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsEnableFallback(params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSParamsDisableFallback(params: cusolverDnIRSParams_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsDisableFallback(params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSInfosDestroy(infos: cusolverDnIRSInfos_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSInfosDestroy(infos) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSInfosCreate() -> Result<cusolverDnIRSInfos_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverDnIRSInfos_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnIRSInfosCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverDnIRSInfos_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSInfosGetNiters(infos: cusolverDnIRSInfos_t) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnIRSInfosGetNiters(infos, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolver_int_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSInfosGetOuterNiters(infos: cusolverDnIRSInfos_t) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnIRSInfosGetOuterNiters(infos, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolver_int_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSInfosRequestResidual(infos: cusolverDnIRSInfos_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSInfosRequestResidual(infos) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSInfosGetResidualHistory<T0: types::CudaAsMutPtr>(infos: cusolverDnIRSInfos_t, mut residual_history: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSInfosGetResidualHistory(infos, residual_history.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSInfosGetMaxIters(infos: cusolverDnIRSInfos_t) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnIRSInfosGetMaxIters(infos, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolver_int_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnZZgesv<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZZgesv(
            handle,
            n,
            nrhs,
            dA.as_const_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_const_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZCgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZCgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZKgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZKgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZEgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZEgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZYgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZYgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCCgesv<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCCgesv(
            handle,
            n,
            nrhs,
            dA.as_const_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_const_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCEgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCEgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCKgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCKgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCYgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCYgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDDgesv<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDDgesv(
            handle,
            n,
            nrhs,
            dA.as_const_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_const_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDSgesv<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDSgesv(
            handle,
            n,
            nrhs,
            dA.as_const_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_const_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDHgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDHgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDBgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDBgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDXgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDXgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSSgesv<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSSgesv(
            handle,
            n,
            nrhs,
            dA.as_const_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_const_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSHgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSHgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSBgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSBgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSXgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    lwork_bytes: usize,
    mut iter: T5,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSXgesv(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZZgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZZgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZCgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZCgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZKgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZKgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZEgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZEgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZYgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZYgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCCgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCCgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCKgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCKgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCEgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCEgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCYgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCYgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDDgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDDgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDSgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDSgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDHgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDHgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDBgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDBgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDXgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDXgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSSgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSSgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSHgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSHgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSBgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSBgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSXgesv_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dipiv: T1,
    mut dB: T2,
    lddb: cusolver_int_t,
    mut dX: T3,
    lddx: cusolver_int_t,
    mut dWorkspace: T4,
    mut lwork_bytes: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSXgesv_bufferSize(
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZZgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZZgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZCgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZCgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZKgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZKgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZEgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZEgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZYgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZYgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCCgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCCgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCKgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCKgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCEgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCEgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCYgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCYgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDDgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDDgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDSgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDSgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDHgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDHgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDBgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDBgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDXgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDXgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSSgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSSgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSHgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSHgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSBgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSBgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSXgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut iter: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSXgels(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZZgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZZgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZCgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZCgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZKgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZKgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZEgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZEgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZYgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZYgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCCgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCCgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCKgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCKgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCEgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCEgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCYgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCYgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDDgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDDgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDSgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDSgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDHgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDHgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDBgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDBgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDXgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDXgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSSgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSSgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSHgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSHgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSBgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSBgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSXgels_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    mut lwork_bytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSXgels_bufferSize(
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSXgesv<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    gesv_irs_params: cusolverDnIRSParams_t,
    gesv_irs_infos: cusolverDnIRSInfos_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut niters: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnIRSXgesv(
            handle,
            gesv_irs_params,
            gesv_irs_infos,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            niters.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSXgesv_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, params: cusolverDnIRSParams_t, n: cusolver_int_t, nrhs: cusolver_int_t, mut lwork_bytes: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSXgesv_bufferSize(handle, params, n, nrhs, lwork_bytes.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSXgels<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    gels_irs_params: cusolverDnIRSParams_t,
    gels_irs_infos: cusolverDnIRSInfos_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut dA: T0,
    ldda: cusolver_int_t,
    mut dB: T1,
    lddb: cusolver_int_t,
    mut dX: T2,
    lddx: cusolver_int_t,
    mut dWorkspace: T3,
    lwork_bytes: usize,
    mut niters: T4,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnIRSXgels(
            handle,
            gels_irs_params,
            gels_irs_infos,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            niters.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnIRSXgels_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, params: cusolverDnIRSParams_t, m: cusolver_int_t, n: cusolver_int_t, nrhs: cusolver_int_t, mut lwork_bytes: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSXgels_bufferSize(handle, params, m, n, nrhs, lwork_bytes.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSpotrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut Lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSpotrf_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDpotrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut Lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDpotrf_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCpotrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut Lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCpotrf_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZpotrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut Lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZpotrf_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSpotrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut Workspace: T1, Lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSpotrf(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, Workspace.as_mut_ptr() as *mut _, Lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDpotrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut Workspace: T1, Lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDpotrf(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, Workspace.as_mut_ptr() as *mut _, Lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCpotrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut Workspace: T1, Lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCpotrf(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, Workspace.as_mut_ptr() as *mut _, Lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZpotrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut Workspace: T1, Lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZpotrf(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, Workspace.as_mut_ptr() as *mut _, Lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSpotrs<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, nrhs: i32, A: T0, lda: i32, mut B: T1, ldb: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSpotrs(handle, uplo, n as _, nrhs as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDpotrs<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, nrhs: i32, A: T0, lda: i32, mut B: T1, ldb: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDpotrs(handle, uplo, n as _, nrhs as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCpotrs<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, nrhs: i32, A: T0, lda: i32, mut B: T1, ldb: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCpotrs(handle, uplo, n as _, nrhs as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZpotrs<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, nrhs: i32, A: T0, lda: i32, mut B: T1, ldb: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZpotrs(handle, uplo, n as _, nrhs as _, A.as_const_ptr() as *const _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSpotrfBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut Aarray: T0, lda: i32, mut infoArray: T1, batchSize: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSpotrfBatched(handle, uplo, n as _, Aarray.as_mut_ptr() as *mut _, lda as _, infoArray.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDpotrfBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut Aarray: T0, lda: i32, mut infoArray: T1, batchSize: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDpotrfBatched(handle, uplo, n as _, Aarray.as_mut_ptr() as *mut _, lda as _, infoArray.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCpotrfBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut Aarray: T0, lda: i32, mut infoArray: T1, batchSize: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCpotrfBatched(handle, uplo, n as _, Aarray.as_mut_ptr() as *mut _, lda as _, infoArray.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZpotrfBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut Aarray: T0, lda: i32, mut infoArray: T1, batchSize: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZpotrfBatched(handle, uplo, n as _, Aarray.as_mut_ptr() as *mut _, lda as _, infoArray.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSpotrsBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut d_info: T2,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSpotrsBatched(handle, uplo, n as _, nrhs as _, A.as_mut_ptr() as *mut _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, d_info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDpotrsBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut d_info: T2,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDpotrsBatched(handle, uplo, n as _, nrhs as _, A.as_mut_ptr() as *mut _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, d_info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCpotrsBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut d_info: T2,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCpotrsBatched(handle, uplo, n as _, nrhs as _, A.as_mut_ptr() as *mut _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, d_info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZpotrsBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut d_info: T2,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZpotrsBatched(handle, uplo, n as _, nrhs as _, A.as_mut_ptr() as *mut _, lda as _, B.as_mut_ptr() as *mut _, ldb as _, d_info.as_mut_ptr() as *mut _, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSpotri_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSpotri_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDpotri_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDpotri_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCpotri_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCpotri_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZpotri_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZpotri_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSpotri<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut work: T1, lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSpotri(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, work.as_mut_ptr() as *mut _, lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDpotri<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut work: T1, lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDpotri(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, work.as_mut_ptr() as *mut _, lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCpotri<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut work: T1, lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCpotri(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, work.as_mut_ptr() as *mut _, lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZpotri<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut work: T1, lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZpotri(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, work.as_mut_ptr() as *mut _, lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXtrtri_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    diag: cublasDiagType_t,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    mut workspaceInBytesOnDevice: T1,
    mut workspaceInBytesOnHost: T2,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXtrtri_bufferSize(handle, uplo, diag, n, dataTypeA, A.as_mut_ptr() as *mut _, lda, workspaceInBytesOnDevice.as_mut_ptr() as *mut _, workspaceInBytesOnHost.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXtrtri<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    diag: cublasDiagType_t,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    mut bufferOnDevice: T1,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T2,
    workspaceInBytesOnHost: usize,
    mut devInfo: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXtrtri(
            handle,
            uplo,
            diag,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSlauum_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSlauum_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDlauum_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDlauum_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnClauum_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnClauum_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZlauum_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZlauum_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSlauum<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut work: T1, lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSlauum(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, work.as_mut_ptr() as *mut _, lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDlauum<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut work: T1, lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDlauum(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, work.as_mut_ptr() as *mut _, lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnClauum<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut work: T1, lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnClauum(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, work.as_mut_ptr() as *mut _, lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZlauum<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, mut work: T1, lwork: i32, mut devInfo: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZlauum(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, work.as_mut_ptr() as *mut _, lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgetrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut Lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSgetrf_bufferSize(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgetrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut Lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDgetrf_bufferSize(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgetrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut Lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCgetrf_bufferSize(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgetrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut Lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZgetrf_bufferSize(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgetrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut Workspace: T1, mut devIpiv: T2, mut devInfo: T3) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSgetrf(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, Workspace.as_mut_ptr() as *mut _, devIpiv.as_mut_ptr() as *mut _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgetrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut Workspace: T1, mut devIpiv: T2, mut devInfo: T3) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDgetrf(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, Workspace.as_mut_ptr() as *mut _, devIpiv.as_mut_ptr() as *mut _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgetrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut Workspace: T1, mut devIpiv: T2, mut devInfo: T3) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCgetrf(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, Workspace.as_mut_ptr() as *mut _, devIpiv.as_mut_ptr() as *mut _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgetrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut Workspace: T1, mut devIpiv: T2, mut devInfo: T3) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZgetrf(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, Workspace.as_mut_ptr() as *mut _, devIpiv.as_mut_ptr() as *mut _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSlaswp<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr>(handle: cusolverDnHandle_t, n: i32, mut A: T0, lda: i32, k1: i32, k2: i32, devIpiv: T1, incx: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSlaswp(handle, n as _, A.as_mut_ptr() as *mut _, lda as _, k1 as _, k2 as _, devIpiv.as_const_ptr() as *const _, incx as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDlaswp<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr>(handle: cusolverDnHandle_t, n: i32, mut A: T0, lda: i32, k1: i32, k2: i32, devIpiv: T1, incx: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDlaswp(handle, n as _, A.as_mut_ptr() as *mut _, lda as _, k1 as _, k2 as _, devIpiv.as_const_ptr() as *const _, incx as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnClaswp<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr>(handle: cusolverDnHandle_t, n: i32, mut A: T0, lda: i32, k1: i32, k2: i32, devIpiv: T1, incx: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnClaswp(handle, n as _, A.as_mut_ptr() as *mut _, lda as _, k1 as _, k2 as _, devIpiv.as_const_ptr() as *const _, incx as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZlaswp<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr>(handle: cusolverDnHandle_t, n: i32, mut A: T0, lda: i32, k1: i32, k2: i32, devIpiv: T1, incx: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZlaswp(handle, n as _, A.as_mut_ptr() as *mut _, lda as _, k1 as _, k2 as _, devIpiv.as_const_ptr() as *const _, incx as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgetrs<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    A: T0,
    lda: i32,
    devIpiv: T1,
    mut B: T2,
    ldb: i32,
    mut devInfo: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSgetrs(handle, trans, n as _, nrhs as _, A.as_const_ptr() as *const _, lda as _, devIpiv.as_const_ptr() as *const _, B.as_mut_ptr() as *mut _, ldb as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgetrs<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    A: T0,
    lda: i32,
    devIpiv: T1,
    mut B: T2,
    ldb: i32,
    mut devInfo: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDgetrs(handle, trans, n as _, nrhs as _, A.as_const_ptr() as *const _, lda as _, devIpiv.as_const_ptr() as *const _, B.as_mut_ptr() as *mut _, ldb as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgetrs<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    A: T0,
    lda: i32,
    devIpiv: T1,
    mut B: T2,
    ldb: i32,
    mut devInfo: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCgetrs(handle, trans, n as _, nrhs as _, A.as_const_ptr() as *const _, lda as _, devIpiv.as_const_ptr() as *const _, B.as_mut_ptr() as *mut _, ldb as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgetrs<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    A: T0,
    lda: i32,
    devIpiv: T1,
    mut B: T2,
    ldb: i32,
    mut devInfo: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZgetrs(handle, trans, n as _, nrhs as _, A.as_const_ptr() as *const _, lda as _, devIpiv.as_const_ptr() as *const _, B.as_mut_ptr() as *mut _, ldb as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgeqrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSgeqrf_bufferSize(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgeqrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDgeqrf_bufferSize(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgeqrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCgeqrf_bufferSize(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgeqrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZgeqrf_bufferSize(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgeqrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut TAU: T1,
    mut Workspace: T2,
    Lwork: i32,
    mut devInfo: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSgeqrf(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, TAU.as_mut_ptr() as *mut _, Workspace.as_mut_ptr() as *mut _, Lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgeqrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut TAU: T1,
    mut Workspace: T2,
    Lwork: i32,
    mut devInfo: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDgeqrf(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, TAU.as_mut_ptr() as *mut _, Workspace.as_mut_ptr() as *mut _, Lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgeqrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut TAU: T1,
    mut Workspace: T2,
    Lwork: i32,
    mut devInfo: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCgeqrf(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, TAU.as_mut_ptr() as *mut _, Workspace.as_mut_ptr() as *mut _, Lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgeqrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut TAU: T1,
    mut Workspace: T2,
    Lwork: i32,
    mut devInfo: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZgeqrf(handle, m as _, n as _, A.as_mut_ptr() as *mut _, lda as _, TAU.as_mut_ptr() as *mut _, Workspace.as_mut_ptr() as *mut _, Lwork as _, devInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSorgqr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, k: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSorgqr_bufferSize(handle, m as _, n as _, k as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDorgqr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, k: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDorgqr_bufferSize(handle, m as _, n as _, k as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCungqr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, k: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCungqr_bufferSize(handle, m as _, n as _, k as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZungqr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, k: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZungqr_bufferSize(handle, m as _, n as _, k as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSorgqr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, k: i32, mut A: T0, lda: i32, tau: T1, mut work: T2, lwork: i32, mut info: T3) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSorgqr(handle, m as _, n as _, k as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDorgqr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, k: i32, mut A: T0, lda: i32, tau: T1, mut work: T2, lwork: i32, mut info: T3) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDorgqr(handle, m as _, n as _, k as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCungqr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, k: i32, mut A: T0, lda: i32, tau: T1, mut work: T2, lwork: i32, mut info: T3) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCungqr(handle, m as _, n as _, k as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZungqr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, k: i32, mut A: T0, lda: i32, tau: T1, mut work: T2, lwork: i32, mut info: T3) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZungqr(handle, m as _, n as _, k as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSormqr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T0,
    lda: i32,
    tau: T1,
    C: T2,
    ldc: i32,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSormqr_bufferSize(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDormqr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T0,
    lda: i32,
    tau: T1,
    C: T2,
    ldc: i32,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDormqr_bufferSize(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCunmqr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T0,
    lda: i32,
    tau: T1,
    C: T2,
    ldc: i32,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCunmqr_bufferSize(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZunmqr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T0,
    lda: i32,
    tau: T1,
    C: T2,
    ldc: i32,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZunmqr_bufferSize(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSormqr<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T0,
    lda: i32,
    tau: T1,
    mut C: T2,
    ldc: i32,
    mut work: T3,
    lwork: i32,
    mut devInfo: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSormqr(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDormqr<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T0,
    lda: i32,
    tau: T1,
    mut C: T2,
    ldc: i32,
    mut work: T3,
    lwork: i32,
    mut devInfo: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDormqr(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCunmqr<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T0,
    lda: i32,
    tau: T1,
    mut C: T2,
    ldc: i32,
    mut work: T3,
    lwork: i32,
    mut devInfo: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCunmqr(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZunmqr<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T0,
    lda: i32,
    tau: T1,
    mut C: T2,
    ldc: i32,
    mut work: T3,
    lwork: i32,
    mut devInfo: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZunmqr(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsytrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsytrf_bufferSize(handle, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsytrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsytrf_bufferSize(handle, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCsytrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCsytrf_bufferSize(handle, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZsytrf_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, n: i32, mut A: T0, lda: i32, mut lwork: T1) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZsytrf_bufferSize(handle, n as _, A.as_mut_ptr() as *mut _, lda as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsytrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut ipiv: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsytrf(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsytrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut ipiv: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsytrf(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCsytrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut ipiv: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCsytrf(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZsytrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut ipiv: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZsytrf(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsytrs_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    ipiv: T1,
    dataTypeB: cudaDataType,
    mut B: T2,
    ldb: i64,
    mut workspaceInBytesOnDevice: T3,
    mut workspaceInBytesOnHost: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsytrs_bufferSize(
            handle,
            uplo,
            n,
            nrhs,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            ipiv.as_const_ptr() as *const _,
            dataTypeB,
            B.as_mut_ptr() as *mut _,
            ldb,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsytrs<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    ipiv: T1,
    dataTypeB: cudaDataType,
    mut B: T2,
    ldb: i64,
    mut bufferOnDevice: T3,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T4,
    workspaceInBytesOnHost: usize,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsytrs(
            handle,
            uplo,
            n,
            nrhs,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            ipiv.as_const_ptr() as *const _,
            dataTypeB,
            B.as_mut_ptr() as *mut _,
            ldb,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsytri_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, ipiv: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsytri_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsytri_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, ipiv: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsytri_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCsytri_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, ipiv: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCsytri_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZsytri_bufferSize<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, mut A: T0, lda: i32, ipiv: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZsytri_bufferSize(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsytri<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    ipiv: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsytri(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsytri<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    ipiv: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsytri(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCsytri<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    ipiv: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCsytri(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZsytri<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    ipiv: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZsytri(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, ipiv.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgebrd_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut Lwork: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSgebrd_bufferSize(handle, m as _, n as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgebrd_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut Lwork: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDgebrd_bufferSize(handle, m as _, n as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgebrd_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut Lwork: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCgebrd_bufferSize(handle, m as _, n as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgebrd_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut Lwork: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZgebrd_bufferSize(handle, m as _, n as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgebrd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut D: T1,
    mut E: T2,
    mut TAUQ: T3,
    mut TAUP: T4,
    mut Work: T5,
    Lwork: i32,
    mut devInfo: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgebrd(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            TAUQ.as_mut_ptr() as *mut _,
            TAUP.as_mut_ptr() as *mut _,
            Work.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgebrd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut D: T1,
    mut E: T2,
    mut TAUQ: T3,
    mut TAUP: T4,
    mut Work: T5,
    Lwork: i32,
    mut devInfo: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgebrd(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            TAUQ.as_mut_ptr() as *mut _,
            TAUP.as_mut_ptr() as *mut _,
            Work.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgebrd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut D: T1,
    mut E: T2,
    mut TAUQ: T3,
    mut TAUP: T4,
    mut Work: T5,
    Lwork: i32,
    mut devInfo: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgebrd(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            TAUQ.as_mut_ptr() as *mut _,
            TAUP.as_mut_ptr() as *mut _,
            Work.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgebrd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut D: T1,
    mut E: T2,
    mut TAUQ: T3,
    mut TAUP: T4,
    mut Work: T5,
    Lwork: i32,
    mut devInfo: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgebrd(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            TAUQ.as_mut_ptr() as *mut _,
            TAUP.as_mut_ptr() as *mut _,
            Work.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSorgbr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: i32, n: i32, k: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSorgbr_bufferSize(handle, side, m as _, n as _, k as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDorgbr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: i32, n: i32, k: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDorgbr_bufferSize(handle, side, m as _, n as _, k as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCungbr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: i32, n: i32, k: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCungbr_bufferSize(handle, side, m as _, n as _, k as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZungbr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: i32, n: i32, k: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZungbr_bufferSize(handle, side, m as _, n as _, k as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSorgbr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T0,
    lda: i32,
    tau: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSorgbr(handle, side, m as _, n as _, k as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDorgbr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T0,
    lda: i32,
    tau: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDorgbr(handle, side, m as _, n as _, k as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCungbr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T0,
    lda: i32,
    tau: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCungbr(handle, side, m as _, n as _, k as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZungbr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T0,
    lda: i32,
    tau: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZungbr(handle, side, m as _, n as _, k as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsytrd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    d: T1,
    e: T2,
    tau: T3,
    mut lwork: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsytrd_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            d.as_const_ptr() as *const _,
            e.as_const_ptr() as *const _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsytrd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    d: T1,
    e: T2,
    tau: T3,
    mut lwork: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsytrd_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            d.as_const_ptr() as *const _,
            e.as_const_ptr() as *const _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnChetrd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    d: T1,
    e: T2,
    tau: T3,
    mut lwork: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChetrd_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            d.as_const_ptr() as *const _,
            e.as_const_ptr() as *const _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZhetrd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    d: T1,
    e: T2,
    tau: T3,
    mut lwork: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhetrd_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            d.as_const_ptr() as *const _,
            e.as_const_ptr() as *const _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsytrd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut d: T1,
    mut e: T2,
    mut tau: T3,
    mut work: T4,
    lwork: i32,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsytrd(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            d.as_mut_ptr() as *mut _,
            e.as_mut_ptr() as *mut _,
            tau.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsytrd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut d: T1,
    mut e: T2,
    mut tau: T3,
    mut work: T4,
    lwork: i32,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsytrd(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            d.as_mut_ptr() as *mut _,
            e.as_mut_ptr() as *mut _,
            tau.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnChetrd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut d: T1,
    mut e: T2,
    mut tau: T3,
    mut work: T4,
    lwork: i32,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChetrd(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            d.as_mut_ptr() as *mut _,
            e.as_mut_ptr() as *mut _,
            tau.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZhetrd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut d: T1,
    mut e: T2,
    mut tau: T3,
    mut work: T4,
    lwork: i32,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhetrd(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            d.as_mut_ptr() as *mut _,
            e.as_mut_ptr() as *mut _,
            tau.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSorgtr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSorgtr_bufferSize(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDorgtr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDorgtr_bufferSize(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCungtr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCungtr_bufferSize(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZungtr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, tau: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZungtr_bufferSize(handle, uplo, n as _, A.as_const_ptr() as *const _, lda as _, tau.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSorgtr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    tau: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSorgtr(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDorgtr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    tau: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDorgtr(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCungtr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    tau: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCungtr(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZungtr<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    tau: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZungtr(handle, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, tau.as_const_ptr() as *const _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSormtr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    tau: T1,
    C: T2,
    ldc: i32,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSormtr_bufferSize(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDormtr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    tau: T1,
    C: T2,
    ldc: i32,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDormtr_bufferSize(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCunmtr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    tau: T1,
    C: T2,
    ldc: i32,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCunmtr_bufferSize(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZunmtr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    tau: T1,
    C: T2,
    ldc: i32,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZunmtr_bufferSize(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSormtr<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut tau: T1,
    mut C: T2,
    ldc: i32,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSormtr(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_mut_ptr() as *mut _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDormtr<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut tau: T1,
    mut C: T2,
    ldc: i32,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDormtr(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_mut_ptr() as *mut _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCunmtr<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut tau: T1,
    mut C: T2,
    ldc: i32,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCunmtr(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_mut_ptr() as *mut _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZunmtr<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut tau: T1,
    mut C: T2,
    ldc: i32,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZunmtr(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_mut_ptr() as *mut _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgesvd_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut lwork: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSgesvd_bufferSize(handle, m as _, n as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgesvd_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut lwork: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDgesvd_bufferSize(handle, m as _, n as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgesvd_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut lwork: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCgesvd_bufferSize(handle, m as _, n as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgesvd_bufferSize<T0: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, m: i32, n: i32, mut lwork: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZgesvd_bufferSize(handle, m as _, n as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgesvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut VT: T3,
    ldvt: i32,
    mut work: T4,
    lwork: i32,
    mut rwork: T5,
    mut info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvd(
            handle,
            jobu,
            jobvt,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            VT.as_mut_ptr() as *mut _,
            ldvt as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            rwork.as_mut_ptr() as *mut _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgesvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut VT: T3,
    ldvt: i32,
    mut work: T4,
    lwork: i32,
    mut rwork: T5,
    mut info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvd(
            handle,
            jobu,
            jobvt,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            VT.as_mut_ptr() as *mut _,
            ldvt as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            rwork.as_mut_ptr() as *mut _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgesvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut VT: T3,
    ldvt: i32,
    mut work: T4,
    lwork: i32,
    mut rwork: T5,
    mut info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvd(
            handle,
            jobu,
            jobvt,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            VT.as_mut_ptr() as *mut _,
            ldvt as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            rwork.as_mut_ptr() as *mut _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgesvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut VT: T3,
    ldvt: i32,
    mut work: T4,
    lwork: i32,
    mut rwork: T5,
    mut info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvd(
            handle,
            jobu,
            jobvt,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            VT.as_mut_ptr() as *mut _,
            ldvt as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            rwork.as_mut_ptr() as *mut _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsyevd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, W: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsyevd_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsyevd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, W: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsyevd_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCheevd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, W: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCheevd_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZheevd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: i32, A: T0, lda: i32, W: T1, mut lwork: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZheevd_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsyevd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsyevd(handle, jobz, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, W.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsyevd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsyevd(handle, jobz, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, W.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCheevd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCheevd(handle, jobz, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, W.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZheevd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZheevd(handle, jobz, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, W.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsyevdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: T1,
    W: T2,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevdx_bufferSize(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsyevdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: T1,
    W: T2,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevdx_bufferSize(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCheevdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: T1,
    W: T2,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevdx_bufferSize(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZheevdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: T1,
    W: T2,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevdx_bufferSize(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsyevdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: T1,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevdx(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsyevdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: T1,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevdx(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCheevdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: T1,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevdx(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZheevdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: T1,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevdx(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsygvdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: T2,
    W: T3,
    mut lwork: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvdx_bufferSize(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsygvdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: T2,
    W: T3,
    mut lwork: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvdx_bufferSize(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnChegvdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: T2,
    W: T3,
    mut lwork: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvdx_bufferSize(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZhegvdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: T2,
    W: T3,
    mut lwork: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvdx_bufferSize(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsygvdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: T2,
    mut W: T3,
    mut work: T4,
    lwork: i32,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvdx(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsygvdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: T2,
    mut W: T3,
    mut work: T4,
    lwork: i32,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvdx(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnChegvdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: T2,
    mut W: T3,
    mut work: T4,
    lwork: i32,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvdx(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZhegvdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: T2,
    mut W: T3,
    mut work: T4,
    lwork: i32,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvdx(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsygvd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    W: T2,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsygvd_bufferSize(handle, itype, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsygvd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    W: T2,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsygvd_bufferSize(handle, itype, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnChegvd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    W: T2,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnChegvd_bufferSize(handle, itype, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZhegvd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    W: T2,
    mut lwork: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZhegvd_bufferSize(handle, itype, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, B.as_const_ptr() as *const _, ldb as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsygvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvd(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsygvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvd(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnChegvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvd(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZhegvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvd(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsygvd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: T0,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: T1,
    ldb: i64,
    dataTypeW: cudaDataType,
    d_W: T2,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T3,
    mut workspaceInBytesOnHost: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsygvd_bufferSize(
            handle,
            params,
            itype,
            jobz,
            uplo,
            n,
            dataTypeA,
            d_A.as_const_ptr() as *const _,
            lda,
            dataTypeB,
            d_B.as_const_ptr() as *const _,
            ldb,
            dataTypeW,
            d_W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsygvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    mut d_A: T0,
    lda: i64,
    dataTypeB: cudaDataType,
    mut d_B: T1,
    ldb: i64,
    dataTypeW: cudaDataType,
    mut d_W: T2,
    computeType: cudaDataType,
    mut bufferOnDevice: T3,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T4,
    workspaceInBytesOnHost: usize,
    mut d_info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsygvd(
            handle,
            params,
            itype,
            jobz,
            uplo,
            n,
            dataTypeA,
            d_A.as_mut_ptr() as *mut _,
            lda,
            dataTypeB,
            d_B.as_mut_ptr() as *mut _,
            ldb,
            dataTypeW,
            d_W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsygvdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: T0,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: T1,
    ldb: i64,
    mut vl: T2,
    mut vu: T3,
    il: i64,
    iu: i64,
    mut meig: T4,
    dataTypeW: cudaDataType,
    d_W: T5,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T6,
    mut workspaceInBytesOnHost: T7,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsygvdx_bufferSize(
            handle,
            params,
            itype,
            jobz,
            uplo,
            n,
            dataTypeA,
            d_A.as_const_ptr() as *const _,
            lda,
            dataTypeB,
            d_B.as_const_ptr() as *const _,
            ldb,
            vl.as_mut_ptr() as *mut _,
            vu.as_mut_ptr() as *mut _,
            il,
            iu,
            meig.as_mut_ptr() as *mut _,
            dataTypeW,
            d_W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsygvdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr, T8: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    mut d_A: T0,
    lda: i64,
    dataTypeB: cudaDataType,
    mut d_B: T1,
    ldb: i64,
    mut vl: T2,
    mut vu: T3,
    il: i64,
    iu: i64,
    mut meig: T4,
    dataTypeW: cudaDataType,
    mut d_W: T5,
    computeType: cudaDataType,
    mut bufferOnDevice: T6,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T7,
    workspaceInBytesOnHost: usize,
    mut d_info: T8,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsygvdx(
            handle,
            params,
            itype,
            jobz,
            range,
            uplo,
            n,
            dataTypeA,
            d_A.as_mut_ptr() as *mut _,
            lda,
            dataTypeB,
            d_B.as_mut_ptr() as *mut _,
            ldb,
            vl.as_mut_ptr() as *mut _,
            vu.as_mut_ptr() as *mut _,
            il,
            iu,
            meig.as_mut_ptr() as *mut _,
            dataTypeW,
            d_W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCreateSyevjInfo() -> Result<syevjInfo_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<syevjInfo_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnCreateSyevjInfo(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as syevjInfo_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnDestroySyevjInfo(info: syevjInfo_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDestroySyevjInfo(info) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevjSetTolerance(info: syevjInfo_t, tolerance: f64) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXsyevjSetTolerance(info, tolerance) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevjSetMaxSweeps(info: syevjInfo_t, max_sweeps: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXsyevjSetMaxSweeps(info, max_sweeps as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevjSetSortEig(info: syevjInfo_t, sort_eig: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXsyevjSetSortEig(info, sort_eig as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevjGetResidual(handle: cusolverDnHandle_t, info: syevjInfo_t) -> Result<f64, crate::sys::cusolverStatus_t> {
    let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnXsyevjGetResidual(handle, info, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as f64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnXsyevjGetSweeps(handle: cusolverDnHandle_t, info: syevjInfo_t) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnXsyevjGetSweeps(handle, info, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSsyevjBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    W: T1,
    mut lwork: T2,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsyevjBatched_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _, params, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsyevjBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    W: T1,
    mut lwork: T2,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsyevjBatched_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _, params, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCheevjBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    W: T1,
    mut lwork: T2,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCheevjBatched_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _, params, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZheevjBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    W: T1,
    mut lwork: T2,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZheevjBatched_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _, params, batchSize as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsyevjBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevjBatched(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsyevjBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevjBatched(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCheevjBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevjBatched(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZheevjBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevjBatched(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsyevj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    W: T1,
    mut lwork: T2,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsyevj_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _, params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsyevj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    W: T1,
    mut lwork: T2,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsyevj_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _, params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCheevj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    W: T1,
    mut lwork: T2,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCheevj_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _, params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZheevj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    W: T1,
    mut lwork: T2,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZheevj_bufferSize(handle, jobz, uplo, n as _, A.as_const_ptr() as *const _, lda as _, W.as_const_ptr() as *const _, lwork.as_mut_ptr() as *mut _, params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsyevj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSsyevj(handle, jobz, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, W.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _, params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsyevj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDsyevj(handle, jobz, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, W.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _, params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCheevj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnCheevj(handle, jobz, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, W.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _, params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZheevj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut W: T1,
    mut work: T2,
    lwork: i32,
    mut info: T3,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnZheevj(handle, jobz, uplo, n as _, A.as_mut_ptr() as *mut _, lda as _, W.as_mut_ptr() as *mut _, work.as_mut_ptr() as *mut _, lwork as _, info.as_mut_ptr() as *mut _, params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsygvj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    W: T2,
    mut lwork: T3,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvj_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsygvj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    W: T2,
    mut lwork: T3,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvj_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnChegvj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    W: T2,
    mut lwork: T3,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvj_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZhegvj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T0,
    lda: i32,
    B: T1,
    ldb: i32,
    W: T2,
    mut lwork: T3,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvj_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSsygvj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvj(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDsygvj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvj(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnChegvj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvj(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZhegvj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T0,
    lda: i32,
    mut B: T1,
    ldb: i32,
    mut W: T2,
    mut work: T3,
    lwork: i32,
    mut info: T4,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvj(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCreateGesvdjInfo() -> Result<gesvdjInfo_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<gesvdjInfo_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnCreateGesvdjInfo(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as gesvdjInfo_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnDestroyGesvdjInfo(info: gesvdjInfo_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDestroyGesvdjInfo(info) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvdjSetTolerance(info: gesvdjInfo_t, tolerance: f64) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgesvdjSetTolerance(info, tolerance) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvdjSetMaxSweeps(info: gesvdjInfo_t, max_sweeps: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgesvdjSetMaxSweeps(info, max_sweeps as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvdjSetSortEig(info: gesvdjInfo_t, sort_svd: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgesvdjSetSortEig(info, sort_svd as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvdjGetResidual(handle: cusolverDnHandle_t, info: gesvdjInfo_t) -> Result<f64, crate::sys::cusolverStatus_t> {
    let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnXgesvdjGetResidual(handle, info, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as f64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnXgesvdjGetSweeps(handle: cusolverDnHandle_t, info: gesvdjInfo_t) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnXgesvdjGetSweeps(handle, info, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSgesvdjBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    S: T1,
    U: T2,
    ldu: i32,
    V: T3,
    ldv: i32,
    mut lwork: T4,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdjBatched_bufferSize(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgesvdjBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    S: T1,
    U: T2,
    ldu: i32,
    V: T3,
    ldv: i32,
    mut lwork: T4,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdjBatched_bufferSize(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgesvdjBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    S: T1,
    U: T2,
    ldu: i32,
    V: T3,
    ldv: i32,
    mut lwork: T4,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdjBatched_bufferSize(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgesvdjBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    S: T1,
    U: T2,
    ldu: i32,
    V: T3,
    ldv: i32,
    mut lwork: T4,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdjBatched_bufferSize(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgesvdjBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut V: T3,
    ldv: i32,
    mut work: T4,
    lwork: i32,
    mut info: T5,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdjBatched(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgesvdjBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut V: T3,
    ldv: i32,
    mut work: T4,
    lwork: i32,
    mut info: T5,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdjBatched(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgesvdjBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut V: T3,
    ldv: i32,
    mut work: T4,
    lwork: i32,
    mut info: T5,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdjBatched(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgesvdjBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut V: T3,
    ldv: i32,
    mut work: T4,
    lwork: i32,
    mut info: T5,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdjBatched(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgesvdj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    S: T1,
    U: T2,
    ldu: i32,
    V: T3,
    ldv: i32,
    mut lwork: T4,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdj_bufferSize(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgesvdj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    S: T1,
    U: T2,
    ldu: i32,
    V: T3,
    ldv: i32,
    mut lwork: T4,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdj_bufferSize(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgesvdj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    S: T1,
    U: T2,
    ldu: i32,
    V: T3,
    ldv: i32,
    mut lwork: T4,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdj_bufferSize(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgesvdj_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    A: T0,
    lda: i32,
    S: T1,
    U: T2,
    ldu: i32,
    V: T3,
    ldv: i32,
    mut lwork: T4,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdj_bufferSize(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgesvdj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut V: T3,
    ldv: i32,
    mut work: T4,
    lwork: i32,
    mut info: T5,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdj(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgesvdj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut V: T3,
    ldv: i32,
    mut work: T4,
    lwork: i32,
    mut info: T5,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdj(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgesvdj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut V: T3,
    ldv: i32,
    mut work: T4,
    lwork: i32,
    mut info: T5,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdj(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgesvdj<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    mut A: T0,
    lda: i32,
    mut S: T1,
    mut U: T2,
    ldu: i32,
    mut V: T3,
    ldv: i32,
    mut work: T4,
    lwork: i32,
    mut info: T5,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdj(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgesvdaStridedBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T0,
    lda: i32,
    strideA: i64,
    d_S: T1,
    strideS: i64,
    d_U: T2,
    ldu: i32,
    strideU: i64,
    d_V: T3,
    ldv: i32,
    strideV: i64,
    mut lwork: T4,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdaStridedBatched_bufferSize(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_const_ptr() as *const _,
            strideS as _,
            d_U.as_const_ptr() as *const _,
            ldu as _,
            strideU as _,
            d_V.as_const_ptr() as *const _,
            ldv as _,
            strideV as _,
            lwork.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgesvdaStridedBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T0,
    lda: i32,
    strideA: i64,
    d_S: T1,
    strideS: i64,
    d_U: T2,
    ldu: i32,
    strideU: i64,
    d_V: T3,
    ldv: i32,
    strideV: i64,
    mut lwork: T4,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdaStridedBatched_bufferSize(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_const_ptr() as *const _,
            strideS as _,
            d_U.as_const_ptr() as *const _,
            ldu as _,
            strideU as _,
            d_V.as_const_ptr() as *const _,
            ldv as _,
            strideV as _,
            lwork.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgesvdaStridedBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T0,
    lda: i32,
    strideA: i64,
    d_S: T1,
    strideS: i64,
    d_U: T2,
    ldu: i32,
    strideU: i64,
    d_V: T3,
    ldv: i32,
    strideV: i64,
    mut lwork: T4,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdaStridedBatched_bufferSize(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_const_ptr() as *const _,
            strideS as _,
            d_U.as_const_ptr() as *const _,
            ldu as _,
            strideU as _,
            d_V.as_const_ptr() as *const _,
            ldv as _,
            strideV as _,
            lwork.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgesvdaStridedBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T0,
    lda: i32,
    strideA: i64,
    d_S: T1,
    strideS: i64,
    d_U: T2,
    ldu: i32,
    strideU: i64,
    d_V: T3,
    ldv: i32,
    strideV: i64,
    mut lwork: T4,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdaStridedBatched_bufferSize(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_const_ptr() as *const _,
            strideS as _,
            d_U.as_const_ptr() as *const _,
            ldu as _,
            strideU as _,
            d_V.as_const_ptr() as *const _,
            ldv as _,
            strideV as _,
            lwork.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSgesvdaStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T0,
    lda: i32,
    strideA: i64,
    mut d_S: T1,
    strideS: i64,
    mut d_U: T2,
    ldu: i32,
    strideU: i64,
    mut d_V: T3,
    ldv: i32,
    strideV: i64,
    mut d_work: T4,
    lwork: i32,
    mut d_info: T5,
    mut h_R_nrmF: T6,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdaStridedBatched(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_mut_ptr() as *mut _,
            strideS as _,
            d_U.as_mut_ptr() as *mut _,
            ldu as _,
            strideU as _,
            d_V.as_mut_ptr() as *mut _,
            ldv as _,
            strideV as _,
            d_work.as_mut_ptr() as *mut _,
            lwork as _,
            d_info.as_mut_ptr() as *mut _,
            h_R_nrmF.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnDgesvdaStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T0,
    lda: i32,
    strideA: i64,
    mut d_S: T1,
    strideS: i64,
    mut d_U: T2,
    ldu: i32,
    strideU: i64,
    mut d_V: T3,
    ldv: i32,
    strideV: i64,
    mut d_work: T4,
    lwork: i32,
    mut d_info: T5,
    mut h_R_nrmF: T6,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdaStridedBatched(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_mut_ptr() as *mut _,
            strideS as _,
            d_U.as_mut_ptr() as *mut _,
            ldu as _,
            strideU as _,
            d_V.as_mut_ptr() as *mut _,
            ldv as _,
            strideV as _,
            d_work.as_mut_ptr() as *mut _,
            lwork as _,
            d_info.as_mut_ptr() as *mut _,
            h_R_nrmF.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCgesvdaStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T0,
    lda: i32,
    strideA: i64,
    mut d_S: T1,
    strideS: i64,
    mut d_U: T2,
    ldu: i32,
    strideU: i64,
    mut d_V: T3,
    ldv: i32,
    strideV: i64,
    mut d_work: T4,
    lwork: i32,
    mut d_info: T5,
    mut h_R_nrmF: T6,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdaStridedBatched(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_mut_ptr() as *mut _,
            strideS as _,
            d_U.as_mut_ptr() as *mut _,
            ldu as _,
            strideU as _,
            d_V.as_mut_ptr() as *mut _,
            ldv as _,
            strideV as _,
            d_work.as_mut_ptr() as *mut _,
            lwork as _,
            d_info.as_mut_ptr() as *mut _,
            h_R_nrmF.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnZgesvdaStridedBatched<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T0,
    lda: i32,
    strideA: i64,
    mut d_S: T1,
    strideS: i64,
    mut d_U: T2,
    ldu: i32,
    strideU: i64,
    mut d_V: T3,
    ldv: i32,
    strideV: i64,
    mut d_work: T4,
    lwork: i32,
    mut d_info: T5,
    mut h_R_nrmF: T6,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdaStridedBatched(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_mut_ptr() as *mut _,
            strideS as _,
            d_U.as_mut_ptr() as *mut _,
            ldu as _,
            strideU as _,
            d_V.as_mut_ptr() as *mut _,
            ldv as _,
            strideV as _,
            d_work.as_mut_ptr() as *mut _,
            lwork as _,
            d_info.as_mut_ptr() as *mut _,
            h_R_nrmF.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnCreateParams() -> Result<cusolverDnParams_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverDnParams_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverDnCreateParams(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverDnParams_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnDestroyParams(params: cusolverDnParams_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDestroyParams(params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnSetAdvOptions(params: cusolverDnParams_t, function: cusolverDnFunction_t, algo: cusolverAlgMode_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetAdvOptions(params, function, algo) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXpotrf_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T1,
    mut workspaceInBytesOnHost: T2,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXpotrf_bufferSize(handle, params, uplo, n, dataTypeA, A.as_const_ptr() as *const _, lda, computeType, workspaceInBytesOnDevice.as_mut_ptr() as *mut _, workspaceInBytesOnHost.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXpotrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    computeType: cudaDataType,
    mut bufferOnDevice: T1,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T2,
    workspaceInBytesOnHost: usize,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXpotrf(
            handle,
            params,
            uplo,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXpotrs<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    dataTypeB: cudaDataType,
    mut B: T1,
    ldb: i64,
    mut info: T2,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXpotrs(handle, params, uplo, n, nrhs, dataTypeA, A.as_const_ptr() as *const _, lda, dataTypeB, B.as_mut_ptr() as *mut _, ldb, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgeqrf_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    dataTypeTau: cudaDataType,
    tau: T1,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T2,
    mut workspaceInBytesOnHost: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgeqrf_bufferSize(
            handle,
            params,
            m,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeTau,
            tau.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgeqrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    dataTypeTau: cudaDataType,
    mut tau: T1,
    computeType: cudaDataType,
    mut bufferOnDevice: T2,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T3,
    workspaceInBytesOnHost: usize,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgeqrf(
            handle,
            params,
            m,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeTau,
            tau.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgetrf_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T1,
    mut workspaceInBytesOnHost: T2,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgetrf_bufferSize(handle, params, m, n, dataTypeA, A.as_const_ptr() as *const _, lda, computeType, workspaceInBytesOnDevice.as_mut_ptr() as *mut _, workspaceInBytesOnHost.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgetrf<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    mut ipiv: T1,
    computeType: cudaDataType,
    mut bufferOnDevice: T2,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T3,
    workspaceInBytesOnHost: usize,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgetrf(
            handle,
            params,
            m,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            ipiv.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgetrs<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    trans: cublasOperation_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    ipiv: T1,
    dataTypeB: cudaDataType,
    mut B: T2,
    ldb: i64,
    mut info: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgetrs(handle, params, trans, n, nrhs, dataTypeA, A.as_const_ptr() as *const _, lda, ipiv.as_const_ptr() as *const _, dataTypeB, B.as_mut_ptr() as *mut _, ldb, info.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    dataTypeW: cudaDataType,
    W: T1,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T2,
    mut workspaceInBytesOnHost: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsyevd_bufferSize(
            handle,
            params,
            jobz,
            uplo,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeW,
            W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    dataTypeW: cudaDataType,
    mut W: T1,
    computeType: cudaDataType,
    mut bufferOnDevice: T2,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T3,
    workspaceInBytesOnHost: usize,
    mut info: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsyevd(
            handle,
            params,
            jobz,
            uplo,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeW,
            W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXstedc_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    compz: cusolverEigComp_t,
    n: i64,
    dataTypeDE: cudaDataType,
    D: T0,
    E: T1,
    dataTypeZ: cudaDataType,
    Z: T2,
    ldz: i64,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T3,
    mut workspaceInBytesOnHost: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXstedc_bufferSize(
            handle,
            params,
            compz,
            n,
            dataTypeDE,
            D.as_const_ptr() as *const _,
            E.as_const_ptr() as *const _,
            dataTypeZ,
            Z.as_const_ptr() as *const _,
            ldz,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXstedc<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    compz: cusolverEigComp_t,
    n: i64,
    dataTypeDE: cudaDataType,
    mut D: T0,
    mut E: T1,
    dataTypeZ: cudaDataType,
    mut Z: T2,
    ldz: i64,
    computeType: cudaDataType,
    mut bufferOnDevice: T3,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T4,
    workspaceInBytesOnHost: usize,
    mut info: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXstedc(
            handle,
            params,
            compz,
            n,
            dataTypeDE,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            dataTypeZ,
            Z.as_mut_ptr() as *mut _,
            ldz,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevBatched_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    dataTypeW: cudaDataType,
    W: T1,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T2,
    mut workspaceInBytesOnHost: T3,
    batchSize: i64,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsyevBatched_bufferSize(
            handle,
            params,
            jobz,
            uplo,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeW,
            W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
            batchSize,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevBatched<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    dataTypeW: cudaDataType,
    mut W: T1,
    computeType: cudaDataType,
    mut bufferOnDevice: T2,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T3,
    workspaceInBytesOnHost: usize,
    mut info: T4,
    batchSize: i64,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsyevBatched(
            handle,
            params,
            jobz,
            uplo,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeW,
            W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
            batchSize,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevdx_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    mut vl: T1,
    mut vu: T2,
    il: i64,
    iu: i64,
    mut h_meig: T3,
    dataTypeW: cudaDataType,
    W: T4,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T5,
    mut workspaceInBytesOnHost: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsyevdx_bufferSize(
            handle,
            params,
            jobz,
            range,
            uplo,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            vl.as_mut_ptr() as *mut _,
            vu.as_mut_ptr() as *mut _,
            il,
            iu,
            h_meig.as_mut_ptr() as *mut _,
            dataTypeW,
            W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXsyevdx<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    mut vl: T1,
    mut vu: T2,
    il: i64,
    iu: i64,
    mut meig64: T3,
    dataTypeW: cudaDataType,
    mut W: T4,
    computeType: cudaDataType,
    mut bufferOnDevice: T5,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T6,
    workspaceInBytesOnHost: usize,
    mut info: T7,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXsyevdx(
            handle,
            params,
            jobz,
            range,
            uplo,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            vl.as_mut_ptr() as *mut _,
            vu.as_mut_ptr() as *mut _,
            il,
            iu,
            meig64.as_mut_ptr() as *mut _,
            dataTypeW,
            W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgeev_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobvl: cusolverEigMode_t,
    jobvr: cusolverEigMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    dataTypeW: cudaDataType,
    W: T1,
    dataTypeVL: cudaDataType,
    VL: T2,
    ldvl: i64,
    dataTypeVR: cudaDataType,
    VR: T3,
    ldvr: i64,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T4,
    mut workspaceInBytesOnHost: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgeev_bufferSize(
            handle,
            params,
            jobvl,
            jobvr,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeW,
            W.as_const_ptr() as *const _,
            dataTypeVL,
            VL.as_const_ptr() as *const _,
            ldvl,
            dataTypeVR,
            VR.as_const_ptr() as *const _,
            ldvr,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgeev<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobvl: cusolverEigMode_t,
    jobvr: cusolverEigMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    dataTypeW: cudaDataType,
    mut W: T1,
    dataTypeVL: cudaDataType,
    mut VL: T2,
    ldvl: i64,
    dataTypeVR: cudaDataType,
    mut VR: T3,
    ldvr: i64,
    computeType: cudaDataType,
    mut bufferOnDevice: T4,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T5,
    workspaceInBytesOnHost: usize,
    mut info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgeev(
            handle,
            params,
            jobvl,
            jobvr,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeW,
            W.as_mut_ptr() as *mut _,
            dataTypeVL,
            VL.as_mut_ptr() as *mut _,
            ldvl,
            dataTypeVR,
            VR.as_mut_ptr() as *mut _,
            ldvr,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvd_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    dataTypeS: cudaDataType,
    S: T1,
    dataTypeU: cudaDataType,
    U: T2,
    ldu: i64,
    dataTypeVT: cudaDataType,
    VT: T3,
    ldvt: i64,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T4,
    mut workspaceInBytesOnHost: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgesvd_bufferSize(
            handle,
            params,
            jobu,
            jobvt,
            m,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeS,
            S.as_const_ptr() as *const _,
            dataTypeU,
            U.as_const_ptr() as *const _,
            ldu,
            dataTypeVT,
            VT.as_const_ptr() as *const _,
            ldvt,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvd<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    dataTypeS: cudaDataType,
    mut S: T1,
    dataTypeU: cudaDataType,
    mut U: T2,
    ldu: i64,
    dataTypeVT: cudaDataType,
    mut VT: T3,
    ldvt: i64,
    computeType: cudaDataType,
    mut bufferOnDevice: T4,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T5,
    workspaceInBytesOnHost: usize,
    mut info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgesvd(
            handle,
            params,
            jobu,
            jobvt,
            m,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeS,
            S.as_mut_ptr() as *mut _,
            dataTypeU,
            U.as_mut_ptr() as *mut _,
            ldu,
            dataTypeVT,
            VT.as_mut_ptr() as *mut _,
            ldvt,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvdp_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    dataTypeS: cudaDataType,
    S: T1,
    dataTypeU: cudaDataType,
    U: T2,
    ldu: i64,
    dataTypeV: cudaDataType,
    V: T3,
    ldv: i64,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T4,
    mut workspaceInBytesOnHost: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgesvdp_bufferSize(
            handle,
            params,
            jobz,
            econ as _,
            m,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeS,
            S.as_const_ptr() as *const _,
            dataTypeU,
            U.as_const_ptr() as *const _,
            ldu,
            dataTypeV,
            V.as_const_ptr() as *const _,
            ldv,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvdp<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    dataTypeS: cudaDataType,
    mut S: T1,
    dataTypeU: cudaDataType,
    mut U: T2,
    ldu: i64,
    dataTypeV: cudaDataType,
    mut V: T3,
    ldv: i64,
    computeType: cudaDataType,
    mut bufferOnDevice: T4,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T5,
    workspaceInBytesOnHost: usize,
    mut d_info: T6,
    mut h_err_sigma: T7,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgesvdp(
            handle,
            params,
            jobz,
            econ as _,
            m,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeS,
            S.as_mut_ptr() as *mut _,
            dataTypeU,
            U.as_mut_ptr() as *mut _,
            ldu,
            dataTypeV,
            V.as_mut_ptr() as *mut _,
            ldv,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_info.as_mut_ptr() as *mut _,
            h_err_sigma.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvdr_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobv: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    k: i64,
    p: i64,
    niters: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    dataTypeSrand: cudaDataType,
    Srand: T1,
    dataTypeUrand: cudaDataType,
    Urand: T2,
    ldUrand: i64,
    dataTypeVrand: cudaDataType,
    Vrand: T3,
    ldVrand: i64,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T4,
    mut workspaceInBytesOnHost: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgesvdr_bufferSize(
            handle,
            params,
            jobu,
            jobv,
            m,
            n,
            k,
            p,
            niters,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeSrand,
            Srand.as_const_ptr() as *const _,
            dataTypeUrand,
            Urand.as_const_ptr() as *const _,
            ldUrand,
            dataTypeVrand,
            Vrand.as_const_ptr() as *const _,
            ldVrand,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXgesvdr<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobv: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    k: i64,
    p: i64,
    niters: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    dataTypeSrand: cudaDataType,
    mut Srand: T1,
    dataTypeUrand: cudaDataType,
    mut Urand: T2,
    ldUrand: i64,
    dataTypeVrand: cudaDataType,
    mut Vrand: T3,
    ldVrand: i64,
    computeType: cudaDataType,
    mut bufferOnDevice: T4,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T5,
    workspaceInBytesOnHost: usize,
    mut d_info: T6,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXgesvdr(
            handle,
            params,
            jobu,
            jobv,
            m,
            n,
            k,
            p,
            niters,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeSrand,
            Srand.as_mut_ptr() as *mut _,
            dataTypeUrand,
            Urand.as_mut_ptr() as *mut _,
            ldUrand,
            dataTypeVrand,
            Vrand.as_mut_ptr() as *mut _,
            ldVrand,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXlarft_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    direct: cusolverDirectMode_t,
    storev: cusolverStorevMode_t,
    n: i64,
    k: i64,
    dataTypeV: cudaDataType,
    V: T0,
    ldv: i64,
    dataTypeTau: cudaDataType,
    tau: T1,
    dataTypeT: cudaDataType,
    mut T: T2,
    ldt: i64,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T3,
    mut workspaceInBytesOnHost: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXlarft_bufferSize(
            handle,
            params,
            direct,
            storev,
            n,
            k,
            dataTypeV,
            V.as_const_ptr() as *const _,
            ldv,
            dataTypeTau,
            tau.as_const_ptr() as *const _,
            dataTypeT,
            T.as_mut_ptr() as *mut _,
            ldt,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXlarft<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    direct: cusolverDirectMode_t,
    storev: cusolverStorevMode_t,
    n: i64,
    k: i64,
    dataTypeV: cudaDataType,
    V: T0,
    ldv: i64,
    dataTypeTau: cudaDataType,
    tau: T1,
    dataTypeT: cudaDataType,
    mut T: T2,
    ldt: i64,
    computeType: cudaDataType,
    mut bufferOnDevice: T3,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T4,
    workspaceInBytesOnHost: usize,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXlarft(
            handle,
            params,
            direct,
            storev,
            n,
            k,
            dataTypeV,
            V.as_const_ptr() as *const _,
            ldv,
            dataTypeTau,
            tau.as_const_ptr() as *const _,
            dataTypeT,
            T.as_mut_ptr() as *mut _,
            ldt,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnLoggerSetCallback(callback: cusolverDnLoggerCallback_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetCallback(callback) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnLoggerSetFile<T0: types::CudaAsMutPtr>(mut file: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetFile(file.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnLoggerOpenFile<T0: types::CudaAsPtr>(logFile: T0) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerOpenFile(logFile.as_const_ptr() as *const _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnLoggerSetLevel(level: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetLevel(level as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnLoggerSetMask(mask: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetMask(mask as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnLoggerForceDisable() -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerForceDisable() };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXpolar_bufferSize<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    M: i64,
    N: i64,
    dataTypeA: cudaDataType,
    A: T0,
    lda: i64,
    dataTypeH: cudaDataType,
    H: T1,
    ldh: i64,
    computeType: cudaDataType,
    mut workspaceInBytesOnDevice: T2,
    mut workspaceInBytesOnHost: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXpolar_bufferSize(
            handle,
            params,
            uplo,
            M,
            N,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeH,
            H.as_const_ptr() as *const _,
            ldh,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverDnXpolar<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    M: i64,
    N: i64,
    dataTypeA: cudaDataType,
    mut A: T0,
    lda: i64,
    dataTypeH: cudaDataType,
    mut H: T1,
    ldh: i64,
    computeType: cudaDataType,
    mut bufferOnDevice: T2,
    workspaceInBytesOnDevice: usize,
    mut bufferOnHost: T3,
    workspaceInBytesOnHost: usize,
    mut d_res_nrm: T4,
    mut d_A_nrmF: T5,
    mut d_rcond: T6,
    mut d_info: T7,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnXpolar(
            handle,
            params,
            uplo,
            M,
            N,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeH,
            H.as_mut_ptr() as *mut _,
            ldh,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_res_nrm.as_mut_ptr() as *mut _,
            d_A_nrmF.as_mut_ptr() as *mut _,
            d_rcond.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCreate() -> Result<cusolverSpHandle_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverSpHandle_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverSpCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverSpHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverSpDestroy(handle: cusolverSpHandle_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpDestroy(handle) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpSetStream(handle: cusolverSpHandle_t, streamId: cudaStream_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpSetStream(handle, streamId) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpGetStream(handle: cusolverSpHandle_t) -> Result<cudaStream_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverSpGetStream(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaStream_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverSpXcsrissymHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T0,
    csrEndPtrA: T1,
    csrColIndA: T2,
    mut issym: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpXcsrissymHost(handle, m as _, nnzA as _, descrA, csrRowPtrA.as_const_ptr() as *const _, csrEndPtrA.as_const_ptr() as *const _, csrColIndA.as_const_ptr() as *const _, issym.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsrlsvluHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvluHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsrlsvluHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvluHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsrlsvluHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvluHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsrlsvluHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvluHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsrlsvqr<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvqr(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsrlsvqr<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvqr(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsrlsvqr<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvqr(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsrlsvqr<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvqr(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsrlsvqrHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvqrHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsrlsvqrHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvqrHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsrlsvqrHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvqrHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsrlsvqrHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvqrHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsrlsvcholHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvcholHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsrlsvcholHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvcholHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsrlsvcholHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvcholHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsrlsvcholHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvcholHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsrlsvchol<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvchol(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsrlsvchol<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvchol(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsrlsvchol<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f32,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvchol(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsrlsvchol<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    b: T3,
    tol: f64,
    reorder: i32,
    mut x: T4,
    mut singularity: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvchol(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsrlsqvqrHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f32,
    mut rankA: T4,
    mut x: T5,
    mut p: T6,
    mut min_norm: T7,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsqvqrHost(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            rankA.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            p.as_mut_ptr() as *mut _,
            min_norm.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsrlsqvqrHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f64,
    mut rankA: T4,
    mut x: T5,
    mut p: T6,
    mut min_norm: T7,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsqvqrHost(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            rankA.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            p.as_mut_ptr() as *mut _,
            min_norm.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsrlsqvqrHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f32,
    mut rankA: T4,
    mut x: T5,
    mut p: T6,
    mut min_norm: T7,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsqvqrHost(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            rankA.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            p.as_mut_ptr() as *mut _,
            min_norm.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsrlsqvqrHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    tol: f64,
    mut rankA: T4,
    mut x: T5,
    mut p: T6,
    mut min_norm: T7,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsqvqrHost(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            rankA.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            p.as_mut_ptr() as *mut _,
            min_norm.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsreigvsiHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mu0: f32,
    x0: T3,
    maxite: i32,
    tol: f32,
    mut mu: T4,
    mut x: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsreigvsiHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            tol,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsreigvsiHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mu0: f64,
    x0: T3,
    maxite: i32,
    tol: f64,
    mut mu: T4,
    mut x: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsreigvsiHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            tol,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsreigvsiHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mu0: cuComplex,
    x0: T3,
    maxite: i32,
    tol: f32,
    mut mu: T4,
    mut x: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsreigvsiHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            tol,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsreigvsiHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mu0: cuDoubleComplex,
    x0: T3,
    maxite: i32,
    tol: f64,
    mut mu: T4,
    mut x: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsreigvsiHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            tol,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsreigvsi<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mu0: f32,
    x0: T3,
    maxite: i32,
    eps: f32,
    mut mu: T4,
    mut x: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsreigvsi(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            eps,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsreigvsi<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mu0: f64,
    x0: T3,
    maxite: i32,
    eps: f64,
    mut mu: T4,
    mut x: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsreigvsi(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            eps,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsreigvsi<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mu0: cuComplex,
    x0: T3,
    maxite: i32,
    eps: f32,
    mut mu: T4,
    mut x: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsreigvsi(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            eps,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsreigvsi<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mu0: cuDoubleComplex,
    x0: T3,
    maxite: i32,
    eps: f64,
    mut mu: T4,
    mut x: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsreigvsi(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            eps,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsreigsHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    left_bottom_corner: cuComplex,
    right_upper_corner: cuComplex,
    mut num_eigs: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsreigsHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsreigsHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    left_bottom_corner: cuDoubleComplex,
    right_upper_corner: cuDoubleComplex,
    mut num_eigs: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsreigsHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsreigsHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    left_bottom_corner: cuComplex,
    right_upper_corner: cuComplex,
    mut num_eigs: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsreigsHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsreigsHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    left_bottom_corner: cuDoubleComplex,
    right_upper_corner: cuDoubleComplex,
    mut num_eigs: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsreigsHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpXcsrsymrcmHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverSpHandle_t, n: i32, nnzA: i32, descrA: cusparseMatDescr_t, csrRowPtrA: T0, csrColIndA: T1, mut p: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpXcsrsymrcmHost(handle, n as _, nnzA as _, descrA, csrRowPtrA.as_const_ptr() as *const _, csrColIndA.as_const_ptr() as *const _, p.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpXcsrsymmdqHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverSpHandle_t, n: i32, nnzA: i32, descrA: cusparseMatDescr_t, csrRowPtrA: T0, csrColIndA: T1, mut p: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpXcsrsymmdqHost(handle, n as _, nnzA as _, descrA, csrRowPtrA.as_const_ptr() as *const _, csrColIndA.as_const_ptr() as *const _, p.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpXcsrsymamdHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr>(handle: cusolverSpHandle_t, n: i32, nnzA: i32, descrA: cusparseMatDescr_t, csrRowPtrA: T0, csrColIndA: T1, mut p: T2) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpXcsrsymamdHost(handle, n as _, nnzA as _, descrA, csrRowPtrA.as_const_ptr() as *const _, csrColIndA.as_const_ptr() as *const _, p.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpXcsrmetisndHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T0,
    csrColIndA: T1,
    options: T2,
    mut p: T3,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpXcsrmetisndHost(handle, n as _, nnzA as _, descrA, csrRowPtrA.as_const_ptr() as *const _, csrColIndA.as_const_ptr() as *const _, options.as_const_ptr() as *const _, p.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsrzfdHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mut P: T3,
    mut numnz: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrzfdHost(
            handle,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            P.as_mut_ptr() as *mut _,
            numnz.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsrzfdHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mut P: T3,
    mut numnz: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrzfdHost(
            handle,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            P.as_mut_ptr() as *mut _,
            numnz.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsrzfdHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mut P: T3,
    mut numnz: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrzfdHost(
            handle,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            P.as_mut_ptr() as *mut _,
            numnz.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsrzfdHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    mut P: T3,
    mut numnz: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrzfdHost(
            handle,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            P.as_mut_ptr() as *mut _,
            numnz.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpXcsrperm_bufferSizeHost<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T0,
    csrColIndA: T1,
    p: T2,
    q: T3,
    mut bufferSizeInBytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrperm_bufferSizeHost(
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            p.as_const_ptr() as *const _,
            q.as_const_ptr() as *const _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpXcsrpermHost<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    mut csrRowPtrA: T0,
    mut csrColIndA: T1,
    p: T2,
    q: T3,
    mut map: T4,
    mut pBuffer: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrpermHost(
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_mut_ptr() as *mut _,
            csrColIndA.as_mut_ptr() as *mut _,
            p.as_const_ptr() as *const _,
            q.as_const_ptr() as *const _,
            map.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCreateCsrqrInfo() -> Result<csrqrInfo_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<csrqrInfo_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cusolverSpCreateCsrqrInfo(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as csrqrInfo_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverSpDestroyCsrqrInfo(info: csrqrInfo_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpDestroyCsrqrInfo(info) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpXcsrqrAnalysisBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr>(handle: cusolverSpHandle_t, m: i32, n: i32, nnzA: i32, descrA: cusparseMatDescr_t, csrRowPtrA: T0, csrColIndA: T1, info: csrqrInfo_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpXcsrqrAnalysisBatched(handle, m as _, n as _, nnzA as _, descrA, csrRowPtrA.as_const_ptr() as *const _, csrColIndA.as_const_ptr() as *const _, info) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsrqrBufferInfoBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    batchSize: i32,
    info: csrqrInfo_t,
    mut internalDataInBytes: T3,
    mut workspaceInBytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrqrBufferInfoBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            batchSize as _,
            info,
            internalDataInBytes.as_mut_ptr() as *mut _,
            workspaceInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsrqrBufferInfoBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    batchSize: i32,
    info: csrqrInfo_t,
    mut internalDataInBytes: T3,
    mut workspaceInBytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrqrBufferInfoBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            batchSize as _,
            info,
            internalDataInBytes.as_mut_ptr() as *mut _,
            workspaceInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsrqrBufferInfoBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    batchSize: i32,
    info: csrqrInfo_t,
    mut internalDataInBytes: T3,
    mut workspaceInBytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrqrBufferInfoBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            batchSize as _,
            info,
            internalDataInBytes.as_mut_ptr() as *mut _,
            workspaceInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsrqrBufferInfoBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T0,
    csrRowPtr: T1,
    csrColInd: T2,
    batchSize: i32,
    info: csrqrInfo_t,
    mut internalDataInBytes: T3,
    mut workspaceInBytes: T4,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrqrBufferInfoBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            batchSize as _,
            info,
            internalDataInBytes.as_mut_ptr() as *mut _,
            workspaceInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpScsrqrsvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    mut x: T4,
    batchSize: i32,
    info: csrqrInfo_t,
    mut pBuffer: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrqrsvBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchSize as _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpDcsrqrsvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    mut x: T4,
    batchSize: i32,
    info: csrqrInfo_t,
    mut pBuffer: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrqrsvBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchSize as _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpCcsrqrsvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    mut x: T4,
    batchSize: i32,
    info: csrqrInfo_t,
    mut pBuffer: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrqrsvBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchSize as _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cusolverSpZcsrqrsvBatched<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T0,
    csrRowPtrA: T1,
    csrColIndA: T2,
    b: T3,
    mut x: T4,
    batchSize: i32,
    info: csrqrInfo_t,
    mut pBuffer: T5,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrqrsvBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchSize as _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
