pub use crate::sys::cublasStatus_t as CudaTargetStatus;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_rt::sys::*;
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
pub struct CublasHandle {
    pub(crate) handle: crate::sys::cublasHandle_t,
}
impl CublasHandle {
    pub unsafe fn cublasGetVersion_v2(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cublasGetVersion_v2(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetWorkspace_v2<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        mut workspace: T,
        workspaceSizeInBytes: usize,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSetWorkspace_v2(
                self.handle,
                workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceSizeInBytes,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetStream_v2(
        &self,
        streamId: cudaStream_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe { crate::sys::cublasSetStream_v2(self.handle, streamId) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetStream_v2(&self) -> Result<cudaStream_t, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cublasGetStream_v2(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetPointerMode_v2(
        &self,
    ) -> Result<cublasPointerMode_t, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cublasPointerMode_t> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasGetPointerMode_v2(self.handle, out_1.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetPointerMode_v2(
        &self,
        mode: cublasPointerMode_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe { crate::sys::cublasSetPointerMode_v2(self.handle, mode) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetAtomicsMode(
        &self,
    ) -> Result<cublasAtomicsMode_t, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cublasAtomicsMode_t> = std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cublasGetAtomicsMode(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetAtomicsMode(
        &self,
        mode: cublasAtomicsMode_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe { crate::sys::cublasSetAtomicsMode(self.handle, mode) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetMathMode(&self) -> Result<cublasMath_t, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cublasMath_t> = std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cublasGetMathMode(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetMathMode(
        &self,
        mode: cublasMath_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe { crate::sys::cublasSetMathMode(self.handle, mode) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetSmCountTarget(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasGetSmCountTarget(self.handle, out_1.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetSmCountTarget(
        &self,
        smCountTarget: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe { crate::sys::cublasSetSmCountTarget(self.handle, smCountTarget) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetEmulationStrategy(
        &self,
    ) -> Result<cublasEmulationStrategy_t, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cublasEmulationStrategy_t> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasGetEmulationStrategy(self.handle, out_1.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetEmulationStrategy(
        &self,
        emulationStrategy: cublasEmulationStrategy_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status =
            unsafe { crate::sys::cublasSetEmulationStrategy(self.handle, emulationStrategy) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetEmulationSpecialValuesSupport(
        &self,
    ) -> Result<cudaEmulationSpecialValuesSupport, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cudaEmulationSpecialValuesSupport> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasGetEmulationSpecialValuesSupport(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetEmulationSpecialValuesSupport(
        &self,
        mask: cudaEmulationSpecialValuesSupport,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status =
            unsafe { crate::sys::cublasSetEmulationSpecialValuesSupport(self.handle, mask) };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetFixedPointEmulationMantissaControl(
        &self,
    ) -> Result<cudaEmulationMantissaControl, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cudaEmulationMantissaControl> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasGetFixedPointEmulationMantissaControl(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetFixedPointEmulationMantissaControl(
        &self,
        mantissaControl: cudaEmulationMantissaControl,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSetFixedPointEmulationMantissaControl(self.handle, mantissaControl)
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetFixedPointEmulationMaxMantissaBitCount(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasGetFixedPointEmulationMaxMantissaBitCount(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetFixedPointEmulationMaxMantissaBitCount(
        &self,
        maxMantissaBitCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSetFixedPointEmulationMaxMantissaBitCount(
                self.handle,
                maxMantissaBitCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetFixedPointEmulationMantissaBitOffset(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasGetFixedPointEmulationMantissaBitOffset(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetFixedPointEmulationMantissaBitOffset(
        &self,
        mantissaBitOffset: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSetFixedPointEmulationMantissaBitOffset(
                self.handle,
                mantissaBitOffset,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGetFixedPointEmulationMantissaBitCountPointer(
        &self,
    ) -> Result<*mut ::std::os::raw::c_int, crate::sys::cublasStatus_t> {
        let mut out_1: std::mem::MaybeUninit<*mut ::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasGetFixedPointEmulationMantissaBitCountPointer(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSetFixedPointEmulationMantissaBitCountPointer<
        T: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut mantissaBitCount: T,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSetFixedPointEmulationMantissaBitCountPointer(
                self.handle,
                mantissaBitCount.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasNrm2Ex<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        mut result: U,
        resultType: cudaDataType,
        executionType: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasNrm2Ex(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultType,
                executionType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasNrm2Ex_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        xType: cudaDataType,
        incx: i64,
        mut result: U,
        resultType: cudaDataType,
        executionType: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasNrm2Ex_64(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultType,
                executionType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSnrm2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSnrm2_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSnrm2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSnrm2_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDnrm2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDnrm2_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDnrm2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDnrm2_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasScnrm2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasScnrm2_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasScnrm2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasScnrm2_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDznrm2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDznrm2_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDznrm2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDznrm2_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDotEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        y: U,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        mut result: V,
        resultType: cudaDataType,
        executionType: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDotEx(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                yType,
                incy,
                result.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultType,
                executionType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDotEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasDotEx_64(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                yType,
                incy,
                result.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultType,
                executionType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDotcEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        y: U,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        mut result: V,
        resultType: cudaDataType,
        executionType: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDotcEx(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                yType,
                incy,
                result.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultType,
                executionType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDotcEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasDotcEx_64(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                yType,
                incy,
                result.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultType,
                executionType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSdot_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        y: U,
        incy: ::std::os::raw::c_int,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSdot_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_const_ptr() as *const f32,
                incy,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSdot_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        y: U,
        incy: i64,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSdot_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_const_ptr() as *const f32,
                incy,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDdot_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        y: U,
        incy: ::std::os::raw::c_int,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDdot_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_const_ptr() as *const f64,
                incy,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDdot_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        y: U,
        incy: i64,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDdot_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_const_ptr() as *const f64,
                incy,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCdotu_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        y: U,
        incy: ::std::os::raw::c_int,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCdotu_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                result.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCdotu_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        y: U,
        incy: i64,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCdotu_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                result.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCdotc_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        y: U,
        incy: ::std::os::raw::c_int,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCdotc_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                result.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCdotc_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        y: U,
        incy: i64,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCdotc_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                result.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdotu_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        y: U,
        incy: ::std::os::raw::c_int,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdotu_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                result.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdotu_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        y: U,
        incy: i64,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdotu_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                result.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdotc_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        y: U,
        incy: ::std::os::raw::c_int,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdotc_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                result.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdotc_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        y: U,
        incy: i64,
        mut result: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdotc_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                result.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasScalEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        alphaType: cudaDataType,
        mut x: U,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        executionType: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasScalEx(
                self.handle,
                n,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                alphaType,
                x.as_mut_ptr() as *mut ::std::os::raw::c_void,
                xType,
                incx,
                executionType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasScalEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        alphaType: cudaDataType,
        mut x: U,
        xType: cudaDataType,
        incx: i64,
        executionType: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasScalEx_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                alphaType,
                x.as_mut_ptr() as *mut ::std::os::raw::c_void,
                xType,
                incx,
                executionType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSscal_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSscal_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSscal_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSscal_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDscal_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDscal_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDscal_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDscal_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCscal_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCscal_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCscal_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCscal_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsscal_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsscal_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsscal_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsscal_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZscal_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZscal_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZscal_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZscal_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdscal_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdscal_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdscal_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdscal_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasAxpyEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        alphaType: cudaDataType,
        x: U,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        mut y: V,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        executiontype: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasAxpyEx(
                self.handle,
                n,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                alphaType,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasAxpyEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasAxpyEx_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                alphaType,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSaxpy_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut y: V,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSaxpy_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSaxpy_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut y: V,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSaxpy_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDaxpy_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut y: V,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDaxpy_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDaxpy_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut y: V,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDaxpy_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCaxpy_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut y: V,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCaxpy_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCaxpy_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut y: V,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCaxpy_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZaxpy_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut y: V,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZaxpy_v2(
                self.handle,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZaxpy_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut y: V,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZaxpy_v2_64(
                self.handle,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCopyEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        mut y: U,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCopyEx(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCopyEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        xType: cudaDataType,
        incx: i64,
        mut y: U,
        yType: cudaDataType,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCopyEx_64(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasScopy_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasScopy_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasScopy_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut y: U,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasScopy_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDcopy_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDcopy_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDcopy_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut y: U,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDcopy_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCcopy_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCcopy_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCcopy_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut y: U,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCcopy_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZcopy_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZcopy_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZcopy_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut y: U,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZcopy_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSswap_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSswap_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSswap_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSswap_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDswap_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDswap_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDswap_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDswap_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCswap_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCswap_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCswap_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCswap_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZswap_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZswap_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZswap_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZswap_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSwapEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        mut y: U,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSwapEx(
                self.handle,
                n,
                x.as_mut_ptr() as *mut ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSwapEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        xType: cudaDataType,
        incx: i64,
        mut y: U,
        yType: cudaDataType,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSwapEx_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIsamax_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIsamax_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIsamax_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIsamax_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIdamax_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIdamax_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIdamax_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIdamax_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIcamax_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIcamax_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIcamax_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIcamax_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIzamax_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIzamax_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIzamax_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIzamax_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIamaxEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIamaxEx(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIamaxEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        xType: cudaDataType,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIamaxEx_64(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIsamin_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIsamin_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIsamin_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIsamin_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIdamin_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIdamin_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIdamin_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIdamin_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIcamin_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIcamin_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIcamin_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIcamin_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIzamin_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIzamin_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIzamin_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIzamin_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIaminEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIaminEx(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasIaminEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        xType: cudaDataType,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasIaminEx_64(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                result.as_mut_ptr() as *mut i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasAsumEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        mut result: U,
        resultType: cudaDataType,
        executiontype: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasAsumEx(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultType,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasAsumEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        xType: cudaDataType,
        incx: i64,
        mut result: U,
        resultType: cudaDataType,
        executiontype: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasAsumEx_64(
                self.handle,
                n,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                xType,
                incx,
                result.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultType,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSasum_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSasum_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSasum_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSasum_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f32,
                incx,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDasum_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDasum_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDasum_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDasum_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const f64,
                incx,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasScasum_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasScasum_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasScasum_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasScasum_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuComplex,
                incx,
                result.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDzasum_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        x: T,
        incx: ::std::os::raw::c_int,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDzasum_v2(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDzasum_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        x: T,
        incx: i64,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDzasum_v2_64(
                self.handle,
                n,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                result.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSrot_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSrot_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
                c.as_const_ptr() as *const f32,
                s.as_const_ptr() as *const f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSrot_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSrot_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
                c.as_const_ptr() as *const f32,
                s.as_const_ptr() as *const f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDrot_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDrot_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
                c.as_const_ptr() as *const f64,
                s.as_const_ptr() as *const f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDrot_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDrot_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
                c.as_const_ptr() as *const f64,
                s.as_const_ptr() as *const f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCrot_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCrot_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
                c.as_const_ptr() as *const f32,
                s.as_const_ptr() as *const cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCrot_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCrot_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
                c.as_const_ptr() as *const f32,
                s.as_const_ptr() as *const cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsrot_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsrot_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
                c.as_const_ptr() as *const f32,
                s.as_const_ptr() as *const f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsrot_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsrot_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
                c.as_const_ptr() as *const f32,
                s.as_const_ptr() as *const f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZrot_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZrot_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
                c.as_const_ptr() as *const f64,
                s.as_const_ptr() as *const cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZrot_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZrot_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
                c.as_const_ptr() as *const f64,
                s.as_const_ptr() as *const cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdrot_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdrot_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
                c.as_const_ptr() as *const f64,
                s.as_const_ptr() as *const f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdrot_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
        c: V,
        s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdrot_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
                c.as_const_ptr() as *const f64,
                s.as_const_ptr() as *const f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasRotEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        mut y: U,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        c: V,
        s: W,
        csType: cudaDataType,
        executiontype: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasRotEx(
                self.handle,
                n,
                x.as_mut_ptr() as *mut ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
                c.as_const_ptr() as *const ::std::os::raw::c_void,
                s.as_const_ptr() as *const ::std::os::raw::c_void,
                csType,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasRotEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasRotEx_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
                c.as_const_ptr() as *const ::std::os::raw::c_void,
                s.as_const_ptr() as *const ::std::os::raw::c_void,
                csType,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSrotg_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut a: T,
        mut b: U,
        mut c: V,
        mut s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSrotg_v2(
                self.handle,
                a.as_mut_ptr() as *mut f32,
                b.as_mut_ptr() as *mut f32,
                c.as_mut_ptr() as *mut f32,
                s.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDrotg_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut a: T,
        mut b: U,
        mut c: V,
        mut s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDrotg_v2(
                self.handle,
                a.as_mut_ptr() as *mut f64,
                b.as_mut_ptr() as *mut f64,
                c.as_mut_ptr() as *mut f64,
                s.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCrotg_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut a: T,
        mut b: U,
        mut c: V,
        mut s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCrotg_v2(
                self.handle,
                a.as_mut_ptr() as *mut cuComplex,
                b.as_mut_ptr() as *mut cuComplex,
                c.as_mut_ptr() as *mut f32,
                s.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZrotg_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut a: T,
        mut b: U,
        mut c: V,
        mut s: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZrotg_v2(
                self.handle,
                a.as_mut_ptr() as *mut cuDoubleComplex,
                b.as_mut_ptr() as *mut cuDoubleComplex,
                c.as_mut_ptr() as *mut f64,
                s.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasRotgEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut a: T,
        mut b: U,
        abType: cudaDataType,
        mut c: V,
        mut s: W,
        csType: cudaDataType,
        executiontype: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasRotgEx(
                self.handle,
                a.as_mut_ptr() as *mut ::std::os::raw::c_void,
                b.as_mut_ptr() as *mut ::std::os::raw::c_void,
                abType,
                c.as_mut_ptr() as *mut ::std::os::raw::c_void,
                s.as_mut_ptr() as *mut ::std::os::raw::c_void,
                csType,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSrotm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
        param: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSrotm_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
                param.as_const_ptr() as *const f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSrotm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
        param: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSrotm_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f32,
                incx,
                y.as_mut_ptr() as *mut f32,
                incy,
                param.as_const_ptr() as *const f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDrotm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        incx: ::std::os::raw::c_int,
        mut y: U,
        incy: ::std::os::raw::c_int,
        param: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDrotm_v2(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
                param.as_const_ptr() as *const f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDrotm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: i64,
        mut x: T,
        incx: i64,
        mut y: U,
        incy: i64,
        param: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDrotm_v2_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut f64,
                incx,
                y.as_mut_ptr() as *mut f64,
                incy,
                param.as_const_ptr() as *const f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasRotmEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut x: T,
        xType: cudaDataType,
        incx: ::std::os::raw::c_int,
        mut y: U,
        yType: cudaDataType,
        incy: ::std::os::raw::c_int,
        param: V,
        paramType: cudaDataType,
        executiontype: cudaDataType,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasRotmEx(
                self.handle,
                n,
                x.as_mut_ptr() as *mut ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
                param.as_const_ptr() as *const ::std::os::raw::c_void,
                paramType,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasRotmEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasRotmEx_64(
                self.handle,
                n,
                x.as_mut_ptr() as *mut ::std::os::raw::c_void,
                xType,
                incx,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                yType,
                incy,
                param.as_const_ptr() as *const ::std::os::raw::c_void,
                paramType,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSrotmg_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut d1: T,
        mut d2: U,
        mut x1: V,
        y1: W,
        mut param: X,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSrotmg_v2(
                self.handle,
                d1.as_mut_ptr() as *mut f32,
                d2.as_mut_ptr() as *mut f32,
                x1.as_mut_ptr() as *mut f32,
                y1.as_const_ptr() as *const f32,
                param.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDrotmg_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut d1: T,
        mut d2: U,
        mut x1: V,
        y1: W,
        mut param: X,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDrotmg_v2(
                self.handle,
                d1.as_mut_ptr() as *mut f64,
                d2.as_mut_ptr() as *mut f64,
                x1.as_mut_ptr() as *mut f64,
                y1.as_const_ptr() as *const f64,
                param.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasRotmgEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                d1.as_mut_ptr() as *mut ::std::os::raw::c_void,
                d1Type,
                d2.as_mut_ptr() as *mut ::std::os::raw::c_void,
                d2Type,
                x1.as_mut_ptr() as *mut ::std::os::raw::c_void,
                x1Type,
                y1.as_const_ptr() as *const ::std::os::raw::c_void,
                y1Type,
                param.as_mut_ptr() as *mut ::std::os::raw::c_void,
                paramType,
                executiontype,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemv_v2(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgemv_v2(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemv_v2(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgemv_v2(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kl: ::std::os::raw::c_int,
        ku: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgbmv_v2(
                self.handle,
                trans,
                m,
                n,
                kl,
                ku,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                kl,
                ku,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kl: ::std::os::raw::c_int,
        ku: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgbmv_v2(
                self.handle,
                trans,
                m,
                n,
                kl,
                ku,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                kl,
                ku,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kl: ::std::os::raw::c_int,
        ku: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgbmv_v2(
                self.handle,
                trans,
                m,
                n,
                kl,
                ku,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                kl,
                ku,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kl: ::std::os::raw::c_int,
        ku: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgbmv_v2(
                self.handle,
                trans,
                m,
                n,
                kl,
                ku,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                kl,
                ku,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStrmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStrmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtrmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtrmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtrmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtrmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtrmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtrmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStbmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        k: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStbmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtbmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        k: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtbmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtbmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        k: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtbmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtbmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        k: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtbmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStpmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStpmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStpmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        AP: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStpmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtpmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtpmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtpmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        AP: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtpmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtpmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtpmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtpmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        AP: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtpmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtpmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtpmv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtpmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        AP: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtpmv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStrsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStrsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtrsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtrsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtrsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtrsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtrsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtrsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStpsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStpsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStpsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        AP: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStpsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtpsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtpsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtpsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        AP: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtpsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtpsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtpsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtpsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        AP: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtpsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtpsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtpsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtpsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        AP: T,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtpsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                AP.as_const_ptr() as *const cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStbsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStbsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStbsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        k: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStbsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_mut_ptr() as *mut f32,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtbsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtbsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtbsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        k: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtbsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_mut_ptr() as *mut f64,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtbsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtbsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtbsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        k: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtbsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_mut_ptr() as *mut cuComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtbsv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut x: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtbsv_v2(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtbsv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        n: i64,
        k: i64,
        A: T,
        lda: i64,
        mut x: U,
        incx: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtbsv_v2_64(
                self.handle,
                uplo,
                trans,
                diag,
                n,
                k,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                incx,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsymv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsymv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsymv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsymv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsymv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsymv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsymv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsymv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsymv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsymv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZsymv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsymv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChemv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasChemv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChemv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhemv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZhemv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhemv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsbmv_v2(
                self.handle,
                uplo,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsbmv_v2(
                self.handle,
                uplo,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasChbmv_v2(
                self.handle,
                uplo,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhbmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZhbmv_v2(
                self.handle,
                uplo,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhbmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSspmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        AP: U,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSspmv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                AP.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSspmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        AP: U,
        x: V,
        incx: i64,
        beta: W,
        mut y: X,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSspmv_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                AP.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDspmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        AP: U,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDspmv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                AP.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDspmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        AP: U,
        x: V,
        incx: i64,
        beta: W,
        mut y: X,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDspmv_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                AP.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChpmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        AP: U,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasChpmv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                AP.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChpmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        AP: U,
        x: V,
        incx: i64,
        beta: W,
        mut y: X,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasChpmv_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                AP.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhpmv_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        AP: U,
        x: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZhpmv_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                AP.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhpmv_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        AP: U,
        x: V,
        incx: i64,
        beta: W,
        mut y: X,
        incy: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZhpmv_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                AP.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSger_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSger_v2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_const_ptr() as *const f32,
                incy,
                A.as_mut_ptr() as *mut f32,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSger_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: i64,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSger_v2_64(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_const_ptr() as *const f32,
                incy,
                A.as_mut_ptr() as *mut f32,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDger_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDger_v2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_const_ptr() as *const f64,
                incy,
                A.as_mut_ptr() as *mut f64,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDger_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: i64,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDger_v2_64(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_const_ptr() as *const f64,
                incy,
                A.as_mut_ptr() as *mut f64,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgeru_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgeru_v2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgeru_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: i64,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgeru_v2_64(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgerc_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgerc_v2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgerc_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: i64,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgerc_v2_64(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgeru_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgeru_v2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgeru_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: i64,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgeru_v2_64(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgerc_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgerc_v2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgerc_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: i64,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgerc_v2_64(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyr_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut A: V,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsyr_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                A.as_mut_ptr() as *mut f32,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyr_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut A: V,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsyr_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                A.as_mut_ptr() as *mut f32,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyr_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut A: V,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsyr_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                A.as_mut_ptr() as *mut f64,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyr_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut A: V,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsyr_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                A.as_mut_ptr() as *mut f64,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyr_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut A: V,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsyr_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyr_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut A: V,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsyr_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyr_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut A: V,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZsyr_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyr_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut A: V,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZsyr_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCher_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut A: V,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCher_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const cuComplex,
                incx,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCher_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut A: V,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCher_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const cuComplex,
                incx,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZher_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut A: V,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZher_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZher_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut A: V,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZher_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSspr_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut AP: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSspr_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                AP.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSspr_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut AP: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSspr_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                AP.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDspr_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut AP: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDspr_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                AP.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDspr_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut AP: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDspr_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                AP.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChpr_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut AP: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasChpr_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const cuComplex,
                incx,
                AP.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChpr_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut AP: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasChpr_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const cuComplex,
                incx,
                AP.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhpr_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        mut AP: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZhpr_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                AP.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhpr_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        mut AP: V,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZhpr_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                AP.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyr2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsyr2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_const_ptr() as *const f32,
                incy,
                A.as_mut_ptr() as *mut f32,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyr2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsyr2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_const_ptr() as *const f32,
                incy,
                A.as_mut_ptr() as *mut f32,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyr2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsyr2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_const_ptr() as *const f64,
                incy,
                A.as_mut_ptr() as *mut f64,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyr2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsyr2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_const_ptr() as *const f64,
                incy,
                A.as_mut_ptr() as *mut f64,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyr2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsyr2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyr2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsyr2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyr2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZsyr2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyr2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZsyr2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCher2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCher2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCher2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCher2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZher2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut A: W,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZher2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZher2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut A: W,
        lda: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZher2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSspr2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut AP: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSspr2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_const_ptr() as *const f32,
                incy,
                AP.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSspr2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut AP: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSspr2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                incx,
                y.as_const_ptr() as *const f32,
                incy,
                AP.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDspr2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut AP: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDspr2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_const_ptr() as *const f64,
                incy,
                AP.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDspr2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut AP: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDspr2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                incx,
                y.as_const_ptr() as *const f64,
                incy,
                AP.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChpr2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut AP: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasChpr2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                AP.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChpr2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut AP: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasChpr2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                incx,
                y.as_const_ptr() as *const cuComplex,
                incy,
                AP.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhpr2_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        alpha: T,
        x: U,
        incx: ::std::os::raw::c_int,
        y: V,
        incy: ::std::os::raw::c_int,
        mut AP: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZhpr2_v2(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                AP.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhpr2_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        alpha: T,
        x: U,
        incx: i64,
        y: V,
        incy: i64,
        mut AP: W,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZhpr2_v2_64(
                self.handle,
                uplo,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                y.as_const_ptr() as *const cuDoubleComplex,
                incy,
                AP.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemvBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        lda: ::std::os::raw::c_int,
        xarray: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        yarray: X,
        incy: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemvBatched(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                Aarray.as_const_ptr() as *const *const f32,
                lda,
                xarray.as_const_ptr() as *const *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                yarray.as_const_ptr() as *const *mut f32,
                incy,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemvBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                Aarray.as_const_ptr() as *const *const f32,
                lda,
                xarray.as_const_ptr() as *const *const f32,
                incx,
                beta.as_const_ptr() as *const f32,
                yarray.as_const_ptr() as *const *mut f32,
                incy,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemvBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        lda: ::std::os::raw::c_int,
        xarray: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        yarray: X,
        incy: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgemvBatched(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                Aarray.as_const_ptr() as *const *const f64,
                lda,
                xarray.as_const_ptr() as *const *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                yarray.as_const_ptr() as *const *mut f64,
                incy,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemvBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                Aarray.as_const_ptr() as *const *const f64,
                lda,
                xarray.as_const_ptr() as *const *const f64,
                incx,
                beta.as_const_ptr() as *const f64,
                yarray.as_const_ptr() as *const *mut f64,
                incy,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemvBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        lda: ::std::os::raw::c_int,
        xarray: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        yarray: X,
        incy: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemvBatched(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                Aarray.as_const_ptr() as *const *const cuComplex,
                lda,
                xarray.as_const_ptr() as *const *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                yarray.as_const_ptr() as *const *mut cuComplex,
                incy,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemvBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                Aarray.as_const_ptr() as *const *const cuComplex,
                lda,
                xarray.as_const_ptr() as *const *const cuComplex,
                incx,
                beta.as_const_ptr() as *const cuComplex,
                yarray.as_const_ptr() as *const *mut cuComplex,
                incy,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemvBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        lda: ::std::os::raw::c_int,
        xarray: V,
        incx: ::std::os::raw::c_int,
        beta: W,
        yarray: X,
        incy: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgemvBatched(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                Aarray.as_const_ptr() as *const *const cuDoubleComplex,
                lda,
                xarray.as_const_ptr() as *const *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                yarray.as_const_ptr() as *const *mut cuDoubleComplex,
                incy,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemvBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                Aarray.as_const_ptr() as *const *const cuDoubleComplex,
                lda,
                xarray.as_const_ptr() as *const *const cuDoubleComplex,
                incx,
                beta.as_const_ptr() as *const cuDoubleComplex,
                yarray.as_const_ptr() as *const *mut cuDoubleComplex,
                incy,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemvStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        x: V,
        incx: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_longlong,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemvStridedBatched(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                strideA,
                x.as_const_ptr() as *const f32,
                incx,
                stridex,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
                stridey,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemvStridedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: T,
        A: U,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        x: V,
        incx: i64,
        stridex: ::std::os::raw::c_longlong,
        beta: W,
        mut y: X,
        incy: i64,
        stridey: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemvStridedBatched_64(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                strideA,
                x.as_const_ptr() as *const f32,
                incx,
                stridex,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                incy,
                stridey,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemvStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        x: V,
        incx: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_longlong,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgemvStridedBatched(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                strideA,
                x.as_const_ptr() as *const f64,
                incx,
                stridex,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
                stridey,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemvStridedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: T,
        A: U,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        x: V,
        incx: i64,
        stridex: ::std::os::raw::c_longlong,
        beta: W,
        mut y: X,
        incy: i64,
        stridey: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgemvStridedBatched_64(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                strideA,
                x.as_const_ptr() as *const f64,
                incx,
                stridex,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                incy,
                stridey,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemvStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        x: V,
        incx: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_longlong,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemvStridedBatched(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                strideA,
                x.as_const_ptr() as *const cuComplex,
                incx,
                stridex,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
                stridey,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemvStridedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: T,
        A: U,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        x: V,
        incx: i64,
        stridex: ::std::os::raw::c_longlong,
        beta: W,
        mut y: X,
        incy: i64,
        stridey: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemvStridedBatched_64(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                strideA,
                x.as_const_ptr() as *const cuComplex,
                incx,
                stridex,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                incy,
                stridey,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemvStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        x: V,
        incx: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_longlong,
        beta: W,
        mut y: X,
        incy: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgemvStridedBatched(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                strideA,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                stridex,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
                stridey,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemvStridedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: i64,
        n: i64,
        alpha: T,
        A: U,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        x: V,
        incx: i64,
        stridex: ::std::os::raw::c_longlong,
        beta: W,
        mut y: X,
        incy: i64,
        stridey: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgemvStridedBatched_64(
                self.handle,
                trans,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                strideA,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                stridex,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                incy,
                stridey,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemm_v2(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgemm_v2(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemm_v2(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm3m<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemm3m(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm3m_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm3mEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        B: V,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemm3mEx(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm3mEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgemm_v2(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemm3m<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgemm3m(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemm3m_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemmEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        B: V,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemmEx(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemmEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGemmEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        B: V,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasGemmEx(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
                computeType,
                algo,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGemmEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
                computeType,
                algo,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemmEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        B: V,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemmEx(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemmEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyrk_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsyrk_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyrk_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasSsyrk_v2_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyrk_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsyrk_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyrk_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasDsyrk_v2_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyrk_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsyrk_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyrk_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasCsyrk_v2_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyrk_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZsyrk_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyrk_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasZsyrk_v2_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyrkEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsyrkEx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyrkEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasCsyrkEx_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyrk3mEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsyrk3mEx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyrk3mEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasCsyrk3mEx_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCherk_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCherk_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const cuComplex,
                lda,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCherk_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasCherk_v2_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const cuComplex,
                lda,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZherk_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZherk_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZherk_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasZherk_v2_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCherkEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCherkEx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCherkEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasCherkEx_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCherk3mEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        beta: V,
        mut C: W,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCherk3mEx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCherk3mEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasCherk3mEx_64(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyr2k_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsyr2k_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyr2k_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyr2k_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsyr2k_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyr2k_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyr2k_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsyr2k_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyr2k_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyr2k_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZsyr2k_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyr2k_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCher2k_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCher2k_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCher2k_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZher2k_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZher2k_v2(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZher2k_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyrkx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsyrkx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsyrkx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyrkx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsyrkx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsyrkx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyrkx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsyrkx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsyrkx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyrkx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZsyrkx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsyrkx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCherkx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCherkx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCherkx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZherkx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZherkx(
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZherkx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                uplo,
                trans,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsymm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSsymm_v2(
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSsymm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsymm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDsymm_v2(
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDsymm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsymm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCsymm_v2(
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCsymm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsymm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZsymm_v2(
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZsymm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChemm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasChemm_v2(
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasChemm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhemm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZhemm_v2(
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZhemm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                side,
                uplo,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrsm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        mut B: V,
        ldb: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStrsm_v2(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_mut_ptr() as *mut f32,
                ldb,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrsm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasStrsm_v2_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_mut_ptr() as *mut f32,
                ldb,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrsm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        mut B: V,
        ldb: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtrsm_v2(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_mut_ptr() as *mut f64,
                ldb,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrsm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasDtrsm_v2_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_mut_ptr() as *mut f64,
                ldb,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrsm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        mut B: V,
        ldb: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtrsm_v2(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_mut_ptr() as *mut cuComplex,
                ldb,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrsm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasCtrsm_v2_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_mut_ptr() as *mut cuComplex,
                ldb,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrsm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        mut B: V,
        ldb: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtrsm_v2(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_mut_ptr() as *mut cuDoubleComplex,
                ldb,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrsm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasZtrsm_v2_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_mut_ptr() as *mut cuDoubleComplex,
                ldb,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrmm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStrmm_v2(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrmm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasStrmm_v2_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrmm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtrmm_v2(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrmm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasDtrmm_v2_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrmm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtrmm_v2(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrmm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasCtrmm_v2_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrmm_v2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        mut C: W,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtrmm_v2(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrmm_v2_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasZtrmm_v2_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemmBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        lda: ::std::os::raw::c_int,
        Barray: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        Carray: X,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemmBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                Aarray.as_const_ptr() as *const *const f32,
                lda,
                Barray.as_const_ptr() as *const *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                Carray.as_const_ptr() as *const *mut f32,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemmBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                Aarray.as_const_ptr() as *const *const f32,
                lda,
                Barray.as_const_ptr() as *const *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                Carray.as_const_ptr() as *const *mut f32,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemmBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        lda: ::std::os::raw::c_int,
        Barray: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        Carray: X,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgemmBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                Aarray.as_const_ptr() as *const *const f64,
                lda,
                Barray.as_const_ptr() as *const *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                Carray.as_const_ptr() as *const *mut f64,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemmBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                Aarray.as_const_ptr() as *const *const f64,
                lda,
                Barray.as_const_ptr() as *const *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                Carray.as_const_ptr() as *const *mut f64,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemmBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        lda: ::std::os::raw::c_int,
        Barray: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        Carray: X,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemmBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                Aarray.as_const_ptr() as *const *const cuComplex,
                lda,
                Barray.as_const_ptr() as *const *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                Carray.as_const_ptr() as *const *mut cuComplex,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemmBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                Aarray.as_const_ptr() as *const *const cuComplex,
                lda,
                Barray.as_const_ptr() as *const *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                Carray.as_const_ptr() as *const *mut cuComplex,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm3mBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        lda: ::std::os::raw::c_int,
        Barray: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        Carray: X,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemm3mBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                Aarray.as_const_ptr() as *const *const cuComplex,
                lda,
                Barray.as_const_ptr() as *const *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                Carray.as_const_ptr() as *const *mut cuComplex,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm3mBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                Aarray.as_const_ptr() as *const *const cuComplex,
                lda,
                Barray.as_const_ptr() as *const *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                Carray.as_const_ptr() as *const *mut cuComplex,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemmBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        lda: ::std::os::raw::c_int,
        Barray: V,
        ldb: ::std::os::raw::c_int,
        beta: W,
        Carray: X,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgemmBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                Aarray.as_const_ptr() as *const *const cuDoubleComplex,
                lda,
                Barray.as_const_ptr() as *const *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                Carray.as_const_ptr() as *const *mut cuDoubleComplex,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemmBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                Aarray.as_const_ptr() as *const *const cuDoubleComplex,
                lda,
                Barray.as_const_ptr() as *const *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                Carray.as_const_ptr() as *const *mut cuDoubleComplex,
                ldc,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemmStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemmStridedBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                strideA,
                B.as_const_ptr() as *const f32,
                ldb,
                strideB,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemmStridedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: T,
        A: U,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemmStridedBatched_64(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                strideA,
                B.as_const_ptr() as *const f32,
                ldb,
                strideB,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemmStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgemmStridedBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                strideA,
                B.as_const_ptr() as *const f64,
                ldb,
                strideB,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemmStridedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: T,
        A: U,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgemmStridedBatched_64(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                strideA,
                B.as_const_ptr() as *const f64,
                ldb,
                strideB,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemmStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemmStridedBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                strideA,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                strideB,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemmStridedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: T,
        A: U,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemmStridedBatched_64(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                strideA,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                strideB,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm3mStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemm3mStridedBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                strideA,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                strideB,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgemm3mStridedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: T,
        A: U,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgemm3mStridedBatched_64(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                strideA,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                strideB,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemmStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgemmStridedBatched(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                strideA,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                strideB,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgemmStridedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: T,
        A: U,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgemmStridedBatched_64(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                strideA,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                strideB,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
                strideC,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGemmBatchedEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        Aarray: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        Barray: V,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: W,
        Carray: X,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasGemmBatchedEx(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                Aarray.as_const_ptr() as *const *const ::std::os::raw::c_void,
                Atype,
                lda,
                Barray.as_const_ptr() as *const *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                Carray.as_const_ptr() as *const *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
                batchCount,
                computeType,
                algo,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGemmBatchedEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                Aarray.as_const_ptr() as *const *const ::std::os::raw::c_void,
                Atype,
                lda,
                Barray.as_const_ptr() as *const *const ::std::os::raw::c_void,
                Btype,
                ldb,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                Carray.as_const_ptr() as *const *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
                batchCount,
                computeType,
                algo,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGemmStridedBatchedEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasGemmStridedBatchedEx(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                strideA,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                strideB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
                strideC,
                batchCount,
                computeType,
                algo,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGemmStridedBatchedEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: i64,
        n: i64,
        k: i64,
        alpha: T,
        A: U,
        Atype: cudaDataType,
        lda: i64,
        strideA: ::std::os::raw::c_longlong,
        B: V,
        Btype: cudaDataType,
        ldb: i64,
        strideB: ::std::os::raw::c_longlong,
        beta: W,
        mut C: X,
        Ctype: cudaDataType,
        ldc: i64,
        strideC: ::std::os::raw::c_longlong,
        batchCount: i64,
        computeType: cublasComputeType_t,
        algo: cublasGemmAlgo_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasGemmStridedBatchedEx_64(
                self.handle,
                transa,
                transb,
                m,
                n,
                k,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Atype,
                lda,
                strideA,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Btype,
                ldb,
                strideB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ctype,
                ldc,
                strideC,
                batchCount,
                computeType,
                algo,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemmGroupedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
        B: ::cuda_libs::types::CudaAsPtr,
        C: ::cuda_libs::types::CudaAsPtr,
        D: ::cuda_libs::types::CudaAsPtr,
        E: ::cuda_libs::types::CudaAsPtr,
        F: ::cuda_libs::types::CudaAsPtr,
        T13: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        group_count: ::std::os::raw::c_int,
        group_size: T13,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgemmGroupedBatched(
                self.handle,
                transa_array.as_const_ptr() as *const cublasOperation_t,
                transb_array.as_const_ptr() as *const cublasOperation_t,
                m_array.as_const_ptr() as *const ::std::os::raw::c_int,
                n_array.as_const_ptr() as *const ::std::os::raw::c_int,
                k_array.as_const_ptr() as *const ::std::os::raw::c_int,
                alpha_array.as_const_ptr() as *const f32,
                Aarray.as_const_ptr() as *const *const f32,
                lda_array.as_const_ptr() as *const ::std::os::raw::c_int,
                Barray.as_const_ptr() as *const *const f32,
                ldb_array.as_const_ptr() as *const ::std::os::raw::c_int,
                beta_array.as_const_ptr() as *const f32,
                Carray.as_const_ptr() as *const *mut f32,
                ldc_array.as_const_ptr() as *const ::std::os::raw::c_int,
                group_count,
                group_size.as_const_ptr() as *const ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgemmGroupedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
        B: ::cuda_libs::types::CudaAsPtr,
        C: ::cuda_libs::types::CudaAsPtr,
        D: ::cuda_libs::types::CudaAsPtr,
        E: ::cuda_libs::types::CudaAsPtr,
        F: ::cuda_libs::types::CudaAsPtr,
        T13: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa_array.as_const_ptr() as *const cublasOperation_t,
                transb_array.as_const_ptr() as *const cublasOperation_t,
                m_array.as_const_ptr() as *const i64,
                n_array.as_const_ptr() as *const i64,
                k_array.as_const_ptr() as *const i64,
                alpha_array.as_const_ptr() as *const f32,
                Aarray.as_const_ptr() as *const *const f32,
                lda_array.as_const_ptr() as *const i64,
                Barray.as_const_ptr() as *const *const f32,
                ldb_array.as_const_ptr() as *const i64,
                beta_array.as_const_ptr() as *const f32,
                Carray.as_const_ptr() as *const *mut f32,
                ldc_array.as_const_ptr() as *const i64,
                group_count,
                group_size.as_const_ptr() as *const i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemmGroupedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
        B: ::cuda_libs::types::CudaAsPtr,
        C: ::cuda_libs::types::CudaAsPtr,
        D: ::cuda_libs::types::CudaAsPtr,
        E: ::cuda_libs::types::CudaAsPtr,
        F: ::cuda_libs::types::CudaAsPtr,
        T13: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        group_count: ::std::os::raw::c_int,
        group_size: T13,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgemmGroupedBatched(
                self.handle,
                transa_array.as_const_ptr() as *const cublasOperation_t,
                transb_array.as_const_ptr() as *const cublasOperation_t,
                m_array.as_const_ptr() as *const ::std::os::raw::c_int,
                n_array.as_const_ptr() as *const ::std::os::raw::c_int,
                k_array.as_const_ptr() as *const ::std::os::raw::c_int,
                alpha_array.as_const_ptr() as *const f64,
                Aarray.as_const_ptr() as *const *const f64,
                lda_array.as_const_ptr() as *const ::std::os::raw::c_int,
                Barray.as_const_ptr() as *const *const f64,
                ldb_array.as_const_ptr() as *const ::std::os::raw::c_int,
                beta_array.as_const_ptr() as *const f64,
                Carray.as_const_ptr() as *const *mut f64,
                ldc_array.as_const_ptr() as *const ::std::os::raw::c_int,
                group_count,
                group_size.as_const_ptr() as *const ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgemmGroupedBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
        B: ::cuda_libs::types::CudaAsPtr,
        C: ::cuda_libs::types::CudaAsPtr,
        D: ::cuda_libs::types::CudaAsPtr,
        E: ::cuda_libs::types::CudaAsPtr,
        F: ::cuda_libs::types::CudaAsPtr,
        T13: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa_array.as_const_ptr() as *const cublasOperation_t,
                transb_array.as_const_ptr() as *const cublasOperation_t,
                m_array.as_const_ptr() as *const i64,
                n_array.as_const_ptr() as *const i64,
                k_array.as_const_ptr() as *const i64,
                alpha_array.as_const_ptr() as *const f64,
                Aarray.as_const_ptr() as *const *const f64,
                lda_array.as_const_ptr() as *const i64,
                Barray.as_const_ptr() as *const *const f64,
                ldb_array.as_const_ptr() as *const i64,
                beta_array.as_const_ptr() as *const f64,
                Carray.as_const_ptr() as *const *mut f64,
                ldc_array.as_const_ptr() as *const i64,
                group_count,
                group_size.as_const_ptr() as *const i64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGemmGroupedBatchedEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
        B: ::cuda_libs::types::CudaAsPtr,
        C: ::cuda_libs::types::CudaAsPtr,
        D: ::cuda_libs::types::CudaAsPtr,
        E: ::cuda_libs::types::CudaAsPtr,
        F: ::cuda_libs::types::CudaAsPtr,
        T13: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        group_count: ::std::os::raw::c_int,
        group_size: T13,
        computeType: cublasComputeType_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasGemmGroupedBatchedEx(
                self.handle,
                transa_array.as_const_ptr() as *const cublasOperation_t,
                transb_array.as_const_ptr() as *const cublasOperation_t,
                m_array.as_const_ptr() as *const ::std::os::raw::c_int,
                n_array.as_const_ptr() as *const ::std::os::raw::c_int,
                k_array.as_const_ptr() as *const ::std::os::raw::c_int,
                alpha_array.as_const_ptr() as *const ::std::os::raw::c_void,
                Aarray.as_const_ptr() as *const *const ::std::os::raw::c_void,
                Atype,
                lda_array.as_const_ptr() as *const ::std::os::raw::c_int,
                Barray.as_const_ptr() as *const *const ::std::os::raw::c_void,
                Btype,
                ldb_array.as_const_ptr() as *const ::std::os::raw::c_int,
                beta_array.as_const_ptr() as *const ::std::os::raw::c_void,
                Carray.as_const_ptr() as *const *mut ::std::os::raw::c_void,
                Ctype,
                ldc_array.as_const_ptr() as *const ::std::os::raw::c_int,
                group_count,
                group_size.as_const_ptr() as *const ::std::os::raw::c_int,
                computeType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasGemmGroupedBatchedEx_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
        B: ::cuda_libs::types::CudaAsPtr,
        C: ::cuda_libs::types::CudaAsPtr,
        D: ::cuda_libs::types::CudaAsPtr,
        E: ::cuda_libs::types::CudaAsPtr,
        F: ::cuda_libs::types::CudaAsPtr,
        T13: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa_array.as_const_ptr() as *const cublasOperation_t,
                transb_array.as_const_ptr() as *const cublasOperation_t,
                m_array.as_const_ptr() as *const i64,
                n_array.as_const_ptr() as *const i64,
                k_array.as_const_ptr() as *const i64,
                alpha_array.as_const_ptr() as *const ::std::os::raw::c_void,
                Aarray.as_const_ptr() as *const *const ::std::os::raw::c_void,
                Atype,
                lda_array.as_const_ptr() as *const i64,
                Barray.as_const_ptr() as *const *const ::std::os::raw::c_void,
                Btype,
                ldb_array.as_const_ptr() as *const i64,
                beta_array.as_const_ptr() as *const ::std::os::raw::c_void,
                Carray.as_const_ptr() as *const *mut ::std::os::raw::c_void,
                Ctype,
                ldc_array.as_const_ptr() as *const i64,
                group_count,
                group_size.as_const_ptr() as *const i64,
                computeType,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgeam<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgeam(
                self.handle,
                transa,
                transb,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                beta.as_const_ptr() as *const f32,
                B.as_const_ptr() as *const f32,
                ldb,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgeam_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                beta.as_const_ptr() as *const f32,
                B.as_const_ptr() as *const f32,
                ldb,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgeam<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgeam(
                self.handle,
                transa,
                transb,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                beta.as_const_ptr() as *const f64,
                B.as_const_ptr() as *const f64,
                ldb,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgeam_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                beta.as_const_ptr() as *const f64,
                B.as_const_ptr() as *const f64,
                ldb,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgeam<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgeam(
                self.handle,
                transa,
                transb,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                beta.as_const_ptr() as *const cuComplex,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgeam_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                beta.as_const_ptr() as *const cuComplex,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgeam<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        beta: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut C: X,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgeam(
                self.handle,
                transa,
                transb,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                beta.as_const_ptr() as *const cuDoubleComplex,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgeam_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
                self.handle,
                transa,
                transb,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                beta.as_const_ptr() as *const cuDoubleComplex,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrsmBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStrsmBatched(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const *const f32,
                lda,
                B.as_const_ptr() as *const *mut f32,
                ldb,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrsmBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasStrsmBatched_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const *const f32,
                lda,
                B.as_const_ptr() as *const *mut f32,
                ldb,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrsmBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtrsmBatched(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const *const f64,
                lda,
                B.as_const_ptr() as *const *mut f64,
                ldb,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrsmBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasDtrsmBatched_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const *const f64,
                lda,
                B.as_const_ptr() as *const *mut f64,
                ldb,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrsmBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtrsmBatched(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const *const cuComplex,
                lda,
                B.as_const_ptr() as *const *mut cuComplex,
                ldb,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrsmBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasCtrsmBatched_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const *const cuComplex,
                lda,
                B.as_const_ptr() as *const *mut cuComplex,
                ldb,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrsmBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        B: V,
        ldb: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtrsmBatched(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const *mut cuDoubleComplex,
                ldb,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrsmBatched_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
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
        let status = unsafe {
            crate::sys::cublasZtrsmBatched_64(
                self.handle,
                side,
                uplo,
                trans,
                diag,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const *mut cuDoubleComplex,
                ldb,
                batchCount,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSdgmm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        x: U,
        incx: ::std::os::raw::c_int,
        mut C: V,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSdgmm(
                self.handle,
                mode,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSdgmm_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cublasSideMode_t,
        m: i64,
        n: i64,
        A: T,
        lda: i64,
        x: U,
        incx: i64,
        mut C: V,
        ldc: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSdgmm_64(
                self.handle,
                mode,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                x.as_const_ptr() as *const f32,
                incx,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDdgmm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        x: U,
        incx: ::std::os::raw::c_int,
        mut C: V,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDdgmm(
                self.handle,
                mode,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDdgmm_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cublasSideMode_t,
        m: i64,
        n: i64,
        A: T,
        lda: i64,
        x: U,
        incx: i64,
        mut C: V,
        ldc: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDdgmm_64(
                self.handle,
                mode,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                x.as_const_ptr() as *const f64,
                incx,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCdgmm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        x: U,
        incx: ::std::os::raw::c_int,
        mut C: V,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCdgmm(
                self.handle,
                mode,
                m,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCdgmm_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cublasSideMode_t,
        m: i64,
        n: i64,
        A: T,
        lda: i64,
        x: U,
        incx: i64,
        mut C: V,
        ldc: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCdgmm_64(
                self.handle,
                mode,
                m,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                x.as_const_ptr() as *const cuComplex,
                incx,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdgmm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        x: U,
        incx: ::std::os::raw::c_int,
        mut C: V,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdgmm(
                self.handle,
                mode,
                m,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZdgmm_64<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cublasSideMode_t,
        m: i64,
        n: i64,
        A: T,
        lda: i64,
        x: U,
        incx: i64,
        mut C: V,
        ldc: i64,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZdgmm_64(
                self.handle,
                mode,
                m,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                x.as_const_ptr() as *const cuDoubleComplex,
                incx,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSmatinvBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        Ainv: U,
        lda_inv: ::std::os::raw::c_int,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSmatinvBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *const f32,
                lda,
                Ainv.as_const_ptr() as *const *mut f32,
                lda_inv,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDmatinvBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        Ainv: U,
        lda_inv: ::std::os::raw::c_int,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDmatinvBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *const f64,
                lda,
                Ainv.as_const_ptr() as *const *mut f64,
                lda_inv,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCmatinvBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        Ainv: U,
        lda_inv: ::std::os::raw::c_int,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCmatinvBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *const cuComplex,
                lda,
                Ainv.as_const_ptr() as *const *mut cuComplex,
                lda_inv,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZmatinvBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        Ainv: U,
        lda_inv: ::std::os::raw::c_int,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZmatinvBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *const cuDoubleComplex,
                lda,
                Ainv.as_const_ptr() as *const *mut cuDoubleComplex,
                lda_inv,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgeqrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        TauArray: U,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgeqrfBatched(
                self.handle,
                m,
                n,
                Aarray.as_const_ptr() as *const *mut f32,
                lda,
                TauArray.as_const_ptr() as *const *mut f32,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgeqrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        TauArray: U,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgeqrfBatched(
                self.handle,
                m,
                n,
                Aarray.as_const_ptr() as *const *mut f64,
                lda,
                TauArray.as_const_ptr() as *const *mut f64,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgeqrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        TauArray: U,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgeqrfBatched(
                self.handle,
                m,
                n,
                Aarray.as_const_ptr() as *const *mut cuComplex,
                lda,
                TauArray.as_const_ptr() as *const *mut cuComplex,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgeqrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        TauArray: U,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgeqrfBatched(
                self.handle,
                m,
                n,
                Aarray.as_const_ptr() as *const *mut cuDoubleComplex,
                lda,
                TauArray.as_const_ptr() as *const *mut cuDoubleComplex,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgelsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        Carray: U,
        ldc: ::std::os::raw::c_int,
        mut info: V,
        mut devInfoArray: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgelsBatched(
                self.handle,
                trans,
                m,
                n,
                nrhs,
                Aarray.as_const_ptr() as *const *mut f32,
                lda,
                Carray.as_const_ptr() as *const *mut f32,
                ldc,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                devInfoArray.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgelsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        Carray: U,
        ldc: ::std::os::raw::c_int,
        mut info: V,
        mut devInfoArray: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgelsBatched(
                self.handle,
                trans,
                m,
                n,
                nrhs,
                Aarray.as_const_ptr() as *const *mut f64,
                lda,
                Carray.as_const_ptr() as *const *mut f64,
                ldc,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                devInfoArray.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgelsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        Carray: U,
        ldc: ::std::os::raw::c_int,
        mut info: V,
        mut devInfoArray: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgelsBatched(
                self.handle,
                trans,
                m,
                n,
                nrhs,
                Aarray.as_const_ptr() as *const *mut cuComplex,
                lda,
                Carray.as_const_ptr() as *const *mut cuComplex,
                ldc,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                devInfoArray.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgelsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        Carray: U,
        ldc: ::std::os::raw::c_int,
        mut info: V,
        mut devInfoArray: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgelsBatched(
                self.handle,
                trans,
                m,
                n,
                nrhs,
                Aarray.as_const_ptr() as *const *mut cuDoubleComplex,
                lda,
                Carray.as_const_ptr() as *const *mut cuDoubleComplex,
                ldc,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                devInfoArray.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStpttr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut A: U,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStpttr(
                self.handle,
                uplo,
                n,
                AP.as_const_ptr() as *const f32,
                A.as_mut_ptr() as *mut f32,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtpttr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut A: U,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtpttr(
                self.handle,
                uplo,
                n,
                AP.as_const_ptr() as *const f64,
                A.as_mut_ptr() as *mut f64,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtpttr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut A: U,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtpttr(
                self.handle,
                uplo,
                n,
                AP.as_const_ptr() as *const cuComplex,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtpttr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        AP: T,
        mut A: U,
        lda: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtpttr(
                self.handle,
                uplo,
                n,
                AP.as_const_ptr() as *const cuDoubleComplex,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasStrttp<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut AP: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasStrttp(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                AP.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDtrttp<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut AP: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDtrttp(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                AP.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCtrttp<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut AP: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCtrttp(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                AP.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZtrttp<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut AP: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZtrttp(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                AP.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgetrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut P: U,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgetrfBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *mut f32,
                lda,
                P.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgetrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut P: U,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgetrfBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *mut f64,
                lda,
                P.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgetrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut P: U,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgetrfBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *mut cuComplex,
                lda,
                P.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgetrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut P: U,
        mut info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgetrfBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *mut cuDoubleComplex,
                lda,
                P.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgetriBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        P: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut info: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgetriBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *const f32,
                lda,
                P.as_const_ptr() as *const ::std::os::raw::c_int,
                C.as_const_ptr() as *const *mut f32,
                ldc,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgetriBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        P: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut info: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgetriBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *const f64,
                lda,
                P.as_const_ptr() as *const ::std::os::raw::c_int,
                C.as_const_ptr() as *const *mut f64,
                ldc,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgetriBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        P: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut info: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgetriBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *const cuComplex,
                lda,
                P.as_const_ptr() as *const ::std::os::raw::c_int,
                C.as_const_ptr() as *const *mut cuComplex,
                ldc,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgetriBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        P: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut info: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgetriBatched(
                self.handle,
                n,
                A.as_const_ptr() as *const *const cuDoubleComplex,
                lda,
                P.as_const_ptr() as *const ::std::os::raw::c_int,
                C.as_const_ptr() as *const *mut cuDoubleComplex,
                ldc,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasSgetrsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        devIpiv: U,
        Barray: V,
        ldb: ::std::os::raw::c_int,
        mut info: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasSgetrsBatched(
                self.handle,
                trans,
                n,
                nrhs,
                Aarray.as_const_ptr() as *const *const f32,
                lda,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                Barray.as_const_ptr() as *const *mut f32,
                ldb,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasDgetrsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        devIpiv: U,
        Barray: V,
        ldb: ::std::os::raw::c_int,
        mut info: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasDgetrsBatched(
                self.handle,
                trans,
                n,
                nrhs,
                Aarray.as_const_ptr() as *const *const f64,
                lda,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                Barray.as_const_ptr() as *const *mut f64,
                ldb,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasCgetrsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        devIpiv: U,
        Barray: V,
        ldb: ::std::os::raw::c_int,
        mut info: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasCgetrsBatched(
                self.handle,
                trans,
                n,
                nrhs,
                Aarray.as_const_ptr() as *const *const cuComplex,
                lda,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                Barray.as_const_ptr() as *const *mut cuComplex,
                ldb,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasZgetrsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        Aarray: T,
        lda: ::std::os::raw::c_int,
        devIpiv: U,
        Barray: V,
        ldb: ::std::os::raw::c_int,
        mut info: W,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasZgetrsBatched(
                self.handle,
                trans,
                n,
                nrhs,
                Aarray.as_const_ptr() as *const *const cuDoubleComplex,
                lda,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                Barray.as_const_ptr() as *const *mut cuDoubleComplex,
                ldb,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cublasUint8gemmBias<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        transc: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        A_bias: ::std::os::raw::c_int,
        lda: ::std::os::raw::c_int,
        B: U,
        B_bias: ::std::os::raw::c_int,
        ldb: ::std::os::raw::c_int,
        mut C: V,
        C_bias: ::std::os::raw::c_int,
        ldc: ::std::os::raw::c_int,
        C_mult: ::std::os::raw::c_int,
        C_shift: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasUint8gemmBias(
                self.handle,
                transa,
                transb,
                transc,
                m,
                n,
                k,
                A.as_const_ptr() as *const ::std::os::raw::c_uchar,
                A_bias,
                lda,
                B.as_const_ptr() as *const ::std::os::raw::c_uchar,
                B_bias,
                ldb,
                C.as_mut_ptr() as *mut ::std::os::raw::c_uchar,
                C_bias,
                ldc,
                C_mult,
                C_shift,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
pub unsafe fn cublasGetProperty(
    type_: libraryPropertyType,
) -> Result<::std::os::raw::c_int, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cublasGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetCudartVersion() -> usize {
    unsafe { crate::sys::cublasGetCudartVersion() }
}
pub unsafe fn cublasGetStatusName(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cublasGetStatusName(status) }
}
pub unsafe fn cublasGetStatusString(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cublasGetStatusString(status) }
}
pub unsafe fn cublasLoggerConfigure<T: ::cuda_libs::types::CudaAsPtr>(
    logIsOn: ::std::os::raw::c_int,
    logToStdOut: ::std::os::raw::c_int,
    logToStdErr: ::std::os::raw::c_int,
    logFileName: T,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLoggerConfigure(
            logIsOn,
            logToStdOut,
            logToStdErr,
            logFileName.as_const_ptr() as *const ::std::os::raw::c_char,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasSetLoggerCallback(
    userCallback: cublasLogCallback,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasSetLoggerCallback(userCallback) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetLoggerCallback() -> Result<cublasLogCallback, crate::sys::cublasStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cublasLogCallback> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cublasGetLoggerCallback(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cublasSetVector<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    n: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    x: T,
    incx: ::std::os::raw::c_int,
    mut devicePtr: U,
    incy: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSetVector(
            n,
            elemSize,
            x.as_const_ptr() as *const ::std::os::raw::c_void,
            incx,
            devicePtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasSetVector_64<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    n: i64,
    elemSize: i64,
    x: T,
    incx: i64,
    mut devicePtr: U,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSetVector_64(
            n,
            elemSize,
            x.as_const_ptr() as *const ::std::os::raw::c_void,
            incx,
            devicePtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetVector<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    n: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    x: T,
    incx: ::std::os::raw::c_int,
    mut y: U,
    incy: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGetVector(
            n,
            elemSize,
            x.as_const_ptr() as *const ::std::os::raw::c_void,
            incx,
            y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetVector_64<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    n: i64,
    elemSize: i64,
    x: T,
    incx: i64,
    mut y: U,
    incy: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGetVector_64(
            n,
            elemSize,
            x.as_const_ptr() as *const ::std::os::raw::c_void,
            incx,
            y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            incy,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasSetMatrix<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    rows: ::std::os::raw::c_int,
    cols: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    A: T,
    lda: ::std::os::raw::c_int,
    mut B: U,
    ldb: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSetMatrix(
            rows,
            cols,
            elemSize,
            A.as_const_ptr() as *const ::std::os::raw::c_void,
            lda,
            B.as_mut_ptr() as *mut ::std::os::raw::c_void,
            ldb,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasSetMatrix_64<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    rows: i64,
    cols: i64,
    elemSize: i64,
    A: T,
    lda: i64,
    mut B: U,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSetMatrix_64(
            rows,
            cols,
            elemSize,
            A.as_const_ptr() as *const ::std::os::raw::c_void,
            lda,
            B.as_mut_ptr() as *mut ::std::os::raw::c_void,
            ldb,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetMatrix<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    rows: ::std::os::raw::c_int,
    cols: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    A: T,
    lda: ::std::os::raw::c_int,
    mut B: U,
    ldb: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGetMatrix(
            rows,
            cols,
            elemSize,
            A.as_const_ptr() as *const ::std::os::raw::c_void,
            lda,
            B.as_mut_ptr() as *mut ::std::os::raw::c_void,
            ldb,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetMatrix_64<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    rows: i64,
    cols: i64,
    elemSize: i64,
    A: T,
    lda: i64,
    mut B: U,
    ldb: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGetMatrix_64(
            rows,
            cols,
            elemSize,
            A.as_const_ptr() as *const ::std::os::raw::c_void,
            lda,
            B.as_mut_ptr() as *mut ::std::os::raw::c_void,
            ldb,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasSetVectorAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    n: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    hostPtr: T,
    incx: ::std::os::raw::c_int,
    mut devicePtr: U,
    incy: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSetVectorAsync(
            n,
            elemSize,
            hostPtr.as_const_ptr() as *const ::std::os::raw::c_void,
            incx,
            devicePtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            incy,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasSetVectorAsync_64<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    n: i64,
    elemSize: i64,
    hostPtr: T,
    incx: i64,
    mut devicePtr: U,
    incy: i64,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSetVectorAsync_64(
            n,
            elemSize,
            hostPtr.as_const_ptr() as *const ::std::os::raw::c_void,
            incx,
            devicePtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            incy,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetVectorAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    n: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    devicePtr: T,
    incx: ::std::os::raw::c_int,
    mut hostPtr: U,
    incy: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGetVectorAsync(
            n,
            elemSize,
            devicePtr.as_const_ptr() as *const ::std::os::raw::c_void,
            incx,
            hostPtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            incy,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetVectorAsync_64<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    n: i64,
    elemSize: i64,
    devicePtr: T,
    incx: i64,
    mut hostPtr: U,
    incy: i64,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGetVectorAsync_64(
            n,
            elemSize,
            devicePtr.as_const_ptr() as *const ::std::os::raw::c_void,
            incx,
            hostPtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            incy,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasSetMatrixAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    rows: ::std::os::raw::c_int,
    cols: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    A: T,
    lda: ::std::os::raw::c_int,
    mut B: U,
    ldb: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSetMatrixAsync(
            rows,
            cols,
            elemSize,
            A.as_const_ptr() as *const ::std::os::raw::c_void,
            lda,
            B.as_mut_ptr() as *mut ::std::os::raw::c_void,
            ldb,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasSetMatrixAsync_64<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    rows: i64,
    cols: i64,
    elemSize: i64,
    A: T,
    lda: i64,
    mut B: U,
    ldb: i64,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasSetMatrixAsync_64(
            rows,
            cols,
            elemSize,
            A.as_const_ptr() as *const ::std::os::raw::c_void,
            lda,
            B.as_mut_ptr() as *mut ::std::os::raw::c_void,
            ldb,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetMatrixAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    rows: ::std::os::raw::c_int,
    cols: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    A: T,
    lda: ::std::os::raw::c_int,
    mut B: U,
    ldb: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGetMatrixAsync(
            rows,
            cols,
            elemSize,
            A.as_const_ptr() as *const ::std::os::raw::c_void,
            lda,
            B.as_mut_ptr() as *mut ::std::os::raw::c_void,
            ldb,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasGetMatrixAsync_64<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    rows: i64,
    cols: i64,
    elemSize: i64,
    A: T,
    lda: i64,
    mut B: U,
    ldb: i64,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasGetMatrixAsync_64(
            rows,
            cols,
            elemSize,
            A.as_const_ptr() as *const ::std::os::raw::c_void,
            lda,
            B.as_mut_ptr() as *mut ::std::os::raw::c_void,
            ldb,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cublasXerbla<T: ::cuda_libs::types::CudaAsPtr>(
    srName: T,
    info: ::std::os::raw::c_int,
) {
    unsafe {
        crate::sys::cublasXerbla(srName.as_const_ptr() as *const ::std::os::raw::c_char, info)
    }
}
impl CublasHandle {
    pub fn new() -> Result<Self, crate::sys::cublasStatus_t> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let status = crate::sys::cublasCreate_v2(&mut handle);
            if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
                Ok(Self { handle })
            } else {
                Err(status)
            }
        }
    }
}
impl Drop for CublasHandle {
    fn drop(&mut self) {
        unsafe {
            crate::sys::cublasDestroy_v2(self.handle);
        }
    }
}
