pub use crate::sys::cufftResult as CudaTargetStatus;
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
    pub fn cufftPlan1d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: *mut cufftHandle,
                nx: ::std::os::raw::c_int,
                type_: cufftType,
                batch: ::std::os::raw::c_int,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftPlan1d = val;
        self
    }
    pub fn cufftPlan2d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: *mut cufftHandle,
                nx: ::std::os::raw::c_int,
                ny: ::std::os::raw::c_int,
                type_: cufftType,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftPlan2d = val;
        self
    }
    pub fn cufftPlan3d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: *mut cufftHandle,
                nx: ::std::os::raw::c_int,
                ny: ::std::os::raw::c_int,
                nz: ::std::os::raw::c_int,
                type_: cufftType,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftPlan3d = val;
        self
    }
    pub fn cufftPlanMany(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cufftPlanMany = val;
        self
    }
    pub fn cufftMakePlan1d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cufftHandle,
                nx: ::std::os::raw::c_int,
                type_: cufftType,
                batch: ::std::os::raw::c_int,
                workSize: *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftMakePlan1d = val;
        self
    }
    pub fn cufftMakePlan2d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cufftHandle,
                nx: ::std::os::raw::c_int,
                ny: ::std::os::raw::c_int,
                type_: cufftType,
                workSize: *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftMakePlan2d = val;
        self
    }
    pub fn cufftMakePlan3d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cufftHandle,
                nx: ::std::os::raw::c_int,
                ny: ::std::os::raw::c_int,
                nz: ::std::os::raw::c_int,
                type_: cufftType,
                workSize: *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftMakePlan3d = val;
        self
    }
    pub fn cufftMakePlanMany(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cufftMakePlanMany = val;
        self
    }
    pub fn cufftMakePlanMany64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cufftMakePlanMany64 = val;
        self
    }
    pub fn cufftGetSizeMany64(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cufftGetSizeMany64 = val;
        self
    }
    pub fn cufftEstimate1d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                nx: ::std::os::raw::c_int,
                type_: cufftType,
                batch: ::std::os::raw::c_int,
                workSize: *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftEstimate1d = val;
        self
    }
    pub fn cufftEstimate2d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                nx: ::std::os::raw::c_int,
                ny: ::std::os::raw::c_int,
                type_: cufftType,
                workSize: *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftEstimate2d = val;
        self
    }
    pub fn cufftEstimate3d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                nx: ::std::os::raw::c_int,
                ny: ::std::os::raw::c_int,
                nz: ::std::os::raw::c_int,
                type_: cufftType,
                workSize: *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftEstimate3d = val;
        self
    }
    pub fn cufftEstimateMany(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cufftEstimateMany = val;
        self
    }
    pub fn cufftCreate(mut self, val: Option<unsafe extern "C" fn(handle: *mut cufftHandle) -> cufftResult>) -> Self {
        self.cufftCreate = val;
        self
    }
    pub fn cufftGetSize1d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cufftHandle,
                nx: ::std::os::raw::c_int,
                type_: cufftType,
                batch: ::std::os::raw::c_int,
                workSize: *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftGetSize1d = val;
        self
    }
    pub fn cufftGetSize2d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cufftHandle,
                nx: ::std::os::raw::c_int,
                ny: ::std::os::raw::c_int,
                type_: cufftType,
                workSize: *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftGetSize2d = val;
        self
    }
    pub fn cufftGetSize3d(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cufftHandle,
                nx: ::std::os::raw::c_int,
                ny: ::std::os::raw::c_int,
                nz: ::std::os::raw::c_int,
                type_: cufftType,
                workSize: *mut usize,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftGetSize3d = val;
        self
    }
    pub fn cufftGetSizeMany(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cufftGetSizeMany = val;
        self
    }
    pub fn cufftGetSize(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cufftHandle, workSize: *mut usize) -> cufftResult>,
    ) -> Self {
        self.cufftGetSize = val;
        self
    }
    pub fn cufftSetWorkArea(
        mut self,
        val: Option<unsafe extern "C" fn(plan: cufftHandle, workArea: *mut ::std::os::raw::c_void) -> cufftResult>,
    ) -> Self {
        self.cufftSetWorkArea = val;
        self
    }
    pub fn cufftSetAutoAllocation(
        mut self,
        val: Option<unsafe extern "C" fn(plan: cufftHandle, autoAllocate: ::std::os::raw::c_int) -> cufftResult>,
    ) -> Self {
        self.cufftSetAutoAllocation = val;
        self
    }
    pub fn cufftExecC2C(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cufftHandle,
                idata: *mut cufftComplex,
                odata: *mut cufftComplex,
                direction: ::std::os::raw::c_int,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftExecC2C = val;
        self
    }
    pub fn cufftExecR2C(
        mut self,
        val: Option<
            unsafe extern "C" fn(plan: cufftHandle, idata: *mut cufftReal, odata: *mut cufftComplex) -> cufftResult,
        >,
    ) -> Self {
        self.cufftExecR2C = val;
        self
    }
    pub fn cufftExecC2R(
        mut self,
        val: Option<
            unsafe extern "C" fn(plan: cufftHandle, idata: *mut cufftComplex, odata: *mut cufftReal) -> cufftResult,
        >,
    ) -> Self {
        self.cufftExecC2R = val;
        self
    }
    pub fn cufftExecZ2Z(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cufftHandle,
                idata: *mut cufftDoubleComplex,
                odata: *mut cufftDoubleComplex,
                direction: ::std::os::raw::c_int,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftExecZ2Z = val;
        self
    }
    pub fn cufftExecD2Z(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cufftHandle,
                idata: *mut cufftDoubleReal,
                odata: *mut cufftDoubleComplex,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftExecD2Z = val;
        self
    }
    pub fn cufftExecZ2D(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cufftHandle,
                idata: *mut cufftDoubleComplex,
                odata: *mut cufftDoubleReal,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftExecZ2D = val;
        self
    }
    pub fn cufftSetStream(
        mut self,
        val: Option<unsafe extern "C" fn(plan: cufftHandle, stream: cudaStream_t) -> cufftResult>,
    ) -> Self {
        self.cufftSetStream = val;
        self
    }
    pub fn cufftDestroy(mut self, val: Option<unsafe extern "C" fn(plan: cufftHandle) -> cufftResult>) -> Self {
        self.cufftDestroy = val;
        self
    }
    pub fn cufftGetVersion(
        mut self,
        val: Option<unsafe extern "C" fn(version: *mut ::std::os::raw::c_int) -> cufftResult>,
    ) -> Self {
        self.cufftGetVersion = val;
        self
    }
    pub fn cufftGetProperty(
        mut self,
        val: Option<unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cufftResult>,
    ) -> Self {
        self.cufftGetProperty = val;
        self
    }
    pub fn cufftSetPlanPropertyInt64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cufftHandle,
                property: cufftProperty,
                inputValueInt: ::std::os::raw::c_longlong,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftSetPlanPropertyInt64 = val;
        self
    }
    pub fn cufftGetPlanPropertyInt64(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cufftHandle,
                property: cufftProperty,
                returnPtrValue: *mut ::std::os::raw::c_longlong,
            ) -> cufftResult,
        >,
    ) -> Self {
        self.cufftGetPlanPropertyInt64 = val;
        self
    }
    pub fn cufftResetPlanProperty(
        mut self,
        val: Option<unsafe extern "C" fn(plan: cufftHandle, property: cufftProperty) -> cufftResult>,
    ) -> Self {
        self.cufftResetPlanProperty = val;
        self
    }
}
pub unsafe fn cufftPlan1d(nx: i32, type_: cufftType, batch: i32) -> Result<cufftHandle, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftPlan1d(out_0.as_mut_ptr() as *mut _, nx as _, type_, batch as _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cufftHandle) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftPlan2d(nx: i32, ny: i32, type_: cufftType) -> Result<cufftHandle, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftPlan2d(out_0.as_mut_ptr() as *mut _, nx as _, ny as _, type_) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cufftHandle) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftPlan3d(nx: i32, ny: i32, nz: i32, type_: cufftType) -> Result<cufftHandle, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftPlan3d(out_0.as_mut_ptr() as *mut _, nx as _, ny as _, nz as _, type_) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cufftHandle) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftPlanMany(
    rank: i32,
    istride: i32,
    idist: i32,
    ostride: i32,
    odist: i32,
    type_: cufftType,
    batch: i32,
) -> Result<(cufftHandle, i32, i32, i32), crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cufftPlanMany(
            out_0.as_mut_ptr() as *mut _,
            rank as _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            istride as _,
            idist as _,
            out_6.as_mut_ptr() as *mut _,
            ostride as _,
            odist as _,
            type_,
            batch as _,
        )
    };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe {
            Ok((
                out_0.assume_init() as cufftHandle,
                out_2.assume_init() as i32,
                out_3.assume_init() as i32,
                out_6.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftMakePlan1d(
    plan: cufftHandle,
    nx: i32,
    type_: cufftType,
    batch: i32,
) -> Result<usize, crate::sys::cufftResult> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftMakePlan1d(plan, nx as _, type_, batch as _, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftMakePlan2d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    type_: cufftType,
) -> Result<usize, crate::sys::cufftResult> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftMakePlan2d(plan, nx as _, ny as _, type_, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftMakePlan3d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    nz: i32,
    type_: cufftType,
) -> Result<usize, crate::sys::cufftResult> {
    let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cufftMakePlan3d(plan, nx as _, ny as _, nz as _, type_, out_5.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_5.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftMakePlanMany(
    plan: cufftHandle,
    rank: i32,
    istride: i32,
    idist: i32,
    ostride: i32,
    odist: i32,
    type_: cufftType,
    batch: i32,
) -> Result<(i32, i32, i32, usize), crate::sys::cufftResult> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_11: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cufftMakePlanMany(
            plan,
            rank as _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            istride as _,
            idist as _,
            out_6.as_mut_ptr() as *mut _,
            ostride as _,
            odist as _,
            type_,
            batch as _,
            out_11.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as i32,
                out_3.assume_init() as i32,
                out_6.assume_init() as i32,
                out_11.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftMakePlanMany64(
    plan: cufftHandle,
    rank: i32,
    istride: i64,
    idist: i64,
    ostride: i64,
    odist: i64,
    type_: cufftType,
    batch: i64,
) -> Result<(i64, i64, i64, usize), crate::sys::cufftResult> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_longlong> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_longlong> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_longlong> = std::mem::MaybeUninit::uninit();
    let mut out_11: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cufftMakePlanMany64(
            plan,
            rank as _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            istride as _,
            idist as _,
            out_6.as_mut_ptr() as *mut _,
            ostride as _,
            odist as _,
            type_,
            batch as _,
            out_11.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_6.assume_init() as i64,
                out_11.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSizeMany64(
    plan: cufftHandle,
    rank: i32,
    istride: i64,
    idist: i64,
    ostride: i64,
    odist: i64,
    type_: cufftType,
    batch: i64,
) -> Result<(i64, i64, i64, usize), crate::sys::cufftResult> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_longlong> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_longlong> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_longlong> = std::mem::MaybeUninit::uninit();
    let mut out_11: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cufftGetSizeMany64(
            plan,
            rank as _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            istride as _,
            idist as _,
            out_6.as_mut_ptr() as *mut _,
            ostride as _,
            odist as _,
            type_,
            batch as _,
            out_11.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_6.assume_init() as i64,
                out_11.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftEstimate1d<T: types::CudaAsPtr>(
    nx: i32,
    type_: cufftType,
    batch: i32,
    mut workSize: T,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftEstimate1d(nx as _, type_, batch as _, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftEstimate2d<T: types::CudaAsPtr>(
    nx: i32,
    ny: i32,
    type_: cufftType,
    mut workSize: T,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftEstimate2d(nx as _, ny as _, type_, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftEstimate3d<T: types::CudaAsPtr>(
    nx: i32,
    ny: i32,
    nz: i32,
    type_: cufftType,
    mut workSize: T,
) -> Result<(), crate::sys::cufftResult> {
    let status =
        unsafe { crate::sys::cufftEstimate3d(nx as _, ny as _, nz as _, type_, workSize.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftEstimateMany<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    rank: i32,
    mut n: T,
    mut inembed: U,
    istride: i32,
    idist: i32,
    mut onembed: V,
    ostride: i32,
    odist: i32,
    type_: cufftType,
    batch: i32,
    mut workSize: W,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe {
        crate::sys::cufftEstimateMany(
            rank as _,
            n.as_mut_ptr() as *mut _,
            inembed.as_mut_ptr() as *mut _,
            istride as _,
            idist as _,
            onembed.as_mut_ptr() as *mut _,
            ostride as _,
            odist as _,
            type_,
            batch as _,
            workSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftCreate() -> Result<cufftHandle, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<cufftHandle> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cufftHandle) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSize1d(
    handle: cufftHandle,
    nx: i32,
    type_: cufftType,
    batch: i32,
) -> Result<usize, crate::sys::cufftResult> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cufftGetSize1d(handle, nx as _, type_, batch as _, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSize2d(
    handle: cufftHandle,
    nx: i32,
    ny: i32,
    type_: cufftType,
) -> Result<usize, crate::sys::cufftResult> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftGetSize2d(handle, nx as _, ny as _, type_, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSize3d(
    handle: cufftHandle,
    nx: i32,
    ny: i32,
    nz: i32,
    type_: cufftType,
) -> Result<usize, crate::sys::cufftResult> {
    let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cufftGetSize3d(handle, nx as _, ny as _, nz as _, type_, out_5.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_5.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSizeMany(
    handle: cufftHandle,
    rank: i32,
    istride: i32,
    idist: i32,
    ostride: i32,
    odist: i32,
    type_: cufftType,
    batch: i32,
) -> Result<(i32, i32, i32, usize), crate::sys::cufftResult> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_11: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cufftGetSizeMany(
            handle,
            rank as _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            istride as _,
            idist as _,
            out_6.as_mut_ptr() as *mut _,
            ostride as _,
            odist as _,
            type_,
            batch as _,
            out_11.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as i32,
                out_3.assume_init() as i32,
                out_6.assume_init() as i32,
                out_11.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetSize(handle: cufftHandle) -> Result<usize, crate::sys::cufftResult> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftGetSize(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftSetWorkArea<T: types::CudaAsPtr>(
    plan: cufftHandle,
    mut workArea: T,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftSetWorkArea(plan, workArea.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftSetAutoAllocation(plan: cufftHandle, autoAllocate: i32) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftSetAutoAllocation(plan, autoAllocate as _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftExecC2C<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    plan: cufftHandle,
    mut idata: T,
    mut odata: U,
    direction: i32,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe {
        crate::sys::cufftExecC2C(
            plan,
            idata.as_mut_ptr() as *mut _,
            odata.as_mut_ptr() as *mut _,
            direction as _,
        )
    };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftExecR2C<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    plan: cufftHandle,
    mut idata: T,
    mut odata: U,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecR2C(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftExecC2R<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    plan: cufftHandle,
    mut idata: T,
    mut odata: U,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecC2R(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftExecZ2Z<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    plan: cufftHandle,
    mut idata: T,
    mut odata: U,
    direction: i32,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe {
        crate::sys::cufftExecZ2Z(
            plan,
            idata.as_mut_ptr() as *mut _,
            odata.as_mut_ptr() as *mut _,
            direction as _,
        )
    };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftExecD2Z<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    plan: cufftHandle,
    mut idata: T,
    mut odata: U,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecD2Z(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftExecZ2D<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    plan: cufftHandle,
    mut idata: T,
    mut odata: U,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftExecZ2D(plan, idata.as_mut_ptr() as *mut _, odata.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftSetStream(plan: cufftHandle, stream: cudaStream_t) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftSetStream(plan, stream) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftDestroy(plan: cufftHandle) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftDestroy(plan) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftGetVersion() -> Result<i32, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftGetProperty(type_: libraryPropertyType) -> Result<i32, crate::sys::cufftResult> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftSetPlanPropertyInt64(
    plan: cufftHandle,
    property: cufftProperty,
    inputValueInt: i64,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftSetPlanPropertyInt64(plan, property, inputValueInt as _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftGetPlanPropertyInt64(
    plan: cufftHandle,
    property: cufftProperty,
) -> Result<i64, crate::sys::cufftResult> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_longlong> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftGetPlanPropertyInt64(plan, property, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cufftResult::CUFFT_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as i64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cufftResetPlanProperty(
    plan: cufftHandle,
    property: cufftProperty,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe { crate::sys::cufftResetPlanProperty(plan, property) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
