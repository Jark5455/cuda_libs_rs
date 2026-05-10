#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
pub use crate::sys::CUFFT_FORWARD;
pub use crate::sys::CUFFT_INVERSE;
pub use crate::sys::CUFFT_PLAN_NULL;
pub use crate::sys::CUFFT_VER_BUILD;
pub use crate::sys::CUFFT_VER_MAJOR;
pub use crate::sys::CUFFT_VER_MINOR;
pub use crate::sys::CUFFT_VER_PATCH;
pub use crate::sys::CUFFT_VERSION;
pub use crate::sys::cufftCompatibility_t;
pub use crate::sys::cufftComplex;
pub use crate::sys::cufftDoubleComplex;
pub use crate::sys::cufftDoubleReal;
pub use crate::sys::cufftHandle;
pub use crate::sys::cufftProperty_t;
pub use crate::sys::cufftReal;
pub use crate::sys::cufftResult as CudaTargetStatus;
pub use crate::sys::cufftResult_t;
pub use crate::sys::cufftType_t;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart;
#[allow(unused_imports)]
use cuda_libs_cudart::sys::*;
#[allow(unused_imports)]
use cuda_libs_cudart::types;
#[cfg(feature = "runtime-link")]
impl crate::sys::DynamicBindings {
    pub fn cufftPlan1d(mut self, val: Option<unsafe extern "C" fn(*mut cufftHandle, ::core::ffi::c_int, cufftType, ::core::ffi::c_int) -> cufftResult>) -> Self {
        self.cufftPlan1d = val;
        self
    }
    pub fn cufftPlan2d(mut self, val: Option<unsafe extern "C" fn(*mut cufftHandle, ::core::ffi::c_int, ::core::ffi::c_int, cufftType) -> cufftResult>) -> Self {
        self.cufftPlan2d = val;
        self
    }
    pub fn cufftPlan3d(mut self, val: Option<unsafe extern "C" fn(*mut cufftHandle, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cufftType) -> cufftResult>) -> Self {
        self.cufftPlan3d = val;
        self
    }
    pub fn cufftPlanMany(
        mut self,
        val: Option<unsafe extern "C" fn(*mut cufftHandle, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cufftType, ::core::ffi::c_int) -> cufftResult>,
    ) -> Self {
        self.cufftPlanMany = val;
        self
    }
    pub fn cufftMakePlan1d(mut self, val: Option<unsafe extern "C" fn(cufftHandle, ::core::ffi::c_int, cufftType, ::core::ffi::c_int, *mut usize) -> cufftResult>) -> Self {
        self.cufftMakePlan1d = val;
        self
    }
    pub fn cufftMakePlan2d(mut self, val: Option<unsafe extern "C" fn(cufftHandle, ::core::ffi::c_int, ::core::ffi::c_int, cufftType, *mut usize) -> cufftResult>) -> Self {
        self.cufftMakePlan2d = val;
        self
    }
    pub fn cufftMakePlan3d(mut self, val: Option<unsafe extern "C" fn(cufftHandle, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cufftType, *mut usize) -> cufftResult>) -> Self {
        self.cufftMakePlan3d = val;
        self
    }
    pub fn cufftMakePlanMany(
        mut self,
        val: Option<unsafe extern "C" fn(cufftHandle, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cufftType, ::core::ffi::c_int, *mut usize) -> cufftResult>,
    ) -> Self {
        self.cufftMakePlanMany = val;
        self
    }
    pub fn cufftMakePlanMany64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cufftHandle,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_longlong,
                *mut ::core::ffi::c_longlong,
                ::core::ffi::c_longlong,
                ::core::ffi::c_longlong,
                *mut ::core::ffi::c_longlong,
                ::core::ffi::c_longlong,
                ::core::ffi::c_longlong,
                cufftType,
                ::core::ffi::c_longlong,
                *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftMakePlanMany64 = val;
        self
    }
    pub fn cufftGetSizeMany64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cufftHandle,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_longlong,
                *mut ::core::ffi::c_longlong,
                ::core::ffi::c_longlong,
                ::core::ffi::c_longlong,
                *mut ::core::ffi::c_longlong,
                ::core::ffi::c_longlong,
                ::core::ffi::c_longlong,
                cufftType,
                ::core::ffi::c_longlong,
                *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftGetSizeMany64 = val;
        self
    }
    pub fn cufftEstimate1d(mut self, val: Option<unsafe extern "C" fn(::core::ffi::c_int, cufftType, ::core::ffi::c_int, *mut usize) -> cufftResult>) -> Self {
        self.cufftEstimate1d = val;
        self
    }
    pub fn cufftEstimate2d(mut self, val: Option<unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int, cufftType, *mut usize) -> cufftResult>) -> Self {
        self.cufftEstimate2d = val;
        self
    }
    pub fn cufftEstimate3d(mut self, val: Option<unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cufftType, *mut usize) -> cufftResult>) -> Self {
        self.cufftEstimate3d = val;
        self
    }
    pub fn cufftEstimateMany(
        mut self,
        val: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cufftType, ::core::ffi::c_int, *mut usize) -> cufftResult>,
    ) -> Self {
        self.cufftEstimateMany = val;
        self
    }
    pub fn cufftCreate(mut self, val: Option<unsafe extern "C" fn(*mut cufftHandle) -> cufftResult>) -> Self {
        self.cufftCreate = val;
        self
    }
    pub fn cufftGetSize1d(mut self, val: Option<unsafe extern "C" fn(cufftHandle, ::core::ffi::c_int, cufftType, ::core::ffi::c_int, *mut usize) -> cufftResult>) -> Self {
        self.cufftGetSize1d = val;
        self
    }
    pub fn cufftGetSize2d(mut self, val: Option<unsafe extern "C" fn(cufftHandle, ::core::ffi::c_int, ::core::ffi::c_int, cufftType, *mut usize) -> cufftResult>) -> Self {
        self.cufftGetSize2d = val;
        self
    }
    pub fn cufftGetSize3d(mut self, val: Option<unsafe extern "C" fn(cufftHandle, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cufftType, *mut usize) -> cufftResult>) -> Self {
        self.cufftGetSize3d = val;
        self
    }
    pub fn cufftGetSizeMany(
        mut self,
        val: Option<unsafe extern "C" fn(cufftHandle, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cufftType, ::core::ffi::c_int, *mut usize) -> cufftResult>,
    ) -> Self {
        self.cufftGetSizeMany = val;
        self
    }
    pub fn cufftGetSize(mut self, val: Option<unsafe extern "C" fn(cufftHandle, *mut usize) -> cufftResult>) -> Self {
        self.cufftGetSize = val;
        self
    }
    pub fn cufftSetWorkArea(mut self, val: Option<unsafe extern "C" fn(cufftHandle, *mut ::core::ffi::c_void) -> cufftResult>) -> Self {
        self.cufftSetWorkArea = val;
        self
    }
    pub fn cufftSetAutoAllocation(mut self, val: Option<unsafe extern "C" fn(cufftHandle, ::core::ffi::c_int) -> cufftResult>) -> Self {
        self.cufftSetAutoAllocation = val;
        self
    }
    pub fn cufftExecC2C(mut self, val: Option<unsafe extern "C" fn(cufftHandle, *mut cufftComplex, *mut cufftComplex, ::core::ffi::c_int) -> cufftResult>) -> Self {
        self.cufftExecC2C = val;
        self
    }
    pub fn cufftExecR2C(mut self, val: Option<unsafe extern "C" fn(cufftHandle, *mut cufftReal, *mut cufftComplex) -> cufftResult>) -> Self {
        self.cufftExecR2C = val;
        self
    }
    pub fn cufftExecC2R(mut self, val: Option<unsafe extern "C" fn(cufftHandle, *mut cufftComplex, *mut cufftReal) -> cufftResult>) -> Self {
        self.cufftExecC2R = val;
        self
    }
    pub fn cufftExecZ2Z(mut self, val: Option<unsafe extern "C" fn(cufftHandle, *mut cufftDoubleComplex, *mut cufftDoubleComplex, ::core::ffi::c_int) -> cufftResult>) -> Self {
        self.cufftExecZ2Z = val;
        self
    }
    pub fn cufftExecD2Z(mut self, val: Option<unsafe extern "C" fn(cufftHandle, *mut cufftDoubleReal, *mut cufftDoubleComplex) -> cufftResult>) -> Self {
        self.cufftExecD2Z = val;
        self
    }
    pub fn cufftExecZ2D(mut self, val: Option<unsafe extern "C" fn(cufftHandle, *mut cufftDoubleComplex, *mut cufftDoubleReal) -> cufftResult>) -> Self {
        self.cufftExecZ2D = val;
        self
    }
    pub fn cufftSetStream(mut self, val: Option<unsafe extern "C" fn(cufftHandle, cudaStream_t) -> cufftResult>) -> Self {
        self.cufftSetStream = val;
        self
    }
    pub fn cufftDestroy(mut self, val: Option<unsafe extern "C" fn(cufftHandle) -> cufftResult>) -> Self {
        self.cufftDestroy = val;
        self
    }
    pub fn cufftGetVersion(mut self, val: Option<unsafe extern "C" fn(*mut ::core::ffi::c_int) -> cufftResult>) -> Self {
        self.cufftGetVersion = val;
        self
    }
    pub fn cufftGetProperty(mut self, val: Option<unsafe extern "C" fn(libraryPropertyType, *mut ::core::ffi::c_int) -> cufftResult>) -> Self {
        self.cufftGetProperty = val;
        self
    }
    pub fn cufftSetPlanPropertyInt64(mut self, val: Option<unsafe extern "C" fn(cufftHandle, cufftProperty, ::core::ffi::c_longlong) -> cufftResult>) -> Self {
        self.cufftSetPlanPropertyInt64 = val;
        self
    }
    pub fn cufftGetPlanPropertyInt64(mut self, val: Option<unsafe extern "C" fn(cufftHandle, cufftProperty, *mut ::core::ffi::c_longlong) -> cufftResult>) -> Self {
        self.cufftGetPlanPropertyInt64 = val;
        self
    }
    pub fn cufftResetPlanProperty(mut self, val: Option<unsafe extern "C" fn(cufftHandle, cufftProperty) -> cufftResult>) -> Self {
        self.cufftResetPlanProperty = val;
        self
    }
}
pub unsafe fn cufftPlan1d(nx: ::core::ffi::c_int, type_: cufftType, batch: ::core::ffi::c_int) -> Result<cufftHandle, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftPlan1d(out_0.as_mut_ptr() as *mut _, nx, type_, batch) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cufftHandle) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftPlan2d(nx: ::core::ffi::c_int, ny: ::core::ffi::c_int, type_: cufftType) -> Result<cufftHandle, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftPlan2d(out_0.as_mut_ptr() as *mut _, nx, ny, type_) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cufftHandle) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftPlan3d(nx: ::core::ffi::c_int, ny: ::core::ffi::c_int, nz: ::core::ffi::c_int, type_: cufftType) -> Result<cufftHandle, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftPlan3d(out_0.as_mut_ptr() as *mut _, nx, ny, nz, type_) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cufftHandle) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftPlanMany(
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
) -> Result<cufftHandle, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftPlanMany(out_0.as_mut_ptr() as *mut _, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cufftHandle) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftMakePlan1d<T0: types::CudaAsMutPtr>(plan: cufftHandle, nx: ::core::ffi::c_int, type_: cufftType, batch: ::core::ffi::c_int, mut workSize: T0) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftMakePlan1d(plan, nx, type_, batch, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftMakePlan2d<T0: types::CudaAsMutPtr>(plan: cufftHandle, nx: ::core::ffi::c_int, ny: ::core::ffi::c_int, type_: cufftType, mut workSize: T0) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftMakePlan2d(plan, nx, ny, type_, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftMakePlan3d<T0: types::CudaAsMutPtr>(plan: cufftHandle, nx: ::core::ffi::c_int, ny: ::core::ffi::c_int, nz: ::core::ffi::c_int, type_: cufftType, mut workSize: T0) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftMakePlan3d(plan, nx, ny, nz, type_, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftMakePlanMany<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    plan: cufftHandle,
    rank: ::core::ffi::c_int,
    mut n: T0,
    mut inembed: T1,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    mut onembed: T2,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
    mut workSize: T3,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftMakePlanMany(plan, rank, n.as_mut_ptr() as *mut _, inembed.as_mut_ptr() as *mut _, istride, idist, onembed.as_mut_ptr() as *mut _, ostride, odist, type_, batch, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftMakePlanMany64<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    plan: cufftHandle,
    rank: ::core::ffi::c_int,
    mut n: T0,
    mut inembed: T1,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    mut onembed: T2,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    type_: cufftType,
    batch: ::core::ffi::c_longlong,
    mut workSize: T3,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftMakePlanMany64(plan, rank, n.as_mut_ptr() as *mut _, inembed.as_mut_ptr() as *mut _, istride, idist, onembed.as_mut_ptr() as *mut _, ostride, odist, type_, batch, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftGetSizeMany64(
    plan: cufftHandle,
    rank: ::core::ffi::c_int,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    type_: cufftType,
    batch: ::core::ffi::c_longlong,
) -> Result<(::core::ffi::c_longlong, ::core::ffi::c_longlong, ::core::ffi::c_longlong, usize), crate::sys::cufftResult> {
    let mut out_2: std::mem::MaybeUninit<::core::ffi::c_longlong> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_longlong> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_longlong> = std::mem::MaybeUninit::zeroed();
    let mut out_11: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftGetSizeMany64(plan, rank, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, istride, idist, out_6.as_mut_ptr() as *mut _, ostride, odist, type_, batch, out_11.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok((out_2.assume_init() as ::core::ffi::c_longlong, out_3.assume_init() as ::core::ffi::c_longlong, out_6.assume_init() as ::core::ffi::c_longlong, out_11.assume_init() as usize)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftEstimate1d<T0: types::CudaAsMutPtr>(nx: ::core::ffi::c_int, type_: cufftType, batch: ::core::ffi::c_int, mut workSize: T0) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftEstimate1d(nx, type_, batch, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftEstimate2d<T0: types::CudaAsMutPtr>(nx: ::core::ffi::c_int, ny: ::core::ffi::c_int, type_: cufftType, mut workSize: T0) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftEstimate2d(nx, ny, type_, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftEstimate3d<T0: types::CudaAsMutPtr>(nx: ::core::ffi::c_int, ny: ::core::ffi::c_int, nz: ::core::ffi::c_int, type_: cufftType, mut workSize: T0) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftEstimate3d(nx, ny, nz, type_, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftEstimateMany<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr>(
    rank: ::core::ffi::c_int,
    mut n: T0,
    mut inembed: T1,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    mut onembed: T2,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
    mut workSize: T3,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftEstimateMany(rank, n.as_mut_ptr() as *mut _, inembed.as_mut_ptr() as *mut _, istride, idist, onembed.as_mut_ptr() as *mut _, ostride, odist, type_, batch, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftCreate() -> Result<cufftHandle, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cufftHandle) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSize1d(handle: cufftHandle, nx: ::core::ffi::c_int, type_: cufftType, batch: ::core::ffi::c_int) -> Result<usize, crate::sys::cufftResult> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftGetSize1d(handle, nx, type_, batch, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSize2d(handle: cufftHandle, nx: ::core::ffi::c_int, ny: ::core::ffi::c_int, type_: cufftType) -> Result<usize, crate::sys::cufftResult> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftGetSize2d(handle, nx, ny, type_, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSize3d(handle: cufftHandle, nx: ::core::ffi::c_int, ny: ::core::ffi::c_int, nz: ::core::ffi::c_int, type_: cufftType) -> Result<usize, crate::sys::cufftResult> {
    let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftGetSize3d(handle, nx, ny, nz, type_, out_5.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_5.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSizeMany(
    handle: cufftHandle,
    rank: ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
) -> Result<(::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, usize), crate::sys::cufftResult> {
    let mut out_2: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_11: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftGetSizeMany(handle, rank, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, istride, idist, out_6.as_mut_ptr() as *mut _, ostride, odist, type_, batch, out_11.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok((out_2.assume_init() as ::core::ffi::c_int, out_3.assume_init() as ::core::ffi::c_int, out_6.assume_init() as ::core::ffi::c_int, out_11.assume_init() as usize)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSize(handle: cufftHandle) -> Result<usize, crate::sys::cufftResult> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftGetSize(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftSetWorkArea<T0: types::CudaAsMutPtr>(plan: cufftHandle, mut workArea: T0) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftSetWorkArea(plan, workArea.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftSetAutoAllocation(plan: cufftHandle, autoAllocate: ::core::ffi::c_int) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftSetAutoAllocation(plan, autoAllocate) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftExecC2C<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(plan: cufftHandle, mut idata: T0, mut odata: T1, direction: ::core::ffi::c_int) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecC2C(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _, direction) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftExecR2C<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(plan: cufftHandle, mut idata: T0, mut odata: T1) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecR2C(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftExecC2R<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(plan: cufftHandle, mut idata: T0, mut odata: T1) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecC2R(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftExecZ2Z<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(plan: cufftHandle, mut idata: T0, mut odata: T1, direction: ::core::ffi::c_int) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecZ2Z(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _, direction) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftExecD2Z<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(plan: cufftHandle, mut idata: T0, mut odata: T1) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecD2Z(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftExecZ2D<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(plan: cufftHandle, mut idata: T0, mut odata: T1) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecZ2D(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftSetStream(plan: cufftHandle, stream: cudaStream_t) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftSetStream(plan, stream) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftDestroy(plan: cufftHandle) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftDestroy(plan) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftGetVersion() -> Result<::core::ffi::c_int, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetProperty(type_: libraryPropertyType) -> Result<::core::ffi::c_int, crate::sys::cufftResult> {
    let mut out_1: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftSetPlanPropertyInt64(plan: cufftHandle, property: cufftProperty, inputValueInt: ::core::ffi::c_longlong) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftSetPlanPropertyInt64(plan, property, inputValueInt) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cufftGetPlanPropertyInt64(plan: cufftHandle, property: cufftProperty) -> Result<::core::ffi::c_longlong, crate::sys::cufftResult> {
    let mut out_2: std::mem::MaybeUninit<::core::ffi::c_longlong> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cufftGetPlanPropertyInt64(plan, property, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as ::core::ffi::c_longlong) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftResetPlanProperty(plan: cufftHandle, property: cufftProperty) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftResetPlanProperty(plan, property) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS { Ok(()) } else { Err(status) }
}
