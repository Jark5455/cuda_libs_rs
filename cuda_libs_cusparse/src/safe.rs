pub use crate::sys::cusparseStatus_t as CudaTargetStatus;
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
pub struct CusparseHandle {
    pub(crate) handle: crate::sys::cusparseHandle_t,
}
impl CusparseHandle {
    pub unsafe fn cusparseGetVersion(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cusparseStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cusparseGetVersion(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSetStream(
        &self,
        streamId: cudaStream_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe { crate::sys::cusparseSetStream(self.handle, streamId) };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseGetStream(&self) -> Result<cudaStream_t, crate::sys::cusparseStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cusparseGetStream(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseGetPointerMode(
        &self,
    ) -> Result<cusparsePointerMode_t, crate::sys::cusparseStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cusparsePointerMode_t> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cusparseGetPointerMode(self.handle, out_1.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSetPointerMode(
        &self,
        mode: cusparsePointerMode_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe { crate::sys::cusparseSetPointerMode(self.handle, mode) };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgemvi<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transA: cusparseOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        xVal: V,
        xInd: W,
        beta: X,
        mut y: Y,
        idxBase: cusparseIndexBase_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgemvi(
                self.handle,
                transA,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                A.as_const_ptr() as *const f32,
                lda,
                nnz,
                xVal.as_const_ptr() as *const f32,
                xInd.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
                idxBase,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgemvi_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        transA: cusparseOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut pBufferSize: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgemvi_bufferSize(
                self.handle,
                transA,
                m,
                n,
                nnz,
                pBufferSize.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgemvi<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transA: cusparseOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        xVal: V,
        xInd: W,
        beta: X,
        mut y: Y,
        idxBase: cusparseIndexBase_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgemvi(
                self.handle,
                transA,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                A.as_const_ptr() as *const f64,
                lda,
                nnz,
                xVal.as_const_ptr() as *const f64,
                xInd.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
                idxBase,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgemvi_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        transA: cusparseOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut pBufferSize: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgemvi_bufferSize(
                self.handle,
                transA,
                m,
                n,
                nnz,
                pBufferSize.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgemvi<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transA: cusparseOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        xVal: V,
        xInd: W,
        beta: X,
        mut y: Y,
        idxBase: cusparseIndexBase_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgemvi(
                self.handle,
                transA,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                A.as_const_ptr() as *const cuComplex,
                lda,
                nnz,
                xVal.as_const_ptr() as *const cuComplex,
                xInd.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
                idxBase,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgemvi_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        transA: cusparseOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut pBufferSize: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgemvi_bufferSize(
                self.handle,
                transA,
                m,
                n,
                nnz,
                pBufferSize.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgemvi<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transA: cusparseOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        A: U,
        lda: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        xVal: V,
        xInd: W,
        beta: X,
        mut y: Y,
        idxBase: cusparseIndexBase_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgemvi(
                self.handle,
                transA,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                nnz,
                xVal.as_const_ptr() as *const cuDoubleComplex,
                xInd.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
                idxBase,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgemvi_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        transA: cusparseOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut pBufferSize: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgemvi_bufferSize(
                self.handle,
                transA,
                m,
                n,
                nnz,
                pBufferSize.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrmv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockDim: ::std::os::raw::c_int,
        x: X,
        beta: Y,
        mut y: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrmv(
                self.handle,
                dirA,
                transA,
                mb,
                nb,
                nnzb,
                alpha.as_const_ptr() as *const f32,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                x.as_const_ptr() as *const f32,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrmv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockDim: ::std::os::raw::c_int,
        x: X,
        beta: Y,
        mut y: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrmv(
                self.handle,
                dirA,
                transA,
                mb,
                nb,
                nnzb,
                alpha.as_const_ptr() as *const f64,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                x.as_const_ptr() as *const f64,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrmv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockDim: ::std::os::raw::c_int,
        x: X,
        beta: Y,
        mut y: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrmv(
                self.handle,
                dirA,
                transA,
                mb,
                nb,
                nnzb,
                alpha.as_const_ptr() as *const cuComplex,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                x.as_const_ptr() as *const cuComplex,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrmv<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockDim: ::std::os::raw::c_int,
        x: X,
        beta: Y,
        mut y: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrmv(
                self.handle,
                dirA,
                transA,
                mb,
                nb,
                nnzb,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                x.as_const_ptr() as *const cuDoubleComplex,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrxmv<
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
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        sizeOfMask: ::std::os::raw::c_int,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedMaskPtrA: V,
        bsrSortedRowPtrA: W,
        bsrSortedEndPtrA: X,
        bsrSortedColIndA: Y,
        blockDim: ::std::os::raw::c_int,
        x: Z,
        beta: A,
        mut y: B,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrxmv(
                self.handle,
                dirA,
                transA,
                sizeOfMask,
                mb,
                nb,
                nnzb,
                alpha.as_const_ptr() as *const f32,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedMaskPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedEndPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                x.as_const_ptr() as *const f32,
                beta.as_const_ptr() as *const f32,
                y.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrxmv<
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
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        sizeOfMask: ::std::os::raw::c_int,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedMaskPtrA: V,
        bsrSortedRowPtrA: W,
        bsrSortedEndPtrA: X,
        bsrSortedColIndA: Y,
        blockDim: ::std::os::raw::c_int,
        x: Z,
        beta: A,
        mut y: B,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrxmv(
                self.handle,
                dirA,
                transA,
                sizeOfMask,
                mb,
                nb,
                nnzb,
                alpha.as_const_ptr() as *const f64,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedMaskPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedEndPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                x.as_const_ptr() as *const f64,
                beta.as_const_ptr() as *const f64,
                y.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrxmv<
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
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        sizeOfMask: ::std::os::raw::c_int,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedMaskPtrA: V,
        bsrSortedRowPtrA: W,
        bsrSortedEndPtrA: X,
        bsrSortedColIndA: Y,
        blockDim: ::std::os::raw::c_int,
        x: Z,
        beta: A,
        mut y: B,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrxmv(
                self.handle,
                dirA,
                transA,
                sizeOfMask,
                mb,
                nb,
                nnzb,
                alpha.as_const_ptr() as *const cuComplex,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedMaskPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedEndPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                x.as_const_ptr() as *const cuComplex,
                beta.as_const_ptr() as *const cuComplex,
                y.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrxmv<
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
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        sizeOfMask: ::std::os::raw::c_int,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedMaskPtrA: V,
        bsrSortedRowPtrA: W,
        bsrSortedEndPtrA: X,
        bsrSortedColIndA: Y,
        blockDim: ::std::os::raw::c_int,
        x: Z,
        beta: A,
        mut y: B,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrxmv(
                self.handle,
                dirA,
                transA,
                sizeOfMask,
                mb,
                nb,
                nnzb,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedMaskPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedEndPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                x.as_const_ptr() as *const cuDoubleComplex,
                beta.as_const_ptr() as *const cuDoubleComplex,
                y.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXbsrsv2_zeroPivot<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        info: bsrsv2Info_t,
        mut position: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXbsrsv2_zeroPivot(
                self.handle,
                info,
                position.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrsv2_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrsv2_bufferSize(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_mut_ptr() as *mut f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrsv2_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrsv2_bufferSize(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_mut_ptr() as *mut f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrsv2_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrsv2_bufferSize(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrsv2_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrsv2_bufferSize(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrsv2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrsv2_bufferSizeExt(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_mut_ptr() as *mut f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrsv2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrsv2_bufferSizeExt(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_mut_ptr() as *mut f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrsv2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrsv2_bufferSizeExt(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrsv2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrsv2_bufferSizeExt(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrsv2_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrsv2_analysis(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrsv2_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrsv2_analysis(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrsv2_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrsv2_analysis(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrsv2_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrsv2_analysis(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrsv2_solve<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        f: X,
        mut x: Y,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrsv2_solve(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                alpha.as_const_ptr() as *const f32,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                f.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut f32,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrsv2_solve<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        f: X,
        mut x: Y,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrsv2_solve(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                alpha.as_const_ptr() as *const f64,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                f.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut f64,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrsv2_solve<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        f: X,
        mut x: Y,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrsv2_solve(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                alpha.as_const_ptr() as *const cuComplex,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                f.as_const_ptr() as *const cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrsv2_solve<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockDim: ::std::os::raw::c_int,
        info: bsrsv2Info_t,
        f: X,
        mut x: Y,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrsv2_solve(
                self.handle,
                dirA,
                transA,
                mb,
                nnzb,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                f.as_const_ptr() as *const cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrmm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transB: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockSize: ::std::os::raw::c_int,
        B: X,
        ldb: ::std::os::raw::c_int,
        beta: Y,
        mut C: Z,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrmm(
                self.handle,
                dirA,
                transA,
                transB,
                mb,
                n,
                kb,
                nnzb,
                alpha.as_const_ptr() as *const f32,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                B.as_const_ptr() as *const f32,
                ldb,
                beta.as_const_ptr() as *const f32,
                C.as_mut_ptr() as *mut f32,
                ldc,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrmm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transB: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockSize: ::std::os::raw::c_int,
        B: X,
        ldb: ::std::os::raw::c_int,
        beta: Y,
        mut C: Z,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrmm(
                self.handle,
                dirA,
                transA,
                transB,
                mb,
                n,
                kb,
                nnzb,
                alpha.as_const_ptr() as *const f64,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                B.as_const_ptr() as *const f64,
                ldb,
                beta.as_const_ptr() as *const f64,
                C.as_mut_ptr() as *mut f64,
                ldc,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrmm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transB: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockSize: ::std::os::raw::c_int,
        B: X,
        ldb: ::std::os::raw::c_int,
        beta: Y,
        mut C: Z,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrmm(
                self.handle,
                dirA,
                transA,
                transB,
                mb,
                n,
                kb,
                nnzb,
                alpha.as_const_ptr() as *const cuComplex,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                beta.as_const_ptr() as *const cuComplex,
                C.as_mut_ptr() as *mut cuComplex,
                ldc,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrmm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transB: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        kb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: U,
        bsrSortedRowPtrA: V,
        bsrSortedColIndA: W,
        blockSize: ::std::os::raw::c_int,
        B: X,
        ldb: ::std::os::raw::c_int,
        beta: Y,
        mut C: Z,
        ldc: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrmm(
                self.handle,
                dirA,
                transA,
                transB,
                mb,
                n,
                kb,
                nnzb,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                beta.as_const_ptr() as *const cuDoubleComplex,
                C.as_mut_ptr() as *mut cuDoubleComplex,
                ldc,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXbsrsm2_zeroPivot<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        info: bsrsm2Info_t,
        mut position: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXbsrsm2_zeroPivot(
                self.handle,
                info,
                position.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrsm2_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrsm2_bufferSize(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrsm2_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrsm2_bufferSize(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrsm2_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrsm2_bufferSize(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrsm2_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrsm2_bufferSize(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrsm2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transB: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrsm2_bufferSizeExt(
                self.handle,
                dirA,
                transA,
                transB,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrsm2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transB: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrsm2_bufferSizeExt(
                self.handle,
                dirA,
                transA,
                transB,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrsm2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transB: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrsm2_bufferSizeExt(
                self.handle,
                dirA,
                transA,
                transB,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrsm2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transB: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrsm2_bufferSizeExt(
                self.handle,
                dirA,
                transA,
                transB,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrsm2_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrsm2_analysis(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_const_ptr() as *const f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrsm2_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrsm2_analysis(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_const_ptr() as *const f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrsm2_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrsm2_analysis(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrsm2_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrsm2_analysis(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                descrA,
                bsrSortedVal.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrsm2_solve<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: U,
        bsrSortedRowPtr: V,
        bsrSortedColInd: W,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        B: X,
        ldb: ::std::os::raw::c_int,
        mut X: Y,
        ldx: ::std::os::raw::c_int,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrsm2_solve(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                alpha.as_const_ptr() as *const f32,
                descrA,
                bsrSortedVal.as_const_ptr() as *const f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                B.as_const_ptr() as *const f32,
                ldb,
                X.as_mut_ptr() as *mut f32,
                ldx,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrsm2_solve<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: U,
        bsrSortedRowPtr: V,
        bsrSortedColInd: W,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        B: X,
        ldb: ::std::os::raw::c_int,
        mut X: Y,
        ldx: ::std::os::raw::c_int,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrsm2_solve(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                alpha.as_const_ptr() as *const f64,
                descrA,
                bsrSortedVal.as_const_ptr() as *const f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                B.as_const_ptr() as *const f64,
                ldb,
                X.as_mut_ptr() as *mut f64,
                ldx,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrsm2_solve<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: U,
        bsrSortedRowPtr: V,
        bsrSortedColInd: W,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        B: X,
        ldb: ::std::os::raw::c_int,
        mut X: Y,
        ldx: ::std::os::raw::c_int,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrsm2_solve(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                alpha.as_const_ptr() as *const cuComplex,
                descrA,
                bsrSortedVal.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                X.as_mut_ptr() as *mut cuComplex,
                ldx,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrsm2_solve<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        transA: cusparseOperation_t,
        transXY: cusparseOperation_t,
        mb: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: U,
        bsrSortedRowPtr: V,
        bsrSortedColInd: W,
        blockSize: ::std::os::raw::c_int,
        info: bsrsm2Info_t,
        B: X,
        ldb: ::std::os::raw::c_int,
        mut X: Y,
        ldx: ::std::os::raw::c_int,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrsm2_solve(
                self.handle,
                dirA,
                transA,
                transXY,
                mb,
                n,
                nnzb,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                descrA,
                bsrSortedVal.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                X.as_mut_ptr() as *mut cuDoubleComplex,
                ldx,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsrilu02_numericBoost<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        info: csrilu02Info_t,
        enable_boost: ::std::os::raw::c_int,
        mut tol: T,
        mut boost_val: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsrilu02_numericBoost(
                self.handle,
                info,
                enable_boost,
                tol.as_mut_ptr() as *mut f64,
                boost_val.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsrilu02_numericBoost<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        info: csrilu02Info_t,
        enable_boost: ::std::os::raw::c_int,
        mut tol: T,
        mut boost_val: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsrilu02_numericBoost(
                self.handle,
                info,
                enable_boost,
                tol.as_mut_ptr() as *mut f64,
                boost_val.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsrilu02_numericBoost<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        info: csrilu02Info_t,
        enable_boost: ::std::os::raw::c_int,
        mut tol: T,
        mut boost_val: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsrilu02_numericBoost(
                self.handle,
                info,
                enable_boost,
                tol.as_mut_ptr() as *mut f64,
                boost_val.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsrilu02_numericBoost<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        info: csrilu02Info_t,
        enable_boost: ::std::os::raw::c_int,
        mut tol: T,
        mut boost_val: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsrilu02_numericBoost(
                self.handle,
                info,
                enable_boost,
                tol.as_mut_ptr() as *mut f64,
                boost_val.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcsrilu02_zeroPivot<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        info: csrilu02Info_t,
        mut position: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcsrilu02_zeroPivot(
                self.handle,
                info,
                position.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsrilu02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsrilu02_bufferSize(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_mut_ptr() as *mut f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsrilu02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsrilu02_bufferSize(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_mut_ptr() as *mut f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsrilu02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsrilu02_bufferSize(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_mut_ptr() as *mut cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsrilu02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsrilu02_bufferSize(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsrilu02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedVal: T,
        csrSortedRowPtr: U,
        csrSortedColInd: V,
        info: csrilu02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsrilu02_bufferSizeExt(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedVal.as_mut_ptr() as *mut f32,
                csrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsrilu02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedVal: T,
        csrSortedRowPtr: U,
        csrSortedColInd: V,
        info: csrilu02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsrilu02_bufferSizeExt(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedVal.as_mut_ptr() as *mut f64,
                csrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsrilu02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedVal: T,
        csrSortedRowPtr: U,
        csrSortedColInd: V,
        info: csrilu02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsrilu02_bufferSizeExt(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedVal.as_mut_ptr() as *mut cuComplex,
                csrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsrilu02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedVal: T,
        csrSortedRowPtr: U,
        csrSortedColInd: V,
        info: csrilu02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsrilu02_bufferSizeExt(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsrilu02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsrilu02_analysis(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsrilu02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsrilu02_analysis(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsrilu02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsrilu02_analysis(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsrilu02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsrilu02_analysis(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsrilu02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA_valM: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsrilu02(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA_valM.as_mut_ptr() as *mut f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsrilu02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA_valM: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsrilu02(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA_valM.as_mut_ptr() as *mut f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsrilu02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA_valM: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsrilu02(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA_valM.as_mut_ptr() as *mut cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsrilu02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA_valM: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsrilu02(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA_valM.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrilu02_numericBoost<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        info: bsrilu02Info_t,
        enable_boost: ::std::os::raw::c_int,
        mut tol: T,
        mut boost_val: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrilu02_numericBoost(
                self.handle,
                info,
                enable_boost,
                tol.as_mut_ptr() as *mut f64,
                boost_val.as_mut_ptr() as *mut f32,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrilu02_numericBoost<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        info: bsrilu02Info_t,
        enable_boost: ::std::os::raw::c_int,
        mut tol: T,
        mut boost_val: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrilu02_numericBoost(
                self.handle,
                info,
                enable_boost,
                tol.as_mut_ptr() as *mut f64,
                boost_val.as_mut_ptr() as *mut f64,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrilu02_numericBoost<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        info: bsrilu02Info_t,
        enable_boost: ::std::os::raw::c_int,
        mut tol: T,
        mut boost_val: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrilu02_numericBoost(
                self.handle,
                info,
                enable_boost,
                tol.as_mut_ptr() as *mut f64,
                boost_val.as_mut_ptr() as *mut cuComplex,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrilu02_numericBoost<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        info: bsrilu02Info_t,
        enable_boost: ::std::os::raw::c_int,
        mut tol: T,
        mut boost_val: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrilu02_numericBoost(
                self.handle,
                info,
                enable_boost,
                tol.as_mut_ptr() as *mut f64,
                boost_val.as_mut_ptr() as *mut cuDoubleComplex,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXbsrilu02_zeroPivot<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        info: bsrilu02Info_t,
        mut position: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXbsrilu02_zeroPivot(
                self.handle,
                info,
                position.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrilu02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrilu02_bufferSize(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrilu02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrilu02_bufferSize(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrilu02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrilu02_bufferSize(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrilu02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrilu02_bufferSize(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrilu02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrilu02_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrilu02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrilu02_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrilu02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrilu02_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrilu02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrilu02_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrilu02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrilu02_analysis(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrilu02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrilu02_analysis(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrilu02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrilu02_analysis(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrilu02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrilu02_analysis(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsrilu02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsrilu02(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsrilu02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsrilu02(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsrilu02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsrilu02(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsrilu02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsrilu02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsrilu02(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcsric02_zeroPivot<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        info: csric02Info_t,
        mut position: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcsric02_zeroPivot(
                self.handle,
                info,
                position.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsric02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsric02_bufferSize(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_mut_ptr() as *mut f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsric02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsric02_bufferSize(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_mut_ptr() as *mut f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsric02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsric02_bufferSize(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_mut_ptr() as *mut cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsric02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsric02_bufferSize(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsric02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedVal: T,
        csrSortedRowPtr: U,
        csrSortedColInd: V,
        info: csric02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsric02_bufferSizeExt(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedVal.as_mut_ptr() as *mut f32,
                csrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsric02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedVal: T,
        csrSortedRowPtr: U,
        csrSortedColInd: V,
        info: csric02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsric02_bufferSizeExt(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedVal.as_mut_ptr() as *mut f64,
                csrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsric02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedVal: T,
        csrSortedRowPtr: U,
        csrSortedColInd: V,
        info: csric02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsric02_bufferSizeExt(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedVal.as_mut_ptr() as *mut cuComplex,
                csrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsric02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedVal: T,
        csrSortedRowPtr: U,
        csrSortedColInd: V,
        info: csric02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsric02_bufferSizeExt(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsric02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsric02_analysis(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsric02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsric02_analysis(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsric02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsric02_analysis(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsric02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsric02_analysis(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsric02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA_valM: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsric02(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA_valM.as_mut_ptr() as *mut f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsric02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA_valM: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsric02(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA_valM.as_mut_ptr() as *mut f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsric02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA_valM: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsric02(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA_valM.as_mut_ptr() as *mut cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsric02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrSortedValA_valM: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        info: csric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsric02(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA_valM.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXbsric02_zeroPivot<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        info: bsric02Info_t,
        mut position: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXbsric02_zeroPivot(
                self.handle,
                info,
                position.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsric02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsric02_bufferSize(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsric02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsric02_bufferSize(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsric02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsric02_bufferSize(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsric02_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsric02_bufferSize(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsric02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsric02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsric02_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsric02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsric02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsric02_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsric02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsric02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsric02_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsric02_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockSize: ::std::os::raw::c_int,
        info: bsric02Info_t,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsric02_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockSize,
                info,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsric02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pInputBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsric02_analysis(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_const_ptr() as *const f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pInputBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsric02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pInputBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsric02_analysis(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_const_ptr() as *const f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pInputBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsric02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pInputBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsric02_analysis(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pInputBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsric02_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pInputBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsric02_analysis(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pInputBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsric02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsric02(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsric02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsric02(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsric02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsric02(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsric02<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        blockDim: ::std::os::raw::c_int,
        info: bsric02Info_t,
        policy: cusparseSolvePolicy_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsric02(
                self.handle,
                dirA,
                mb,
                nnzb,
                descrA,
                bsrSortedVal.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                info,
                policy,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgtsv2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgtsv2_bufferSizeExt(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const f32,
                d.as_const_ptr() as *const f32,
                du.as_const_ptr() as *const f32,
                B.as_const_ptr() as *const f32,
                ldb,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgtsv2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgtsv2_bufferSizeExt(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const f64,
                d.as_const_ptr() as *const f64,
                du.as_const_ptr() as *const f64,
                B.as_const_ptr() as *const f64,
                ldb,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgtsv2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgtsv2_bufferSizeExt(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const cuComplex,
                d.as_const_ptr() as *const cuComplex,
                du.as_const_ptr() as *const cuComplex,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgtsv2_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgtsv2_bufferSizeExt(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const cuDoubleComplex,
                d.as_const_ptr() as *const cuDoubleComplex,
                du.as_const_ptr() as *const cuDoubleComplex,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgtsv2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut B: W,
        ldb: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgtsv2(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const f32,
                d.as_const_ptr() as *const f32,
                du.as_const_ptr() as *const f32,
                B.as_mut_ptr() as *mut f32,
                ldb,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgtsv2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut B: W,
        ldb: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgtsv2(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const f64,
                d.as_const_ptr() as *const f64,
                du.as_const_ptr() as *const f64,
                B.as_mut_ptr() as *mut f64,
                ldb,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgtsv2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut B: W,
        ldb: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgtsv2(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const cuComplex,
                d.as_const_ptr() as *const cuComplex,
                du.as_const_ptr() as *const cuComplex,
                B.as_mut_ptr() as *mut cuComplex,
                ldb,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgtsv2<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut B: W,
        ldb: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgtsv2(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const cuDoubleComplex,
                d.as_const_ptr() as *const cuDoubleComplex,
                du.as_const_ptr() as *const cuDoubleComplex,
                B.as_mut_ptr() as *mut cuDoubleComplex,
                ldb,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgtsv2_nopivot_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgtsv2_nopivot_bufferSizeExt(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const f32,
                d.as_const_ptr() as *const f32,
                du.as_const_ptr() as *const f32,
                B.as_const_ptr() as *const f32,
                ldb,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgtsv2_nopivot_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgtsv2_nopivot_bufferSizeExt(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const f64,
                d.as_const_ptr() as *const f64,
                du.as_const_ptr() as *const f64,
                B.as_const_ptr() as *const f64,
                ldb,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgtsv2_nopivot_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgtsv2_nopivot_bufferSizeExt(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const cuComplex,
                d.as_const_ptr() as *const cuComplex,
                du.as_const_ptr() as *const cuComplex,
                B.as_const_ptr() as *const cuComplex,
                ldb,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgtsv2_nopivot_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        B: W,
        ldb: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgtsv2_nopivot_bufferSizeExt(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const cuDoubleComplex,
                d.as_const_ptr() as *const cuDoubleComplex,
                du.as_const_ptr() as *const cuDoubleComplex,
                B.as_const_ptr() as *const cuDoubleComplex,
                ldb,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgtsv2_nopivot<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut B: W,
        ldb: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgtsv2_nopivot(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const f32,
                d.as_const_ptr() as *const f32,
                du.as_const_ptr() as *const f32,
                B.as_mut_ptr() as *mut f32,
                ldb,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgtsv2_nopivot<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut B: W,
        ldb: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgtsv2_nopivot(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const f64,
                d.as_const_ptr() as *const f64,
                du.as_const_ptr() as *const f64,
                B.as_mut_ptr() as *mut f64,
                ldb,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgtsv2_nopivot<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut B: W,
        ldb: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgtsv2_nopivot(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const cuComplex,
                d.as_const_ptr() as *const cuComplex,
                du.as_const_ptr() as *const cuComplex,
                B.as_mut_ptr() as *mut cuComplex,
                ldb,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgtsv2_nopivot<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut B: W,
        ldb: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgtsv2_nopivot(
                self.handle,
                m,
                n,
                dl.as_const_ptr() as *const cuDoubleComplex,
                d.as_const_ptr() as *const cuDoubleComplex,
                du.as_const_ptr() as *const cuDoubleComplex,
                B.as_mut_ptr() as *mut cuDoubleComplex,
                ldb,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgtsv2StridedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        x: W,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgtsv2StridedBatch_bufferSizeExt(
                self.handle,
                m,
                dl.as_const_ptr() as *const f32,
                d.as_const_ptr() as *const f32,
                du.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                batchCount,
                batchStride,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgtsv2StridedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        x: W,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgtsv2StridedBatch_bufferSizeExt(
                self.handle,
                m,
                dl.as_const_ptr() as *const f64,
                d.as_const_ptr() as *const f64,
                du.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                batchCount,
                batchStride,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgtsv2StridedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        x: W,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgtsv2StridedBatch_bufferSizeExt(
                self.handle,
                m,
                dl.as_const_ptr() as *const cuComplex,
                d.as_const_ptr() as *const cuComplex,
                du.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                batchCount,
                batchStride,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgtsv2StridedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        x: W,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        mut bufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgtsv2StridedBatch_bufferSizeExt(
                self.handle,
                m,
                dl.as_const_ptr() as *const cuDoubleComplex,
                d.as_const_ptr() as *const cuDoubleComplex,
                du.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                batchCount,
                batchStride,
                bufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgtsv2StridedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut x: W,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgtsv2StridedBatch(
                self.handle,
                m,
                dl.as_const_ptr() as *const f32,
                d.as_const_ptr() as *const f32,
                du.as_const_ptr() as *const f32,
                x.as_mut_ptr() as *mut f32,
                batchCount,
                batchStride,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgtsv2StridedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut x: W,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgtsv2StridedBatch(
                self.handle,
                m,
                dl.as_const_ptr() as *const f64,
                d.as_const_ptr() as *const f64,
                du.as_const_ptr() as *const f64,
                x.as_mut_ptr() as *mut f64,
                batchCount,
                batchStride,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgtsv2StridedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut x: W,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgtsv2StridedBatch(
                self.handle,
                m,
                dl.as_const_ptr() as *const cuComplex,
                d.as_const_ptr() as *const cuComplex,
                du.as_const_ptr() as *const cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                batchCount,
                batchStride,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgtsv2StridedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        mut x: W,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgtsv2StridedBatch(
                self.handle,
                m,
                dl.as_const_ptr() as *const cuDoubleComplex,
                d.as_const_ptr() as *const cuDoubleComplex,
                du.as_const_ptr() as *const cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                batchCount,
                batchStride,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgtsvInterleavedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        x: W,
        batchCount: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgtsvInterleavedBatch_bufferSizeExt(
                self.handle,
                algo,
                m,
                dl.as_const_ptr() as *const f32,
                d.as_const_ptr() as *const f32,
                du.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                batchCount,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgtsvInterleavedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        x: W,
        batchCount: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgtsvInterleavedBatch_bufferSizeExt(
                self.handle,
                algo,
                m,
                dl.as_const_ptr() as *const f64,
                d.as_const_ptr() as *const f64,
                du.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                batchCount,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgtsvInterleavedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        x: W,
        batchCount: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgtsvInterleavedBatch_bufferSizeExt(
                self.handle,
                algo,
                m,
                dl.as_const_ptr() as *const cuComplex,
                d.as_const_ptr() as *const cuComplex,
                du.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                batchCount,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgtsvInterleavedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        dl: T,
        d: U,
        du: V,
        x: W,
        batchCount: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgtsvInterleavedBatch_bufferSizeExt(
                self.handle,
                algo,
                m,
                dl.as_const_ptr() as *const cuDoubleComplex,
                d.as_const_ptr() as *const cuDoubleComplex,
                du.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                batchCount,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgtsvInterleavedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut dl: T,
        mut d: U,
        mut du: V,
        mut x: W,
        batchCount: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgtsvInterleavedBatch(
                self.handle,
                algo,
                m,
                dl.as_mut_ptr() as *mut f32,
                d.as_mut_ptr() as *mut f32,
                du.as_mut_ptr() as *mut f32,
                x.as_mut_ptr() as *mut f32,
                batchCount,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgtsvInterleavedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut dl: T,
        mut d: U,
        mut du: V,
        mut x: W,
        batchCount: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgtsvInterleavedBatch(
                self.handle,
                algo,
                m,
                dl.as_mut_ptr() as *mut f64,
                d.as_mut_ptr() as *mut f64,
                du.as_mut_ptr() as *mut f64,
                x.as_mut_ptr() as *mut f64,
                batchCount,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgtsvInterleavedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut dl: T,
        mut d: U,
        mut du: V,
        mut x: W,
        batchCount: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgtsvInterleavedBatch(
                self.handle,
                algo,
                m,
                dl.as_mut_ptr() as *mut cuComplex,
                d.as_mut_ptr() as *mut cuComplex,
                du.as_mut_ptr() as *mut cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                batchCount,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgtsvInterleavedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut dl: T,
        mut d: U,
        mut du: V,
        mut x: W,
        batchCount: ::std::os::raw::c_int,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgtsvInterleavedBatch(
                self.handle,
                algo,
                m,
                dl.as_mut_ptr() as *mut cuDoubleComplex,
                d.as_mut_ptr() as *mut cuDoubleComplex,
                du.as_mut_ptr() as *mut cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                batchCount,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgpsvInterleavedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        ds: T,
        dl: U,
        d: V,
        du: W,
        dw: X,
        x: Y,
        batchCount: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgpsvInterleavedBatch_bufferSizeExt(
                self.handle,
                algo,
                m,
                ds.as_const_ptr() as *const f32,
                dl.as_const_ptr() as *const f32,
                d.as_const_ptr() as *const f32,
                du.as_const_ptr() as *const f32,
                dw.as_const_ptr() as *const f32,
                x.as_const_ptr() as *const f32,
                batchCount,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgpsvInterleavedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        ds: T,
        dl: U,
        d: V,
        du: W,
        dw: X,
        x: Y,
        batchCount: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgpsvInterleavedBatch_bufferSizeExt(
                self.handle,
                algo,
                m,
                ds.as_const_ptr() as *const f64,
                dl.as_const_ptr() as *const f64,
                d.as_const_ptr() as *const f64,
                du.as_const_ptr() as *const f64,
                dw.as_const_ptr() as *const f64,
                x.as_const_ptr() as *const f64,
                batchCount,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgpsvInterleavedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        ds: T,
        dl: U,
        d: V,
        du: W,
        dw: X,
        x: Y,
        batchCount: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgpsvInterleavedBatch_bufferSizeExt(
                self.handle,
                algo,
                m,
                ds.as_const_ptr() as *const cuComplex,
                dl.as_const_ptr() as *const cuComplex,
                d.as_const_ptr() as *const cuComplex,
                du.as_const_ptr() as *const cuComplex,
                dw.as_const_ptr() as *const cuComplex,
                x.as_const_ptr() as *const cuComplex,
                batchCount,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgpsvInterleavedBatch_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        ds: T,
        dl: U,
        d: V,
        du: W,
        dw: X,
        x: Y,
        batchCount: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgpsvInterleavedBatch_bufferSizeExt(
                self.handle,
                algo,
                m,
                ds.as_const_ptr() as *const cuDoubleComplex,
                dl.as_const_ptr() as *const cuDoubleComplex,
                d.as_const_ptr() as *const cuDoubleComplex,
                du.as_const_ptr() as *const cuDoubleComplex,
                dw.as_const_ptr() as *const cuDoubleComplex,
                x.as_const_ptr() as *const cuDoubleComplex,
                batchCount,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgpsvInterleavedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut ds: T,
        mut dl: U,
        mut d: V,
        mut du: W,
        mut dw: X,
        mut x: Y,
        batchCount: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgpsvInterleavedBatch(
                self.handle,
                algo,
                m,
                ds.as_mut_ptr() as *mut f32,
                dl.as_mut_ptr() as *mut f32,
                d.as_mut_ptr() as *mut f32,
                du.as_mut_ptr() as *mut f32,
                dw.as_mut_ptr() as *mut f32,
                x.as_mut_ptr() as *mut f32,
                batchCount,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgpsvInterleavedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut ds: T,
        mut dl: U,
        mut d: V,
        mut du: W,
        mut dw: X,
        mut x: Y,
        batchCount: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgpsvInterleavedBatch(
                self.handle,
                algo,
                m,
                ds.as_mut_ptr() as *mut f64,
                dl.as_mut_ptr() as *mut f64,
                d.as_mut_ptr() as *mut f64,
                du.as_mut_ptr() as *mut f64,
                dw.as_mut_ptr() as *mut f64,
                x.as_mut_ptr() as *mut f64,
                batchCount,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgpsvInterleavedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut ds: T,
        mut dl: U,
        mut d: V,
        mut du: W,
        mut dw: X,
        mut x: Y,
        batchCount: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgpsvInterleavedBatch(
                self.handle,
                algo,
                m,
                ds.as_mut_ptr() as *mut cuComplex,
                dl.as_mut_ptr() as *mut cuComplex,
                d.as_mut_ptr() as *mut cuComplex,
                du.as_mut_ptr() as *mut cuComplex,
                dw.as_mut_ptr() as *mut cuComplex,
                x.as_mut_ptr() as *mut cuComplex,
                batchCount,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgpsvInterleavedBatch<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut ds: T,
        mut dl: U,
        mut d: V,
        mut du: W,
        mut dw: X,
        mut x: Y,
        batchCount: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgpsvInterleavedBatch(
                self.handle,
                algo,
                m,
                ds.as_mut_ptr() as *mut cuDoubleComplex,
                dl.as_mut_ptr() as *mut cuDoubleComplex,
                d.as_mut_ptr() as *mut cuDoubleComplex,
                du.as_mut_ptr() as *mut cuDoubleComplex,
                dw.as_mut_ptr() as *mut cuDoubleComplex,
                x.as_mut_ptr() as *mut cuDoubleComplex,
                batchCount,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsrgeam2_bufferSizeExt<
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
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        nnzA: ::std::os::raw::c_int,
        csrSortedValA: U,
        csrSortedRowPtrA: V,
        csrSortedColIndA: W,
        beta: X,
        descrB: cusparseMatDescr_t,
        nnzB: ::std::os::raw::c_int,
        csrSortedValB: Y,
        csrSortedRowPtrB: Z,
        csrSortedColIndB: A,
        descrC: cusparseMatDescr_t,
        csrSortedValC: B,
        csrSortedRowPtrC: C,
        csrSortedColIndC: D,
        mut pBufferSizeInBytes: E,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsrgeam2_bufferSizeExt(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                descrA,
                nnzA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const f32,
                descrB,
                nnzB,
                csrSortedValB.as_const_ptr() as *const f32,
                csrSortedRowPtrB.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndB.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                csrSortedValC.as_const_ptr() as *const f32,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsrgeam2_bufferSizeExt<
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
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        nnzA: ::std::os::raw::c_int,
        csrSortedValA: U,
        csrSortedRowPtrA: V,
        csrSortedColIndA: W,
        beta: X,
        descrB: cusparseMatDescr_t,
        nnzB: ::std::os::raw::c_int,
        csrSortedValB: Y,
        csrSortedRowPtrB: Z,
        csrSortedColIndB: A,
        descrC: cusparseMatDescr_t,
        csrSortedValC: B,
        csrSortedRowPtrC: C,
        csrSortedColIndC: D,
        mut pBufferSizeInBytes: E,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsrgeam2_bufferSizeExt(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                descrA,
                nnzA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const f64,
                descrB,
                nnzB,
                csrSortedValB.as_const_ptr() as *const f64,
                csrSortedRowPtrB.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndB.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                csrSortedValC.as_const_ptr() as *const f64,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsrgeam2_bufferSizeExt<
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
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        nnzA: ::std::os::raw::c_int,
        csrSortedValA: U,
        csrSortedRowPtrA: V,
        csrSortedColIndA: W,
        beta: X,
        descrB: cusparseMatDescr_t,
        nnzB: ::std::os::raw::c_int,
        csrSortedValB: Y,
        csrSortedRowPtrB: Z,
        csrSortedColIndB: A,
        descrC: cusparseMatDescr_t,
        csrSortedValC: B,
        csrSortedRowPtrC: C,
        csrSortedColIndC: D,
        mut pBufferSizeInBytes: E,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsrgeam2_bufferSizeExt(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                descrA,
                nnzA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const cuComplex,
                descrB,
                nnzB,
                csrSortedValB.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrB.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndB.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                csrSortedValC.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsrgeam2_bufferSizeExt<
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
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        nnzA: ::std::os::raw::c_int,
        csrSortedValA: U,
        csrSortedRowPtrA: V,
        csrSortedColIndA: W,
        beta: X,
        descrB: cusparseMatDescr_t,
        nnzB: ::std::os::raw::c_int,
        csrSortedValB: Y,
        csrSortedRowPtrB: Z,
        csrSortedColIndB: A,
        descrC: cusparseMatDescr_t,
        csrSortedValC: B,
        csrSortedRowPtrC: C,
        csrSortedColIndC: D,
        mut pBufferSizeInBytes: E,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsrgeam2_bufferSizeExt(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                descrA,
                nnzA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const cuDoubleComplex,
                descrB,
                nnzB,
                csrSortedValB.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrB.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndB.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                csrSortedValC.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcsrgeam2Nnz<
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
        descrA: cusparseMatDescr_t,
        nnzA: ::std::os::raw::c_int,
        csrSortedRowPtrA: T,
        csrSortedColIndA: U,
        descrB: cusparseMatDescr_t,
        nnzB: ::std::os::raw::c_int,
        csrSortedRowPtrB: V,
        csrSortedColIndB: W,
        descrC: cusparseMatDescr_t,
        mut csrSortedRowPtrC: X,
        mut nnzTotalDevHostPtr: Y,
        mut workspace: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcsrgeam2Nnz(
                self.handle,
                m,
                n,
                descrA,
                nnzA,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                descrB,
                nnzB,
                csrSortedRowPtrB.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndB.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsrgeam2<
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
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        nnzA: ::std::os::raw::c_int,
        csrSortedValA: U,
        csrSortedRowPtrA: V,
        csrSortedColIndA: W,
        beta: X,
        descrB: cusparseMatDescr_t,
        nnzB: ::std::os::raw::c_int,
        csrSortedValB: Y,
        csrSortedRowPtrB: Z,
        csrSortedColIndB: A,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: B,
        mut csrSortedRowPtrC: C,
        mut csrSortedColIndC: D,
        mut pBuffer: E,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsrgeam2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const f32,
                descrA,
                nnzA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const f32,
                descrB,
                nnzB,
                csrSortedValB.as_const_ptr() as *const f32,
                csrSortedRowPtrB.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndB.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f32,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsrgeam2<
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
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        nnzA: ::std::os::raw::c_int,
        csrSortedValA: U,
        csrSortedRowPtrA: V,
        csrSortedColIndA: W,
        beta: X,
        descrB: cusparseMatDescr_t,
        nnzB: ::std::os::raw::c_int,
        csrSortedValB: Y,
        csrSortedRowPtrB: Z,
        csrSortedColIndB: A,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: B,
        mut csrSortedRowPtrC: C,
        mut csrSortedColIndC: D,
        mut pBuffer: E,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsrgeam2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const f64,
                descrA,
                nnzA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const f64,
                descrB,
                nnzB,
                csrSortedValB.as_const_ptr() as *const f64,
                csrSortedRowPtrB.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndB.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f64,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsrgeam2<
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
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        nnzA: ::std::os::raw::c_int,
        csrSortedValA: U,
        csrSortedRowPtrA: V,
        csrSortedColIndA: W,
        beta: X,
        descrB: cusparseMatDescr_t,
        nnzB: ::std::os::raw::c_int,
        csrSortedValB: Y,
        csrSortedRowPtrB: Z,
        csrSortedColIndB: A,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: B,
        mut csrSortedRowPtrC: C,
        mut csrSortedColIndC: D,
        mut pBuffer: E,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsrgeam2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuComplex,
                descrA,
                nnzA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const cuComplex,
                descrB,
                nnzB,
                csrSortedValB.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrB.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndB.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut cuComplex,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsrgeam2<
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
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: T,
        descrA: cusparseMatDescr_t,
        nnzA: ::std::os::raw::c_int,
        csrSortedValA: U,
        csrSortedRowPtrA: V,
        csrSortedColIndA: W,
        beta: X,
        descrB: cusparseMatDescr_t,
        nnzB: ::std::os::raw::c_int,
        csrSortedValB: Y,
        csrSortedRowPtrB: Z,
        csrSortedColIndB: A,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: B,
        mut csrSortedRowPtrC: C,
        mut csrSortedColIndC: D,
        mut pBuffer: E,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsrgeam2(
                self.handle,
                m,
                n,
                alpha.as_const_ptr() as *const cuDoubleComplex,
                descrA,
                nnzA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                beta.as_const_ptr() as *const cuDoubleComplex,
                descrB,
                nnzB,
                csrSortedValB.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrB.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndB.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsrcolor<
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
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        fractionToColor: W,
        mut ncolors: X,
        mut coloring: Y,
        mut reordering: Z,
        info: cusparseColorInfo_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsrcolor(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                fractionToColor.as_const_ptr() as *const f32,
                ncolors.as_mut_ptr() as *mut ::std::os::raw::c_int,
                coloring.as_mut_ptr() as *mut ::std::os::raw::c_int,
                reordering.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsrcolor<
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
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        fractionToColor: W,
        mut ncolors: X,
        mut coloring: Y,
        mut reordering: Z,
        info: cusparseColorInfo_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsrcolor(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                fractionToColor.as_const_ptr() as *const f64,
                ncolors.as_mut_ptr() as *mut ::std::os::raw::c_int,
                coloring.as_mut_ptr() as *mut ::std::os::raw::c_int,
                reordering.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsrcolor<
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
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        fractionToColor: W,
        mut ncolors: X,
        mut coloring: Y,
        mut reordering: Z,
        info: cusparseColorInfo_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsrcolor(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                fractionToColor.as_const_ptr() as *const f32,
                ncolors.as_mut_ptr() as *mut ::std::os::raw::c_int,
                coloring.as_mut_ptr() as *mut ::std::os::raw::c_int,
                reordering.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsrcolor<
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
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        fractionToColor: W,
        mut ncolors: X,
        mut coloring: Y,
        mut reordering: Z,
        info: cusparseColorInfo_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsrcolor(
                self.handle,
                m,
                nnz,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                fractionToColor.as_const_ptr() as *const f64,
                ncolors.as_mut_ptr() as *mut ::std::os::raw::c_int,
                coloring.as_mut_ptr() as *mut ::std::os::raw::c_int,
                reordering.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSnnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        A: T,
        lda: ::std::os::raw::c_int,
        mut nnzPerRowCol: U,
        mut nnzTotalDevHostPtr: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSnnz(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                A.as_const_ptr() as *const f32,
                lda,
                nnzPerRowCol.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDnnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        A: T,
        lda: ::std::os::raw::c_int,
        mut nnzPerRowCol: U,
        mut nnzTotalDevHostPtr: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDnnz(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                A.as_const_ptr() as *const f64,
                lda,
                nnzPerRowCol.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCnnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        A: T,
        lda: ::std::os::raw::c_int,
        mut nnzPerRowCol: U,
        mut nnzTotalDevHostPtr: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCnnz(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                A.as_const_ptr() as *const cuComplex,
                lda,
                nnzPerRowCol.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZnnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        A: T,
        lda: ::std::os::raw::c_int,
        mut nnzPerRowCol: U,
        mut nnzTotalDevHostPtr: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZnnz(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                A.as_const_ptr() as *const cuDoubleComplex,
                lda,
                nnzPerRowCol.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSnnz_compress<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        descr: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        mut nnzPerRow: V,
        mut nnzC: W,
        tol: f32,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSnnz_compress(
                self.handle,
                m,
                descr,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                nnzPerRow.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                tol,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDnnz_compress<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        descr: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        mut nnzPerRow: V,
        mut nnzC: W,
        tol: f64,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDnnz_compress(
                self.handle,
                m,
                descr,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                nnzPerRow.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                tol,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCnnz_compress<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        descr: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        mut nnzPerRow: V,
        mut nnzC: W,
        tol: cuComplex,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCnnz_compress(
                self.handle,
                m,
                descr,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                nnzPerRow.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                tol,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZnnz_compress<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        descr: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        mut nnzPerRow: V,
        mut nnzC: W,
        tol: cuDoubleComplex,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZnnz_compress(
                self.handle,
                m,
                descr,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                nnzPerRow.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                tol,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsr2csr_compress<
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
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedColIndA: U,
        csrSortedRowPtrA: V,
        nnzA: ::std::os::raw::c_int,
        nnzPerRow: W,
        mut csrSortedValC: X,
        mut csrSortedColIndC: Y,
        mut csrSortedRowPtrC: Z,
        tol: f32,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsr2csr_compress(
                self.handle,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                nnzA,
                nnzPerRow.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedValC.as_mut_ptr() as *mut f32,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                tol,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsr2csr_compress<
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
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedColIndA: U,
        csrSortedRowPtrA: V,
        nnzA: ::std::os::raw::c_int,
        nnzPerRow: W,
        mut csrSortedValC: X,
        mut csrSortedColIndC: Y,
        mut csrSortedRowPtrC: Z,
        tol: f64,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsr2csr_compress(
                self.handle,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                nnzA,
                nnzPerRow.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedValC.as_mut_ptr() as *mut f64,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                tol,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsr2csr_compress<
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
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedColIndA: U,
        csrSortedRowPtrA: V,
        nnzA: ::std::os::raw::c_int,
        nnzPerRow: W,
        mut csrSortedValC: X,
        mut csrSortedColIndC: Y,
        mut csrSortedRowPtrC: Z,
        tol: cuComplex,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsr2csr_compress(
                self.handle,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                nnzA,
                nnzPerRow.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedValC.as_mut_ptr() as *mut cuComplex,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                tol,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsr2csr_compress<
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
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedColIndA: U,
        csrSortedRowPtrA: V,
        nnzA: ::std::os::raw::c_int,
        nnzPerRow: W,
        mut csrSortedValC: X,
        mut csrSortedColIndC: Y,
        mut csrSortedRowPtrC: Z,
        tol: cuDoubleComplex,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsr2csr_compress(
                self.handle,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                nnzA,
                nnzPerRow.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedValC.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                tol,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcoo2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        cooRowInd: T,
        nnz: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut csrSortedRowPtr: U,
        idxBase: cusparseIndexBase_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcoo2csr(
                self.handle,
                cooRowInd.as_const_ptr() as *const ::std::os::raw::c_int,
                nnz,
                m,
                csrSortedRowPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                idxBase,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcsr2coo<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        csrSortedRowPtr: T,
        nnz: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        mut cooRowInd: U,
        idxBase: cusparseIndexBase_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcsr2coo(
                self.handle,
                csrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                nnz,
                m,
                cooRowInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                idxBase,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcsr2bsrNnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedRowPtrA: T,
        csrSortedColIndA: U,
        blockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedRowPtrC: V,
        mut nnzTotalDevHostPtr: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcsr2bsrNnz(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                descrC,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsr2bsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsr2bsr(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut f32,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsr2bsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsr2bsr(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut f64,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsr2bsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsr2bsr(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsr2bsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsr2bsr(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSbsr2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        mut csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSbsr2csr(
                self.handle,
                dirA,
                mb,
                nb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f32,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDbsr2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        mut csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDbsr2csr(
                self.handle,
                dirA,
                mb,
                nb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f64,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCbsr2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        mut csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCbsr2csr(
                self.handle,
                dirA,
                mb,
                nb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut cuComplex,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZbsr2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        blockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        mut csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZbsr2csr(
                self.handle,
                dirA,
                mb,
                nb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                blockDim,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgebsr2gebsc_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgebsr2gebsc_bufferSize(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgebsr2gebsc_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgebsr2gebsc_bufferSize(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgebsr2gebsc_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgebsr2gebsc_bufferSize(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgebsr2gebsc_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgebsr2gebsc_bufferSize(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgebsr2gebsc_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgebsr2gebsc_bufferSizeExt(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgebsr2gebsc_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgebsr2gebsc_bufferSizeExt(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgebsr2gebsc_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgebsr2gebsc_bufferSizeExt(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgebsr2gebsc_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgebsr2gebsc_bufferSizeExt(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgebsr2gebsc<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut bscVal: W,
        mut bscRowInd: X,
        mut bscColPtr: Y,
        copyValues: cusparseAction_t,
        idxBase: cusparseIndexBase_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgebsr2gebsc(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const f32,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                bscVal.as_mut_ptr() as *mut f32,
                bscRowInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bscColPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                copyValues,
                idxBase,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgebsr2gebsc<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut bscVal: W,
        mut bscRowInd: X,
        mut bscColPtr: Y,
        copyValues: cusparseAction_t,
        idxBase: cusparseIndexBase_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgebsr2gebsc(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const f64,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                bscVal.as_mut_ptr() as *mut f64,
                bscRowInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bscColPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                copyValues,
                idxBase,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgebsr2gebsc<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut bscVal: W,
        mut bscRowInd: X,
        mut bscColPtr: Y,
        copyValues: cusparseAction_t,
        idxBase: cusparseIndexBase_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgebsr2gebsc(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                bscVal.as_mut_ptr() as *mut cuComplex,
                bscRowInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bscColPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                copyValues,
                idxBase,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgebsr2gebsc<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        bsrSortedVal: T,
        bsrSortedRowPtr: U,
        bsrSortedColInd: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut bscVal: W,
        mut bscRowInd: X,
        mut bscColPtr: Y,
        copyValues: cusparseAction_t,
        idxBase: cusparseIndexBase_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgebsr2gebsc(
                self.handle,
                mb,
                nb,
                nnzb,
                bsrSortedVal.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                bscVal.as_mut_ptr() as *mut cuDoubleComplex,
                bscRowInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bscColPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                copyValues,
                idxBase,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXgebsr2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedRowPtrA: T,
        bsrSortedColIndA: U,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut csrSortedRowPtrC: V,
        mut csrSortedColIndC: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXgebsr2csr(
                self.handle,
                dirA,
                mb,
                nb,
                descrA,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                descrC,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgebsr2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        mut csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgebsr2csr(
                self.handle,
                dirA,
                mb,
                nb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f32,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgebsr2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        mut csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgebsr2csr(
                self.handle,
                dirA,
                mb,
                nb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f64,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgebsr2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        mut csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgebsr2csr(
                self.handle,
                dirA,
                mb,
                nb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut cuComplex,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgebsr2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        mut csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgebsr2csr(
                self.handle,
                dirA,
                mb,
                nb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut cuDoubleComplex,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsr2gebsr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsr2gebsr_bufferSize(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsr2gebsr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsr2gebsr_bufferSize(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsr2gebsr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsr2gebsr_bufferSize(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsr2gebsr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsr2gebsr_bufferSize(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsr2gebsr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsr2gebsr_bufferSizeExt(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsr2gebsr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsr2gebsr_bufferSizeExt(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsr2gebsr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsr2gebsr_bufferSizeExt(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsr2gebsr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsr2gebsr_bufferSizeExt(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcsr2gebsrNnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedRowPtrA: T,
        csrSortedColIndA: U,
        descrC: cusparseMatDescr_t,
        mut bsrSortedRowPtrC: V,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut nnzTotalDevHostPtr: W,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcsr2gebsrNnz(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsr2gebsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsr2gebsr(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut f32,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsr2gebsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsr2gebsr(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut f64,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsr2gebsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsr2gebsr(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsr2gebsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
        rowBlockDim: ::std::os::raw::c_int,
        colBlockDim: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsr2gebsr(
                self.handle,
                dirA,
                m,
                n,
                descrA,
                csrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDim,
                colBlockDim,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgebsr2gebsr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgebsr2gebsr_bufferSize(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                rowBlockDimC,
                colBlockDimC,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgebsr2gebsr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgebsr2gebsr_bufferSize(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                rowBlockDimC,
                colBlockDimC,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgebsr2gebsr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgebsr2gebsr_bufferSize(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                rowBlockDimC,
                colBlockDimC,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgebsr2gebsr_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgebsr2gebsr_bufferSize(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                rowBlockDimC,
                colBlockDimC,
                pBufferSizeInBytes.as_mut_ptr() as *mut ::std::os::raw::c_int,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgebsr2gebsr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgebsr2gebsr_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                rowBlockDimC,
                colBlockDimC,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgebsr2gebsr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgebsr2gebsr_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                rowBlockDimC,
                colBlockDimC,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgebsr2gebsr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgebsr2gebsr_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                rowBlockDimC,
                colBlockDimC,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgebsr2gebsr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBufferSize: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgebsr2gebsr_bufferSizeExt(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                rowBlockDimC,
                colBlockDimC,
                pBufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXgebsr2gebsrNnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedRowPtrA: T,
        bsrSortedColIndA: U,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedRowPtrC: V,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut nnzTotalDevHostPtr: W,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXgebsr2gebsrNnz(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDimC,
                colBlockDimC,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSgebsr2gebsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSgebsr2gebsr(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f32,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut f32,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDimC,
                colBlockDimC,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDgebsr2gebsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDgebsr2gebsr(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const f64,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut f64,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDimC,
                colBlockDimC,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCgebsr2gebsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCgebsr2gebsr(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut cuComplex,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDimC,
                colBlockDimC,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZgebsr2gebsr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dirA: cusparseDirection_t,
        mb: ::std::os::raw::c_int,
        nb: ::std::os::raw::c_int,
        nnzb: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        bsrSortedValA: T,
        bsrSortedRowPtrA: U,
        bsrSortedColIndA: V,
        rowBlockDimA: ::std::os::raw::c_int,
        colBlockDimA: ::std::os::raw::c_int,
        descrC: cusparseMatDescr_t,
        mut bsrSortedValC: W,
        mut bsrSortedRowPtrC: X,
        mut bsrSortedColIndC: Y,
        rowBlockDimC: ::std::os::raw::c_int,
        colBlockDimC: ::std::os::raw::c_int,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZgebsr2gebsr(
                self.handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA.as_const_ptr() as *const cuDoubleComplex,
                bsrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                bsrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedValC.as_mut_ptr() as *mut cuDoubleComplex,
                bsrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                bsrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                rowBlockDimC,
                colBlockDimC,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcoosort_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        cooRowsA: T,
        cooColsA: U,
        mut pBufferSizeInBytes: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcoosort_bufferSizeExt(
                self.handle,
                m,
                n,
                nnz,
                cooRowsA.as_const_ptr() as *const ::std::os::raw::c_int,
                cooColsA.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcoosortByRow<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut cooRowsA: T,
        mut cooColsA: U,
        mut P: V,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcoosortByRow(
                self.handle,
                m,
                n,
                nnz,
                cooRowsA.as_mut_ptr() as *mut ::std::os::raw::c_int,
                cooColsA.as_mut_ptr() as *mut ::std::os::raw::c_int,
                P.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcoosortByColumn<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut cooRowsA: T,
        mut cooColsA: U,
        mut P: V,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcoosortByColumn(
                self.handle,
                m,
                n,
                nnz,
                cooRowsA.as_mut_ptr() as *mut ::std::os::raw::c_int,
                cooColsA.as_mut_ptr() as *mut ::std::os::raw::c_int,
                P.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcsrsort_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        csrRowPtrA: T,
        csrColIndA: U,
        mut pBufferSizeInBytes: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcsrsort_bufferSizeExt(
                self.handle,
                m,
                n,
                nnz,
                csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcsrsort<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrRowPtrA: T,
        mut csrColIndA: U,
        mut P: V,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcsrsort(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                csrRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColIndA.as_mut_ptr() as *mut ::std::os::raw::c_int,
                P.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcscsort_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        cscColPtrA: T,
        cscRowIndA: U,
        mut pBufferSizeInBytes: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcscsort_bufferSizeExt(
                self.handle,
                m,
                n,
                nnz,
                cscColPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                cscRowIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseXcscsort<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        cscColPtrA: T,
        mut cscRowIndA: U,
        mut P: V,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseXcscsort(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                cscColPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                cscRowIndA.as_mut_ptr() as *mut ::std::os::raw::c_int,
                P.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsru2csr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsru2csr_bufferSizeExt(
                self.handle,
                m,
                n,
                nnz,
                csrVal.as_mut_ptr() as *mut f32,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsru2csr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsru2csr_bufferSizeExt(
                self.handle,
                m,
                n,
                nnz,
                csrVal.as_mut_ptr() as *mut f64,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsru2csr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsru2csr_bufferSizeExt(
                self.handle,
                m,
                n,
                nnz,
                csrVal.as_mut_ptr() as *mut cuComplex,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsru2csr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBufferSizeInBytes: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsru2csr_bufferSizeExt(
                self.handle,
                m,
                n,
                nnz,
                csrVal.as_mut_ptr() as *mut cuDoubleComplex,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsru2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsru2csr(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                csrVal.as_mut_ptr() as *mut f32,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsru2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsru2csr(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                csrVal.as_mut_ptr() as *mut f64,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsru2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsru2csr(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                csrVal.as_mut_ptr() as *mut cuComplex,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsru2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsru2csr(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                csrVal.as_mut_ptr() as *mut cuDoubleComplex,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScsr2csru<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseScsr2csru(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                csrVal.as_mut_ptr() as *mut f32,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDcsr2csru<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDcsr2csru(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                csrVal.as_mut_ptr() as *mut f64,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCcsr2csru<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCcsr2csru(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                csrVal.as_mut_ptr() as *mut cuComplex,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseZcsr2csru<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        mut csrVal: T,
        csrRowPtr: U,
        mut csrColInd: V,
        info: csru2csrInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseZcsr2csru(
                self.handle,
                m,
                n,
                nnz,
                descrA,
                csrVal.as_mut_ptr() as *mut cuDoubleComplex,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneDense2csr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        threshold: U,
        descrC: cusparseMatDescr_t,
        csrSortedValC: V,
        csrSortedRowPtrC: W,
        csrSortedColIndC: X,
        mut pBufferSizeInBytes: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneDense2csr_bufferSizeExt(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                threshold.as_const_ptr() as *const f32,
                descrC,
                csrSortedValC.as_const_ptr() as *const f32,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneDense2csr_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        threshold: U,
        descrC: cusparseMatDescr_t,
        csrSortedValC: V,
        csrSortedRowPtrC: W,
        csrSortedColIndC: X,
        mut pBufferSizeInBytes: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneDense2csr_bufferSizeExt(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                threshold.as_const_ptr() as *const f64,
                descrC,
                csrSortedValC.as_const_ptr() as *const f64,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneDense2csrNnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        threshold: U,
        descrC: cusparseMatDescr_t,
        mut csrRowPtrC: V,
        mut nnzTotalDevHostPtr: W,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneDense2csrNnz(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                threshold.as_const_ptr() as *const f32,
                descrC,
                csrRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneDense2csrNnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        threshold: U,
        descrC: cusparseMatDescr_t,
        mut csrSortedRowPtrC: V,
        mut nnzTotalDevHostPtr: W,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneDense2csrNnz(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                threshold.as_const_ptr() as *const f64,
                descrC,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneDense2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        threshold: U,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: V,
        csrSortedRowPtrC: W,
        mut csrSortedColIndC: X,
        mut pBuffer: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneDense2csr(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                threshold.as_const_ptr() as *const f32,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f32,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneDense2csr<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        threshold: U,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: V,
        csrSortedRowPtrC: W,
        mut csrSortedColIndC: X,
        mut pBuffer: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneDense2csr(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                threshold.as_const_ptr() as *const f64,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f64,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneCsr2csr_bufferSizeExt<
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
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        threshold: W,
        descrC: cusparseMatDescr_t,
        csrSortedValC: X,
        csrSortedRowPtrC: Y,
        csrSortedColIndC: Z,
        mut pBufferSizeInBytes: A,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneCsr2csr_bufferSizeExt(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                threshold.as_const_ptr() as *const f32,
                descrC,
                csrSortedValC.as_const_ptr() as *const f32,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneCsr2csr_bufferSizeExt<
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
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        threshold: W,
        descrC: cusparseMatDescr_t,
        csrSortedValC: X,
        csrSortedRowPtrC: Y,
        csrSortedColIndC: Z,
        mut pBufferSizeInBytes: A,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneCsr2csr_bufferSizeExt(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                threshold.as_const_ptr() as *const f64,
                descrC,
                csrSortedValC.as_const_ptr() as *const f64,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneCsr2csrNnz<
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
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        threshold: W,
        descrC: cusparseMatDescr_t,
        mut csrSortedRowPtrC: X,
        mut nnzTotalDevHostPtr: Y,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneCsr2csrNnz(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                threshold.as_const_ptr() as *const f32,
                descrC,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneCsr2csrNnz<
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
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        threshold: W,
        descrC: cusparseMatDescr_t,
        mut csrSortedRowPtrC: X,
        mut nnzTotalDevHostPtr: Y,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneCsr2csrNnz(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                threshold.as_const_ptr() as *const f64,
                descrC,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneCsr2csr<
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
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        threshold: W,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: X,
        csrSortedRowPtrC: Y,
        mut csrSortedColIndC: Z,
        mut pBuffer: A,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneCsr2csr(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                threshold.as_const_ptr() as *const f32,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f32,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneCsr2csr<
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
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        threshold: W,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: X,
        csrSortedRowPtrC: Y,
        mut csrSortedColIndC: Z,
        mut pBuffer: A,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneCsr2csr(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                threshold.as_const_ptr() as *const f64,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f64,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneDense2csrByPercentage_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        csrSortedValC: U,
        csrSortedRowPtrC: V,
        csrSortedColIndC: W,
        info: pruneInfo_t,
        mut pBufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneDense2csrByPercentage_bufferSizeExt(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                percentage,
                descrC,
                csrSortedValC.as_const_ptr() as *const f32,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneDense2csrByPercentage_bufferSizeExt<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        csrSortedValC: U,
        csrSortedRowPtrC: V,
        csrSortedColIndC: W,
        info: pruneInfo_t,
        mut pBufferSizeInBytes: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneDense2csrByPercentage_bufferSizeExt(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                percentage,
                descrC,
                csrSortedValC.as_const_ptr() as *const f64,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneDense2csrNnzByPercentage<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        mut csrRowPtrC: U,
        mut nnzTotalDevHostPtr: V,
        info: pruneInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneDense2csrNnzByPercentage(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                percentage,
                descrC,
                csrRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneDense2csrNnzByPercentage<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        mut csrRowPtrC: U,
        mut nnzTotalDevHostPtr: V,
        info: pruneInfo_t,
        mut pBuffer: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneDense2csrNnzByPercentage(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                percentage,
                descrC,
                csrRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneDense2csrByPercentage<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: U,
        csrSortedRowPtrC: V,
        mut csrSortedColIndC: W,
        info: pruneInfo_t,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneDense2csrByPercentage(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f32,
                lda,
                percentage,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f32,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneDense2csrByPercentage<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: T,
        lda: ::std::os::raw::c_int,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: U,
        csrSortedRowPtrC: V,
        mut csrSortedColIndC: W,
        info: pruneInfo_t,
        mut pBuffer: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneDense2csrByPercentage(
                self.handle,
                m,
                n,
                A.as_const_ptr() as *const f64,
                lda,
                percentage,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f64,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneCsr2csrByPercentage_bufferSizeExt<
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
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        csrSortedValC: W,
        csrSortedRowPtrC: X,
        csrSortedColIndC: Y,
        info: pruneInfo_t,
        mut pBufferSizeInBytes: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneCsr2csrByPercentage_bufferSizeExt(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                percentage,
                descrC,
                csrSortedValC.as_const_ptr() as *const f32,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneCsr2csrByPercentage_bufferSizeExt<
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
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        csrSortedValC: W,
        csrSortedRowPtrC: X,
        csrSortedColIndC: Y,
        info: pruneInfo_t,
        mut pBufferSizeInBytes: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneCsr2csrByPercentage_bufferSizeExt(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                percentage,
                descrC,
                csrSortedValC.as_const_ptr() as *const f64,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_const_ptr() as *const ::std::os::raw::c_int,
                info,
                pBufferSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneCsr2csrNnzByPercentage<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        mut csrSortedRowPtrC: W,
        mut nnzTotalDevHostPtr: X,
        info: pruneInfo_t,
        mut pBuffer: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneCsr2csrNnzByPercentage(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                percentage,
                descrC,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneCsr2csrNnzByPercentage<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        mut csrSortedRowPtrC: W,
        mut nnzTotalDevHostPtr: X,
        info: pruneInfo_t,
        mut pBuffer: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneCsr2csrNnzByPercentage(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                percentage,
                descrC,
                csrSortedRowPtrC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpruneCsr2csrByPercentage<
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
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
        info: pruneInfo_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpruneCsr2csrByPercentage(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f32,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                percentage,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f32,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDpruneCsr2csrByPercentage<
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
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: T,
        csrSortedRowPtrA: U,
        csrSortedColIndA: V,
        percentage: f32,
        descrC: cusparseMatDescr_t,
        mut csrSortedValC: W,
        csrSortedRowPtrC: X,
        mut csrSortedColIndC: Y,
        info: pruneInfo_t,
        mut pBuffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDpruneCsr2csrByPercentage(
                self.handle,
                m,
                n,
                nnzA,
                descrA,
                csrSortedValA.as_const_ptr() as *const f64,
                csrSortedRowPtrA.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndA.as_const_ptr() as *const ::std::os::raw::c_int,
                percentage,
                descrC,
                csrSortedValC.as_mut_ptr() as *mut f64,
                csrSortedRowPtrC.as_const_ptr() as *const ::std::os::raw::c_int,
                csrSortedColIndC.as_mut_ptr() as *mut ::std::os::raw::c_int,
                info,
                pBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCsr2cscEx2<
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
        nnz: ::std::os::raw::c_int,
        csrVal: T,
        csrRowPtr: U,
        csrColInd: V,
        mut cscVal: W,
        mut cscColPtr: X,
        mut cscRowInd: Y,
        valType: cudaDataType,
        copyValues: cusparseAction_t,
        idxBase: cusparseIndexBase_t,
        alg: cusparseCsr2CscAlg_t,
        mut buffer: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCsr2cscEx2(
                self.handle,
                m,
                n,
                nnz,
                csrVal.as_const_ptr() as *const ::std::os::raw::c_void,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                cscVal.as_mut_ptr() as *mut ::std::os::raw::c_void,
                cscColPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                cscRowInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                valType,
                copyValues,
                idxBase,
                alg,
                buffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseCsr2cscEx2_bufferSize<
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
        nnz: ::std::os::raw::c_int,
        csrVal: T,
        csrRowPtr: U,
        csrColInd: V,
        mut cscVal: W,
        mut cscColPtr: X,
        mut cscRowInd: Y,
        valType: cudaDataType,
        copyValues: cusparseAction_t,
        idxBase: cusparseIndexBase_t,
        alg: cusparseCsr2CscAlg_t,
        mut bufferSize: Z,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseCsr2cscEx2_bufferSize(
                self.handle,
                m,
                n,
                nnz,
                csrVal.as_const_ptr() as *const ::std::os::raw::c_void,
                csrRowPtr.as_const_ptr() as *const ::std::os::raw::c_int,
                csrColInd.as_const_ptr() as *const ::std::os::raw::c_int,
                cscVal.as_mut_ptr() as *mut ::std::os::raw::c_void,
                cscColPtr.as_mut_ptr() as *mut ::std::os::raw::c_int,
                cscRowInd.as_mut_ptr() as *mut ::std::os::raw::c_int,
                valType,
                copyValues,
                idxBase,
                alg,
                bufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseAxpby<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        alpha: T,
        vecX: cusparseConstSpVecDescr_t,
        beta: U,
        vecY: cusparseDnVecDescr_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseAxpby(
                self.handle,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                vecX,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                vecY,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseGather(
        &self,
        vecY: cusparseConstDnVecDescr_t,
        vecX: cusparseSpVecDescr_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe { crate::sys::cusparseGather(self.handle, vecY, vecX) };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseScatter(
        &self,
        vecX: cusparseConstSpVecDescr_t,
        vecY: cusparseDnVecDescr_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe { crate::sys::cusparseScatter(self.handle, vecX, vecY) };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseRot<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        c_coeff: T,
        s_coeff: U,
        vecX: cusparseSpVecDescr_t,
        vecY: cusparseDnVecDescr_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseRot(
                self.handle,
                c_coeff.as_const_ptr() as *const ::std::os::raw::c_void,
                s_coeff.as_const_ptr() as *const ::std::os::raw::c_void,
                vecX,
                vecY,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpVV_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opX: cusparseOperation_t,
        vecX: cusparseConstSpVecDescr_t,
        vecY: cusparseConstDnVecDescr_t,
        result: T,
        computeType: cudaDataType,
        mut bufferSize: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpVV_bufferSize(
                self.handle,
                opX,
                vecX,
                vecY,
                result.as_const_ptr() as *const ::std::os::raw::c_void,
                computeType,
                bufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpVV<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opX: cusparseOperation_t,
        vecX: cusparseConstSpVecDescr_t,
        vecY: cusparseConstDnVecDescr_t,
        mut result: T,
        computeType: cudaDataType,
        mut externalBuffer: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpVV(
                self.handle,
                opX,
                vecX,
                vecY,
                result.as_mut_ptr() as *mut ::std::os::raw::c_void,
                computeType,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSparseToDense_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseDnMatDescr_t,
        alg: cusparseSparseToDenseAlg_t,
        mut bufferSize: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSparseToDense_bufferSize(
                self.handle,
                matA,
                matB,
                alg,
                bufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSparseToDense<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseDnMatDescr_t,
        alg: cusparseSparseToDenseAlg_t,
        mut externalBuffer: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSparseToDense(
                self.handle,
                matA,
                matB,
                alg,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDenseToSparse_bufferSize<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        matA: cusparseConstDnMatDescr_t,
        matB: cusparseSpMatDescr_t,
        alg: cusparseDenseToSparseAlg_t,
        mut bufferSize: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDenseToSparse_bufferSize(
                self.handle,
                matA,
                matB,
                alg,
                bufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDenseToSparse_analysis<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        matA: cusparseConstDnMatDescr_t,
        matB: cusparseSpMatDescr_t,
        alg: cusparseDenseToSparseAlg_t,
        mut externalBuffer: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDenseToSparse_analysis(
                self.handle,
                matA,
                matB,
                alg,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseDenseToSparse_convert<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        matA: cusparseConstDnMatDescr_t,
        matB: cusparseSpMatDescr_t,
        alg: cusparseDenseToSparseAlg_t,
        mut externalBuffer: T,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseDenseToSparse_convert(
                self.handle,
                matA,
                matB,
                alg,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpMV<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        vecX: cusparseConstDnVecDescr_t,
        beta: U,
        vecY: cusparseDnVecDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpMVAlg_t,
        mut externalBuffer: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpMV(
                self.handle,
                opA,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                vecX,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                vecY,
                computeType,
                alg,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpMV_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        vecX: cusparseConstDnVecDescr_t,
        beta: U,
        vecY: cusparseDnVecDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpMVAlg_t,
        mut bufferSize: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpMV_bufferSize(
                self.handle,
                opA,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                vecX,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                vecY,
                computeType,
                alg,
                bufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpMV_preprocess<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        vecX: cusparseConstDnVecDescr_t,
        beta: U,
        vecY: cusparseDnVecDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpMVAlg_t,
        mut externalBuffer: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpMV_preprocess(
                self.handle,
                opA,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                vecX,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                vecY,
                computeType,
                alg,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpSV_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        vecX: cusparseConstDnVecDescr_t,
        vecY: cusparseDnVecDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpSVAlg_t,
        spsvDescr: cusparseSpSVDescr_t,
        mut bufferSize: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpSV_bufferSize(
                self.handle,
                opA,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                vecX,
                vecY,
                computeType,
                alg,
                spsvDescr,
                bufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpSV_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        vecX: cusparseConstDnVecDescr_t,
        vecY: cusparseDnVecDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpSVAlg_t,
        spsvDescr: cusparseSpSVDescr_t,
        mut externalBuffer: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpSV_analysis(
                self.handle,
                opA,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                vecX,
                vecY,
                computeType,
                alg,
                spsvDescr,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpSV_solve<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        opA: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        vecX: cusparseConstDnVecDescr_t,
        vecY: cusparseDnVecDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpSVAlg_t,
        spsvDescr: cusparseSpSVDescr_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpSV_solve(
                self.handle,
                opA,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                vecX,
                vecY,
                computeType,
                alg,
                spsvDescr,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpSV_updateMatrix<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        spsvDescr: cusparseSpSVDescr_t,
        mut newValues: T,
        updatePart: cusparseSpSVUpdate_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpSV_updateMatrix(
                self.handle,
                spsvDescr,
                newValues.as_mut_ptr() as *mut ::std::os::raw::c_void,
                updatePart,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpSM_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        matC: cusparseDnMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpSMAlg_t,
        spsmDescr: cusparseSpSMDescr_t,
        mut bufferSize: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpSM_bufferSize(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                matC,
                computeType,
                alg,
                spsmDescr,
                bufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpSM_analysis<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        matC: cusparseDnMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpSMAlg_t,
        spsmDescr: cusparseSpSMDescr_t,
        mut externalBuffer: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpSM_analysis(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                matC,
                computeType,
                alg,
                spsmDescr,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpSM_solve<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        matC: cusparseDnMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpSMAlg_t,
        spsmDescr: cusparseSpSMDescr_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpSM_solve(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                matC,
                computeType,
                alg,
                spsmDescr,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpSM_updateMatrix<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        spsmDescr: cusparseSpSMDescr_t,
        mut newValues: T,
        updatePart: cusparseSpSMUpdate_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpSM_updateMatrix(
                self.handle,
                spsmDescr,
                newValues.as_mut_ptr() as *mut ::std::os::raw::c_void,
                updatePart,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpMM_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        beta: U,
        matC: cusparseDnMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpMMAlg_t,
        mut bufferSize: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpMM_bufferSize(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                bufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpMM_preprocess<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        beta: U,
        matC: cusparseDnMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpMMAlg_t,
        mut externalBuffer: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpMM_preprocess(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpMM<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        beta: U,
        matC: cusparseDnMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpMMAlg_t,
        mut externalBuffer: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpMM(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpGEMM_workEstimation<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstSpMatDescr_t,
        beta: U,
        matC: cusparseSpMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpGEMMAlg_t,
        spgemmDescr: cusparseSpGEMMDescr_t,
        mut bufferSize1: V,
        mut externalBuffer1: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpGEMM_workEstimation(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                spgemmDescr,
                bufferSize1.as_mut_ptr() as *mut usize,
                externalBuffer1.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpGEMM_estimateMemory<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstSpMatDescr_t,
        beta: U,
        matC: cusparseSpMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpGEMMAlg_t,
        spgemmDescr: cusparseSpGEMMDescr_t,
        chunk_fraction: f32,
        mut bufferSize3: V,
        mut externalBuffer3: W,
        mut bufferSize2: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpGEMM_estimateMemory(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                spgemmDescr,
                chunk_fraction,
                bufferSize3.as_mut_ptr() as *mut usize,
                externalBuffer3.as_mut_ptr() as *mut ::std::os::raw::c_void,
                bufferSize2.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpGEMM_compute<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstSpMatDescr_t,
        beta: U,
        matC: cusparseSpMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpGEMMAlg_t,
        spgemmDescr: cusparseSpGEMMDescr_t,
        mut bufferSize2: V,
        mut externalBuffer2: W,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpGEMM_compute(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                spgemmDescr,
                bufferSize2.as_mut_ptr() as *mut usize,
                externalBuffer2.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpGEMM_copy<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstSpMatDescr_t,
        beta: U,
        matC: cusparseSpMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpGEMMAlg_t,
        spgemmDescr: cusparseSpGEMMDescr_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpGEMM_copy(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                spgemmDescr,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpGEMMreuse_workEstimation<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstSpMatDescr_t,
        matC: cusparseSpMatDescr_t,
        alg: cusparseSpGEMMAlg_t,
        spgemmDescr: cusparseSpGEMMDescr_t,
        mut bufferSize1: T,
        mut externalBuffer1: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpGEMMreuse_workEstimation(
                self.handle,
                opA,
                opB,
                matA,
                matB,
                matC,
                alg,
                spgemmDescr,
                bufferSize1.as_mut_ptr() as *mut usize,
                externalBuffer1.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpGEMMreuse_nnz<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstSpMatDescr_t,
        matC: cusparseSpMatDescr_t,
        alg: cusparseSpGEMMAlg_t,
        spgemmDescr: cusparseSpGEMMDescr_t,
        mut bufferSize2: T,
        mut externalBuffer2: U,
        mut bufferSize3: V,
        mut externalBuffer3: W,
        mut bufferSize4: X,
        mut externalBuffer4: Y,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpGEMMreuse_nnz(
                self.handle,
                opA,
                opB,
                matA,
                matB,
                matC,
                alg,
                spgemmDescr,
                bufferSize2.as_mut_ptr() as *mut usize,
                externalBuffer2.as_mut_ptr() as *mut ::std::os::raw::c_void,
                bufferSize3.as_mut_ptr() as *mut usize,
                externalBuffer3.as_mut_ptr() as *mut ::std::os::raw::c_void,
                bufferSize4.as_mut_ptr() as *mut usize,
                externalBuffer4.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpGEMMreuse_copy<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstSpMatDescr_t,
        matC: cusparseSpMatDescr_t,
        alg: cusparseSpGEMMAlg_t,
        spgemmDescr: cusparseSpGEMMDescr_t,
        mut bufferSize5: T,
        mut externalBuffer5: U,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpGEMMreuse_copy(
                self.handle,
                opA,
                opB,
                matA,
                matB,
                matC,
                alg,
                spgemmDescr,
                bufferSize5.as_mut_ptr() as *mut usize,
                externalBuffer5.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpGEMMreuse_compute<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstSpMatDescr_t,
        beta: U,
        matC: cusparseSpMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpGEMMAlg_t,
        spgemmDescr: cusparseSpGEMMDescr_t,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpGEMMreuse_compute(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                spgemmDescr,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSDDMM_bufferSize<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstDnMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        beta: U,
        matC: cusparseSpMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSDDMMAlg_t,
        mut bufferSize: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSDDMM_bufferSize(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                bufferSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSDDMM_preprocess<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstDnMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        beta: U,
        matC: cusparseSpMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSDDMMAlg_t,
        mut externalBuffer: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSDDMM_preprocess(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSDDMM<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        alpha: T,
        matA: cusparseConstDnMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        beta: U,
        matC: cusparseSpMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSDDMMAlg_t,
        mut externalBuffer: V,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSDDMM(
                self.handle,
                opA,
                opB,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                matA,
                matB,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                matC,
                computeType,
                alg,
                externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cusparseSpMMOp_createPlan<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut plan: T,
        opA: cusparseOperation_t,
        opB: cusparseOperation_t,
        matA: cusparseConstSpMatDescr_t,
        matB: cusparseConstDnMatDescr_t,
        matC: cusparseDnMatDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpMMOpAlg_t,
        addOperationLtoirBuffer: U,
        addOperationBufferSize: usize,
        mulOperationLtoirBuffer: V,
        mulOperationBufferSize: usize,
        epilogueLtoirBuffer: W,
        epilogueBufferSize: usize,
        mut SpMMWorkspaceSize: X,
    ) -> Result<(), crate::sys::cusparseStatus_t> {
        let status = unsafe {
            crate::sys::cusparseSpMMOp_createPlan(
                self.handle,
                plan.as_mut_ptr() as *mut cusparseSpMMOpPlan_t,
                opA,
                opB,
                matA,
                matB,
                matC,
                computeType,
                alg,
                addOperationLtoirBuffer.as_const_ptr() as *const ::std::os::raw::c_void,
                addOperationBufferSize,
                mulOperationLtoirBuffer.as_const_ptr() as *const ::std::os::raw::c_void,
                mulOperationBufferSize,
                epilogueLtoirBuffer.as_const_ptr() as *const ::std::os::raw::c_void,
                epilogueBufferSize,
                SpMMWorkspaceSize.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
pub unsafe fn cusparseGetProperty(
    type_: libraryPropertyType,
) -> Result<::std::os::raw::c_int, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseGetErrorName(status: cusparseStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cusparseGetErrorName(status) }
}
pub unsafe fn cusparseGetErrorString(status: cusparseStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cusparseGetErrorString(status) }
}
pub unsafe fn cusparseLoggerSetCallback(
    callback: cusparseLoggerCallback_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseLoggerSetCallback(callback) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseLoggerSetFile<T: ::cuda_libs::types::CudaAsPtr>(
    mut file: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseLoggerSetFile(file.as_mut_ptr() as *mut FILE) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseLoggerOpenFile<T: ::cuda_libs::types::CudaAsPtr>(
    logFile: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseLoggerOpenFile(logFile.as_const_ptr() as *const ::std::os::raw::c_char)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseLoggerSetLevel(
    level: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseLoggerSetLevel(level) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseLoggerSetMask(
    mask: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseLoggerSetMask(mask) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseLoggerForceDisable() -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseLoggerForceDisable() };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSetMatType(
    descrA: cusparseMatDescr_t,
    type_: cusparseMatrixType_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSetMatType(descrA, type_) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseGetMatType(descrA: cusparseMatDescr_t) -> cusparseMatrixType_t {
    unsafe { crate::sys::cusparseGetMatType(descrA) }
}
pub unsafe fn cusparseSetMatFillMode(
    descrA: cusparseMatDescr_t,
    fillMode: cusparseFillMode_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSetMatFillMode(descrA, fillMode) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseGetMatFillMode(descrA: cusparseMatDescr_t) -> cusparseFillMode_t {
    unsafe { crate::sys::cusparseGetMatFillMode(descrA) }
}
pub unsafe fn cusparseSetMatDiagType(
    descrA: cusparseMatDescr_t,
    diagType: cusparseDiagType_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSetMatDiagType(descrA, diagType) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseGetMatDiagType(descrA: cusparseMatDescr_t) -> cusparseDiagType_t {
    unsafe { crate::sys::cusparseGetMatDiagType(descrA) }
}
pub unsafe fn cusparseSetMatIndexBase(
    descrA: cusparseMatDescr_t,
    base: cusparseIndexBase_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSetMatIndexBase(descrA, base) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseGetMatIndexBase(descrA: cusparseMatDescr_t) -> cusparseIndexBase_t {
    unsafe { crate::sys::cusparseGetMatIndexBase(descrA) }
}
pub unsafe fn cusparseSpVecGet(
    spVecDescr: cusparseSpVecDescr_t,
    indices: *mut *mut ::std::os::raw::c_void,
    values: *mut *mut ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseSpVecGet(
            spVecDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            indices,
            values,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
                out_7.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstSpVecGet(
    spVecDescr: cusparseConstSpVecDescr_t,
    indices: *mut *const ::std::os::raw::c_void,
    values: *mut *const ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseConstSpVecGet(
            spVecDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            indices,
            values,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
                out_7.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpVecGetIndexBase(
    spVecDescr: cusparseConstSpVecDescr_t,
) -> Result<cusparseIndexBase_t, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cusparseSpVecGetIndexBase(spVecDescr, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpVecGetValues<T: ::cuda_libs::types::CudaAsPtr>(
    spVecDescr: cusparseSpVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpVecGetValues(
            spVecDescr,
            values.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstSpVecGetValues<T: ::cuda_libs::types::CudaAsPtr>(
    spVecDescr: cusparseConstSpVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseConstSpVecGetValues(
            spVecDescr,
            values.as_mut_ptr() as *mut *const ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpVecSetValues<T: ::cuda_libs::types::CudaAsPtr>(
    spVecDescr: cusparseSpVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpVecSetValues(
            spVecDescr,
            values.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnVecGet(
    dnVecDescr: cusparseDnVecDescr_t,
    values: *mut *mut ::std::os::raw::c_void,
) -> Result<(i64, cudaDataType), crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseDnVecGet(
            dnVecDescr,
            out_1.as_mut_ptr() as *mut _,
            values,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe { Ok((out_1.assume_init(), out_3.assume_init())) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstDnVecGet(
    dnVecDescr: cusparseConstDnVecDescr_t,
    values: *mut *const ::std::os::raw::c_void,
) -> Result<(i64, cudaDataType), crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseConstDnVecGet(
            dnVecDescr,
            out_1.as_mut_ptr() as *mut _,
            values,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe { Ok((out_1.assume_init(), out_3.assume_init())) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnVecGetValues<T: ::cuda_libs::types::CudaAsPtr>(
    dnVecDescr: cusparseDnVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDnVecGetValues(
            dnVecDescr,
            values.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstDnVecGetValues<T: ::cuda_libs::types::CudaAsPtr>(
    dnVecDescr: cusparseConstDnVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseConstDnVecGetValues(
            dnVecDescr,
            values.as_mut_ptr() as *mut *const ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnVecSetValues<T: ::cuda_libs::types::CudaAsPtr>(
    dnVecDescr: cusparseDnVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDnVecSetValues(
            dnVecDescr,
            values.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatGetFormat(
    spMatDescr: cusparseConstSpMatDescr_t,
) -> Result<cusparseFormat_t, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusparseFormat_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cusparseSpMatGetFormat(spMatDescr, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatGetIndexBase(
    spMatDescr: cusparseConstSpMatDescr_t,
) -> Result<cusparseIndexBase_t, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cusparseSpMatGetIndexBase(spMatDescr, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatGetValues<T: ::cuda_libs::types::CudaAsPtr>(
    spMatDescr: cusparseSpMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpMatGetValues(
            spMatDescr,
            values.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstSpMatGetValues<T: ::cuda_libs::types::CudaAsPtr>(
    spMatDescr: cusparseConstSpMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseConstSpMatGetValues(
            spMatDescr,
            values.as_mut_ptr() as *mut *const ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatSetValues<T: ::cuda_libs::types::CudaAsPtr>(
    spMatDescr: cusparseSpMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpMatSetValues(
            spMatDescr,
            values.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatGetSize(
    spMatDescr: cusparseConstSpMatDescr_t,
) -> Result<(i64, i64, i64), crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseSpMatGetSize(
            spMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatGetStridedBatch(
    spMatDescr: cusparseConstSpMatDescr_t,
) -> Result<::std::os::raw::c_int, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseSpMatGetStridedBatch(spMatDescr, out_1.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCooSetStridedBatch(
    spMatDescr: cusparseSpMatDescr_t,
    batchCount: ::std::os::raw::c_int,
    batchStride: i64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status =
        unsafe { crate::sys::cusparseCooSetStridedBatch(spMatDescr, batchCount, batchStride) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCsrSetStridedBatch(
    spMatDescr: cusparseSpMatDescr_t,
    batchCount: ::std::os::raw::c_int,
    offsetsBatchStride: i64,
    columnsValuesBatchStride: i64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCsrSetStridedBatch(
            spMatDescr,
            batchCount,
            offsetsBatchStride,
            columnsValuesBatchStride,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseBsrSetStridedBatch(
    spMatDescr: cusparseSpMatDescr_t,
    batchCount: ::std::os::raw::c_int,
    offsetsBatchStride: i64,
    columnsBatchStride: i64,
    ValuesBatchStride: i64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseBsrSetStridedBatch(
            spMatDescr,
            batchCount,
            offsetsBatchStride,
            columnsBatchStride,
            ValuesBatchStride,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatGetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    spMatDescr: cusparseConstSpMatDescr_t,
    attribute: cusparseSpMatAttribute_t,
    mut data: T,
    dataSize: usize,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpMatGetAttribute(
            spMatDescr,
            attribute,
            data.as_mut_ptr() as *mut ::std::os::raw::c_void,
            dataSize,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    spMatDescr: cusparseSpMatDescr_t,
    attribute: cusparseSpMatAttribute_t,
    mut data: T,
    dataSize: usize,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpMatSetAttribute(
            spMatDescr,
            attribute,
            data.as_mut_ptr() as *mut ::std::os::raw::c_void,
            dataSize,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCsrGet(
    spMatDescr: cusparseSpMatDescr_t,
    csrRowOffsets: *mut *mut ::std::os::raw::c_void,
    csrColInd: *mut *mut ::std::os::raw::c_void,
    csrValues: *mut *mut ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_10: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCsrGet(
            spMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            csrRowOffsets,
            csrColInd,
            csrValues,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
            out_10.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
                out_10.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstCsrGet(
    spMatDescr: cusparseConstSpMatDescr_t,
    csrRowOffsets: *mut *const ::std::os::raw::c_void,
    csrColInd: *mut *const ::std::os::raw::c_void,
    csrValues: *mut *const ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_10: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseConstCsrGet(
            spMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            csrRowOffsets,
            csrColInd,
            csrValues,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
            out_10.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
                out_10.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCscGet(
    spMatDescr: cusparseSpMatDescr_t,
    cscColOffsets: *mut *mut ::std::os::raw::c_void,
    cscRowInd: *mut *mut ::std::os::raw::c_void,
    cscValues: *mut *mut ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_10: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCscGet(
            spMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            cscColOffsets,
            cscRowInd,
            cscValues,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
            out_10.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
                out_10.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstCscGet(
    spMatDescr: cusparseConstSpMatDescr_t,
    cscColOffsets: *mut *const ::std::os::raw::c_void,
    cscRowInd: *mut *const ::std::os::raw::c_void,
    cscValues: *mut *const ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_10: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseConstCscGet(
            spMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            cscColOffsets,
            cscRowInd,
            cscValues,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
            out_10.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
                out_10.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCsrSetPointers<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    spMatDescr: cusparseSpMatDescr_t,
    mut csrRowOffsets: T,
    mut csrColInd: U,
    mut csrValues: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCsrSetPointers(
            spMatDescr,
            csrRowOffsets.as_mut_ptr() as *mut ::std::os::raw::c_void,
            csrColInd.as_mut_ptr() as *mut ::std::os::raw::c_void,
            csrValues.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCscSetPointers<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    spMatDescr: cusparseSpMatDescr_t,
    mut cscColOffsets: T,
    mut cscRowInd: U,
    mut cscValues: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCscSetPointers(
            spMatDescr,
            cscColOffsets.as_mut_ptr() as *mut ::std::os::raw::c_void,
            cscRowInd.as_mut_ptr() as *mut ::std::os::raw::c_void,
            cscValues.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCooGet(
    spMatDescr: cusparseSpMatDescr_t,
    cooRowInd: *mut *mut ::std::os::raw::c_void,
    cooColInd: *mut *mut ::std::os::raw::c_void,
    cooValues: *mut *mut ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCooGet(
            spMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            cooRowInd,
            cooColInd,
            cooValues,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstCooGet(
    spMatDescr: cusparseConstSpMatDescr_t,
    cooRowInd: *mut *const ::std::os::raw::c_void,
    cooColInd: *mut *const ::std::os::raw::c_void,
    cooValues: *mut *const ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseConstCooGet(
            spMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            cooRowInd,
            cooColInd,
            cooValues,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCooSetPointers<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    spMatDescr: cusparseSpMatDescr_t,
    mut cooRows: T,
    mut cooColumns: U,
    mut cooValues: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCooSetPointers(
            spMatDescr,
            cooRows.as_mut_ptr() as *mut ::std::os::raw::c_void,
            cooColumns.as_mut_ptr() as *mut ::std::os::raw::c_void,
            cooValues.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseBlockedEllGet(
    spMatDescr: cusparseSpMatDescr_t,
    ellColInd: *mut *mut ::std::os::raw::c_void,
    ellValue: *mut *mut ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseBlockedEllGet(
            spMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            ellColInd,
            ellValue,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstBlockedEllGet(
    spMatDescr: cusparseConstSpMatDescr_t,
    ellColInd: *mut *const ::std::os::raw::c_void,
    ellValue: *mut *const ::std::os::raw::c_void,
) -> Result<
    (
        i64,
        i64,
        i64,
        i64,
        cusparseIndexType_t,
        cusparseIndexBase_t,
        cudaDataType,
    ),
    crate::sys::cusparseStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cusparseIndexType_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseConstBlockedEllGet(
            spMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            ellColInd,
            ellValue,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnMatGet(
    dnMatDescr: cusparseDnMatDescr_t,
    values: *mut *mut ::std::os::raw::c_void,
) -> Result<(i64, i64, i64, cudaDataType, cusparseOrder_t), crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cusparseOrder_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseDnMatGet(
            dnMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            values,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstDnMatGet(
    dnMatDescr: cusparseConstDnMatDescr_t,
    values: *mut *const ::std::os::raw::c_void,
) -> Result<(i64, i64, i64, cudaDataType, cusparseOrder_t), crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cudaDataType> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cusparseOrder_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseConstDnMatGet(
            dnMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            values,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnMatGetValues<T: ::cuda_libs::types::CudaAsPtr>(
    dnMatDescr: cusparseDnMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDnMatGetValues(
            dnMatDescr,
            values.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstDnMatGetValues<T: ::cuda_libs::types::CudaAsPtr>(
    dnMatDescr: cusparseConstDnMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseConstDnMatGetValues(
            dnMatDescr,
            values.as_mut_ptr() as *mut *const ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnMatSetValues<T: ::cuda_libs::types::CudaAsPtr>(
    dnMatDescr: cusparseDnMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDnMatSetValues(
            dnMatDescr,
            values.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnMatSetStridedBatch(
    dnMatDescr: cusparseDnMatDescr_t,
    batchCount: ::std::os::raw::c_int,
    batchStride: i64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status =
        unsafe { crate::sys::cusparseDnMatSetStridedBatch(dnMatDescr, batchCount, batchStride) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnMatGetStridedBatch(
    dnMatDescr: cusparseConstDnMatDescr_t,
) -> Result<(::std::os::raw::c_int, i64), crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseDnMatGetStridedBatch(
            dnMatDescr,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        unsafe { Ok((out_1.assume_init(), out_2.assume_init())) }
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSV_createDescr<T: ::cuda_libs::types::CudaAsPtr>(
    mut descr: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpSV_createDescr(descr.as_mut_ptr() as *mut cusparseSpSVDescr_t)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSV_destroyDescr(
    descr: cusparseSpSVDescr_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpSV_destroyDescr(descr) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSM_createDescr<T: ::cuda_libs::types::CudaAsPtr>(
    mut descr: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpSM_createDescr(descr.as_mut_ptr() as *mut cusparseSpSMDescr_t)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSM_destroyDescr(
    descr: cusparseSpSMDescr_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpSM_destroyDescr(descr) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_createDescr<T: ::cuda_libs::types::CudaAsPtr>(
    mut descr: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpGEMM_createDescr(descr.as_mut_ptr() as *mut cusparseSpGEMMDescr_t)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_destroyDescr(
    descr: cusparseSpGEMMDescr_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpGEMM_destroyDescr(descr) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_getNumProducts<T: ::cuda_libs::types::CudaAsPtr>(
    spgemmDescr: cusparseSpGEMMDescr_t,
    mut num_prods: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpGEMM_getNumProducts(spgemmDescr, num_prods.as_mut_ptr() as *mut i64)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMMOp<T: ::cuda_libs::types::CudaAsPtr>(
    plan: cusparseSpMMOpPlan_t,
    mut externalBuffer: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpMMOp(
            plan,
            externalBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMMOp_destroyPlan(
    plan: cusparseSpMMOpPlan_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpMMOp_destroyPlan(plan) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
impl CusparseHandle {
    pub fn new() -> Result<Self, crate::sys::cusparseStatus_t> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let status = crate::sys::cusparseCreate(&mut handle);
            if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
                Ok(Self { handle })
            } else {
                Err(status)
            }
        }
    }
}
impl Drop for CusparseHandle {
    fn drop(&mut self) {
        unsafe {
            crate::sys::cusparseDestroy(self.handle);
        }
    }
}
