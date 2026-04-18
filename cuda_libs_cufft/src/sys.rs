use cuda_libs_cudart::sys::*;
pub const CUFFT_VER_MAJOR: u32 = 12;
pub const CUFFT_VER_MINOR: u32 = 2;
pub const CUFFT_VER_PATCH: u32 = 0;
pub const CUFFT_VER_BUILD: u32 = 46;
pub const CUFFT_VERSION: u32 = 12200;
pub const CUFFT_FORWARD: i32 = -1;
pub const CUFFT_INVERSE: u32 = 1;
pub const CUFFT_PLAN_NULL: i32 = -1;
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
pub type cuFloatComplex = float2;
pub type cuDoubleComplex = double2;
pub type cuComplex = cuFloatComplex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
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
pub enum cufftResult_t {
    CUFFT_SUCCESS = 0,
    CUFFT_INVALID_PLAN = 1,
    CUFFT_ALLOC_FAILED = 2,
    CUFFT_INVALID_TYPE = 3,
    CUFFT_INVALID_VALUE = 4,
    CUFFT_INTERNAL_ERROR = 5,
    CUFFT_EXEC_FAILED = 6,
    CUFFT_SETUP_FAILED = 7,
    CUFFT_INVALID_SIZE = 8,
    CUFFT_UNALIGNED_DATA = 9,
    CUFFT_INVALID_DEVICE = 11,
    CUFFT_NO_WORKSPACE = 13,
    CUFFT_NOT_IMPLEMENTED = 14,
    CUFFT_NOT_SUPPORTED = 16,
    CUFFT_MISSING_DEPENDENCY = 17,
    CUFFT_NVRTC_FAILURE = 18,
    CUFFT_NVJITLINK_FAILURE = 19,
    CUFFT_NVSHMEM_FAILURE = 20,
}
pub use self::cufftResult_t as cufftResult;
pub type cufftReal = f32;
pub type cufftDoubleReal = f64;
pub type cufftComplex = cuComplex;
pub type cufftDoubleComplex = cuDoubleComplex;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cufftType_t {
    CUFFT_R2C = 42,
    CUFFT_C2R = 44,
    CUFFT_C2C = 41,
    CUFFT_D2Z = 106,
    CUFFT_Z2D = 108,
    CUFFT_Z2Z = 105,
}
pub use self::cufftType_t as cufftType;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cufftCompatibility_t {
    CUFFT_COMPATIBILITY_FFTW_PADDING = 1,
}
pub use self::cufftCompatibility_t as cufftCompatibility;
pub type cufftHandle = ::std::os::raw::c_int;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftPlan1d(plan: *mut cufftHandle, nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftPlan2d(plan: *mut cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftPlan3d(plan: *mut cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftPlanMany(
        plan: *mut cufftHandle,
        rank: ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        inembed: *mut ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        onembed: *mut ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        type_: cufftType,
        batch: ::std::os::raw::c_int,
    ) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftMakePlan1d(plan: cufftHandle, nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftMakePlan2d(plan: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftMakePlan3d(plan: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftMakePlanMany(
        plan: cufftHandle,
        rank: ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        inembed: *mut ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        onembed: *mut ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        type_: cufftType,
        batch: ::std::os::raw::c_int,
        workSize: *mut usize,
    ) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftMakePlanMany64(
        plan: cufftHandle,
        rank: ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_longlong,
        inembed: *mut ::std::os::raw::c_longlong,
        istride: ::std::os::raw::c_longlong,
        idist: ::std::os::raw::c_longlong,
        onembed: *mut ::std::os::raw::c_longlong,
        ostride: ::std::os::raw::c_longlong,
        odist: ::std::os::raw::c_longlong,
        type_: cufftType,
        batch: ::std::os::raw::c_longlong,
        workSize: *mut usize,
    ) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftGetSizeMany64(
        plan: cufftHandle,
        rank: ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_longlong,
        inembed: *mut ::std::os::raw::c_longlong,
        istride: ::std::os::raw::c_longlong,
        idist: ::std::os::raw::c_longlong,
        onembed: *mut ::std::os::raw::c_longlong,
        ostride: ::std::os::raw::c_longlong,
        odist: ::std::os::raw::c_longlong,
        type_: cufftType,
        batch: ::std::os::raw::c_longlong,
        workSize: *mut usize,
    ) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftEstimate1d(nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftEstimate2d(nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftEstimate3d(nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftEstimateMany(
        rank: ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        inembed: *mut ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        onembed: *mut ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        type_: cufftType,
        batch: ::std::os::raw::c_int,
        workSize: *mut usize,
    ) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftCreate(handle: *mut cufftHandle) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftGetSize1d(handle: cufftHandle, nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftGetSize2d(handle: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftGetSize3d(handle: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftGetSizeMany(
        handle: cufftHandle,
        rank: ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        inembed: *mut ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        onembed: *mut ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        type_: cufftType,
        batch: ::std::os::raw::c_int,
        workArea: *mut usize,
    ) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftGetSize(handle: cufftHandle, workSize: *mut usize) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftSetWorkArea(plan: cufftHandle, workArea: *mut ::std::os::raw::c_void) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftSetAutoAllocation(plan: cufftHandle, autoAllocate: ::std::os::raw::c_int) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftExecC2C(plan: cufftHandle, idata: *mut cufftComplex, odata: *mut cufftComplex, direction: ::std::os::raw::c_int) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftExecR2C(plan: cufftHandle, idata: *mut cufftReal, odata: *mut cufftComplex) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftExecC2R(plan: cufftHandle, idata: *mut cufftComplex, odata: *mut cufftReal) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftExecZ2Z(plan: cufftHandle, idata: *mut cufftDoubleComplex, odata: *mut cufftDoubleComplex, direction: ::std::os::raw::c_int) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftExecD2Z(plan: cufftHandle, idata: *mut cufftDoubleReal, odata: *mut cufftDoubleComplex) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftExecZ2D(plan: cufftHandle, idata: *mut cufftDoubleComplex, odata: *mut cufftDoubleReal) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftSetStream(plan: cufftHandle, stream: cudaStream_t) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftDestroy(plan: cufftHandle) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftGetVersion(version: *mut ::std::os::raw::c_int) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftGetProperty(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cufftResult;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cufftProperty_t {
    NVFFT_PLAN_PROPERTY_INT64_PATIENT_JIT = 1,
    NVFFT_PLAN_PROPERTY_INT64_MAX_NUM_HOST_THREADS = 2,
}
pub use self::cufftProperty_t as cufftProperty;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftSetPlanPropertyInt64(plan: cufftHandle, property: cufftProperty, inputValueInt: ::std::os::raw::c_longlong) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftGetPlanPropertyInt64(plan: cufftHandle, property: cufftProperty, returnPtrValue: *mut ::std::os::raw::c_longlong) -> cufftResult;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cufftResetPlanProperty(plan: cufftHandle, property: cufftProperty) -> cufftResult;
}
#[cfg(feature = "runtime-link")]
pub struct DynamicBindings {
    pub cufftPlan1d: Option<unsafe extern "C" fn(plan: *mut cufftHandle, nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int) -> cufftResult>,
    pub cufftPlan2d: Option<unsafe extern "C" fn(plan: *mut cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType) -> cufftResult>,
    pub cufftPlan3d: Option<unsafe extern "C" fn(plan: *mut cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType) -> cufftResult>,
    pub cufftPlanMany: Option<
        unsafe extern "C" fn(
            plan: *mut cufftHandle,
            rank: ::std::os::raw::c_int,
            n: *mut ::std::os::raw::c_int,
            inembed: *mut ::std::os::raw::c_int,
            istride: ::std::os::raw::c_int,
            idist: ::std::os::raw::c_int,
            onembed: *mut ::std::os::raw::c_int,
            ostride: ::std::os::raw::c_int,
            odist: ::std::os::raw::c_int,
            type_: cufftType,
            batch: ::std::os::raw::c_int,
        ) -> cufftResult,
    >,
    pub cufftMakePlan1d: Option<unsafe extern "C" fn(plan: cufftHandle, nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int, workSize: *mut usize) -> cufftResult>,
    pub cufftMakePlan2d: Option<unsafe extern "C" fn(plan: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult>,
    pub cufftMakePlan3d: Option<unsafe extern "C" fn(plan: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult>,
    pub cufftMakePlanMany: Option<
        unsafe extern "C" fn(
            plan: cufftHandle,
            rank: ::std::os::raw::c_int,
            n: *mut ::std::os::raw::c_int,
            inembed: *mut ::std::os::raw::c_int,
            istride: ::std::os::raw::c_int,
            idist: ::std::os::raw::c_int,
            onembed: *mut ::std::os::raw::c_int,
            ostride: ::std::os::raw::c_int,
            odist: ::std::os::raw::c_int,
            type_: cufftType,
            batch: ::std::os::raw::c_int,
            workSize: *mut usize,
        ) -> cufftResult,
    >,
    pub cufftMakePlanMany64: Option<
        unsafe extern "C" fn(
            plan: cufftHandle,
            rank: ::std::os::raw::c_int,
            n: *mut ::std::os::raw::c_longlong,
            inembed: *mut ::std::os::raw::c_longlong,
            istride: ::std::os::raw::c_longlong,
            idist: ::std::os::raw::c_longlong,
            onembed: *mut ::std::os::raw::c_longlong,
            ostride: ::std::os::raw::c_longlong,
            odist: ::std::os::raw::c_longlong,
            type_: cufftType,
            batch: ::std::os::raw::c_longlong,
            workSize: *mut usize,
        ) -> cufftResult,
    >,
    pub cufftGetSizeMany64: Option<
        unsafe extern "C" fn(
            plan: cufftHandle,
            rank: ::std::os::raw::c_int,
            n: *mut ::std::os::raw::c_longlong,
            inembed: *mut ::std::os::raw::c_longlong,
            istride: ::std::os::raw::c_longlong,
            idist: ::std::os::raw::c_longlong,
            onembed: *mut ::std::os::raw::c_longlong,
            ostride: ::std::os::raw::c_longlong,
            odist: ::std::os::raw::c_longlong,
            type_: cufftType,
            batch: ::std::os::raw::c_longlong,
            workSize: *mut usize,
        ) -> cufftResult,
    >,
    pub cufftEstimate1d: Option<unsafe extern "C" fn(nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int, workSize: *mut usize) -> cufftResult>,
    pub cufftEstimate2d: Option<unsafe extern "C" fn(nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult>,
    pub cufftEstimate3d: Option<unsafe extern "C" fn(nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult>,
    pub cufftEstimateMany: Option<
        unsafe extern "C" fn(
            rank: ::std::os::raw::c_int,
            n: *mut ::std::os::raw::c_int,
            inembed: *mut ::std::os::raw::c_int,
            istride: ::std::os::raw::c_int,
            idist: ::std::os::raw::c_int,
            onembed: *mut ::std::os::raw::c_int,
            ostride: ::std::os::raw::c_int,
            odist: ::std::os::raw::c_int,
            type_: cufftType,
            batch: ::std::os::raw::c_int,
            workSize: *mut usize,
        ) -> cufftResult,
    >,
    pub cufftCreate: Option<unsafe extern "C" fn(handle: *mut cufftHandle) -> cufftResult>,
    pub cufftGetSize1d: Option<unsafe extern "C" fn(handle: cufftHandle, nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int, workSize: *mut usize) -> cufftResult>,
    pub cufftGetSize2d: Option<unsafe extern "C" fn(handle: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult>,
    pub cufftGetSize3d: Option<unsafe extern "C" fn(handle: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult>,
    pub cufftGetSizeMany: Option<
        unsafe extern "C" fn(
            handle: cufftHandle,
            rank: ::std::os::raw::c_int,
            n: *mut ::std::os::raw::c_int,
            inembed: *mut ::std::os::raw::c_int,
            istride: ::std::os::raw::c_int,
            idist: ::std::os::raw::c_int,
            onembed: *mut ::std::os::raw::c_int,
            ostride: ::std::os::raw::c_int,
            odist: ::std::os::raw::c_int,
            type_: cufftType,
            batch: ::std::os::raw::c_int,
            workArea: *mut usize,
        ) -> cufftResult,
    >,
    pub cufftGetSize: Option<unsafe extern "C" fn(handle: cufftHandle, workSize: *mut usize) -> cufftResult>,
    pub cufftSetWorkArea: Option<unsafe extern "C" fn(plan: cufftHandle, workArea: *mut ::std::os::raw::c_void) -> cufftResult>,
    pub cufftSetAutoAllocation: Option<unsafe extern "C" fn(plan: cufftHandle, autoAllocate: ::std::os::raw::c_int) -> cufftResult>,
    pub cufftExecC2C: Option<unsafe extern "C" fn(plan: cufftHandle, idata: *mut cufftComplex, odata: *mut cufftComplex, direction: ::std::os::raw::c_int) -> cufftResult>,
    pub cufftExecR2C: Option<unsafe extern "C" fn(plan: cufftHandle, idata: *mut cufftReal, odata: *mut cufftComplex) -> cufftResult>,
    pub cufftExecC2R: Option<unsafe extern "C" fn(plan: cufftHandle, idata: *mut cufftComplex, odata: *mut cufftReal) -> cufftResult>,
    pub cufftExecZ2Z: Option<unsafe extern "C" fn(plan: cufftHandle, idata: *mut cufftDoubleComplex, odata: *mut cufftDoubleComplex, direction: ::std::os::raw::c_int) -> cufftResult>,
    pub cufftExecD2Z: Option<unsafe extern "C" fn(plan: cufftHandle, idata: *mut cufftDoubleReal, odata: *mut cufftDoubleComplex) -> cufftResult>,
    pub cufftExecZ2D: Option<unsafe extern "C" fn(plan: cufftHandle, idata: *mut cufftDoubleComplex, odata: *mut cufftDoubleReal) -> cufftResult>,
    pub cufftSetStream: Option<unsafe extern "C" fn(plan: cufftHandle, stream: cudaStream_t) -> cufftResult>,
    pub cufftDestroy: Option<unsafe extern "C" fn(plan: cufftHandle) -> cufftResult>,
    pub cufftGetVersion: Option<unsafe extern "C" fn(version: *mut ::std::os::raw::c_int) -> cufftResult>,
    pub cufftGetProperty: Option<unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cufftResult>,
    pub cufftSetPlanPropertyInt64: Option<unsafe extern "C" fn(plan: cufftHandle, property: cufftProperty, inputValueInt: ::std::os::raw::c_longlong) -> cufftResult>,
    pub cufftGetPlanPropertyInt64: Option<unsafe extern "C" fn(plan: cufftHandle, property: cufftProperty, returnPtrValue: *mut ::std::os::raw::c_longlong) -> cufftResult>,
    pub cufftResetPlanProperty: Option<unsafe extern "C" fn(plan: cufftHandle, property: cufftProperty) -> cufftResult>,
}
#[cfg(feature = "runtime-link")]
unsafe impl Send for DynamicBindings {}
#[cfg(feature = "runtime-link")]
unsafe impl Sync for DynamicBindings {}
#[cfg(feature = "runtime-link")]
pub static DYNAMIC_BINDINGS: std::sync::OnceLock<Box<DynamicBindings>> = std::sync::OnceLock::new();
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftPlan1d(plan: *mut cufftHandle, nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftPlan1d {
        Some(____func) => unsafe { ____func(plan, nx, type_, batch) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftPlan1d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftPlan2d(plan: *mut cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftPlan2d {
        Some(____func) => unsafe { ____func(plan, nx, ny, type_) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftPlan2d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftPlan3d(plan: *mut cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftPlan3d {
        Some(____func) => unsafe { ____func(plan, nx, ny, nz, type_) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftPlan3d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftPlanMany(
    plan: *mut cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_int,
    inembed: *mut ::std::os::raw::c_int,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    onembed: *mut ::std::os::raw::c_int,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftPlanMany {
        Some(____func) => unsafe { ____func(plan, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftPlanMany"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftMakePlan1d(plan: cufftHandle, nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftMakePlan1d {
        Some(____func) => unsafe { ____func(plan, nx, type_, batch, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftMakePlan1d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftMakePlan2d(plan: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftMakePlan2d {
        Some(____func) => unsafe { ____func(plan, nx, ny, type_, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftMakePlan2d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftMakePlan3d(plan: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftMakePlan3d {
        Some(____func) => unsafe { ____func(plan, nx, ny, nz, type_, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftMakePlan3d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftMakePlanMany(
    plan: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_int,
    inembed: *mut ::std::os::raw::c_int,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    onembed: *mut ::std::os::raw::c_int,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    workSize: *mut usize,
) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftMakePlanMany {
        Some(____func) => unsafe { ____func(plan, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftMakePlanMany"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftMakePlanMany64(
    plan: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_longlong,
    inembed: *mut ::std::os::raw::c_longlong,
    istride: ::std::os::raw::c_longlong,
    idist: ::std::os::raw::c_longlong,
    onembed: *mut ::std::os::raw::c_longlong,
    ostride: ::std::os::raw::c_longlong,
    odist: ::std::os::raw::c_longlong,
    type_: cufftType,
    batch: ::std::os::raw::c_longlong,
    workSize: *mut usize,
) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftMakePlanMany64 {
        Some(____func) => unsafe { ____func(plan, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftMakePlanMany64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftGetSizeMany64(
    plan: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_longlong,
    inembed: *mut ::std::os::raw::c_longlong,
    istride: ::std::os::raw::c_longlong,
    idist: ::std::os::raw::c_longlong,
    onembed: *mut ::std::os::raw::c_longlong,
    ostride: ::std::os::raw::c_longlong,
    odist: ::std::os::raw::c_longlong,
    type_: cufftType,
    batch: ::std::os::raw::c_longlong,
    workSize: *mut usize,
) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftGetSizeMany64 {
        Some(____func) => unsafe { ____func(plan, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftGetSizeMany64"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftEstimate1d(nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftEstimate1d {
        Some(____func) => unsafe { ____func(nx, type_, batch, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftEstimate1d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftEstimate2d(nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftEstimate2d {
        Some(____func) => unsafe { ____func(nx, ny, type_, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftEstimate2d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftEstimate3d(nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftEstimate3d {
        Some(____func) => unsafe { ____func(nx, ny, nz, type_, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftEstimate3d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftEstimateMany(
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_int,
    inembed: *mut ::std::os::raw::c_int,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    onembed: *mut ::std::os::raw::c_int,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    workSize: *mut usize,
) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftEstimateMany {
        Some(____func) => unsafe { ____func(rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftEstimateMany"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftCreate(handle: *mut cufftHandle) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftCreate {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftCreate"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftGetSize1d(handle: cufftHandle, nx: ::std::os::raw::c_int, type_: cufftType, batch: ::std::os::raw::c_int, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftGetSize1d {
        Some(____func) => unsafe { ____func(handle, nx, type_, batch, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftGetSize1d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftGetSize2d(handle: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftGetSize2d {
        Some(____func) => unsafe { ____func(handle, nx, ny, type_, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftGetSize2d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftGetSize3d(handle: cufftHandle, nx: ::std::os::raw::c_int, ny: ::std::os::raw::c_int, nz: ::std::os::raw::c_int, type_: cufftType, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftGetSize3d {
        Some(____func) => unsafe { ____func(handle, nx, ny, nz, type_, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftGetSize3d"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftGetSizeMany(
    handle: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_int,
    inembed: *mut ::std::os::raw::c_int,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    onembed: *mut ::std::os::raw::c_int,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    workArea: *mut usize,
) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftGetSizeMany {
        Some(____func) => unsafe { ____func(handle, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch, workArea) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftGetSizeMany"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftGetSize(handle: cufftHandle, workSize: *mut usize) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftGetSize {
        Some(____func) => unsafe { ____func(handle, workSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftGetSize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftSetWorkArea(plan: cufftHandle, workArea: *mut ::std::os::raw::c_void) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftSetWorkArea {
        Some(____func) => unsafe { ____func(plan, workArea) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftSetWorkArea"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftSetAutoAllocation(plan: cufftHandle, autoAllocate: ::std::os::raw::c_int) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftSetAutoAllocation {
        Some(____func) => unsafe { ____func(plan, autoAllocate) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftSetAutoAllocation"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftExecC2C(plan: cufftHandle, idata: *mut cufftComplex, odata: *mut cufftComplex, direction: ::std::os::raw::c_int) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftExecC2C {
        Some(____func) => unsafe { ____func(plan, idata, odata, direction) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftExecC2C"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftExecR2C(plan: cufftHandle, idata: *mut cufftReal, odata: *mut cufftComplex) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftExecR2C {
        Some(____func) => unsafe { ____func(plan, idata, odata) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftExecR2C"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftExecC2R(plan: cufftHandle, idata: *mut cufftComplex, odata: *mut cufftReal) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftExecC2R {
        Some(____func) => unsafe { ____func(plan, idata, odata) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftExecC2R"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftExecZ2Z(plan: cufftHandle, idata: *mut cufftDoubleComplex, odata: *mut cufftDoubleComplex, direction: ::std::os::raw::c_int) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftExecZ2Z {
        Some(____func) => unsafe { ____func(plan, idata, odata, direction) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftExecZ2Z"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftExecD2Z(plan: cufftHandle, idata: *mut cufftDoubleReal, odata: *mut cufftDoubleComplex) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftExecD2Z {
        Some(____func) => unsafe { ____func(plan, idata, odata) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftExecD2Z"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftExecZ2D(plan: cufftHandle, idata: *mut cufftDoubleComplex, odata: *mut cufftDoubleReal) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftExecZ2D {
        Some(____func) => unsafe { ____func(plan, idata, odata) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftExecZ2D"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftSetStream(plan: cufftHandle, stream: cudaStream_t) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftSetStream {
        Some(____func) => unsafe { ____func(plan, stream) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftSetStream"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftDestroy(plan: cufftHandle) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftDestroy {
        Some(____func) => unsafe { ____func(plan) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftDestroy"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftGetVersion(version: *mut ::std::os::raw::c_int) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftGetVersion {
        Some(____func) => unsafe { ____func(version) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftGetVersion"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftGetProperty(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftGetProperty {
        Some(____func) => unsafe { ____func(type_, value) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftGetProperty"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftSetPlanPropertyInt64(plan: cufftHandle, property: cufftProperty, inputValueInt: ::std::os::raw::c_longlong) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftSetPlanPropertyInt64 {
        Some(____func) => unsafe { ____func(plan, property, inputValueInt) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cufftSetPlanPropertyInt64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftGetPlanPropertyInt64(plan: cufftHandle, property: cufftProperty, returnPtrValue: *mut ::std::os::raw::c_longlong) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftGetPlanPropertyInt64 {
        Some(____func) => unsafe { ____func(plan, property, returnPtrValue) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cufftGetPlanPropertyInt64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cufftResetPlanProperty(plan: cufftHandle, property: cufftProperty) -> cufftResult {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cufftResetPlanProperty {
        Some(____func) => unsafe { ____func(plan, property) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cufftResetPlanProperty"),
    }
}
#[cfg(feature = "runtime-link")]
pub unsafe fn load_dynamic_bindings(lib: *mut std::ffi::c_void, get_proc_addr: unsafe fn(*mut std::ffi::c_void, *const u8) -> *mut std::ffi::c_void) {
    let bindings = unsafe {
        Box::new(DynamicBindings {
            cufftPlan1d: {
                let p = get_proc_addr(lib, b"cufftPlan1d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftPlan2d: {
                let p = get_proc_addr(lib, b"cufftPlan2d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftPlan3d: {
                let p = get_proc_addr(lib, b"cufftPlan3d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftPlanMany: {
                let p = get_proc_addr(lib, b"cufftPlanMany\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftMakePlan1d: {
                let p = get_proc_addr(lib, b"cufftMakePlan1d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftMakePlan2d: {
                let p = get_proc_addr(lib, b"cufftMakePlan2d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftMakePlan3d: {
                let p = get_proc_addr(lib, b"cufftMakePlan3d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftMakePlanMany: {
                let p = get_proc_addr(lib, b"cufftMakePlanMany\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftMakePlanMany64: {
                let p = get_proc_addr(lib, b"cufftMakePlanMany64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftGetSizeMany64: {
                let p = get_proc_addr(lib, b"cufftGetSizeMany64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftEstimate1d: {
                let p = get_proc_addr(lib, b"cufftEstimate1d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftEstimate2d: {
                let p = get_proc_addr(lib, b"cufftEstimate2d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftEstimate3d: {
                let p = get_proc_addr(lib, b"cufftEstimate3d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftEstimateMany: {
                let p = get_proc_addr(lib, b"cufftEstimateMany\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftCreate: {
                let p = get_proc_addr(lib, b"cufftCreate\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftGetSize1d: {
                let p = get_proc_addr(lib, b"cufftGetSize1d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftGetSize2d: {
                let p = get_proc_addr(lib, b"cufftGetSize2d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftGetSize3d: {
                let p = get_proc_addr(lib, b"cufftGetSize3d\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftGetSizeMany: {
                let p = get_proc_addr(lib, b"cufftGetSizeMany\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftGetSize: {
                let p = get_proc_addr(lib, b"cufftGetSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftSetWorkArea: {
                let p = get_proc_addr(lib, b"cufftSetWorkArea\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftSetAutoAllocation: {
                let p = get_proc_addr(lib, b"cufftSetAutoAllocation\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftExecC2C: {
                let p = get_proc_addr(lib, b"cufftExecC2C\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftExecR2C: {
                let p = get_proc_addr(lib, b"cufftExecR2C\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftExecC2R: {
                let p = get_proc_addr(lib, b"cufftExecC2R\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftExecZ2Z: {
                let p = get_proc_addr(lib, b"cufftExecZ2Z\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftExecD2Z: {
                let p = get_proc_addr(lib, b"cufftExecD2Z\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftExecZ2D: {
                let p = get_proc_addr(lib, b"cufftExecZ2D\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftSetStream: {
                let p = get_proc_addr(lib, b"cufftSetStream\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftDestroy: {
                let p = get_proc_addr(lib, b"cufftDestroy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftGetVersion: {
                let p = get_proc_addr(lib, b"cufftGetVersion\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftGetProperty: {
                let p = get_proc_addr(lib, b"cufftGetProperty\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftSetPlanPropertyInt64: {
                let p = get_proc_addr(lib, b"cufftSetPlanPropertyInt64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftGetPlanPropertyInt64: {
                let p = get_proc_addr(lib, b"cufftGetPlanPropertyInt64\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cufftResetPlanProperty: {
                let p = get_proc_addr(lib, b"cufftResetPlanProperty\0".as_ptr());
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
unsafe impl Send for cufftResult_t {}
unsafe impl Sync for cufftResult_t {}
unsafe impl Send for cufftType_t {}
unsafe impl Sync for cufftType_t {}
unsafe impl Send for cufftCompatibility_t {}
unsafe impl Sync for cufftCompatibility_t {}
unsafe impl Send for cufftProperty_t {}
unsafe impl Sync for cufftProperty_t {}
