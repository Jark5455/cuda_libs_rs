pub use crate::sys::cufftResult as CudaTargetStatus;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart::sys::*;
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
pub struct CufftHandle {
    pub(crate) handle: crate::sys::cufftHandle,
}
impl CufftHandle {
    pub unsafe fn cufftMakePlan1d<T: ::cuda_libs_cudart::types::CudaAsPtr>(
        &self,
        nx: ::std::os::raw::c_int,
        type_: cufftType,
        batch: ::std::os::raw::c_int,
        mut workSize: T,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftMakePlan1d(
                self.handle,
                nx,
                type_,
                batch,
                workSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftMakePlan2d<T: ::cuda_libs_cudart::types::CudaAsPtr>(
        &self,
        nx: ::std::os::raw::c_int,
        ny: ::std::os::raw::c_int,
        type_: cufftType,
        mut workSize: T,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftMakePlan2d(
                self.handle,
                nx,
                ny,
                type_,
                workSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftMakePlan3d<T: ::cuda_libs_cudart::types::CudaAsPtr>(
        &self,
        nx: ::std::os::raw::c_int,
        ny: ::std::os::raw::c_int,
        nz: ::std::os::raw::c_int,
        type_: cufftType,
        mut workSize: T,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftMakePlan3d(
                self.handle,
                nx,
                ny,
                nz,
                type_,
                workSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftMakePlanMany<
        T: ::cuda_libs_cudart::types::CudaAsPtr,
        U: ::cuda_libs_cudart::types::CudaAsPtr,
        V: ::cuda_libs_cudart::types::CudaAsPtr,
        W: ::cuda_libs_cudart::types::CudaAsPtr,
    >(
        &self,
        rank: ::std::os::raw::c_int,
        mut n: T,
        mut inembed: U,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        mut onembed: V,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        type_: cufftType,
        batch: ::std::os::raw::c_int,
        mut workSize: W,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftMakePlanMany(
                self.handle,
                rank,
                n.as_mut_ptr() as *mut ::std::os::raw::c_int,
                inembed.as_mut_ptr() as *mut ::std::os::raw::c_int,
                istride,
                idist,
                onembed.as_mut_ptr() as *mut ::std::os::raw::c_int,
                ostride,
                odist,
                type_,
                batch,
                workSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftMakePlanMany64<
        T: ::cuda_libs_cudart::types::CudaAsPtr,
        U: ::cuda_libs_cudart::types::CudaAsPtr,
        V: ::cuda_libs_cudart::types::CudaAsPtr,
        W: ::cuda_libs_cudart::types::CudaAsPtr,
    >(
        &self,
        rank: ::std::os::raw::c_int,
        mut n: T,
        mut inembed: U,
        istride: ::std::os::raw::c_longlong,
        idist: ::std::os::raw::c_longlong,
        mut onembed: V,
        ostride: ::std::os::raw::c_longlong,
        odist: ::std::os::raw::c_longlong,
        type_: cufftType,
        batch: ::std::os::raw::c_longlong,
        mut workSize: W,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftMakePlanMany64(
                self.handle,
                rank,
                n.as_mut_ptr() as *mut ::std::os::raw::c_longlong,
                inembed.as_mut_ptr() as *mut ::std::os::raw::c_longlong,
                istride,
                idist,
                onembed.as_mut_ptr() as *mut ::std::os::raw::c_longlong,
                ostride,
                odist,
                type_,
                batch,
                workSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftGetSizeMany64(
        &self,
        rank: ::std::os::raw::c_int,
        istride: ::std::os::raw::c_longlong,
        idist: ::std::os::raw::c_longlong,
        ostride: ::std::os::raw::c_longlong,
        odist: ::std::os::raw::c_longlong,
        type_: cufftType,
        batch: ::std::os::raw::c_longlong,
    ) -> Result<
        (
            ::std::os::raw::c_longlong,
            ::std::os::raw::c_longlong,
            ::std::os::raw::c_longlong,
            usize,
        ),
        crate::sys::cufftResult,
    > {
        let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_longlong> =
            std::mem::MaybeUninit::uninit();
        let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_longlong> =
            std::mem::MaybeUninit::uninit();
        let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_longlong> =
            std::mem::MaybeUninit::uninit();
        let mut out_11: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cufftGetSizeMany64(
                self.handle,
                rank,
                out_2.as_mut_ptr() as *mut _,
                out_3.as_mut_ptr() as *mut _,
                istride,
                idist,
                out_6.as_mut_ptr() as *mut _,
                ostride,
                odist,
                type_,
                batch,
                out_11.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            unsafe {
                Ok((
                    out_2.assume_init(),
                    out_3.assume_init(),
                    out_6.assume_init(),
                    out_11.assume_init(),
                ))
            }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftGetSize1d(
        &self,
        nx: ::std::os::raw::c_int,
        type_: cufftType,
        batch: ::std::os::raw::c_int,
    ) -> Result<usize, crate::sys::cufftResult> {
        let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cufftGetSize1d(self.handle, nx, type_, batch, out_4.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            unsafe { Ok(out_4.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftGetSize2d(
        &self,
        nx: ::std::os::raw::c_int,
        ny: ::std::os::raw::c_int,
        type_: cufftType,
    ) -> Result<usize, crate::sys::cufftResult> {
        let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cufftGetSize2d(self.handle, nx, ny, type_, out_4.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            unsafe { Ok(out_4.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftGetSize3d(
        &self,
        nx: ::std::os::raw::c_int,
        ny: ::std::os::raw::c_int,
        nz: ::std::os::raw::c_int,
        type_: cufftType,
    ) -> Result<usize, crate::sys::cufftResult> {
        let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cufftGetSize3d(self.handle, nx, ny, nz, type_, out_5.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            unsafe { Ok(out_5.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftGetSizeMany(
        &self,
        rank: ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        type_: cufftType,
        batch: ::std::os::raw::c_int,
    ) -> Result<
        (
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            usize,
        ),
        crate::sys::cufftResult,
    > {
        let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let mut out_11: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cufftGetSizeMany(
                self.handle,
                rank,
                out_2.as_mut_ptr() as *mut _,
                out_3.as_mut_ptr() as *mut _,
                istride,
                idist,
                out_6.as_mut_ptr() as *mut _,
                ostride,
                odist,
                type_,
                batch,
                out_11.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            unsafe {
                Ok((
                    out_2.assume_init(),
                    out_3.assume_init(),
                    out_6.assume_init(),
                    out_11.assume_init(),
                ))
            }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftGetSize(&self) -> Result<usize, crate::sys::cufftResult> {
        let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe { crate::sys::cufftGetSize(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftSetWorkArea<T: ::cuda_libs_cudart::types::CudaAsPtr>(
        &self,
        mut workArea: T,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftSetWorkArea(
                self.handle,
                workArea.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftSetAutoAllocation(
        &self,
        autoAllocate: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe { crate::sys::cufftSetAutoAllocation(self.handle, autoAllocate) };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftExecC2C<
        T: ::cuda_libs_cudart::types::CudaAsPtr,
        U: ::cuda_libs_cudart::types::CudaAsPtr,
    >(
        &self,
        mut idata: T,
        mut odata: U,
        direction: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftExecC2C(
                self.handle,
                idata.as_mut_ptr() as *mut cufftComplex,
                odata.as_mut_ptr() as *mut cufftComplex,
                direction,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftExecR2C<
        T: ::cuda_libs_cudart::types::CudaAsPtr,
        U: ::cuda_libs_cudart::types::CudaAsPtr,
    >(
        &self,
        mut idata: T,
        mut odata: U,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftExecR2C(
                self.handle,
                idata.as_mut_ptr() as *mut cufftReal,
                odata.as_mut_ptr() as *mut cufftComplex,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftExecC2R<
        T: ::cuda_libs_cudart::types::CudaAsPtr,
        U: ::cuda_libs_cudart::types::CudaAsPtr,
    >(
        &self,
        mut idata: T,
        mut odata: U,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftExecC2R(
                self.handle,
                idata.as_mut_ptr() as *mut cufftComplex,
                odata.as_mut_ptr() as *mut cufftReal,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftExecZ2Z<
        T: ::cuda_libs_cudart::types::CudaAsPtr,
        U: ::cuda_libs_cudart::types::CudaAsPtr,
    >(
        &self,
        mut idata: T,
        mut odata: U,
        direction: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftExecZ2Z(
                self.handle,
                idata.as_mut_ptr() as *mut cufftDoubleComplex,
                odata.as_mut_ptr() as *mut cufftDoubleComplex,
                direction,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftExecD2Z<
        T: ::cuda_libs_cudart::types::CudaAsPtr,
        U: ::cuda_libs_cudart::types::CudaAsPtr,
    >(
        &self,
        mut idata: T,
        mut odata: U,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftExecD2Z(
                self.handle,
                idata.as_mut_ptr() as *mut cufftDoubleReal,
                odata.as_mut_ptr() as *mut cufftDoubleComplex,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftExecZ2D<
        T: ::cuda_libs_cudart::types::CudaAsPtr,
        U: ::cuda_libs_cudart::types::CudaAsPtr,
    >(
        &self,
        mut idata: T,
        mut odata: U,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe {
            crate::sys::cufftExecZ2D(
                self.handle,
                idata.as_mut_ptr() as *mut cufftDoubleComplex,
                odata.as_mut_ptr() as *mut cufftDoubleReal,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftSetStream(
        &self,
        stream: cudaStream_t,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe { crate::sys::cufftSetStream(self.handle, stream) };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftSetPlanPropertyInt64(
        &self,
        property: cufftProperty,
        inputValueInt: ::std::os::raw::c_longlong,
    ) -> Result<(), crate::sys::cufftResult> {
        let status =
            unsafe { crate::sys::cufftSetPlanPropertyInt64(self.handle, property, inputValueInt) };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftGetPlanPropertyInt64(
        &self,
        property: cufftProperty,
    ) -> Result<::std::os::raw::c_longlong, crate::sys::cufftResult> {
        let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_longlong> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cufftGetPlanPropertyInt64(
                self.handle,
                property,
                out_2.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            unsafe { Ok(out_2.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cufftResetPlanProperty(
        &self,
        property: cufftProperty,
    ) -> Result<(), crate::sys::cufftResult> {
        let status = unsafe { crate::sys::cufftResetPlanProperty(self.handle, property) };
        if status == crate::sys::cufftResult::CUFFT_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
pub unsafe fn cufftPlan1d<T: ::cuda_libs_cudart::types::CudaAsPtr>(
    mut plan: T,
    nx: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cufftResult> {
    let status =
        unsafe { crate::sys::cufftPlan1d(plan.as_mut_ptr() as *mut cufftHandle, nx, type_, batch) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftPlan2d<T: ::cuda_libs_cudart::types::CudaAsPtr>(
    mut plan: T,
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    type_: cufftType,
) -> Result<(), crate::sys::cufftResult> {
    let status =
        unsafe { crate::sys::cufftPlan2d(plan.as_mut_ptr() as *mut cufftHandle, nx, ny, type_) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftPlan3d<T: ::cuda_libs_cudart::types::CudaAsPtr>(
    mut plan: T,
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    nz: ::std::os::raw::c_int,
    type_: cufftType,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe {
        crate::sys::cufftPlan3d(plan.as_mut_ptr() as *mut cufftHandle, nx, ny, nz, type_)
    };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftPlanMany<
    T: ::cuda_libs_cudart::types::CudaAsPtr,
    U: ::cuda_libs_cudart::types::CudaAsPtr,
    V: ::cuda_libs_cudart::types::CudaAsPtr,
    W: ::cuda_libs_cudart::types::CudaAsPtr,
>(
    mut plan: T,
    rank: ::std::os::raw::c_int,
    mut n: U,
    mut inembed: V,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    mut onembed: W,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe {
        crate::sys::cufftPlanMany(
            plan.as_mut_ptr() as *mut cufftHandle,
            rank,
            n.as_mut_ptr() as *mut ::std::os::raw::c_int,
            inembed.as_mut_ptr() as *mut ::std::os::raw::c_int,
            istride,
            idist,
            onembed.as_mut_ptr() as *mut ::std::os::raw::c_int,
            ostride,
            odist,
            type_,
            batch,
        )
    };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftEstimate1d<T: ::cuda_libs_cudart::types::CudaAsPtr>(
    nx: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    mut workSize: T,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe {
        crate::sys::cufftEstimate1d(nx, type_, batch, workSize.as_mut_ptr() as *mut usize)
    };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftEstimate2d<T: ::cuda_libs_cudart::types::CudaAsPtr>(
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    type_: cufftType,
    mut workSize: T,
) -> Result<(), crate::sys::cufftResult> {
    let status =
        unsafe { crate::sys::cufftEstimate2d(nx, ny, type_, workSize.as_mut_ptr() as *mut usize) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftEstimate3d<T: ::cuda_libs_cudart::types::CudaAsPtr>(
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    nz: ::std::os::raw::c_int,
    type_: cufftType,
    mut workSize: T,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe {
        crate::sys::cufftEstimate3d(nx, ny, nz, type_, workSize.as_mut_ptr() as *mut usize)
    };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftEstimateMany<
    T: ::cuda_libs_cudart::types::CudaAsPtr,
    U: ::cuda_libs_cudart::types::CudaAsPtr,
    V: ::cuda_libs_cudart::types::CudaAsPtr,
    W: ::cuda_libs_cudart::types::CudaAsPtr,
>(
    rank: ::std::os::raw::c_int,
    mut n: T,
    mut inembed: U,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    mut onembed: V,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    mut workSize: W,
) -> Result<(), crate::sys::cufftResult> {
    let status = unsafe {
        crate::sys::cufftEstimateMany(
            rank,
            n.as_mut_ptr() as *mut ::std::os::raw::c_int,
            inembed.as_mut_ptr() as *mut ::std::os::raw::c_int,
            istride,
            idist,
            onembed.as_mut_ptr() as *mut ::std::os::raw::c_int,
            ostride,
            odist,
            type_,
            batch,
            workSize.as_mut_ptr() as *mut usize,
        )
    };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cufftGetVersion() -> Result<::std::os::raw::c_int, crate::sys::cufftResult> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cufftGetProperty(
    type_: libraryPropertyType,
) -> Result<::std::os::raw::c_int, crate::sys::cufftResult> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cufftGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cufftResult::CUFFT_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
impl CufftHandle {
    pub fn new() -> Result<Self, crate::sys::cufftResult> {
        unsafe {
            let mut handle: crate::sys::cufftHandle = 0;
            let status = crate::sys::cufftCreate(&mut handle);
            if status == crate::sys::cufftResult_t::CUFFT_SUCCESS {
                Ok(Self { handle })
            } else {
                Err(status)
            }
        }
    }
}
impl Drop for CufftHandle {
    fn drop(&mut self) {
        unsafe {
            crate::sys::cufftDestroy(self.handle);
        }
    }
}
