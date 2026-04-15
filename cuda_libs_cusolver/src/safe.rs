pub use crate::sys::cusolverStatus_t as CudaTargetStatus;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_rt::sys::*;
impl crate::sys::_IO_FILE {
    pub fn _flags(mut self, val: ::std::os::raw::c_int) -> Self {
        self._flags = val;
        self
    }
    pub fn _IO_read_ptr(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_read_ptr = val;
        self
    }
    pub fn _IO_read_end(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_read_end = val;
        self
    }
    pub fn _IO_read_base(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_read_base = val;
        self
    }
    pub fn _IO_write_base(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_write_base = val;
        self
    }
    pub fn _IO_write_ptr(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_write_ptr = val;
        self
    }
    pub fn _IO_write_end(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_write_end = val;
        self
    }
    pub fn _IO_buf_base(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_buf_base = val;
        self
    }
    pub fn _IO_buf_end(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_buf_end = val;
        self
    }
    pub fn _IO_save_base(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_save_base = val;
        self
    }
    pub fn _IO_backup_base(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_backup_base = val;
        self
    }
    pub fn _IO_save_end(mut self, val: *mut ::std::os::raw::c_char) -> Self {
        self._IO_save_end = val;
        self
    }
    pub fn _markers(mut self, val: *mut _IO_marker) -> Self {
        self._markers = val;
        self
    }
    pub fn _chain(mut self, val: *mut _IO_FILE) -> Self {
        self._chain = val;
        self
    }
    pub fn _fileno(mut self, val: ::std::os::raw::c_int) -> Self {
        self._fileno = val;
        self
    }
    pub fn _bitfield_align_1(mut self, val: [u32; 0]) -> Self {
        self._bitfield_align_1 = val;
        self
    }
    pub fn _bitfield_1(mut self, val: __BindgenBitfieldUnit<[u8; 3usize]>) -> Self {
        self._bitfield_1 = val;
        self
    }
    pub fn _short_backupbuf(mut self, val: [::std::os::raw::c_char; 1usize]) -> Self {
        self._short_backupbuf = val;
        self
    }
    pub fn _old_offset(mut self, val: __off_t) -> Self {
        self._old_offset = val;
        self
    }
    pub fn _cur_column(mut self, val: ::std::os::raw::c_ushort) -> Self {
        self._cur_column = val;
        self
    }
    pub fn _vtable_offset(mut self, val: ::std::os::raw::c_schar) -> Self {
        self._vtable_offset = val;
        self
    }
    pub fn _shortbuf(mut self, val: [::std::os::raw::c_char; 1usize]) -> Self {
        self._shortbuf = val;
        self
    }
    pub fn _lock(mut self, val: *mut _IO_lock_t) -> Self {
        self._lock = val;
        self
    }
    pub fn _offset(mut self, val: __off64_t) -> Self {
        self._offset = val;
        self
    }
    pub fn _codecvt(mut self, val: *mut _IO_codecvt) -> Self {
        self._codecvt = val;
        self
    }
    pub fn _wide_data(mut self, val: *mut _IO_wide_data) -> Self {
        self._wide_data = val;
        self
    }
    pub fn _freeres_list(mut self, val: *mut _IO_FILE) -> Self {
        self._freeres_list = val;
        self
    }
    pub fn _freeres_buf(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self._freeres_buf = val;
        self
    }
    pub fn _prevchain(mut self, val: *mut *mut _IO_FILE) -> Self {
        self._prevchain = val;
        self
    }
    pub fn _mode(mut self, val: ::std::os::raw::c_int) -> Self {
        self._mode = val;
        self
    }
    pub fn _unused3(mut self, val: ::std::os::raw::c_int) -> Self {
        self._unused3 = val;
        self
    }
    pub fn _total_written(mut self, val: __uint64_t) -> Self {
        self._total_written = val;
        self
    }
    pub fn _unused2(mut self, val: [::std::os::raw::c_char; 8usize]) -> Self {
        self._unused2 = val;
        self
    }
}
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
pub struct CusolverDnHandle {
    pub(crate) handle: crate::sys::cusolverDnHandle_t,
}
impl CusolverDnHandle {
    pub unsafe fn cusolverDnSetStream(
        &self,
        streamId: cudaStream_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe { crate::sys::cusolverDnSetStream(self.handle, streamId) };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnGetStream(&self) -> Result<cudaStream_t, crate::sys::cusolverStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cusolverDnGetStream(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSetDeterministicMode(
        &self,
        mode: cusolverDeterministicMode_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe { crate::sys::cusolverDnSetDeterministicMode(self.handle, mode) };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnGetDeterministicMode(
        &self,
    ) -> Result<cusolverDeterministicMode_t, crate::sys::cusolverStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cusolverDeterministicMode_t> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnGetDeterministicMode(self.handle, out_1.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSetMathMode(
        &self,
        mode: cusolverMathMode_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe { crate::sys::cusolverDnSetMathMode(self.handle, mode) };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnGetMathMode(
        &self,
    ) -> Result<cusolverMathMode_t, crate::sys::cusolverStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cusolverMathMode_t> = std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cusolverDnGetMathMode(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSetEmulationStrategy(
        &self,
        strategy: cudaEmulationStrategy_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe { crate::sys::cusolverDnSetEmulationStrategy(self.handle, strategy) };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnGetEmulationStrategy(
        &self,
    ) -> Result<cudaEmulationStrategy_t, crate::sys::cusolverStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cudaEmulationStrategy_t> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnGetEmulationStrategy(self.handle, out_1.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSetFixedPointEmulationMantissaControl(
        &self,
        control: cudaEmulationMantissaControl_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSetFixedPointEmulationMantissaControl(self.handle, control)
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnGetFixedPointEmulationMantissaControl(
        &self,
    ) -> Result<cudaEmulationMantissaControl_t, crate::sys::cusolverStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cudaEmulationMantissaControl_t> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnGetFixedPointEmulationMantissaControl(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSetFixedPointEmulationMaxMantissaBitCount(
        &self,
        mantissaBitCount: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSetFixedPointEmulationMaxMantissaBitCount(
                self.handle,
                mantissaBitCount,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnGetFixedPointEmulationMaxMantissaBitCount(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cusolverStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnGetFixedPointEmulationMaxMantissaBitCount(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSetFixedPointEmulationMantissaBitOffset(
        &self,
        mantissaBitOffset: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSetFixedPointEmulationMantissaBitOffset(
                self.handle,
                mantissaBitOffset,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnGetFixedPointEmulationMantissaBitOffset(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cusolverStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnGetFixedPointEmulationMantissaBitOffset(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSetEmulationSpecialValuesSupport(
        &self,
        mask: cudaEmulationSpecialValuesSupport_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status =
            unsafe { crate::sys::cusolverDnSetEmulationSpecialValuesSupport(self.handle, mask) };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnGetEmulationSpecialValuesSupport(
        &self,
    ) -> Result<cudaEmulationSpecialValuesSupport_t, crate::sys::cusolverStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cudaEmulationSpecialValuesSupport_t> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnGetEmulationSpecialValuesSupport(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZZgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZZgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZCgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZCgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZKgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZKgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZEgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZEgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZYgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZYgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCCgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCCgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCEgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCEgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCKgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCKgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCYgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCYgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDDgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDDgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDSgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDSgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDHgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDHgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDBgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDBgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDXgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDXgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSSgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSSgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSHgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSHgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSBgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSBgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSXgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        lwork_bytes: usize,
        mut iter: Y,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSXgesv(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZZgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZZgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZCgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZCgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZKgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZKgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZEgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZEgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZYgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZYgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCCgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCCgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCKgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCKgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCEgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCEgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCYgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCYgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDDgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDDgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDSgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDSgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDHgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDHgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDBgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDBgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDXgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDXgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSSgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSSgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSHgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSHgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSBgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSBgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSXgesv_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dipiv: U,
        mut dB: V,
        lddb: cusolver_int_t,
        mut dX: W,
        lddx: cusolver_int_t,
        mut dWorkspace: X,
        mut lwork_bytes: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSXgesv_bufferSize(
                self.handle,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dipiv.as_mut_ptr() as *mut cusolver_int_t,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZZgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZZgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZCgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZCgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZKgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZKgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZEgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZEgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZYgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZYgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCCgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCCgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCKgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCKgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCEgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCEgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCYgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCYgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDDgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDDgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDSgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDSgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDHgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDHgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDBgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDBgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDXgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDXgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSSgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSSgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSHgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSHgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSBgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSBgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSXgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut iter: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSXgels(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                iter.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZZgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZZgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZCgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZCgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZKgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZKgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZEgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZEgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZYgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZYgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuDoubleComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuDoubleComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuDoubleComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCCgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCCgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCKgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCKgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCEgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCEgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCYgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCYgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut cuComplex,
                ldda,
                dB.as_mut_ptr() as *mut cuComplex,
                lddb,
                dX.as_mut_ptr() as *mut cuComplex,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDDgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDDgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDSgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDSgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDHgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDHgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDBgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDBgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDXgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDXgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f64,
                ldda,
                dB.as_mut_ptr() as *mut f64,
                lddb,
                dX.as_mut_ptr() as *mut f64,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSSgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSSgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSHgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSHgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSBgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSBgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSXgels_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        mut lwork_bytes: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSXgels_bufferSize(
                self.handle,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut f32,
                ldda,
                dB.as_mut_ptr() as *mut f32,
                lddb,
                dX.as_mut_ptr() as *mut f32,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnIRSXgesv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        gesv_irs_params: cusolverDnIRSParams_t,
        gesv_irs_infos: cusolverDnIRSInfos_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut niters: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnIRSXgesv(
                self.handle,
                gesv_irs_params,
                gesv_irs_infos,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldda,
                dB.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lddb,
                dX.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                niters.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnIRSXgesv_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        params: cusolverDnIRSParams_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut lwork_bytes: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnIRSXgesv_bufferSize(
                self.handle,
                params,
                n,
                nrhs,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnIRSXgels<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        gels_irs_params: cusolverDnIRSParams_t,
        gels_irs_infos: cusolverDnIRSInfos_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut dA: T,
        ldda: cusolver_int_t,
        mut dB: U,
        lddb: cusolver_int_t,
        mut dX: V,
        lddx: cusolver_int_t,
        mut dWorkspace: W,
        lwork_bytes: usize,
        mut niters: X,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnIRSXgels(
                self.handle,
                gels_irs_params,
                gels_irs_infos,
                m,
                n,
                nrhs,
                dA.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldda,
                dB.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lddb,
                dX.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lddx,
                dWorkspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lwork_bytes,
                niters.as_mut_ptr() as *mut cusolver_int_t,
                d_info.as_mut_ptr() as *mut cusolver_int_t,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnIRSXgels_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        params: cusolverDnIRSParams_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        mut lwork_bytes: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnIRSXgels_bufferSize(
                self.handle,
                params,
                m,
                n,
                nrhs,
                lwork_bytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSpotrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSpotrf_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDpotrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDpotrf_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCpotrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCpotrf_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZpotrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZpotrf_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSpotrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Workspace: U,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSpotrf(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                Workspace.as_mut_ptr() as *mut f32,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDpotrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Workspace: U,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDpotrf(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                Workspace.as_mut_ptr() as *mut f64,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCpotrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Workspace: U,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCpotrf(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                Workspace.as_mut_ptr() as *mut cuComplex,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZpotrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Workspace: U,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZpotrf(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                Workspace.as_mut_ptr() as *mut cuDoubleComplex,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSpotrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSpotrs(
                self.handle,
                uplo,
                n,
                nrhs,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_mut_ptr() as *mut f32,
                ldb,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDpotrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDpotrs(
                self.handle,
                uplo,
                n,
                nrhs,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_mut_ptr() as *mut f64,
                ldb,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCpotrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCpotrs(
                self.handle,
                uplo,
                n,
                nrhs,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_mut_ptr() as *mut cuComplex,
                ldb,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZpotrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZpotrs(
                self.handle,
                uplo,
                n,
                nrhs,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_mut_ptr() as *mut cuDoubleComplex,
                ldb,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSpotrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut Aarray: T,
        lda: ::std::os::raw::c_int,
        mut infoArray: U,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSpotrfBatched(
                self.handle,
                uplo,
                n,
                Aarray.as_mut_ptr() as *mut *mut f32,
                lda,
                infoArray.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDpotrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut Aarray: T,
        lda: ::std::os::raw::c_int,
        mut infoArray: U,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDpotrfBatched(
                self.handle,
                uplo,
                n,
                Aarray.as_mut_ptr() as *mut *mut f64,
                lda,
                infoArray.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCpotrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut Aarray: T,
        lda: ::std::os::raw::c_int,
        mut infoArray: U,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCpotrfBatched(
                self.handle,
                uplo,
                n,
                Aarray.as_mut_ptr() as *mut *mut cuComplex,
                lda,
                infoArray.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZpotrfBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut Aarray: T,
        lda: ::std::os::raw::c_int,
        mut infoArray: U,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZpotrfBatched(
                self.handle,
                uplo,
                n,
                Aarray.as_mut_ptr() as *mut *mut cuDoubleComplex,
                lda,
                infoArray.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSpotrsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut d_info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSpotrsBatched(
                self.handle,
                uplo,
                n,
                nrhs,
                A.as_mut_ptr() as *mut *mut f32,
                lda,
                B.as_mut_ptr() as *mut *mut f32,
                ldb,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDpotrsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut d_info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDpotrsBatched(
                self.handle,
                uplo,
                n,
                nrhs,
                A.as_mut_ptr() as *mut *mut f64,
                lda,
                B.as_mut_ptr() as *mut *mut f64,
                ldb,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCpotrsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut d_info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCpotrsBatched(
                self.handle,
                uplo,
                n,
                nrhs,
                A.as_mut_ptr() as *mut *mut cuComplex,
                lda,
                B.as_mut_ptr() as *mut *mut cuComplex,
                ldb,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZpotrsBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut d_info: V,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZpotrsBatched(
                self.handle,
                uplo,
                n,
                nrhs,
                A.as_mut_ptr() as *mut *mut cuDoubleComplex,
                lda,
                B.as_mut_ptr() as *mut *mut cuDoubleComplex,
                ldb,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSpotri_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSpotri_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDpotri_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDpotri_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCpotri_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCpotri_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZpotri_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZpotri_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSpotri<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut work: U,
        lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSpotri(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                work.as_mut_ptr() as *mut f32,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDpotri<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut work: U,
        lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDpotri(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                work.as_mut_ptr() as *mut f64,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCpotri<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut work: U,
        lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCpotri(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZpotri<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut work: U,
        lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZpotri(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXtrtri_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        diag: cublasDiagType_t,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        mut workspaceInBytesOnDevice: U,
        mut workspaceInBytesOnHost: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXtrtri_bufferSize(
                self.handle,
                uplo,
                diag,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXtrtri<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        diag: cublasDiagType_t,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        mut bufferOnDevice: U,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: V,
        workspaceInBytesOnHost: usize,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXtrtri(
                self.handle,
                uplo,
                diag,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSlauum_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSlauum_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDlauum_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDlauum_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnClauum_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnClauum_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZlauum_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZlauum_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSlauum<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut work: U,
        lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSlauum(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                work.as_mut_ptr() as *mut f32,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDlauum<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut work: U,
        lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDlauum(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                work.as_mut_ptr() as *mut f64,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnClauum<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut work: U,
        lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnClauum(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZlauum<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut work: U,
        lwork: ::std::os::raw::c_int,
        mut devInfo: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZlauum(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgetrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgetrf_bufferSize(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgetrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgetrf_bufferSize(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgetrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgetrf_bufferSize(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgetrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgetrf_bufferSize(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgetrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Workspace: U,
        mut devIpiv: V,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgetrf(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                Workspace.as_mut_ptr() as *mut f32,
                devIpiv.as_mut_ptr() as *mut ::std::os::raw::c_int,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgetrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Workspace: U,
        mut devIpiv: V,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgetrf(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                Workspace.as_mut_ptr() as *mut f64,
                devIpiv.as_mut_ptr() as *mut ::std::os::raw::c_int,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgetrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Workspace: U,
        mut devIpiv: V,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgetrf(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                Workspace.as_mut_ptr() as *mut cuComplex,
                devIpiv.as_mut_ptr() as *mut ::std::os::raw::c_int,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgetrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut Workspace: U,
        mut devIpiv: V,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgetrf(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                Workspace.as_mut_ptr() as *mut cuDoubleComplex,
                devIpiv.as_mut_ptr() as *mut ::std::os::raw::c_int,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSlaswp<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        k1: ::std::os::raw::c_int,
        k2: ::std::os::raw::c_int,
        devIpiv: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSlaswp(
                self.handle,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                k1,
                k2,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                incx,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDlaswp<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        k1: ::std::os::raw::c_int,
        k2: ::std::os::raw::c_int,
        devIpiv: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDlaswp(
                self.handle,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                k1,
                k2,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                incx,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnClaswp<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        k1: ::std::os::raw::c_int,
        k2: ::std::os::raw::c_int,
        devIpiv: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnClaswp(
                self.handle,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                k1,
                k2,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                incx,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZlaswp<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        k1: ::std::os::raw::c_int,
        k2: ::std::os::raw::c_int,
        devIpiv: U,
        incx: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZlaswp(
                self.handle,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                k1,
                k2,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                incx,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgetrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        devIpiv: U,
        mut B: V,
        ldb: ::std::os::raw::c_int,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgetrs(
                self.handle,
                trans,
                n,
                nrhs,
                A.as_const_ptr() as *const f32,
                lda,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                B.as_mut_ptr() as *mut f32,
                ldb,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgetrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        devIpiv: U,
        mut B: V,
        ldb: ::std::os::raw::c_int,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgetrs(
                self.handle,
                trans,
                n,
                nrhs,
                A.as_const_ptr() as *const f64,
                lda,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                B.as_mut_ptr() as *mut f64,
                ldb,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgetrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        devIpiv: U,
        mut B: V,
        ldb: ::std::os::raw::c_int,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgetrs(
                self.handle,
                trans,
                n,
                nrhs,
                A.as_const_ptr() as *const cuComplex,
                lda,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                B.as_mut_ptr() as *mut cuComplex,
                ldb,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgetrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        devIpiv: U,
        mut B: V,
        ldb: ::std::os::raw::c_int,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgetrs(
                self.handle,
                trans,
                n,
                nrhs,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                devIpiv.as_const_ptr() as *const ::std::os::raw::c_int,
                B.as_mut_ptr() as *mut cuDoubleComplex,
                ldb,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgeqrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgeqrf_bufferSize(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgeqrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgeqrf_bufferSize(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgeqrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgeqrf_bufferSize(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgeqrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgeqrf_bufferSize(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgeqrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut TAU: U,
        mut Workspace: V,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgeqrf(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                TAU.as_mut_ptr() as *mut f32,
                Workspace.as_mut_ptr() as *mut f32,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgeqrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut TAU: U,
        mut Workspace: V,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgeqrf(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                TAU.as_mut_ptr() as *mut f64,
                Workspace.as_mut_ptr() as *mut f64,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgeqrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut TAU: U,
        mut Workspace: V,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgeqrf(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                TAU.as_mut_ptr() as *mut cuComplex,
                Workspace.as_mut_ptr() as *mut cuComplex,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgeqrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut TAU: U,
        mut Workspace: V,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgeqrf(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                TAU.as_mut_ptr() as *mut cuDoubleComplex,
                Workspace.as_mut_ptr() as *mut cuDoubleComplex,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSorgqr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSorgqr_bufferSize(
                self.handle,
                m,
                n,
                k,
                A.as_const_ptr() as *const f32,
                lda,
                tau.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDorgqr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDorgqr_bufferSize(
                self.handle,
                m,
                n,
                k,
                A.as_const_ptr() as *const f64,
                lda,
                tau.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCungqr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCungqr_bufferSize(
                self.handle,
                m,
                n,
                k,
                A.as_const_ptr() as *const cuComplex,
                lda,
                tau.as_const_ptr() as *const cuComplex,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZungqr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZungqr_bufferSize(
                self.handle,
                m,
                n,
                k,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                tau.as_const_ptr() as *const cuDoubleComplex,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSorgqr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSorgqr(
                self.handle,
                m,
                n,
                k,
                A.as_mut_ptr() as *mut f32,
                lda,
                tau.as_const_ptr() as *const f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDorgqr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDorgqr(
                self.handle,
                m,
                n,
                k,
                A.as_mut_ptr() as *mut f64,
                lda,
                tau.as_const_ptr() as *const f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCungqr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCungqr(
                self.handle,
                m,
                n,
                k,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                tau.as_const_ptr() as *const cuComplex,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZungqr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZungqr(
                self.handle,
                m,
                n,
                k,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                tau.as_const_ptr() as *const cuDoubleComplex,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSormqr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSormqr_bufferSize(
                self.handle,
                side,
                trans,
                m,
                n,
                k,
                A.as_const_ptr() as *const f32,
                lda,
                tau.as_const_ptr() as *const f32,
                C.as_const_ptr() as *const f32,
                ldc,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDormqr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDormqr_bufferSize(
                self.handle,
                side,
                trans,
                m,
                n,
                k,
                A.as_const_ptr() as *const f64,
                lda,
                tau.as_const_ptr() as *const f64,
                C.as_const_ptr() as *const f64,
                ldc,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCunmqr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCunmqr_bufferSize(
                self.handle,
                side,
                trans,
                m,
                n,
                k,
                A.as_const_ptr() as *const cuComplex,
                lda,
                tau.as_const_ptr() as *const cuComplex,
                C.as_const_ptr() as *const cuComplex,
                ldc,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZunmqr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZunmqr_bufferSize(
                self.handle,
                side,
                trans,
                m,
                n,
                k,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                tau.as_const_ptr() as *const cuDoubleComplex,
                C.as_const_ptr() as *const cuDoubleComplex,
                ldc,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSormqr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut C: V,
        ldc: ::std::os::raw::c_int,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut devInfo: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSormqr(
                self.handle,
                side,
                trans,
                m,
                n,
                k,
                A.as_const_ptr() as *const f32,
                lda,
                tau.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
                work.as_mut_ptr() as *mut f32,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDormqr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut C: V,
        ldc: ::std::os::raw::c_int,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut devInfo: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDormqr(
                self.handle,
                side,
                trans,
                m,
                n,
                k,
                A.as_const_ptr() as *const f64,
                lda,
                tau.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
                work.as_mut_ptr() as *mut f64,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCunmqr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut C: V,
        ldc: ::std::os::raw::c_int,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut devInfo: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCunmqr(
                self.handle,
                side,
                trans,
                m,
                n,
                k,
                A.as_const_ptr() as *const cuComplex,
                lda,
                tau.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZunmqr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut C: V,
        ldc: ::std::os::raw::c_int,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut devInfo: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZunmqr(
                self.handle,
                side,
                trans,
                m,
                n,
                k,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                tau.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsytrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsytrf_bufferSize(
                self.handle,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsytrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsytrf_bufferSize(
                self.handle,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCsytrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCsytrf_bufferSize(
                self.handle,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZsytrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut lwork: U,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZsytrf_bufferSize(
                self.handle,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsytrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut ipiv: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsytrf(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                ipiv.as_mut_ptr() as *mut ::std::os::raw::c_int,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsytrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut ipiv: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsytrf(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                ipiv.as_mut_ptr() as *mut ::std::os::raw::c_int,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCsytrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut ipiv: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCsytrf(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                ipiv.as_mut_ptr() as *mut ::std::os::raw::c_int,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZsytrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut ipiv: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZsytrf(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                ipiv.as_mut_ptr() as *mut ::std::os::raw::c_int,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsytrs_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        nrhs: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        ipiv: U,
        dataTypeB: cudaDataType,
        mut B: V,
        ldb: i64,
        mut workspaceInBytesOnDevice: W,
        mut workspaceInBytesOnHost: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsytrs_bufferSize(
                self.handle,
                uplo,
                n,
                nrhs,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                ipiv.as_const_ptr() as *const i64,
                dataTypeB,
                B.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldb,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsytrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: i64,
        nrhs: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        ipiv: U,
        dataTypeB: cudaDataType,
        mut B: V,
        ldb: i64,
        mut bufferOnDevice: W,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: X,
        workspaceInBytesOnHost: usize,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsytrs(
                self.handle,
                uplo,
                n,
                nrhs,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                ipiv.as_const_ptr() as *const i64,
                dataTypeB,
                B.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldb,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsytri_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        ipiv: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsytri_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                ipiv.as_const_ptr() as *const ::std::os::raw::c_int,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsytri_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        ipiv: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsytri_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                ipiv.as_const_ptr() as *const ::std::os::raw::c_int,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCsytri_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        ipiv: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCsytri_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                ipiv.as_const_ptr() as *const ::std::os::raw::c_int,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZsytri_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        ipiv: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZsytri_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                ipiv.as_const_ptr() as *const ::std::os::raw::c_int,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsytri<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        ipiv: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsytri(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                ipiv.as_const_ptr() as *const ::std::os::raw::c_int,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsytri<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        ipiv: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsytri(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                ipiv.as_const_ptr() as *const ::std::os::raw::c_int,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCsytri<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        ipiv: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCsytri(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                ipiv.as_const_ptr() as *const ::std::os::raw::c_int,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZsytri<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        ipiv: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZsytri(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                ipiv.as_const_ptr() as *const ::std::os::raw::c_int,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgebrd_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut Lwork: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgebrd_bufferSize(
                self.handle,
                m,
                n,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgebrd_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut Lwork: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgebrd_bufferSize(
                self.handle,
                m,
                n,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgebrd_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut Lwork: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgebrd_bufferSize(
                self.handle,
                m,
                n,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgebrd_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut Lwork: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgebrd_bufferSize(
                self.handle,
                m,
                n,
                Lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgebrd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut D: U,
        mut E: V,
        mut TAUQ: W,
        mut TAUP: X,
        mut Work: Y,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgebrd(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                D.as_mut_ptr() as *mut f32,
                E.as_mut_ptr() as *mut f32,
                TAUQ.as_mut_ptr() as *mut f32,
                TAUP.as_mut_ptr() as *mut f32,
                Work.as_mut_ptr() as *mut f32,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgebrd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut D: U,
        mut E: V,
        mut TAUQ: W,
        mut TAUP: X,
        mut Work: Y,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgebrd(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                D.as_mut_ptr() as *mut f64,
                E.as_mut_ptr() as *mut f64,
                TAUQ.as_mut_ptr() as *mut f64,
                TAUP.as_mut_ptr() as *mut f64,
                Work.as_mut_ptr() as *mut f64,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgebrd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut D: U,
        mut E: V,
        mut TAUQ: W,
        mut TAUP: X,
        mut Work: Y,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgebrd(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                D.as_mut_ptr() as *mut f32,
                E.as_mut_ptr() as *mut f32,
                TAUQ.as_mut_ptr() as *mut cuComplex,
                TAUP.as_mut_ptr() as *mut cuComplex,
                Work.as_mut_ptr() as *mut cuComplex,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgebrd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut D: U,
        mut E: V,
        mut TAUQ: W,
        mut TAUP: X,
        mut Work: Y,
        Lwork: ::std::os::raw::c_int,
        mut devInfo: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgebrd(
                self.handle,
                m,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                D.as_mut_ptr() as *mut f64,
                E.as_mut_ptr() as *mut f64,
                TAUQ.as_mut_ptr() as *mut cuDoubleComplex,
                TAUP.as_mut_ptr() as *mut cuDoubleComplex,
                Work.as_mut_ptr() as *mut cuDoubleComplex,
                Lwork,
                devInfo.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSorgbr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSorgbr_bufferSize(
                self.handle,
                side,
                m,
                n,
                k,
                A.as_const_ptr() as *const f32,
                lda,
                tau.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDorgbr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDorgbr_bufferSize(
                self.handle,
                side,
                m,
                n,
                k,
                A.as_const_ptr() as *const f64,
                lda,
                tau.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCungbr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCungbr_bufferSize(
                self.handle,
                side,
                m,
                n,
                k,
                A.as_const_ptr() as *const cuComplex,
                lda,
                tau.as_const_ptr() as *const cuComplex,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZungbr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZungbr_bufferSize(
                self.handle,
                side,
                m,
                n,
                k,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                tau.as_const_ptr() as *const cuDoubleComplex,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSorgbr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSorgbr(
                self.handle,
                side,
                m,
                n,
                k,
                A.as_mut_ptr() as *mut f32,
                lda,
                tau.as_const_ptr() as *const f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDorgbr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDorgbr(
                self.handle,
                side,
                m,
                n,
                k,
                A.as_mut_ptr() as *mut f64,
                lda,
                tau.as_const_ptr() as *const f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCungbr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCungbr(
                self.handle,
                side,
                m,
                n,
                k,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                tau.as_const_ptr() as *const cuComplex,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZungbr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZungbr(
                self.handle,
                side,
                m,
                n,
                k,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                tau.as_const_ptr() as *const cuDoubleComplex,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsytrd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        d: U,
        e: V,
        tau: W,
        mut lwork: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsytrd_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                d.as_const_ptr() as *const f32,
                e.as_const_ptr() as *const f32,
                tau.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsytrd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        d: U,
        e: V,
        tau: W,
        mut lwork: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsytrd_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                d.as_const_ptr() as *const f64,
                e.as_const_ptr() as *const f64,
                tau.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnChetrd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        d: U,
        e: V,
        tau: W,
        mut lwork: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnChetrd_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                d.as_const_ptr() as *const f32,
                e.as_const_ptr() as *const f32,
                tau.as_const_ptr() as *const cuComplex,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZhetrd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        d: U,
        e: V,
        tau: W,
        mut lwork: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZhetrd_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                d.as_const_ptr() as *const f64,
                e.as_const_ptr() as *const f64,
                tau.as_const_ptr() as *const cuDoubleComplex,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsytrd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut d: U,
        mut e: V,
        mut tau: W,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsytrd(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                d.as_mut_ptr() as *mut f32,
                e.as_mut_ptr() as *mut f32,
                tau.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsytrd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut d: U,
        mut e: V,
        mut tau: W,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsytrd(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                d.as_mut_ptr() as *mut f64,
                e.as_mut_ptr() as *mut f64,
                tau.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnChetrd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut d: U,
        mut e: V,
        mut tau: W,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnChetrd(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                d.as_mut_ptr() as *mut f32,
                e.as_mut_ptr() as *mut f32,
                tau.as_mut_ptr() as *mut cuComplex,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZhetrd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut d: U,
        mut e: V,
        mut tau: W,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZhetrd(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                d.as_mut_ptr() as *mut f64,
                e.as_mut_ptr() as *mut f64,
                tau.as_mut_ptr() as *mut cuDoubleComplex,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSorgtr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSorgtr_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                tau.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDorgtr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDorgtr_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                tau.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCungtr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCungtr_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                tau.as_const_ptr() as *const cuComplex,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZungtr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZungtr_bufferSize(
                self.handle,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                tau.as_const_ptr() as *const cuDoubleComplex,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSorgtr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSorgtr(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                tau.as_const_ptr() as *const f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDorgtr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDorgtr(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                tau.as_const_ptr() as *const f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCungtr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCungtr(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                tau.as_const_ptr() as *const cuComplex,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZungtr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZungtr(
                self.handle,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                tau.as_const_ptr() as *const cuDoubleComplex,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSormtr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSormtr_bufferSize(
                self.handle,
                side,
                uplo,
                trans,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                tau.as_const_ptr() as *const f32,
                C.as_const_ptr() as *const f32,
                ldc,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDormtr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDormtr_bufferSize(
                self.handle,
                side,
                uplo,
                trans,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                tau.as_const_ptr() as *const f64,
                C.as_const_ptr() as *const f64,
                ldc,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCunmtr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCunmtr_bufferSize(
                self.handle,
                side,
                uplo,
                trans,
                m,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                tau.as_const_ptr() as *const cuComplex,
                C.as_const_ptr() as *const cuComplex,
                ldc,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZunmtr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        tau: U,
        C: V,
        ldc: ::std::os::raw::c_int,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZunmtr_bufferSize(
                self.handle,
                side,
                uplo,
                trans,
                m,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                tau.as_const_ptr() as *const cuDoubleComplex,
                C.as_const_ptr() as *const cuDoubleComplex,
                ldc,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSormtr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut tau: U,
        mut C: V,
        ldc: ::std::os::raw::c_int,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSormtr(
                self.handle,
                side,
                uplo,
                trans,
                m,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                tau.as_mut_ptr() as *mut f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDormtr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut tau: U,
        mut C: V,
        ldc: ::std::os::raw::c_int,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDormtr(
                self.handle,
                side,
                uplo,
                trans,
                m,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                tau.as_mut_ptr() as *mut f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCunmtr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut tau: U,
        mut C: V,
        ldc: ::std::os::raw::c_int,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCunmtr(
                self.handle,
                side,
                uplo,
                trans,
                m,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                tau.as_mut_ptr() as *mut cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZunmtr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut tau: U,
        mut C: V,
        ldc: ::std::os::raw::c_int,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZunmtr(
                self.handle,
                side,
                uplo,
                trans,
                m,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                tau.as_mut_ptr() as *mut cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgesvd_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut lwork: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgesvd_bufferSize(
                self.handle,
                m,
                n,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgesvd_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut lwork: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgesvd_bufferSize(
                self.handle,
                m,
                n,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgesvd_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut lwork: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgesvd_bufferSize(
                self.handle,
                m,
                n,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgesvd_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut lwork: T,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgesvd_bufferSize(
                self.handle,
                m,
                n,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgesvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut VT: W,
        ldvt: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut rwork: Y,
        mut info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgesvd(
                self.handle,
                jobu,
                jobvt,
                m,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                S.as_mut_ptr() as *mut f32,
                U.as_mut_ptr() as *mut f32,
                ldu,
                VT.as_mut_ptr() as *mut f32,
                ldvt,
                work.as_mut_ptr() as *mut f32,
                lwork,
                rwork.as_mut_ptr() as *mut f32,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgesvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut VT: W,
        ldvt: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut rwork: Y,
        mut info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgesvd(
                self.handle,
                jobu,
                jobvt,
                m,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                S.as_mut_ptr() as *mut f64,
                U.as_mut_ptr() as *mut f64,
                ldu,
                VT.as_mut_ptr() as *mut f64,
                ldvt,
                work.as_mut_ptr() as *mut f64,
                lwork,
                rwork.as_mut_ptr() as *mut f64,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgesvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut VT: W,
        ldvt: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut rwork: Y,
        mut info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgesvd(
                self.handle,
                jobu,
                jobvt,
                m,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                S.as_mut_ptr() as *mut f32,
                U.as_mut_ptr() as *mut cuComplex,
                ldu,
                VT.as_mut_ptr() as *mut cuComplex,
                ldvt,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                rwork.as_mut_ptr() as *mut f32,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgesvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut VT: W,
        ldvt: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut rwork: Y,
        mut info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgesvd(
                self.handle,
                jobu,
                jobvt,
                m,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                S.as_mut_ptr() as *mut f64,
                U.as_mut_ptr() as *mut cuDoubleComplex,
                ldu,
                VT.as_mut_ptr() as *mut cuDoubleComplex,
                ldvt,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                rwork.as_mut_ptr() as *mut f64,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsyevd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsyevd_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsyevd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsyevd_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCheevd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCheevd_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZheevd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZheevd_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsyevd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsyevd(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsyevd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsyevd(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCheevd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCheevd(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZheevd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZheevd(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsyevdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: U,
        W: V,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsyevdx_bufferSize(
                self.handle,
                jobz,
                range,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsyevdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: U,
        W: V,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsyevdx_bufferSize(
                self.handle,
                jobz,
                range,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCheevdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: U,
        W: V,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCheevdx_bufferSize(
                self.handle,
                jobz,
                range,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZheevdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: U,
        W: V,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZheevdx_bufferSize(
                self.handle,
                jobz,
                range,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsyevdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: U,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsyevdx(
                self.handle,
                jobz,
                range,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsyevdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: U,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsyevdx(
                self.handle,
                jobz,
                range,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCheevdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: U,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCheevdx(
                self.handle,
                jobz,
                range,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZheevdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: U,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZheevdx(
                self.handle,
                jobz,
                range,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsygvdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: V,
        W: W,
        mut lwork: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsygvdx_bufferSize(
                self.handle,
                itype,
                jobz,
                range,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsygvdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: V,
        W: W,
        mut lwork: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsygvdx_bufferSize(
                self.handle,
                itype,
                jobz,
                range,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnChegvdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: V,
        W: W,
        mut lwork: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnChegvdx_bufferSize(
                self.handle,
                itype,
                jobz,
                range,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZhegvdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: V,
        W: W,
        mut lwork: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZhegvdx_bufferSize(
                self.handle,
                itype,
                jobz,
                range,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsygvdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: V,
        mut W: W,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsygvdx(
                self.handle,
                itype,
                jobz,
                range,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                B.as_mut_ptr() as *mut f32,
                ldb,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsygvdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: V,
        mut W: W,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsygvdx(
                self.handle,
                itype,
                jobz,
                range,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                B.as_mut_ptr() as *mut f64,
                ldb,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnChegvdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: V,
        mut W: W,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnChegvdx(
                self.handle,
                itype,
                jobz,
                range,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                B.as_mut_ptr() as *mut cuComplex,
                ldb,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZhegvdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        mut meig: V,
        mut W: W,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZhegvdx(
                self.handle,
                itype,
                jobz,
                range,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                B.as_mut_ptr() as *mut cuDoubleComplex,
                ldb,
                vl,
                vu,
                il,
                iu,
                meig.as_mut_ptr() as *mut ::std::os::raw::c_int,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsygvd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        W: V,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsygvd_bufferSize(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsygvd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        W: V,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsygvd_bufferSize(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnChegvd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        W: V,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnChegvd_bufferSize(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZhegvd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        W: V,
        mut lwork: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZhegvd_bufferSize(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsygvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsygvd(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                B.as_mut_ptr() as *mut f32,
                ldb,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsygvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsygvd(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                B.as_mut_ptr() as *mut f64,
                ldb,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnChegvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnChegvd(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                B.as_mut_ptr() as *mut cuComplex,
                ldb,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZhegvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZhegvd(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                B.as_mut_ptr() as *mut cuDoubleComplex,
                ldb,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsygvd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        d_A: T,
        lda: i64,
        dataTypeB: cudaDataType,
        d_B: U,
        ldb: i64,
        dataTypeW: cudaDataType,
        d_W: V,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: W,
        mut workspaceInBytesOnHost: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsygvd_bufferSize(
                self.handle,
                params,
                itype,
                jobz,
                uplo,
                n,
                dataTypeA,
                d_A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeB,
                d_B.as_const_ptr() as *const ::std::os::raw::c_void,
                ldb,
                dataTypeW,
                d_W.as_const_ptr() as *const ::std::os::raw::c_void,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsygvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        mut d_A: T,
        lda: i64,
        dataTypeB: cudaDataType,
        mut d_B: U,
        ldb: i64,
        dataTypeW: cudaDataType,
        mut d_W: V,
        computeType: cudaDataType,
        mut bufferOnDevice: W,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: X,
        workspaceInBytesOnHost: usize,
        mut d_info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsygvd(
                self.handle,
                params,
                itype,
                jobz,
                uplo,
                n,
                dataTypeA,
                d_A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeB,
                d_B.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldb,
                dataTypeW,
                d_W.as_mut_ptr() as *mut ::std::os::raw::c_void,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsygvdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        d_A: T,
        lda: i64,
        dataTypeB: cudaDataType,
        d_B: U,
        ldb: i64,
        mut vl: V,
        mut vu: W,
        il: i64,
        iu: i64,
        mut meig: X,
        dataTypeW: cudaDataType,
        d_W: Y,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: Z,
        mut workspaceInBytesOnHost: A,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsygvdx_bufferSize(
                self.handle,
                params,
                itype,
                jobz,
                uplo,
                n,
                dataTypeA,
                d_A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeB,
                d_B.as_const_ptr() as *const ::std::os::raw::c_void,
                ldb,
                vl.as_mut_ptr() as *mut ::std::os::raw::c_void,
                vu.as_mut_ptr() as *mut ::std::os::raw::c_void,
                il,
                iu,
                meig.as_mut_ptr() as *mut i64,
                dataTypeW,
                d_W.as_const_ptr() as *const ::std::os::raw::c_void,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsygvdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
        B: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        mut d_A: T,
        lda: i64,
        dataTypeB: cudaDataType,
        mut d_B: U,
        ldb: i64,
        mut vl: V,
        mut vu: W,
        il: i64,
        iu: i64,
        mut meig: X,
        dataTypeW: cudaDataType,
        mut d_W: Y,
        computeType: cudaDataType,
        mut bufferOnDevice: Z,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: A,
        workspaceInBytesOnHost: usize,
        mut d_info: B,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsygvdx(
                self.handle,
                params,
                itype,
                jobz,
                range,
                uplo,
                n,
                dataTypeA,
                d_A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeB,
                d_B.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldb,
                vl.as_mut_ptr() as *mut ::std::os::raw::c_void,
                vu.as_mut_ptr() as *mut ::std::os::raw::c_void,
                il,
                iu,
                meig.as_mut_ptr() as *mut i64,
                dataTypeW,
                d_W.as_mut_ptr() as *mut ::std::os::raw::c_void,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsyevjGetResidual(
        &self,
        info: syevjInfo_t,
    ) -> Result<f64, crate::sys::cusolverStatus_t> {
        let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnXsyevjGetResidual(self.handle, info, out_2.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_2.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsyevjGetSweeps(
        &self,
        info: syevjInfo_t,
    ) -> Result<::std::os::raw::c_int, crate::sys::cusolverStatus_t> {
        let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnXsyevjGetSweeps(self.handle, info, out_2.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_2.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsyevjBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsyevjBatched_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsyevjBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsyevjBatched_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCheevjBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCheevjBatched_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZheevjBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZheevjBatched_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsyevjBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsyevjBatched(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsyevjBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsyevjBatched(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCheevjBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCheevjBatched(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZheevjBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZheevjBatched(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsyevj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsyevj_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsyevj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsyevj_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCheevj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCheevj_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZheevj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        W: U,
        mut lwork: V,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZheevj_bufferSize(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsyevj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsyevj(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsyevj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsyevj(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCheevj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCheevj(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZheevj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut W: U,
        mut work: V,
        lwork: ::std::os::raw::c_int,
        mut info: W,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZheevj(
                self.handle,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsygvj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        W: V,
        mut lwork: W,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsygvj_bufferSize(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                B.as_const_ptr() as *const f32,
                ldb,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsygvj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        W: V,
        mut lwork: W,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsygvj_bufferSize(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                B.as_const_ptr() as *const f64,
                ldb,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnChegvj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        W: V,
        mut lwork: W,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnChegvj_bufferSize(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                W.as_const_ptr() as *const f32,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZhegvj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        B: U,
        ldb: ::std::os::raw::c_int,
        W: V,
        mut lwork: W,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZhegvj_bufferSize(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                W.as_const_ptr() as *const f64,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSsygvj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSsygvj(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                B.as_mut_ptr() as *mut f32,
                ldb,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDsygvj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDsygvj(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                B.as_mut_ptr() as *mut f64,
                ldb,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnChegvj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnChegvj(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                B.as_mut_ptr() as *mut cuComplex,
                ldb,
                W.as_mut_ptr() as *mut f32,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZhegvj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut B: U,
        ldb: ::std::os::raw::c_int,
        mut W: V,
        mut work: W,
        lwork: ::std::os::raw::c_int,
        mut info: X,
        params: syevjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZhegvj(
                self.handle,
                itype,
                jobz,
                uplo,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                B.as_mut_ptr() as *mut cuDoubleComplex,
                ldb,
                W.as_mut_ptr() as *mut f64,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgesvdjGetResidual(
        &self,
        info: gesvdjInfo_t,
    ) -> Result<f64, crate::sys::cusolverStatus_t> {
        let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnXgesvdjGetResidual(
                self.handle,
                info,
                out_2.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_2.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgesvdjGetSweeps(
        &self,
        info: gesvdjInfo_t,
    ) -> Result<::std::os::raw::c_int, crate::sys::cusolverStatus_t> {
        let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusolverDnXgesvdjGetSweeps(self.handle, info, out_2.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            unsafe { Ok(out_2.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgesvdjBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        S: U,
        U: V,
        ldu: ::std::os::raw::c_int,
        V: W,
        ldv: ::std::os::raw::c_int,
        mut lwork: X,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgesvdjBatched_bufferSize(
                self.handle,
                jobz,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                S.as_const_ptr() as *const f32,
                U.as_const_ptr() as *const f32,
                ldu,
                V.as_const_ptr() as *const f32,
                ldv,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgesvdjBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        S: U,
        U: V,
        ldu: ::std::os::raw::c_int,
        V: W,
        ldv: ::std::os::raw::c_int,
        mut lwork: X,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgesvdjBatched_bufferSize(
                self.handle,
                jobz,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                S.as_const_ptr() as *const f64,
                U.as_const_ptr() as *const f64,
                ldu,
                V.as_const_ptr() as *const f64,
                ldv,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgesvdjBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        S: U,
        U: V,
        ldu: ::std::os::raw::c_int,
        V: W,
        ldv: ::std::os::raw::c_int,
        mut lwork: X,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgesvdjBatched_bufferSize(
                self.handle,
                jobz,
                m,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                S.as_const_ptr() as *const f32,
                U.as_const_ptr() as *const cuComplex,
                ldu,
                V.as_const_ptr() as *const cuComplex,
                ldv,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgesvdjBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        S: U,
        U: V,
        ldu: ::std::os::raw::c_int,
        V: W,
        ldv: ::std::os::raw::c_int,
        mut lwork: X,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgesvdjBatched_bufferSize(
                self.handle,
                jobz,
                m,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                S.as_const_ptr() as *const f64,
                U.as_const_ptr() as *const cuDoubleComplex,
                ldu,
                V.as_const_ptr() as *const cuDoubleComplex,
                ldv,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgesvdjBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut V: W,
        ldv: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgesvdjBatched(
                self.handle,
                jobz,
                m,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                S.as_mut_ptr() as *mut f32,
                U.as_mut_ptr() as *mut f32,
                ldu,
                V.as_mut_ptr() as *mut f32,
                ldv,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgesvdjBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut V: W,
        ldv: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgesvdjBatched(
                self.handle,
                jobz,
                m,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                S.as_mut_ptr() as *mut f64,
                U.as_mut_ptr() as *mut f64,
                ldu,
                V.as_mut_ptr() as *mut f64,
                ldv,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgesvdjBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut V: W,
        ldv: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgesvdjBatched(
                self.handle,
                jobz,
                m,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                S.as_mut_ptr() as *mut f32,
                U.as_mut_ptr() as *mut cuComplex,
                ldu,
                V.as_mut_ptr() as *mut cuComplex,
                ldv,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgesvdjBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut V: W,
        ldv: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgesvdjBatched(
                self.handle,
                jobz,
                m,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                S.as_mut_ptr() as *mut f64,
                U.as_mut_ptr() as *mut cuDoubleComplex,
                ldu,
                V.as_mut_ptr() as *mut cuDoubleComplex,
                ldv,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgesvdj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        S: U,
        U: V,
        ldu: ::std::os::raw::c_int,
        V: W,
        ldv: ::std::os::raw::c_int,
        mut lwork: X,
        params: gesvdjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgesvdj_bufferSize(
                self.handle,
                jobz,
                econ,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                S.as_const_ptr() as *const f32,
                U.as_const_ptr() as *const f32,
                ldu,
                V.as_const_ptr() as *const f32,
                ldv,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgesvdj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        S: U,
        U: V,
        ldu: ::std::os::raw::c_int,
        V: W,
        ldv: ::std::os::raw::c_int,
        mut lwork: X,
        params: gesvdjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgesvdj_bufferSize(
                self.handle,
                jobz,
                econ,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                S.as_const_ptr() as *const f64,
                U.as_const_ptr() as *const f64,
                ldu,
                V.as_const_ptr() as *const f64,
                ldv,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgesvdj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        S: U,
        U: V,
        ldu: ::std::os::raw::c_int,
        V: W,
        ldv: ::std::os::raw::c_int,
        mut lwork: X,
        params: gesvdjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgesvdj_bufferSize(
                self.handle,
                jobz,
                econ,
                m,
                n,
                A.as_const_ptr() as *const cuComplex,
                lda,
                S.as_const_ptr() as *const f32,
                U.as_const_ptr() as *const cuComplex,
                ldu,
                V.as_const_ptr() as *const cuComplex,
                ldv,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgesvdj_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        S: U,
        U: V,
        ldu: ::std::os::raw::c_int,
        V: W,
        ldv: ::std::os::raw::c_int,
        mut lwork: X,
        params: gesvdjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgesvdj_bufferSize(
                self.handle,
                jobz,
                econ,
                m,
                n,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                S.as_const_ptr() as *const f64,
                U.as_const_ptr() as *const cuDoubleComplex,
                ldu,
                V.as_const_ptr() as *const cuDoubleComplex,
                ldv,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgesvdj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut V: W,
        ldv: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
        params: gesvdjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgesvdj(
                self.handle,
                jobz,
                econ,
                m,
                n,
                A.as_mut_ptr() as *mut f32,
                lda,
                S.as_mut_ptr() as *mut f32,
                U.as_mut_ptr() as *mut f32,
                ldu,
                V.as_mut_ptr() as *mut f32,
                ldv,
                work.as_mut_ptr() as *mut f32,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgesvdj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut V: W,
        ldv: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
        params: gesvdjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgesvdj(
                self.handle,
                jobz,
                econ,
                m,
                n,
                A.as_mut_ptr() as *mut f64,
                lda,
                S.as_mut_ptr() as *mut f64,
                U.as_mut_ptr() as *mut f64,
                ldu,
                V.as_mut_ptr() as *mut f64,
                ldv,
                work.as_mut_ptr() as *mut f64,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgesvdj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut V: W,
        ldv: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
        params: gesvdjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgesvdj(
                self.handle,
                jobz,
                econ,
                m,
                n,
                A.as_mut_ptr() as *mut cuComplex,
                lda,
                S.as_mut_ptr() as *mut f32,
                U.as_mut_ptr() as *mut cuComplex,
                ldu,
                V.as_mut_ptr() as *mut cuComplex,
                ldv,
                work.as_mut_ptr() as *mut cuComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgesvdj<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        mut A: T,
        lda: ::std::os::raw::c_int,
        mut S: U,
        mut U: V,
        ldu: ::std::os::raw::c_int,
        mut V: W,
        ldv: ::std::os::raw::c_int,
        mut work: X,
        lwork: ::std::os::raw::c_int,
        mut info: Y,
        params: gesvdjInfo_t,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgesvdj(
                self.handle,
                jobz,
                econ,
                m,
                n,
                A.as_mut_ptr() as *mut cuDoubleComplex,
                lda,
                S.as_mut_ptr() as *mut f64,
                U.as_mut_ptr() as *mut cuDoubleComplex,
                ldu,
                V.as_mut_ptr() as *mut cuDoubleComplex,
                ldv,
                work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                params,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgesvdaStridedBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: T,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: U,
        strideS: ::std::os::raw::c_longlong,
        d_U: V,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: W,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        mut lwork: X,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgesvdaStridedBatched_bufferSize(
                self.handle,
                jobz,
                rank,
                m,
                n,
                d_A.as_const_ptr() as *const f32,
                lda,
                strideA,
                d_S.as_const_ptr() as *const f32,
                strideS,
                d_U.as_const_ptr() as *const f32,
                ldu,
                strideU,
                d_V.as_const_ptr() as *const f32,
                ldv,
                strideV,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgesvdaStridedBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: T,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: U,
        strideS: ::std::os::raw::c_longlong,
        d_U: V,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: W,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        mut lwork: X,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgesvdaStridedBatched_bufferSize(
                self.handle,
                jobz,
                rank,
                m,
                n,
                d_A.as_const_ptr() as *const f64,
                lda,
                strideA,
                d_S.as_const_ptr() as *const f64,
                strideS,
                d_U.as_const_ptr() as *const f64,
                ldu,
                strideU,
                d_V.as_const_ptr() as *const f64,
                ldv,
                strideV,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgesvdaStridedBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: T,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: U,
        strideS: ::std::os::raw::c_longlong,
        d_U: V,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: W,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        mut lwork: X,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgesvdaStridedBatched_bufferSize(
                self.handle,
                jobz,
                rank,
                m,
                n,
                d_A.as_const_ptr() as *const cuComplex,
                lda,
                strideA,
                d_S.as_const_ptr() as *const f32,
                strideS,
                d_U.as_const_ptr() as *const cuComplex,
                ldu,
                strideU,
                d_V.as_const_ptr() as *const cuComplex,
                ldv,
                strideV,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgesvdaStridedBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: T,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: U,
        strideS: ::std::os::raw::c_longlong,
        d_U: V,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: W,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        mut lwork: X,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgesvdaStridedBatched_bufferSize(
                self.handle,
                jobz,
                rank,
                m,
                n,
                d_A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                strideA,
                d_S.as_const_ptr() as *const f64,
                strideS,
                d_U.as_const_ptr() as *const cuDoubleComplex,
                ldu,
                strideU,
                d_V.as_const_ptr() as *const cuDoubleComplex,
                ldv,
                strideV,
                lwork.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnSgesvdaStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: T,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        mut d_S: U,
        strideS: ::std::os::raw::c_longlong,
        mut d_U: V,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        mut d_V: W,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        mut d_work: X,
        lwork: ::std::os::raw::c_int,
        mut d_info: Y,
        mut h_R_nrmF: Z,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnSgesvdaStridedBatched(
                self.handle,
                jobz,
                rank,
                m,
                n,
                d_A.as_const_ptr() as *const f32,
                lda,
                strideA,
                d_S.as_mut_ptr() as *mut f32,
                strideS,
                d_U.as_mut_ptr() as *mut f32,
                ldu,
                strideU,
                d_V.as_mut_ptr() as *mut f32,
                ldv,
                strideV,
                d_work.as_mut_ptr() as *mut f32,
                lwork,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                h_R_nrmF.as_mut_ptr() as *mut f64,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnDgesvdaStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: T,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        mut d_S: U,
        strideS: ::std::os::raw::c_longlong,
        mut d_U: V,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        mut d_V: W,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        mut d_work: X,
        lwork: ::std::os::raw::c_int,
        mut d_info: Y,
        mut h_R_nrmF: Z,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnDgesvdaStridedBatched(
                self.handle,
                jobz,
                rank,
                m,
                n,
                d_A.as_const_ptr() as *const f64,
                lda,
                strideA,
                d_S.as_mut_ptr() as *mut f64,
                strideS,
                d_U.as_mut_ptr() as *mut f64,
                ldu,
                strideU,
                d_V.as_mut_ptr() as *mut f64,
                ldv,
                strideV,
                d_work.as_mut_ptr() as *mut f64,
                lwork,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                h_R_nrmF.as_mut_ptr() as *mut f64,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnCgesvdaStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: T,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        mut d_S: U,
        strideS: ::std::os::raw::c_longlong,
        mut d_U: V,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        mut d_V: W,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        mut d_work: X,
        lwork: ::std::os::raw::c_int,
        mut d_info: Y,
        mut h_R_nrmF: Z,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnCgesvdaStridedBatched(
                self.handle,
                jobz,
                rank,
                m,
                n,
                d_A.as_const_ptr() as *const cuComplex,
                lda,
                strideA,
                d_S.as_mut_ptr() as *mut f32,
                strideS,
                d_U.as_mut_ptr() as *mut cuComplex,
                ldu,
                strideU,
                d_V.as_mut_ptr() as *mut cuComplex,
                ldv,
                strideV,
                d_work.as_mut_ptr() as *mut cuComplex,
                lwork,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                h_R_nrmF.as_mut_ptr() as *mut f64,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnZgesvdaStridedBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: T,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        mut d_S: U,
        strideS: ::std::os::raw::c_longlong,
        mut d_U: V,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        mut d_V: W,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        mut d_work: X,
        lwork: ::std::os::raw::c_int,
        mut d_info: Y,
        mut h_R_nrmF: Z,
        batchSize: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnZgesvdaStridedBatched(
                self.handle,
                jobz,
                rank,
                m,
                n,
                d_A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                strideA,
                d_S.as_mut_ptr() as *mut f64,
                strideS,
                d_U.as_mut_ptr() as *mut cuDoubleComplex,
                ldu,
                strideU,
                d_V.as_mut_ptr() as *mut cuDoubleComplex,
                ldv,
                strideV,
                d_work.as_mut_ptr() as *mut cuDoubleComplex,
                lwork,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                h_R_nrmF.as_mut_ptr() as *mut f64,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXpotrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: U,
        mut workspaceInBytesOnHost: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXpotrf_bufferSize(
                self.handle,
                params,
                uplo,
                n,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXpotrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        computeType: cudaDataType,
        mut bufferOnDevice: U,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: V,
        workspaceInBytesOnHost: usize,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXpotrf(
                self.handle,
                params,
                uplo,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXpotrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        n: i64,
        nrhs: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        dataTypeB: cudaDataType,
        mut B: U,
        ldb: i64,
        mut info: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXpotrs(
                self.handle,
                params,
                uplo,
                n,
                nrhs,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeB,
                B.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldb,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgeqrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        dataTypeTau: cudaDataType,
        tau: U,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: V,
        mut workspaceInBytesOnHost: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgeqrf_bufferSize(
                self.handle,
                params,
                m,
                n,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeTau,
                tau.as_const_ptr() as *const ::std::os::raw::c_void,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgeqrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        dataTypeTau: cudaDataType,
        mut tau: U,
        computeType: cudaDataType,
        mut bufferOnDevice: V,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: W,
        workspaceInBytesOnHost: usize,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgeqrf(
                self.handle,
                params,
                m,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeTau,
                tau.as_mut_ptr() as *mut ::std::os::raw::c_void,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgetrf_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: U,
        mut workspaceInBytesOnHost: V,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgetrf_bufferSize(
                self.handle,
                params,
                m,
                n,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgetrf<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        mut ipiv: U,
        computeType: cudaDataType,
        mut bufferOnDevice: V,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: W,
        workspaceInBytesOnHost: usize,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgetrf(
                self.handle,
                params,
                m,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                ipiv.as_mut_ptr() as *mut i64,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgetrs<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        trans: cublasOperation_t,
        n: i64,
        nrhs: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        ipiv: U,
        dataTypeB: cudaDataType,
        mut B: V,
        ldb: i64,
        mut info: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgetrs(
                self.handle,
                params,
                trans,
                n,
                nrhs,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                ipiv.as_const_ptr() as *const i64,
                dataTypeB,
                B.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldb,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsyevd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        dataTypeW: cudaDataType,
        W: U,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: V,
        mut workspaceInBytesOnHost: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsyevd_bufferSize(
                self.handle,
                params,
                jobz,
                uplo,
                n,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeW,
                W.as_const_ptr() as *const ::std::os::raw::c_void,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsyevd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        dataTypeW: cudaDataType,
        mut W: U,
        computeType: cudaDataType,
        mut bufferOnDevice: V,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: W,
        workspaceInBytesOnHost: usize,
        mut info: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsyevd(
                self.handle,
                params,
                jobz,
                uplo,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeW,
                W.as_mut_ptr() as *mut ::std::os::raw::c_void,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXstedc_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        compz: cusolverEigComp_t,
        n: i64,
        dataTypeDE: cudaDataType,
        D: T,
        E: U,
        dataTypeZ: cudaDataType,
        Z: V,
        ldz: i64,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: W,
        mut workspaceInBytesOnHost: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXstedc_bufferSize(
                self.handle,
                params,
                compz,
                n,
                dataTypeDE,
                D.as_const_ptr() as *const ::std::os::raw::c_void,
                E.as_const_ptr() as *const ::std::os::raw::c_void,
                dataTypeZ,
                Z.as_const_ptr() as *const ::std::os::raw::c_void,
                ldz,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXstedc<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        compz: cusolverEigComp_t,
        n: i64,
        dataTypeDE: cudaDataType,
        mut D: T,
        mut E: U,
        dataTypeZ: cudaDataType,
        mut Z: V,
        ldz: i64,
        computeType: cudaDataType,
        mut bufferOnDevice: W,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: X,
        workspaceInBytesOnHost: usize,
        mut info: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXstedc(
                self.handle,
                params,
                compz,
                n,
                dataTypeDE,
                D.as_mut_ptr() as *mut ::std::os::raw::c_void,
                E.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dataTypeZ,
                Z.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldz,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsyevBatched_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        dataTypeW: cudaDataType,
        W: U,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: V,
        mut workspaceInBytesOnHost: W,
        batchSize: i64,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsyevBatched_bufferSize(
                self.handle,
                params,
                jobz,
                uplo,
                n,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeW,
                W.as_const_ptr() as *const ::std::os::raw::c_void,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsyevBatched<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        dataTypeW: cudaDataType,
        mut W: U,
        computeType: cudaDataType,
        mut bufferOnDevice: V,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: W,
        workspaceInBytesOnHost: usize,
        mut info: X,
        batchSize: i64,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsyevBatched(
                self.handle,
                params,
                jobz,
                uplo,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeW,
                W.as_mut_ptr() as *mut ::std::os::raw::c_void,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                batchSize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsyevdx_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        mut vl: U,
        mut vu: V,
        il: i64,
        iu: i64,
        mut h_meig: W,
        dataTypeW: cudaDataType,
        W: X,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: Y,
        mut workspaceInBytesOnHost: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsyevdx_bufferSize(
                self.handle,
                params,
                jobz,
                range,
                uplo,
                n,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                vl.as_mut_ptr() as *mut ::std::os::raw::c_void,
                vu.as_mut_ptr() as *mut ::std::os::raw::c_void,
                il,
                iu,
                h_meig.as_mut_ptr() as *mut i64,
                dataTypeW,
                W.as_const_ptr() as *const ::std::os::raw::c_void,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXsyevdx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        mut vl: U,
        mut vu: V,
        il: i64,
        iu: i64,
        mut meig64: W,
        dataTypeW: cudaDataType,
        mut W: X,
        computeType: cudaDataType,
        mut bufferOnDevice: Y,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: Z,
        workspaceInBytesOnHost: usize,
        mut info: A,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXsyevdx(
                self.handle,
                params,
                jobz,
                range,
                uplo,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                vl.as_mut_ptr() as *mut ::std::os::raw::c_void,
                vu.as_mut_ptr() as *mut ::std::os::raw::c_void,
                il,
                iu,
                meig64.as_mut_ptr() as *mut i64,
                dataTypeW,
                W.as_mut_ptr() as *mut ::std::os::raw::c_void,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgeev_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobvl: cusolverEigMode_t,
        jobvr: cusolverEigMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        dataTypeW: cudaDataType,
        W: U,
        dataTypeVL: cudaDataType,
        VL: V,
        ldvl: i64,
        dataTypeVR: cudaDataType,
        VR: W,
        ldvr: i64,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: X,
        mut workspaceInBytesOnHost: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgeev_bufferSize(
                self.handle,
                params,
                jobvl,
                jobvr,
                n,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeW,
                W.as_const_ptr() as *const ::std::os::raw::c_void,
                dataTypeVL,
                VL.as_const_ptr() as *const ::std::os::raw::c_void,
                ldvl,
                dataTypeVR,
                VR.as_const_ptr() as *const ::std::os::raw::c_void,
                ldvr,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgeev<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobvl: cusolverEigMode_t,
        jobvr: cusolverEigMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        dataTypeW: cudaDataType,
        mut W: U,
        dataTypeVL: cudaDataType,
        mut VL: V,
        ldvl: i64,
        dataTypeVR: cudaDataType,
        mut VR: W,
        ldvr: i64,
        computeType: cudaDataType,
        mut bufferOnDevice: X,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: Y,
        workspaceInBytesOnHost: usize,
        mut info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgeev(
                self.handle,
                params,
                jobvl,
                jobvr,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeW,
                W.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dataTypeVL,
                VL.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldvl,
                dataTypeVR,
                VR.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldvr,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgesvd_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        dataTypeS: cudaDataType,
        S: U,
        dataTypeU: cudaDataType,
        U: V,
        ldu: i64,
        dataTypeVT: cudaDataType,
        VT: W,
        ldvt: i64,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: X,
        mut workspaceInBytesOnHost: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgesvd_bufferSize(
                self.handle,
                params,
                jobu,
                jobvt,
                m,
                n,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeS,
                S.as_const_ptr() as *const ::std::os::raw::c_void,
                dataTypeU,
                U.as_const_ptr() as *const ::std::os::raw::c_void,
                ldu,
                dataTypeVT,
                VT.as_const_ptr() as *const ::std::os::raw::c_void,
                ldvt,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgesvd<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        dataTypeS: cudaDataType,
        mut S: U,
        dataTypeU: cudaDataType,
        mut U: V,
        ldu: i64,
        dataTypeVT: cudaDataType,
        mut VT: W,
        ldvt: i64,
        computeType: cudaDataType,
        mut bufferOnDevice: X,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: Y,
        workspaceInBytesOnHost: usize,
        mut info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgesvd(
                self.handle,
                params,
                jobu,
                jobvt,
                m,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeS,
                S.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dataTypeU,
                U.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldu,
                dataTypeVT,
                VT.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldvt,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgesvdp_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        dataTypeS: cudaDataType,
        S: U,
        dataTypeU: cudaDataType,
        U: V,
        ldu: i64,
        dataTypeV: cudaDataType,
        V: W,
        ldv: i64,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: X,
        mut workspaceInBytesOnHost: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgesvdp_bufferSize(
                self.handle,
                params,
                jobz,
                econ,
                m,
                n,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeS,
                S.as_const_ptr() as *const ::std::os::raw::c_void,
                dataTypeU,
                U.as_const_ptr() as *const ::std::os::raw::c_void,
                ldu,
                dataTypeV,
                V.as_const_ptr() as *const ::std::os::raw::c_void,
                ldv,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgesvdp<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        dataTypeS: cudaDataType,
        mut S: U,
        dataTypeU: cudaDataType,
        mut U: V,
        ldu: i64,
        dataTypeV: cudaDataType,
        mut V: W,
        ldv: i64,
        computeType: cudaDataType,
        mut bufferOnDevice: X,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: Y,
        workspaceInBytesOnHost: usize,
        mut d_info: Z,
        mut h_err_sigma: A,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgesvdp(
                self.handle,
                params,
                jobz,
                econ,
                m,
                n,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeS,
                S.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dataTypeU,
                U.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldu,
                dataTypeV,
                V.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldv,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
                h_err_sigma.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgesvdr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobu: ::std::os::raw::c_schar,
        jobv: ::std::os::raw::c_schar,
        m: i64,
        n: i64,
        k: i64,
        p: i64,
        niters: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        dataTypeSrand: cudaDataType,
        Srand: U,
        dataTypeUrand: cudaDataType,
        Urand: V,
        ldUrand: i64,
        dataTypeVrand: cudaDataType,
        Vrand: W,
        ldVrand: i64,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: X,
        mut workspaceInBytesOnHost: Y,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgesvdr_bufferSize(
                self.handle,
                params,
                jobu,
                jobv,
                m,
                n,
                k,
                p,
                niters,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeSrand,
                Srand.as_const_ptr() as *const ::std::os::raw::c_void,
                dataTypeUrand,
                Urand.as_const_ptr() as *const ::std::os::raw::c_void,
                ldUrand,
                dataTypeVrand,
                Vrand.as_const_ptr() as *const ::std::os::raw::c_void,
                ldVrand,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXgesvdr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        jobu: ::std::os::raw::c_schar,
        jobv: ::std::os::raw::c_schar,
        m: i64,
        n: i64,
        k: i64,
        p: i64,
        niters: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        dataTypeSrand: cudaDataType,
        mut Srand: U,
        dataTypeUrand: cudaDataType,
        mut Urand: V,
        ldUrand: i64,
        dataTypeVrand: cudaDataType,
        mut Vrand: W,
        ldVrand: i64,
        computeType: cudaDataType,
        mut bufferOnDevice: X,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: Y,
        workspaceInBytesOnHost: usize,
        mut d_info: Z,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXgesvdr(
                self.handle,
                params,
                jobu,
                jobv,
                m,
                n,
                k,
                p,
                niters,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeSrand,
                Srand.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dataTypeUrand,
                Urand.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldUrand,
                dataTypeVrand,
                Vrand.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldVrand,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXlarft_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        direct: cusolverDirectMode_t,
        storev: cusolverStorevMode_t,
        n: i64,
        k: i64,
        dataTypeV: cudaDataType,
        V: T,
        ldv: i64,
        dataTypeTau: cudaDataType,
        tau: U,
        dataTypeT: cudaDataType,
        mut T: V,
        ldt: i64,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: W,
        mut workspaceInBytesOnHost: X,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXlarft_bufferSize(
                self.handle,
                params,
                direct,
                storev,
                n,
                k,
                dataTypeV,
                V.as_const_ptr() as *const ::std::os::raw::c_void,
                ldv,
                dataTypeTau,
                tau.as_const_ptr() as *const ::std::os::raw::c_void,
                dataTypeT,
                T.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldt,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXlarft<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        direct: cusolverDirectMode_t,
        storev: cusolverStorevMode_t,
        n: i64,
        k: i64,
        dataTypeV: cudaDataType,
        V: T,
        ldv: i64,
        dataTypeTau: cudaDataType,
        tau: U,
        dataTypeT: cudaDataType,
        mut T: V,
        ldt: i64,
        computeType: cudaDataType,
        mut bufferOnDevice: W,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: X,
        workspaceInBytesOnHost: usize,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXlarft(
                self.handle,
                params,
                direct,
                storev,
                n,
                k,
                dataTypeV,
                V.as_const_ptr() as *const ::std::os::raw::c_void,
                ldv,
                dataTypeTau,
                tau.as_const_ptr() as *const ::std::os::raw::c_void,
                dataTypeT,
                T.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldt,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXpolar_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        M: i64,
        N: i64,
        dataTypeA: cudaDataType,
        A: T,
        lda: i64,
        dataTypeH: cudaDataType,
        H: U,
        ldh: i64,
        computeType: cudaDataType,
        mut workspaceInBytesOnDevice: V,
        mut workspaceInBytesOnHost: W,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXpolar_bufferSize(
                self.handle,
                params,
                uplo,
                M,
                N,
                dataTypeA,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                lda,
                dataTypeH,
                H.as_const_ptr() as *const ::std::os::raw::c_void,
                ldh,
                computeType,
                workspaceInBytesOnDevice.as_mut_ptr() as *mut usize,
                workspaceInBytesOnHost.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusolverDnXpolar<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
        A: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        M: i64,
        N: i64,
        dataTypeA: cudaDataType,
        mut A: T,
        lda: i64,
        dataTypeH: cudaDataType,
        mut H: U,
        ldh: i64,
        computeType: cudaDataType,
        mut bufferOnDevice: V,
        workspaceInBytesOnDevice: usize,
        mut bufferOnHost: W,
        workspaceInBytesOnHost: usize,
        mut d_res_nrm: X,
        mut d_A_nrmF: Y,
        mut d_rcond: Z,
        mut d_info: A,
    ) -> Result<(), crate::sys::cusolverStatus_t> {
        let status = unsafe {
            crate::sys::cusolverDnXpolar(
                self.handle,
                params,
                uplo,
                M,
                N,
                dataTypeA,
                A.as_mut_ptr() as *mut ::std::os::raw::c_void,
                lda,
                dataTypeH,
                H.as_mut_ptr() as *mut ::std::os::raw::c_void,
                ldh,
                computeType,
                bufferOnDevice.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnDevice,
                bufferOnHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceInBytesOnHost,
                d_res_nrm.as_mut_ptr() as *mut f64,
                d_A_nrmF.as_mut_ptr() as *mut f64,
                d_rcond.as_mut_ptr() as *mut f64,
                d_info.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
pub unsafe fn cusolverGetProperty(
    type_: libraryPropertyType,
) -> Result<::std::os::raw::c_int, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverGetVersion() -> Result<::std::os::raw::c_int, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsSetRefinementSolver(
    params: cusolverDnIRSParams_t,
    refinement_solver: cusolverIRSRefinement_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnIRSParamsSetRefinementSolver(params, refinement_solver) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsSetSolverMainPrecision(
    params: cusolverDnIRSParams_t,
    solver_main_precision: cusolverPrecType_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnIRSParamsSetSolverMainPrecision(params, solver_main_precision)
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsSetSolverLowestPrecision(
    params: cusolverDnIRSParams_t,
    solver_lowest_precision: cusolverPrecType_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnIRSParamsSetSolverLowestPrecision(params, solver_lowest_precision)
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsSetSolverPrecisions(
    params: cusolverDnIRSParams_t,
    solver_main_precision: cusolverPrecType_t,
    solver_lowest_precision: cusolverPrecType_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnIRSParamsSetSolverPrecisions(
            params,
            solver_main_precision,
            solver_lowest_precision,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsSetTol(
    params: cusolverDnIRSParams_t,
    val: f64,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetTol(params, val) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsSetTolInner(
    params: cusolverDnIRSParams_t,
    val: f64,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetTolInner(params, val) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsSetMaxIters(
    params: cusolverDnIRSParams_t,
    maxiters: cusolver_int_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetMaxIters(params, maxiters) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsSetMaxItersInner(
    params: cusolverDnIRSParams_t,
    maxiters_inner: cusolver_int_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetMaxItersInner(params, maxiters_inner) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsGetMaxIters(
    params: cusolverDnIRSParams_t,
) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cusolverDnIRSParamsGetMaxIters(params, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsEnableFallback(
    params: cusolverDnIRSParams_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsEnableFallback(params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsDisableFallback(
    params: cusolverDnIRSParams_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsDisableFallback(params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSInfosGetNiters(
    infos: cusolverDnIRSInfos_t,
) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cusolverDnIRSInfosGetNiters(infos, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSInfosGetOuterNiters(
    infos: cusolverDnIRSInfos_t,
) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusolverDnIRSInfosGetOuterNiters(infos, out_1.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSInfosRequestResidual(
    infos: cusolverDnIRSInfos_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSInfosRequestResidual(infos) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSInfosGetResidualHistory<T: ::cuda_libs::types::CudaAsPtr>(
    infos: cusolverDnIRSInfos_t,
    mut residual_history: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnIRSInfosGetResidualHistory(
            infos,
            residual_history.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSInfosGetMaxIters(
    infos: cusolverDnIRSInfos_t,
) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cusolverDnIRSInfosGetMaxIters(infos, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsyevjSetTolerance(
    info: syevjInfo_t,
    tolerance: f64,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXsyevjSetTolerance(info, tolerance) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsyevjSetMaxSweeps(
    info: syevjInfo_t,
    max_sweeps: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXsyevjSetMaxSweeps(info, max_sweeps) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsyevjSetSortEig(
    info: syevjInfo_t,
    sort_eig: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXsyevjSetSortEig(info, sort_eig) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvdjSetTolerance(
    info: gesvdjInfo_t,
    tolerance: f64,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgesvdjSetTolerance(info, tolerance) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvdjSetMaxSweeps(
    info: gesvdjInfo_t,
    max_sweeps: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgesvdjSetMaxSweeps(info, max_sweeps) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvdjSetSortEig(
    info: gesvdjInfo_t,
    sort_svd: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgesvdjSetSortEig(info, sort_svd) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSetAdvOptions(
    params: cusolverDnParams_t,
    function: cusolverDnFunction_t,
    algo: cusolverAlgMode_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetAdvOptions(params, function, algo) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnLoggerSetCallback(
    callback: cusolverDnLoggerCallback_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetCallback(callback) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnLoggerSetFile<T: ::cuda_libs::types::CudaAsPtr>(
    mut file: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetFile(file.as_mut_ptr() as *mut FILE) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnLoggerOpenFile<T: ::cuda_libs::types::CudaAsPtr>(
    logFile: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnLoggerOpenFile(logFile.as_const_ptr() as *const ::std::os::raw::c_char)
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnLoggerSetLevel(
    level: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetLevel(level) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnLoggerSetMask(
    mask: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetMask(mask) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnLoggerForceDisable() -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerForceDisable() };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpSetStream(
    handle: cusolverSpHandle_t,
    streamId: cudaStream_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpSetStream(handle, streamId) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpGetStream(
    handle: cusolverSpHandle_t,
) -> Result<cudaStream_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverSpGetStream(handle, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrissymHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrEndPtrA: U,
    csrColIndA: V,
    mut issym: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrissymHost(
            handle,
            m,
            nnzA,
            descrA,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrEndPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            issym.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvluHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvluHost(
            handle,
            n,
            nnzA,
            descrA,
            csrValA.as_const_ptr() as *const f32,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f32,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f32,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvluHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvluHost(
            handle,
            n,
            nnzA,
            descrA,
            csrValA.as_const_ptr() as *const f64,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f64,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f64,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvluHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvluHost(
            handle,
            n,
            nnzA,
            descrA,
            csrValA.as_const_ptr() as *const cuComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvluHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvluHost(
            handle,
            n,
            nnzA,
            descrA,
            csrValA.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuDoubleComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuDoubleComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvqr<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvqr(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const f32,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f32,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f32,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvqr<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvqr(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const f64,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f64,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f64,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvqr<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvqr(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const cuComplex,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvqr<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvqr(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuDoubleComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuDoubleComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvqrHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvqrHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f32,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f32,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f32,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvqrHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvqrHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f64,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f64,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f64,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvqrHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvqrHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvqrHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvqrHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuDoubleComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuDoubleComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvcholHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvcholHost(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const f32,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f32,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f32,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvcholHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvcholHost(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const f64,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f64,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f64,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvcholHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvcholHost(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const cuComplex,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvcholHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvcholHost(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuDoubleComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuDoubleComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvchol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvchol(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const f32,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f32,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f32,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvchol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvchol(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const f64,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f64,
            tol,
            reorder,
            x.as_mut_ptr() as *mut f64,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvchol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvchol(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const cuComplex,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvchol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvchol(
            handle,
            m,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuDoubleComplex,
            tol,
            reorder,
            x.as_mut_ptr() as *mut cuDoubleComplex,
            singularity.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsqvqrHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
    Z: ::cuda_libs::types::CudaAsPtr,
    A: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    mut rankA: X,
    mut x: Y,
    mut p: Z,
    mut min_norm: A,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsqvqrHost(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f32,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f32,
            tol,
            rankA.as_mut_ptr() as *mut ::std::os::raw::c_int,
            x.as_mut_ptr() as *mut f32,
            p.as_mut_ptr() as *mut ::std::os::raw::c_int,
            min_norm.as_mut_ptr() as *mut f32,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsqvqrHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
    Z: ::cuda_libs::types::CudaAsPtr,
    A: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    mut rankA: X,
    mut x: Y,
    mut p: Z,
    mut min_norm: A,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsqvqrHost(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f64,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f64,
            tol,
            rankA.as_mut_ptr() as *mut ::std::os::raw::c_int,
            x.as_mut_ptr() as *mut f64,
            p.as_mut_ptr() as *mut ::std::os::raw::c_int,
            min_norm.as_mut_ptr() as *mut f64,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsqvqrHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
    Z: ::cuda_libs::types::CudaAsPtr,
    A: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    mut rankA: X,
    mut x: Y,
    mut p: Z,
    mut min_norm: A,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsqvqrHost(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuComplex,
            tol,
            rankA.as_mut_ptr() as *mut ::std::os::raw::c_int,
            x.as_mut_ptr() as *mut cuComplex,
            p.as_mut_ptr() as *mut ::std::os::raw::c_int,
            min_norm.as_mut_ptr() as *mut f32,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsqvqrHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
    Z: ::cuda_libs::types::CudaAsPtr,
    A: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    mut rankA: X,
    mut x: Y,
    mut p: Z,
    mut min_norm: A,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsqvqrHost(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuDoubleComplex,
            tol,
            rankA.as_mut_ptr() as *mut ::std::os::raw::c_int,
            x.as_mut_ptr() as *mut cuDoubleComplex,
            p.as_mut_ptr() as *mut ::std::os::raw::c_int,
            min_norm.as_mut_ptr() as *mut f64,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsreigvsiHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: f32,
    x0: W,
    maxite: ::std::os::raw::c_int,
    tol: f32,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsreigvsiHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f32,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            mu0,
            x0.as_const_ptr() as *const f32,
            maxite,
            tol,
            mu.as_mut_ptr() as *mut f32,
            x.as_mut_ptr() as *mut f32,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsreigvsiHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: f64,
    x0: W,
    maxite: ::std::os::raw::c_int,
    tol: f64,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsreigvsiHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f64,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            mu0,
            x0.as_const_ptr() as *const f64,
            maxite,
            tol,
            mu.as_mut_ptr() as *mut f64,
            x.as_mut_ptr() as *mut f64,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsreigvsiHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: cuComplex,
    x0: W,
    maxite: ::std::os::raw::c_int,
    tol: f32,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsreigvsiHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            mu0,
            x0.as_const_ptr() as *const cuComplex,
            maxite,
            tol,
            mu.as_mut_ptr() as *mut cuComplex,
            x.as_mut_ptr() as *mut cuComplex,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsreigvsiHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: cuDoubleComplex,
    x0: W,
    maxite: ::std::os::raw::c_int,
    tol: f64,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsreigvsiHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            mu0,
            x0.as_const_ptr() as *const cuDoubleComplex,
            maxite,
            tol,
            mu.as_mut_ptr() as *mut cuDoubleComplex,
            x.as_mut_ptr() as *mut cuDoubleComplex,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsreigvsi<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: f32,
    x0: W,
    maxite: ::std::os::raw::c_int,
    eps: f32,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsreigvsi(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f32,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            mu0,
            x0.as_const_ptr() as *const f32,
            maxite,
            eps,
            mu.as_mut_ptr() as *mut f32,
            x.as_mut_ptr() as *mut f32,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsreigvsi<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: f64,
    x0: W,
    maxite: ::std::os::raw::c_int,
    eps: f64,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsreigvsi(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f64,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            mu0,
            x0.as_const_ptr() as *const f64,
            maxite,
            eps,
            mu.as_mut_ptr() as *mut f64,
            x.as_mut_ptr() as *mut f64,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsreigvsi<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: cuComplex,
    x0: W,
    maxite: ::std::os::raw::c_int,
    eps: f32,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsreigvsi(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            mu0,
            x0.as_const_ptr() as *const cuComplex,
            maxite,
            eps,
            mu.as_mut_ptr() as *mut cuComplex,
            x.as_mut_ptr() as *mut cuComplex,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsreigvsi<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: cuDoubleComplex,
    x0: W,
    maxite: ::std::os::raw::c_int,
    eps: f64,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsreigvsi(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            mu0,
            x0.as_const_ptr() as *const cuDoubleComplex,
            maxite,
            eps,
            mu.as_mut_ptr() as *mut cuDoubleComplex,
            x.as_mut_ptr() as *mut cuDoubleComplex,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsreigsHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    left_bottom_corner: cuComplex,
    right_upper_corner: cuComplex,
    mut num_eigs: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsreigsHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f32,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsreigsHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    left_bottom_corner: cuDoubleComplex,
    right_upper_corner: cuDoubleComplex,
    mut num_eigs: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsreigsHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f64,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsreigsHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    left_bottom_corner: cuComplex,
    right_upper_corner: cuComplex,
    mut num_eigs: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsreigsHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsreigsHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    left_bottom_corner: cuDoubleComplex,
    right_upper_corner: cuDoubleComplex,
    mut num_eigs: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsreigsHost(
            handle,
            m,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrsymrcmHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    mut p: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrsymrcmHost(
            handle,
            n,
            nnzA,
            descrA,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            p.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrsymmdqHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    mut p: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrsymmdqHost(
            handle,
            n,
            nnzA,
            descrA,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            p.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrsymamdHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    mut p: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrsymamdHost(
            handle,
            n,
            nnzA,
            descrA,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            p.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrmetisndHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    options: V,
    mut p: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrmetisndHost(
            handle,
            n,
            nnzA,
            descrA,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            options.as_const_ptr() as *const i64,
            p.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrzfdHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mut P: W,
    mut numnz: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrzfdHost(
            handle,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f32,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            P.as_mut_ptr() as *mut ::std::os::raw::c_int,
            numnz.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrzfdHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mut P: W,
    mut numnz: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrzfdHost(
            handle,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f64,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            P.as_mut_ptr() as *mut ::std::os::raw::c_int,
            numnz.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrzfdHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mut P: W,
    mut numnz: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrzfdHost(
            handle,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            P.as_mut_ptr() as *mut ::std::os::raw::c_int,
            numnz.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrzfdHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mut P: W,
    mut numnz: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrzfdHost(
            handle,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            P.as_mut_ptr() as *mut ::std::os::raw::c_int,
            numnz.as_mut_ptr() as *mut ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrperm_bufferSizeHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    p: V,
    q: W,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrperm_bufferSizeHost(
            handle,
            m,
            n,
            nnzA,
            descrA,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            p.as_const_ptr() as *const ::std::os::raw::c_int,
            q.as_const_ptr() as *const ::std::os::raw::c_int,
            bufferSizeInBytes.as_mut_ptr() as *mut usize,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrpermHost<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    mut csrRowPtrA: T,
    mut csrColIndA: U,
    p: V,
    q: W,
    mut map: X,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrpermHost(
            handle,
            m,
            n,
            nnzA,
            descrA,
            csrRowPtrA.as_mut_ptr() as *mut ::std::os::raw::c_int,
            csrColIndA.as_mut_ptr() as *mut ::std::os::raw::c_int,
            p.as_const_ptr() as *const ::std::os::raw::c_int,
            q.as_const_ptr() as *const ::std::os::raw::c_int,
            map.as_mut_ptr() as *mut ::std::os::raw::c_int,
            pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrqrAnalysisBatched<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    info: csrqrInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrqrAnalysisBatched(
            handle,
            m,
            n,
            nnzA,
            descrA,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            info,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrqrBufferInfoBatched<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    mut internalDataInBytes: W,
    mut workspaceInBytes: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrqrBufferInfoBatched(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const f32,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            batchSize,
            info,
            internalDataInBytes.as_mut_ptr() as *mut usize,
            workspaceInBytes.as_mut_ptr() as *mut usize,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrqrBufferInfoBatched<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    mut internalDataInBytes: W,
    mut workspaceInBytes: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrqrBufferInfoBatched(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const f64,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            batchSize,
            info,
            internalDataInBytes.as_mut_ptr() as *mut usize,
            workspaceInBytes.as_mut_ptr() as *mut usize,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrqrBufferInfoBatched<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    mut internalDataInBytes: W,
    mut workspaceInBytes: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrqrBufferInfoBatched(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const cuComplex,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            batchSize,
            info,
            internalDataInBytes.as_mut_ptr() as *mut usize,
            workspaceInBytes.as_mut_ptr() as *mut usize,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrqrBufferInfoBatched<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    mut internalDataInBytes: W,
    mut workspaceInBytes: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrqrBufferInfoBatched(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrVal.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
            batchSize,
            info,
            internalDataInBytes.as_mut_ptr() as *mut usize,
            workspaceInBytes.as_mut_ptr() as *mut usize,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrqrsvBatched<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    mut x: X,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrqrsvBatched(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f32,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f32,
            x.as_mut_ptr() as *mut f32,
            batchSize,
            info,
            pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrqrsvBatched<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    mut x: X,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrqrsvBatched(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const f64,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const f64,
            x.as_mut_ptr() as *mut f64,
            batchSize,
            info,
            pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrqrsvBatched<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    mut x: X,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrqrsvBatched(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuComplex,
            x.as_mut_ptr() as *mut cuComplex,
            batchSize,
            info,
            pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrqrsvBatched<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
    Y: ::cuda_libs::types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    mut x: X,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrqrsvBatched(
            handle,
            m,
            n,
            nnz,
            descrA,
            csrValA.as_const_ptr() as *const cuDoubleComplex,
            csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
            csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
            b.as_const_ptr() as *const cuDoubleComplex,
            x.as_mut_ptr() as *mut cuDoubleComplex,
            batchSize,
            info,
            pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
impl CusolverDnHandle {
    pub fn new() -> Result<Self, crate::sys::cusolverStatus_t> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let status = crate::sys::cusolverDnCreate(&mut handle);
            if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
                Ok(Self { handle })
            } else {
                Err(status)
            }
        }
    }
}
impl Drop for CusolverDnHandle {
    fn drop(&mut self) {
        unsafe {
            crate::sys::cusolverDnDestroy(self.handle);
        }
    }
}
