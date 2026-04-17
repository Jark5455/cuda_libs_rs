pub use crate::sys::cusparseStatus_t as CudaTargetStatus;
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
impl crate::sys::_IO_FILE {
    pub fn _flags(mut self, val: i32) -> Self {
        self._flags = val as _;
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
    pub fn _fileno(mut self, val: i32) -> Self {
        self._fileno = val as _;
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
    pub fn _cur_column(mut self, val: u16) -> Self {
        self._cur_column = val as _;
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
    pub fn _mode(mut self, val: i32) -> Self {
        self._mode = val as _;
        self
    }
    pub fn _unused3(mut self, val: i32) -> Self {
        self._unused3 = val as _;
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
#[cfg(feature = "runtime-link")]
impl crate::sys::DynamicBindings {
    pub fn cusparseCreate(
        mut self,
        val: Option<unsafe extern "C" fn(handle: *mut cusparseHandle_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreate = val;
        self
    }
    pub fn cusparseDestroy(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusparseHandle_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroy = val;
        self
    }
    pub fn cusparseGetVersion(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cusparseHandle_t, version: *mut ::std::os::raw::c_int) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseGetVersion = val;
        self
    }
    pub fn cusparseGetProperty(
        mut self,
        val: Option<
            unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseGetProperty = val;
        self
    }
    pub fn cusparseGetErrorName(
        mut self,
        val: Option<unsafe extern "C" fn(status: cusparseStatus_t) -> *const ::std::os::raw::c_char>,
    ) -> Self {
        self.cusparseGetErrorName = val;
        self
    }
    pub fn cusparseGetErrorString(
        mut self,
        val: Option<unsafe extern "C" fn(status: cusparseStatus_t) -> *const ::std::os::raw::c_char>,
    ) -> Self {
        self.cusparseGetErrorString = val;
        self
    }
    pub fn cusparseSetStream(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusparseHandle_t, streamId: cudaStream_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSetStream = val;
        self
    }
    pub fn cusparseGetStream(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusparseHandle_t, streamId: *mut cudaStream_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseGetStream = val;
        self
    }
    pub fn cusparseGetPointerMode(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cusparseHandle_t, mode: *mut cusparsePointerMode_t) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseGetPointerMode = val;
        self
    }
    pub fn cusparseSetPointerMode(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusparseHandle_t, mode: cusparsePointerMode_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSetPointerMode = val;
        self
    }
    pub fn cusparseLoggerSetCallback(
        mut self,
        val: Option<unsafe extern "C" fn(callback: cusparseLoggerCallback_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseLoggerSetCallback = val;
        self
    }
    pub fn cusparseLoggerSetFile(
        mut self,
        val: Option<unsafe extern "C" fn(file: *mut FILE) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseLoggerSetFile = val;
        self
    }
    pub fn cusparseLoggerOpenFile(
        mut self,
        val: Option<unsafe extern "C" fn(logFile: *const ::std::os::raw::c_char) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseLoggerOpenFile = val;
        self
    }
    pub fn cusparseLoggerSetLevel(
        mut self,
        val: Option<unsafe extern "C" fn(level: ::std::os::raw::c_int) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseLoggerSetLevel = val;
        self
    }
    pub fn cusparseLoggerSetMask(
        mut self,
        val: Option<unsafe extern "C" fn(mask: ::std::os::raw::c_int) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseLoggerSetMask = val;
        self
    }
    pub fn cusparseLoggerForceDisable(mut self, val: Option<unsafe extern "C" fn() -> cusparseStatus_t>) -> Self {
        self.cusparseLoggerForceDisable = val;
        self
    }
    pub fn cusparseCreateMatDescr(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: *mut cusparseMatDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreateMatDescr = val;
        self
    }
    pub fn cusparseDestroyMatDescr(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyMatDescr = val;
        self
    }
    pub fn cusparseSetMatType(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t, type_: cusparseMatrixType_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSetMatType = val;
        self
    }
    pub fn cusparseGetMatType(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseMatrixType_t>,
    ) -> Self {
        self.cusparseGetMatType = val;
        self
    }
    pub fn cusparseSetMatFillMode(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t, fillMode: cusparseFillMode_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSetMatFillMode = val;
        self
    }
    pub fn cusparseGetMatFillMode(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseFillMode_t>,
    ) -> Self {
        self.cusparseGetMatFillMode = val;
        self
    }
    pub fn cusparseSetMatDiagType(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t, diagType: cusparseDiagType_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSetMatDiagType = val;
        self
    }
    pub fn cusparseGetMatDiagType(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseDiagType_t>,
    ) -> Self {
        self.cusparseGetMatDiagType = val;
        self
    }
    pub fn cusparseSetMatIndexBase(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t, base: cusparseIndexBase_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSetMatIndexBase = val;
        self
    }
    pub fn cusparseGetMatIndexBase(
        mut self,
        val: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseIndexBase_t>,
    ) -> Self {
        self.cusparseGetMatIndexBase = val;
        self
    }
    pub fn cusparseCreateCsric02Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut csric02Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreateCsric02Info = val;
        self
    }
    pub fn cusparseDestroyCsric02Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: csric02Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyCsric02Info = val;
        self
    }
    pub fn cusparseCreateBsric02Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut bsric02Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreateBsric02Info = val;
        self
    }
    pub fn cusparseDestroyBsric02Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: bsric02Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyBsric02Info = val;
        self
    }
    pub fn cusparseCreateCsrilu02Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut csrilu02Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreateCsrilu02Info = val;
        self
    }
    pub fn cusparseDestroyCsrilu02Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: csrilu02Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyCsrilu02Info = val;
        self
    }
    pub fn cusparseCreateBsrilu02Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut bsrilu02Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreateBsrilu02Info = val;
        self
    }
    pub fn cusparseDestroyBsrilu02Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: bsrilu02Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyBsrilu02Info = val;
        self
    }
    pub fn cusparseCreateBsrsv2Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut bsrsv2Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreateBsrsv2Info = val;
        self
    }
    pub fn cusparseDestroyBsrsv2Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: bsrsv2Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyBsrsv2Info = val;
        self
    }
    pub fn cusparseCreateBsrsm2Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut bsrsm2Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreateBsrsm2Info = val;
        self
    }
    pub fn cusparseDestroyBsrsm2Info(
        mut self,
        val: Option<unsafe extern "C" fn(info: bsrsm2Info_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyBsrsm2Info = val;
        self
    }
    pub fn cusparseCreateCsru2csrInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut csru2csrInfo_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreateCsru2csrInfo = val;
        self
    }
    pub fn cusparseDestroyCsru2csrInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: csru2csrInfo_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyCsru2csrInfo = val;
        self
    }
    pub fn cusparseCreateColorInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut cusparseColorInfo_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreateColorInfo = val;
        self
    }
    pub fn cusparseDestroyColorInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: cusparseColorInfo_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyColorInfo = val;
        self
    }
    pub fn cusparseCreatePruneInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut pruneInfo_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseCreatePruneInfo = val;
        self
    }
    pub fn cusparseDestroyPruneInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: pruneInfo_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyPruneInfo = val;
        self
    }
    pub fn cusparseSgemvi(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                transA: cusparseOperation_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const f32,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                xVal: *const f32,
                xInd: *const ::std::os::raw::c_int,
                beta: *const f32,
                y: *mut f32,
                idxBase: cusparseIndexBase_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgemvi = val;
        self
    }
    pub fn cusparseSgemvi_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                transA: cusparseOperation_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                pBufferSize: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgemvi_bufferSize = val;
        self
    }
    pub fn cusparseDgemvi(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                transA: cusparseOperation_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const f64,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                xVal: *const f64,
                xInd: *const ::std::os::raw::c_int,
                beta: *const f64,
                y: *mut f64,
                idxBase: cusparseIndexBase_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgemvi = val;
        self
    }
    pub fn cusparseDgemvi_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                transA: cusparseOperation_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                pBufferSize: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgemvi_bufferSize = val;
        self
    }
    pub fn cusparseCgemvi(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                transA: cusparseOperation_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const cuComplex,
                A: *const cuComplex,
                lda: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                xVal: *const cuComplex,
                xInd: *const ::std::os::raw::c_int,
                beta: *const cuComplex,
                y: *mut cuComplex,
                idxBase: cusparseIndexBase_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgemvi = val;
        self
    }
    pub fn cusparseCgemvi_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                transA: cusparseOperation_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                pBufferSize: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgemvi_bufferSize = val;
        self
    }
    pub fn cusparseZgemvi(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                transA: cusparseOperation_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const cuDoubleComplex,
                A: *const cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                xVal: *const cuDoubleComplex,
                xInd: *const ::std::os::raw::c_int,
                beta: *const cuDoubleComplex,
                y: *mut cuDoubleComplex,
                idxBase: cusparseIndexBase_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgemvi = val;
        self
    }
    pub fn cusparseZgemvi_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                transA: cusparseOperation_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                pBufferSize: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgemvi_bufferSize = val;
        self
    }
    pub fn cusparseSbsrmv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f32,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                x: *const f32,
                beta: *const f32,
                y: *mut f32,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrmv = val;
        self
    }
    pub fn cusparseDbsrmv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f64,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                x: *const f64,
                beta: *const f64,
                y: *mut f64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrmv = val;
        self
    }
    pub fn cusparseCbsrmv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                x: *const cuComplex,
                beta: *const cuComplex,
                y: *mut cuComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrmv = val;
        self
    }
    pub fn cusparseZbsrmv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuDoubleComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                x: *const cuDoubleComplex,
                beta: *const cuDoubleComplex,
                y: *mut cuDoubleComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrmv = val;
        self
    }
    pub fn cusparseSbsrxmv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                sizeOfMask: ::std::os::raw::c_int,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f32,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedMaskPtrA: *const ::std::os::raw::c_int,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedEndPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                x: *const f32,
                beta: *const f32,
                y: *mut f32,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrxmv = val;
        self
    }
    pub fn cusparseDbsrxmv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                sizeOfMask: ::std::os::raw::c_int,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f64,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedMaskPtrA: *const ::std::os::raw::c_int,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedEndPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                x: *const f64,
                beta: *const f64,
                y: *mut f64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrxmv = val;
        self
    }
    pub fn cusparseCbsrxmv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                sizeOfMask: ::std::os::raw::c_int,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedMaskPtrA: *const ::std::os::raw::c_int,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedEndPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                x: *const cuComplex,
                beta: *const cuComplex,
                y: *mut cuComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrxmv = val;
        self
    }
    pub fn cusparseZbsrxmv(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                sizeOfMask: ::std::os::raw::c_int,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuDoubleComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedMaskPtrA: *const ::std::os::raw::c_int,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedEndPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                x: *const cuDoubleComplex,
                beta: *const cuDoubleComplex,
                y: *mut cuDoubleComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrxmv = val;
        self
    }
    pub fn cusparseXbsrsv2_zeroPivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: bsrsv2Info_t,
                position: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXbsrsv2_zeroPivot = val;
        self
    }
    pub fn cusparseSbsrsv2_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *mut f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrsv2_bufferSize = val;
        self
    }
    pub fn cusparseDbsrsv2_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *mut f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrsv2_bufferSize = val;
        self
    }
    pub fn cusparseCbsrsv2_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *mut cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrsv2_bufferSize = val;
        self
    }
    pub fn cusparseZbsrsv2_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *mut cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrsv2_bufferSize = val;
        self
    }
    pub fn cusparseSbsrsv2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *mut f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrsv2_bufferSizeExt = val;
        self
    }
    pub fn cusparseDbsrsv2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *mut f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrsv2_bufferSizeExt = val;
        self
    }
    pub fn cusparseCbsrsv2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *mut cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrsv2_bufferSizeExt = val;
        self
    }
    pub fn cusparseZbsrsv2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *mut cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrsv2_bufferSizeExt = val;
        self
    }
    pub fn cusparseSbsrsv2_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrsv2_analysis = val;
        self
    }
    pub fn cusparseDbsrsv2_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrsv2_analysis = val;
        self
    }
    pub fn cusparseCbsrsv2_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrsv2_analysis = val;
        self
    }
    pub fn cusparseZbsrsv2_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrsv2_analysis = val;
        self
    }
    pub fn cusparseSbsrsv2_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f32,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                f: *const f32,
                x: *mut f32,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrsv2_solve = val;
        self
    }
    pub fn cusparseDbsrsv2_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f64,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                f: *const f64,
                x: *mut f64,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrsv2_solve = val;
        self
    }
    pub fn cusparseCbsrsv2_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                f: *const cuComplex,
                x: *mut cuComplex,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrsv2_solve = val;
        self
    }
    pub fn cusparseZbsrsv2_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuDoubleComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrsv2Info_t,
                f: *const cuDoubleComplex,
                x: *mut cuDoubleComplex,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrsv2_solve = val;
        self
    }
    pub fn cusparseSbsrmm(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transB: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                kb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f32,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                B: *const f32,
                ldb: ::std::os::raw::c_int,
                beta: *const f32,
                C: *mut f32,
                ldc: ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrmm = val;
        self
    }
    pub fn cusparseDbsrmm(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transB: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                kb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f64,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                B: *const f64,
                ldb: ::std::os::raw::c_int,
                beta: *const f64,
                C: *mut f64,
                ldc: ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrmm = val;
        self
    }
    pub fn cusparseCbsrmm(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transB: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                kb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                B: *const cuComplex,
                ldb: ::std::os::raw::c_int,
                beta: *const cuComplex,
                C: *mut cuComplex,
                ldc: ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrmm = val;
        self
    }
    pub fn cusparseZbsrmm(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transB: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                kb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuDoubleComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                B: *const cuDoubleComplex,
                ldb: ::std::os::raw::c_int,
                beta: *const cuDoubleComplex,
                C: *mut cuDoubleComplex,
                ldc: ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrmm = val;
        self
    }
    pub fn cusparseXbsrsm2_zeroPivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: bsrsm2Info_t,
                position: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXbsrsm2_zeroPivot = val;
        self
    }
    pub fn cusparseSbsrsm2_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrsm2_bufferSize = val;
        self
    }
    pub fn cusparseDbsrsm2_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrsm2_bufferSize = val;
        self
    }
    pub fn cusparseCbsrsm2_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrsm2_bufferSize = val;
        self
    }
    pub fn cusparseZbsrsm2_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrsm2_bufferSize = val;
        self
    }
    pub fn cusparseSbsrsm2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transB: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrsm2_bufferSizeExt = val;
        self
    }
    pub fn cusparseDbsrsm2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transB: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrsm2_bufferSizeExt = val;
        self
    }
    pub fn cusparseCbsrsm2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transB: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrsm2_bufferSizeExt = val;
        self
    }
    pub fn cusparseZbsrsm2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transB: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrsm2_bufferSizeExt = val;
        self
    }
    pub fn cusparseSbsrsm2_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrsm2_analysis = val;
        self
    }
    pub fn cusparseDbsrsm2_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrsm2_analysis = val;
        self
    }
    pub fn cusparseCbsrsm2_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrsm2_analysis = val;
        self
    }
    pub fn cusparseZbsrsm2_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrsm2_analysis = val;
        self
    }
    pub fn cusparseSbsrsm2_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f32,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                B: *const f32,
                ldb: ::std::os::raw::c_int,
                X: *mut f32,
                ldx: ::std::os::raw::c_int,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrsm2_solve = val;
        self
    }
    pub fn cusparseDbsrsm2_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const f64,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                B: *const f64,
                ldb: ::std::os::raw::c_int,
                X: *mut f64,
                ldx: ::std::os::raw::c_int,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrsm2_solve = val;
        self
    }
    pub fn cusparseCbsrsm2_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                B: *const cuComplex,
                ldb: ::std::os::raw::c_int,
                X: *mut cuComplex,
                ldx: ::std::os::raw::c_int,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrsm2_solve = val;
        self
    }
    pub fn cusparseZbsrsm2_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                transA: cusparseOperation_t,
                transXY: cusparseOperation_t,
                mb: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                alpha: *const cuDoubleComplex,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrsm2Info_t,
                B: *const cuDoubleComplex,
                ldb: ::std::os::raw::c_int,
                X: *mut cuDoubleComplex,
                ldx: ::std::os::raw::c_int,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrsm2_solve = val;
        self
    }
    pub fn cusparseScsrilu02_numericBoost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: csrilu02Info_t,
                enable_boost: ::std::os::raw::c_int,
                tol: *mut f64,
                boost_val: *mut f32,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsrilu02_numericBoost = val;
        self
    }
    pub fn cusparseDcsrilu02_numericBoost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: csrilu02Info_t,
                enable_boost: ::std::os::raw::c_int,
                tol: *mut f64,
                boost_val: *mut f64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsrilu02_numericBoost = val;
        self
    }
    pub fn cusparseCcsrilu02_numericBoost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: csrilu02Info_t,
                enable_boost: ::std::os::raw::c_int,
                tol: *mut f64,
                boost_val: *mut cuComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsrilu02_numericBoost = val;
        self
    }
    pub fn cusparseZcsrilu02_numericBoost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: csrilu02Info_t,
                enable_boost: ::std::os::raw::c_int,
                tol: *mut f64,
                boost_val: *mut cuDoubleComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsrilu02_numericBoost = val;
        self
    }
    pub fn cusparseXcsrilu02_zeroPivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: csrilu02Info_t,
                position: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcsrilu02_zeroPivot = val;
        self
    }
    pub fn cusparseScsrilu02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *mut f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsrilu02_bufferSize = val;
        self
    }
    pub fn cusparseDcsrilu02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *mut f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsrilu02_bufferSize = val;
        self
    }
    pub fn cusparseCcsrilu02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *mut cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsrilu02_bufferSize = val;
        self
    }
    pub fn cusparseZcsrilu02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *mut cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsrilu02_bufferSize = val;
        self
    }
    pub fn cusparseScsrilu02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedVal: *mut f32,
                csrSortedRowPtr: *const ::std::os::raw::c_int,
                csrSortedColInd: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsrilu02_bufferSizeExt = val;
        self
    }
    pub fn cusparseDcsrilu02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedVal: *mut f64,
                csrSortedRowPtr: *const ::std::os::raw::c_int,
                csrSortedColInd: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsrilu02_bufferSizeExt = val;
        self
    }
    pub fn cusparseCcsrilu02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedVal: *mut cuComplex,
                csrSortedRowPtr: *const ::std::os::raw::c_int,
                csrSortedColInd: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsrilu02_bufferSizeExt = val;
        self
    }
    pub fn cusparseZcsrilu02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedVal: *mut cuDoubleComplex,
                csrSortedRowPtr: *const ::std::os::raw::c_int,
                csrSortedColInd: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsrilu02_bufferSizeExt = val;
        self
    }
    pub fn cusparseScsrilu02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsrilu02_analysis = val;
        self
    }
    pub fn cusparseDcsrilu02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsrilu02_analysis = val;
        self
    }
    pub fn cusparseCcsrilu02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsrilu02_analysis = val;
        self
    }
    pub fn cusparseZcsrilu02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsrilu02_analysis = val;
        self
    }
    pub fn cusparseScsrilu02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA_valM: *mut f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsrilu02 = val;
        self
    }
    pub fn cusparseDcsrilu02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA_valM: *mut f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsrilu02 = val;
        self
    }
    pub fn cusparseCcsrilu02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA_valM: *mut cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsrilu02 = val;
        self
    }
    pub fn cusparseZcsrilu02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA_valM: *mut cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsrilu02 = val;
        self
    }
    pub fn cusparseSbsrilu02_numericBoost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: bsrilu02Info_t,
                enable_boost: ::std::os::raw::c_int,
                tol: *mut f64,
                boost_val: *mut f32,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrilu02_numericBoost = val;
        self
    }
    pub fn cusparseDbsrilu02_numericBoost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: bsrilu02Info_t,
                enable_boost: ::std::os::raw::c_int,
                tol: *mut f64,
                boost_val: *mut f64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrilu02_numericBoost = val;
        self
    }
    pub fn cusparseCbsrilu02_numericBoost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: bsrilu02Info_t,
                enable_boost: ::std::os::raw::c_int,
                tol: *mut f64,
                boost_val: *mut cuComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrilu02_numericBoost = val;
        self
    }
    pub fn cusparseZbsrilu02_numericBoost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: bsrilu02Info_t,
                enable_boost: ::std::os::raw::c_int,
                tol: *mut f64,
                boost_val: *mut cuDoubleComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrilu02_numericBoost = val;
        self
    }
    pub fn cusparseXbsrilu02_zeroPivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: bsrilu02Info_t,
                position: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXbsrilu02_zeroPivot = val;
        self
    }
    pub fn cusparseSbsrilu02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrilu02_bufferSize = val;
        self
    }
    pub fn cusparseDbsrilu02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrilu02_bufferSize = val;
        self
    }
    pub fn cusparseCbsrilu02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrilu02_bufferSize = val;
        self
    }
    pub fn cusparseZbsrilu02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrilu02_bufferSize = val;
        self
    }
    pub fn cusparseSbsrilu02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrilu02_bufferSizeExt = val;
        self
    }
    pub fn cusparseDbsrilu02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrilu02_bufferSizeExt = val;
        self
    }
    pub fn cusparseCbsrilu02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrilu02_bufferSizeExt = val;
        self
    }
    pub fn cusparseZbsrilu02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrilu02_bufferSizeExt = val;
        self
    }
    pub fn cusparseSbsrilu02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrilu02_analysis = val;
        self
    }
    pub fn cusparseDbsrilu02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrilu02_analysis = val;
        self
    }
    pub fn cusparseCbsrilu02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrilu02_analysis = val;
        self
    }
    pub fn cusparseZbsrilu02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrilu02_analysis = val;
        self
    }
    pub fn cusparseSbsrilu02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsrilu02 = val;
        self
    }
    pub fn cusparseDbsrilu02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsrilu02 = val;
        self
    }
    pub fn cusparseCbsrilu02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsrilu02 = val;
        self
    }
    pub fn cusparseZbsrilu02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsrilu02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsrilu02 = val;
        self
    }
    pub fn cusparseXcsric02_zeroPivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: csric02Info_t,
                position: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcsric02_zeroPivot = val;
        self
    }
    pub fn cusparseScsric02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *mut f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsric02_bufferSize = val;
        self
    }
    pub fn cusparseDcsric02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *mut f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsric02_bufferSize = val;
        self
    }
    pub fn cusparseCcsric02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *mut cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsric02_bufferSize = val;
        self
    }
    pub fn cusparseZcsric02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *mut cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsric02_bufferSize = val;
        self
    }
    pub fn cusparseScsric02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedVal: *mut f32,
                csrSortedRowPtr: *const ::std::os::raw::c_int,
                csrSortedColInd: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsric02_bufferSizeExt = val;
        self
    }
    pub fn cusparseDcsric02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedVal: *mut f64,
                csrSortedRowPtr: *const ::std::os::raw::c_int,
                csrSortedColInd: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsric02_bufferSizeExt = val;
        self
    }
    pub fn cusparseCcsric02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedVal: *mut cuComplex,
                csrSortedRowPtr: *const ::std::os::raw::c_int,
                csrSortedColInd: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsric02_bufferSizeExt = val;
        self
    }
    pub fn cusparseZcsric02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedVal: *mut cuDoubleComplex,
                csrSortedRowPtr: *const ::std::os::raw::c_int,
                csrSortedColInd: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsric02_bufferSizeExt = val;
        self
    }
    pub fn cusparseScsric02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsric02_analysis = val;
        self
    }
    pub fn cusparseDcsric02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsric02_analysis = val;
        self
    }
    pub fn cusparseCcsric02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsric02_analysis = val;
        self
    }
    pub fn cusparseZcsric02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsric02_analysis = val;
        self
    }
    pub fn cusparseScsric02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA_valM: *mut f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsric02 = val;
        self
    }
    pub fn cusparseDcsric02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA_valM: *mut f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsric02 = val;
        self
    }
    pub fn cusparseCcsric02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA_valM: *mut cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsric02 = val;
        self
    }
    pub fn cusparseZcsric02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA_valM: *mut cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                info: csric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsric02 = val;
        self
    }
    pub fn cusparseXbsric02_zeroPivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                info: bsric02Info_t,
                position: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXbsric02_zeroPivot = val;
        self
    }
    pub fn cusparseSbsric02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsric02_bufferSize = val;
        self
    }
    pub fn cusparseDbsric02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsric02_bufferSize = val;
        self
    }
    pub fn cusparseCbsric02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsric02_bufferSize = val;
        self
    }
    pub fn cusparseZbsric02_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsric02_bufferSize = val;
        self
    }
    pub fn cusparseSbsric02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsric02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsric02_bufferSizeExt = val;
        self
    }
    pub fn cusparseDbsric02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsric02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsric02_bufferSizeExt = val;
        self
    }
    pub fn cusparseCbsric02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsric02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsric02_bufferSizeExt = val;
        self
    }
    pub fn cusparseZbsric02_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
                info: bsric02Info_t,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsric02_bufferSizeExt = val;
        self
    }
    pub fn cusparseSbsric02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                policy: cusparseSolvePolicy_t,
                pInputBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsric02_analysis = val;
        self
    }
    pub fn cusparseDbsric02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                policy: cusparseSolvePolicy_t,
                pInputBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsric02_analysis = val;
        self
    }
    pub fn cusparseCbsric02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                policy: cusparseSolvePolicy_t,
                pInputBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsric02_analysis = val;
        self
    }
    pub fn cusparseZbsric02_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *const cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                policy: cusparseSolvePolicy_t,
                pInputBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsric02_analysis = val;
        self
    }
    pub fn cusparseSbsric02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsric02 = val;
        self
    }
    pub fn cusparseDbsric02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsric02 = val;
        self
    }
    pub fn cusparseCbsric02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsric02 = val;
        self
    }
    pub fn cusparseZbsric02(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedVal: *mut cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                info: bsric02Info_t,
                policy: cusparseSolvePolicy_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsric02 = val;
        self
    }
    pub fn cusparseSgtsv2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const f32,
                d: *const f32,
                du: *const f32,
                B: *const f32,
                ldb: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgtsv2_bufferSizeExt = val;
        self
    }
    pub fn cusparseDgtsv2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const f64,
                d: *const f64,
                du: *const f64,
                B: *const f64,
                ldb: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgtsv2_bufferSizeExt = val;
        self
    }
    pub fn cusparseCgtsv2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const cuComplex,
                d: *const cuComplex,
                du: *const cuComplex,
                B: *const cuComplex,
                ldb: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgtsv2_bufferSizeExt = val;
        self
    }
    pub fn cusparseZgtsv2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const cuDoubleComplex,
                d: *const cuDoubleComplex,
                du: *const cuDoubleComplex,
                B: *const cuDoubleComplex,
                ldb: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgtsv2_bufferSizeExt = val;
        self
    }
    pub fn cusparseSgtsv2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const f32,
                d: *const f32,
                du: *const f32,
                B: *mut f32,
                ldb: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgtsv2 = val;
        self
    }
    pub fn cusparseDgtsv2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const f64,
                d: *const f64,
                du: *const f64,
                B: *mut f64,
                ldb: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgtsv2 = val;
        self
    }
    pub fn cusparseCgtsv2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const cuComplex,
                d: *const cuComplex,
                du: *const cuComplex,
                B: *mut cuComplex,
                ldb: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgtsv2 = val;
        self
    }
    pub fn cusparseZgtsv2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const cuDoubleComplex,
                d: *const cuDoubleComplex,
                du: *const cuDoubleComplex,
                B: *mut cuDoubleComplex,
                ldb: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgtsv2 = val;
        self
    }
    pub fn cusparseSgtsv2_nopivot_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const f32,
                d: *const f32,
                du: *const f32,
                B: *const f32,
                ldb: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgtsv2_nopivot_bufferSizeExt = val;
        self
    }
    pub fn cusparseDgtsv2_nopivot_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const f64,
                d: *const f64,
                du: *const f64,
                B: *const f64,
                ldb: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgtsv2_nopivot_bufferSizeExt = val;
        self
    }
    pub fn cusparseCgtsv2_nopivot_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const cuComplex,
                d: *const cuComplex,
                du: *const cuComplex,
                B: *const cuComplex,
                ldb: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgtsv2_nopivot_bufferSizeExt = val;
        self
    }
    pub fn cusparseZgtsv2_nopivot_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const cuDoubleComplex,
                d: *const cuDoubleComplex,
                du: *const cuDoubleComplex,
                B: *const cuDoubleComplex,
                ldb: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgtsv2_nopivot_bufferSizeExt = val;
        self
    }
    pub fn cusparseSgtsv2_nopivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const f32,
                d: *const f32,
                du: *const f32,
                B: *mut f32,
                ldb: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgtsv2_nopivot = val;
        self
    }
    pub fn cusparseDgtsv2_nopivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const f64,
                d: *const f64,
                du: *const f64,
                B: *mut f64,
                ldb: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgtsv2_nopivot = val;
        self
    }
    pub fn cusparseCgtsv2_nopivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const cuComplex,
                d: *const cuComplex,
                du: *const cuComplex,
                B: *mut cuComplex,
                ldb: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgtsv2_nopivot = val;
        self
    }
    pub fn cusparseZgtsv2_nopivot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                dl: *const cuDoubleComplex,
                d: *const cuDoubleComplex,
                du: *const cuDoubleComplex,
                B: *mut cuDoubleComplex,
                ldb: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgtsv2_nopivot = val;
        self
    }
    pub fn cusparseSgtsv2StridedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                dl: *const f32,
                d: *const f32,
                du: *const f32,
                x: *const f32,
                batchCount: ::std::os::raw::c_int,
                batchStride: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgtsv2StridedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseDgtsv2StridedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                dl: *const f64,
                d: *const f64,
                du: *const f64,
                x: *const f64,
                batchCount: ::std::os::raw::c_int,
                batchStride: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgtsv2StridedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseCgtsv2StridedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                dl: *const cuComplex,
                d: *const cuComplex,
                du: *const cuComplex,
                x: *const cuComplex,
                batchCount: ::std::os::raw::c_int,
                batchStride: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgtsv2StridedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseZgtsv2StridedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                dl: *const cuDoubleComplex,
                d: *const cuDoubleComplex,
                du: *const cuDoubleComplex,
                x: *const cuDoubleComplex,
                batchCount: ::std::os::raw::c_int,
                batchStride: ::std::os::raw::c_int,
                bufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgtsv2StridedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseSgtsv2StridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                dl: *const f32,
                d: *const f32,
                du: *const f32,
                x: *mut f32,
                batchCount: ::std::os::raw::c_int,
                batchStride: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgtsv2StridedBatch = val;
        self
    }
    pub fn cusparseDgtsv2StridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                dl: *const f64,
                d: *const f64,
                du: *const f64,
                x: *mut f64,
                batchCount: ::std::os::raw::c_int,
                batchStride: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgtsv2StridedBatch = val;
        self
    }
    pub fn cusparseCgtsv2StridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                dl: *const cuComplex,
                d: *const cuComplex,
                du: *const cuComplex,
                x: *mut cuComplex,
                batchCount: ::std::os::raw::c_int,
                batchStride: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgtsv2StridedBatch = val;
        self
    }
    pub fn cusparseZgtsv2StridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                dl: *const cuDoubleComplex,
                d: *const cuDoubleComplex,
                du: *const cuDoubleComplex,
                x: *mut cuDoubleComplex,
                batchCount: ::std::os::raw::c_int,
                batchStride: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgtsv2StridedBatch = val;
        self
    }
    pub fn cusparseSgtsvInterleavedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                dl: *const f32,
                d: *const f32,
                du: *const f32,
                x: *const f32,
                batchCount: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgtsvInterleavedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseDgtsvInterleavedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                dl: *const f64,
                d: *const f64,
                du: *const f64,
                x: *const f64,
                batchCount: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgtsvInterleavedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseCgtsvInterleavedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                dl: *const cuComplex,
                d: *const cuComplex,
                du: *const cuComplex,
                x: *const cuComplex,
                batchCount: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgtsvInterleavedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseZgtsvInterleavedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                dl: *const cuDoubleComplex,
                d: *const cuDoubleComplex,
                du: *const cuDoubleComplex,
                x: *const cuDoubleComplex,
                batchCount: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgtsvInterleavedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseSgtsvInterleavedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                dl: *mut f32,
                d: *mut f32,
                du: *mut f32,
                x: *mut f32,
                batchCount: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgtsvInterleavedBatch = val;
        self
    }
    pub fn cusparseDgtsvInterleavedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                dl: *mut f64,
                d: *mut f64,
                du: *mut f64,
                x: *mut f64,
                batchCount: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgtsvInterleavedBatch = val;
        self
    }
    pub fn cusparseCgtsvInterleavedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                dl: *mut cuComplex,
                d: *mut cuComplex,
                du: *mut cuComplex,
                x: *mut cuComplex,
                batchCount: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgtsvInterleavedBatch = val;
        self
    }
    pub fn cusparseZgtsvInterleavedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                dl: *mut cuDoubleComplex,
                d: *mut cuDoubleComplex,
                du: *mut cuDoubleComplex,
                x: *mut cuDoubleComplex,
                batchCount: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgtsvInterleavedBatch = val;
        self
    }
    pub fn cusparseSgpsvInterleavedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                ds: *const f32,
                dl: *const f32,
                d: *const f32,
                du: *const f32,
                dw: *const f32,
                x: *const f32,
                batchCount: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgpsvInterleavedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseDgpsvInterleavedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                ds: *const f64,
                dl: *const f64,
                d: *const f64,
                du: *const f64,
                dw: *const f64,
                x: *const f64,
                batchCount: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgpsvInterleavedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseCgpsvInterleavedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                ds: *const cuComplex,
                dl: *const cuComplex,
                d: *const cuComplex,
                du: *const cuComplex,
                dw: *const cuComplex,
                x: *const cuComplex,
                batchCount: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgpsvInterleavedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseZgpsvInterleavedBatch_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                ds: *const cuDoubleComplex,
                dl: *const cuDoubleComplex,
                d: *const cuDoubleComplex,
                du: *const cuDoubleComplex,
                dw: *const cuDoubleComplex,
                x: *const cuDoubleComplex,
                batchCount: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgpsvInterleavedBatch_bufferSizeExt = val;
        self
    }
    pub fn cusparseSgpsvInterleavedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                ds: *mut f32,
                dl: *mut f32,
                d: *mut f32,
                du: *mut f32,
                dw: *mut f32,
                x: *mut f32,
                batchCount: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgpsvInterleavedBatch = val;
        self
    }
    pub fn cusparseDgpsvInterleavedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                ds: *mut f64,
                dl: *mut f64,
                d: *mut f64,
                du: *mut f64,
                dw: *mut f64,
                x: *mut f64,
                batchCount: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgpsvInterleavedBatch = val;
        self
    }
    pub fn cusparseCgpsvInterleavedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                ds: *mut cuComplex,
                dl: *mut cuComplex,
                d: *mut cuComplex,
                du: *mut cuComplex,
                dw: *mut cuComplex,
                x: *mut cuComplex,
                batchCount: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgpsvInterleavedBatch = val;
        self
    }
    pub fn cusparseZgpsvInterleavedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                algo: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                ds: *mut cuDoubleComplex,
                dl: *mut cuDoubleComplex,
                d: *mut cuDoubleComplex,
                du: *mut cuDoubleComplex,
                dw: *mut cuDoubleComplex,
                x: *mut cuDoubleComplex,
                batchCount: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgpsvInterleavedBatch = val;
        self
    }
    pub fn cusparseScsrgeam2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const f32,
                descrA: cusparseMatDescr_t,
                nnzA: ::std::os::raw::c_int,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                beta: *const f32,
                descrB: cusparseMatDescr_t,
                nnzB: ::std::os::raw::c_int,
                csrSortedValB: *const f32,
                csrSortedRowPtrB: *const ::std::os::raw::c_int,
                csrSortedColIndB: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f32,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsrgeam2_bufferSizeExt = val;
        self
    }
    pub fn cusparseDcsrgeam2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const f64,
                descrA: cusparseMatDescr_t,
                nnzA: ::std::os::raw::c_int,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                beta: *const f64,
                descrB: cusparseMatDescr_t,
                nnzB: ::std::os::raw::c_int,
                csrSortedValB: *const f64,
                csrSortedRowPtrB: *const ::std::os::raw::c_int,
                csrSortedColIndB: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f64,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsrgeam2_bufferSizeExt = val;
        self
    }
    pub fn cusparseCcsrgeam2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const cuComplex,
                descrA: cusparseMatDescr_t,
                nnzA: ::std::os::raw::c_int,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                beta: *const cuComplex,
                descrB: cusparseMatDescr_t,
                nnzB: ::std::os::raw::c_int,
                csrSortedValB: *const cuComplex,
                csrSortedRowPtrB: *const ::std::os::raw::c_int,
                csrSortedColIndB: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const cuComplex,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsrgeam2_bufferSizeExt = val;
        self
    }
    pub fn cusparseZcsrgeam2_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const cuDoubleComplex,
                descrA: cusparseMatDescr_t,
                nnzA: ::std::os::raw::c_int,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                beta: *const cuDoubleComplex,
                descrB: cusparseMatDescr_t,
                nnzB: ::std::os::raw::c_int,
                csrSortedValB: *const cuDoubleComplex,
                csrSortedRowPtrB: *const ::std::os::raw::c_int,
                csrSortedColIndB: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const cuDoubleComplex,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsrgeam2_bufferSizeExt = val;
        self
    }
    pub fn cusparseXcsrgeam2Nnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                nnzA: ::std::os::raw::c_int,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                descrB: cusparseMatDescr_t,
                nnzB: ::std::os::raw::c_int,
                csrSortedRowPtrB: *const ::std::os::raw::c_int,
                csrSortedColIndB: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                workspace: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcsrgeam2Nnz = val;
        self
    }
    pub fn cusparseScsrgeam2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const f32,
                descrA: cusparseMatDescr_t,
                nnzA: ::std::os::raw::c_int,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                beta: *const f32,
                descrB: cusparseMatDescr_t,
                nnzB: ::std::os::raw::c_int,
                csrSortedValB: *const f32,
                csrSortedRowPtrB: *const ::std::os::raw::c_int,
                csrSortedColIndB: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f32,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsrgeam2 = val;
        self
    }
    pub fn cusparseDcsrgeam2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const f64,
                descrA: cusparseMatDescr_t,
                nnzA: ::std::os::raw::c_int,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                beta: *const f64,
                descrB: cusparseMatDescr_t,
                nnzB: ::std::os::raw::c_int,
                csrSortedValB: *const f64,
                csrSortedRowPtrB: *const ::std::os::raw::c_int,
                csrSortedColIndB: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f64,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsrgeam2 = val;
        self
    }
    pub fn cusparseCcsrgeam2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const cuComplex,
                descrA: cusparseMatDescr_t,
                nnzA: ::std::os::raw::c_int,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                beta: *const cuComplex,
                descrB: cusparseMatDescr_t,
                nnzB: ::std::os::raw::c_int,
                csrSortedValB: *const cuComplex,
                csrSortedRowPtrB: *const ::std::os::raw::c_int,
                csrSortedColIndB: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut cuComplex,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsrgeam2 = val;
        self
    }
    pub fn cusparseZcsrgeam2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                alpha: *const cuDoubleComplex,
                descrA: cusparseMatDescr_t,
                nnzA: ::std::os::raw::c_int,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                beta: *const cuDoubleComplex,
                descrB: cusparseMatDescr_t,
                nnzB: ::std::os::raw::c_int,
                csrSortedValB: *const cuDoubleComplex,
                csrSortedRowPtrB: *const ::std::os::raw::c_int,
                csrSortedColIndB: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut cuDoubleComplex,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsrgeam2 = val;
        self
    }
    pub fn cusparseScsrcolor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                fractionToColor: *const f32,
                ncolors: *mut ::std::os::raw::c_int,
                coloring: *mut ::std::os::raw::c_int,
                reordering: *mut ::std::os::raw::c_int,
                info: cusparseColorInfo_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsrcolor = val;
        self
    }
    pub fn cusparseDcsrcolor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                fractionToColor: *const f64,
                ncolors: *mut ::std::os::raw::c_int,
                coloring: *mut ::std::os::raw::c_int,
                reordering: *mut ::std::os::raw::c_int,
                info: cusparseColorInfo_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsrcolor = val;
        self
    }
    pub fn cusparseCcsrcolor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                fractionToColor: *const f32,
                ncolors: *mut ::std::os::raw::c_int,
                coloring: *mut ::std::os::raw::c_int,
                reordering: *mut ::std::os::raw::c_int,
                info: cusparseColorInfo_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsrcolor = val;
        self
    }
    pub fn cusparseZcsrcolor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                fractionToColor: *const f64,
                ncolors: *mut ::std::os::raw::c_int,
                coloring: *mut ::std::os::raw::c_int,
                reordering: *mut ::std::os::raw::c_int,
                info: cusparseColorInfo_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsrcolor = val;
        self
    }
    pub fn cusparseSnnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                nnzPerRowCol: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSnnz = val;
        self
    }
    pub fn cusparseDnnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                nnzPerRowCol: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnnz = val;
        self
    }
    pub fn cusparseCnnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                A: *const cuComplex,
                lda: ::std::os::raw::c_int,
                nnzPerRowCol: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCnnz = val;
        self
    }
    pub fn cusparseZnnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                A: *const cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                nnzPerRowCol: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZnnz = val;
        self
    }
    pub fn cusparseSnnz_compress(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                descr: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                nnzPerRow: *mut ::std::os::raw::c_int,
                nnzC: *mut ::std::os::raw::c_int,
                tol: f32,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSnnz_compress = val;
        self
    }
    pub fn cusparseDnnz_compress(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                descr: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                nnzPerRow: *mut ::std::os::raw::c_int,
                nnzC: *mut ::std::os::raw::c_int,
                tol: f64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnnz_compress = val;
        self
    }
    pub fn cusparseCnnz_compress(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                descr: cusparseMatDescr_t,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                nnzPerRow: *mut ::std::os::raw::c_int,
                nnzC: *mut ::std::os::raw::c_int,
                tol: cuComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCnnz_compress = val;
        self
    }
    pub fn cusparseZnnz_compress(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                descr: cusparseMatDescr_t,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                nnzPerRow: *mut ::std::os::raw::c_int,
                nnzC: *mut ::std::os::raw::c_int,
                tol: cuDoubleComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZnnz_compress = val;
        self
    }
    pub fn cusparseScsr2csr_compress(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                nnzPerRow: *const ::std::os::raw::c_int,
                csrSortedValC: *mut f32,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                tol: f32,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsr2csr_compress = val;
        self
    }
    pub fn cusparseDcsr2csr_compress(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                nnzPerRow: *const ::std::os::raw::c_int,
                csrSortedValC: *mut f64,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                tol: f64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsr2csr_compress = val;
        self
    }
    pub fn cusparseCcsr2csr_compress(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuComplex,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                nnzPerRow: *const ::std::os::raw::c_int,
                csrSortedValC: *mut cuComplex,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                tol: cuComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsr2csr_compress = val;
        self
    }
    pub fn cusparseZcsr2csr_compress(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                nnzPerRow: *const ::std::os::raw::c_int,
                csrSortedValC: *mut cuDoubleComplex,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                tol: cuDoubleComplex,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsr2csr_compress = val;
        self
    }
    pub fn cusparseXcoo2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                cooRowInd: *const ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                csrSortedRowPtr: *mut ::std::os::raw::c_int,
                idxBase: cusparseIndexBase_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcoo2csr = val;
        self
    }
    pub fn cusparseXcsr2coo(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                csrSortedRowPtr: *const ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                m: ::std::os::raw::c_int,
                cooRowInd: *mut ::std::os::raw::c_int,
                idxBase: cusparseIndexBase_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcsr2coo = val;
        self
    }
    pub fn cusparseXcsr2bsrNnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcsr2bsrNnz = val;
        self
    }
    pub fn cusparseScsr2bsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut f32,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsr2bsr = val;
        self
    }
    pub fn cusparseDcsr2bsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut f64,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsr2bsr = val;
        self
    }
    pub fn cusparseCcsr2bsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut cuComplex,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsr2bsr = val;
        self
    }
    pub fn cusparseZcsr2bsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut cuDoubleComplex,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsr2bsr = val;
        self
    }
    pub fn cusparseSbsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f32,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSbsr2csr = val;
        self
    }
    pub fn cusparseDbsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f64,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDbsr2csr = val;
        self
    }
    pub fn cusparseCbsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut cuComplex,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCbsr2csr = val;
        self
    }
    pub fn cusparseZbsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                blockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut cuDoubleComplex,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZbsr2csr = val;
        self
    }
    pub fn cusparseSgebsr2gebsc_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgebsr2gebsc_bufferSize = val;
        self
    }
    pub fn cusparseDgebsr2gebsc_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgebsr2gebsc_bufferSize = val;
        self
    }
    pub fn cusparseCgebsr2gebsc_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgebsr2gebsc_bufferSize = val;
        self
    }
    pub fn cusparseZgebsr2gebsc_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgebsr2gebsc_bufferSize = val;
        self
    }
    pub fn cusparseSgebsr2gebsc_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgebsr2gebsc_bufferSizeExt = val;
        self
    }
    pub fn cusparseDgebsr2gebsc_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgebsr2gebsc_bufferSizeExt = val;
        self
    }
    pub fn cusparseCgebsr2gebsc_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgebsr2gebsc_bufferSizeExt = val;
        self
    }
    pub fn cusparseZgebsr2gebsc_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgebsr2gebsc_bufferSizeExt = val;
        self
    }
    pub fn cusparseSgebsr2gebsc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const f32,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                bscVal: *mut f32,
                bscRowInd: *mut ::std::os::raw::c_int,
                bscColPtr: *mut ::std::os::raw::c_int,
                copyValues: cusparseAction_t,
                idxBase: cusparseIndexBase_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgebsr2gebsc = val;
        self
    }
    pub fn cusparseDgebsr2gebsc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const f64,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                bscVal: *mut f64,
                bscRowInd: *mut ::std::os::raw::c_int,
                bscColPtr: *mut ::std::os::raw::c_int,
                copyValues: cusparseAction_t,
                idxBase: cusparseIndexBase_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgebsr2gebsc = val;
        self
    }
    pub fn cusparseCgebsr2gebsc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const cuComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                bscVal: *mut cuComplex,
                bscRowInd: *mut ::std::os::raw::c_int,
                bscColPtr: *mut ::std::os::raw::c_int,
                copyValues: cusparseAction_t,
                idxBase: cusparseIndexBase_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgebsr2gebsc = val;
        self
    }
    pub fn cusparseZgebsr2gebsc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                bsrSortedVal: *const cuDoubleComplex,
                bsrSortedRowPtr: *const ::std::os::raw::c_int,
                bsrSortedColInd: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                bscVal: *mut cuDoubleComplex,
                bscRowInd: *mut ::std::os::raw::c_int,
                bscColPtr: *mut ::std::os::raw::c_int,
                copyValues: cusparseAction_t,
                idxBase: cusparseIndexBase_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgebsr2gebsc = val;
        self
    }
    pub fn cusparseXgebsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXgebsr2csr = val;
        self
    }
    pub fn cusparseSgebsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f32,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgebsr2csr = val;
        self
    }
    pub fn cusparseDgebsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f64,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgebsr2csr = val;
        self
    }
    pub fn cusparseCgebsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut cuComplex,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgebsr2csr = val;
        self
    }
    pub fn cusparseZgebsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut cuDoubleComplex,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgebsr2csr = val;
        self
    }
    pub fn cusparseScsr2gebsr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsr2gebsr_bufferSize = val;
        self
    }
    pub fn cusparseDcsr2gebsr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsr2gebsr_bufferSize = val;
        self
    }
    pub fn cusparseCcsr2gebsr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsr2gebsr_bufferSize = val;
        self
    }
    pub fn cusparseZcsr2gebsr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsr2gebsr_bufferSize = val;
        self
    }
    pub fn cusparseScsr2gebsr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsr2gebsr_bufferSizeExt = val;
        self
    }
    pub fn cusparseDcsr2gebsr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsr2gebsr_bufferSizeExt = val;
        self
    }
    pub fn cusparseCcsr2gebsr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsr2gebsr_bufferSizeExt = val;
        self
    }
    pub fn cusparseZcsr2gebsr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsr2gebsr_bufferSizeExt = val;
        self
    }
    pub fn cusparseXcsr2gebsrNnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcsr2gebsrNnz = val;
        self
    }
    pub fn cusparseScsr2gebsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut f32,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsr2gebsr = val;
        self
    }
    pub fn cusparseDcsr2gebsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut f64,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsr2gebsr = val;
        self
    }
    pub fn cusparseCcsr2gebsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut cuComplex,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsr2gebsr = val;
        self
    }
    pub fn cusparseZcsr2gebsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const cuDoubleComplex,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut cuDoubleComplex,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
                rowBlockDim: ::std::os::raw::c_int,
                colBlockDim: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsr2gebsr = val;
        self
    }
    pub fn cusparseSgebsr2gebsr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgebsr2gebsr_bufferSize = val;
        self
    }
    pub fn cusparseDgebsr2gebsr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgebsr2gebsr_bufferSize = val;
        self
    }
    pub fn cusparseCgebsr2gebsr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgebsr2gebsr_bufferSize = val;
        self
    }
    pub fn cusparseZgebsr2gebsr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgebsr2gebsr_bufferSize = val;
        self
    }
    pub fn cusparseSgebsr2gebsr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgebsr2gebsr_bufferSizeExt = val;
        self
    }
    pub fn cusparseDgebsr2gebsr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgebsr2gebsr_bufferSizeExt = val;
        self
    }
    pub fn cusparseCgebsr2gebsr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgebsr2gebsr_bufferSizeExt = val;
        self
    }
    pub fn cusparseZgebsr2gebsr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgebsr2gebsr_bufferSizeExt = val;
        self
    }
    pub fn cusparseXgebsr2gebsrNnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXgebsr2gebsrNnz = val;
        self
    }
    pub fn cusparseSgebsr2gebsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f32,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut f32,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSgebsr2gebsr = val;
        self
    }
    pub fn cusparseDgebsr2gebsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const f64,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut f64,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDgebsr2gebsr = val;
        self
    }
    pub fn cusparseCgebsr2gebsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut cuComplex,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCgebsr2gebsr = val;
        self
    }
    pub fn cusparseZgebsr2gebsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                dirA: cusparseDirection_t,
                mb: ::std::os::raw::c_int,
                nb: ::std::os::raw::c_int,
                nnzb: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                bsrSortedValA: *const cuDoubleComplex,
                bsrSortedRowPtrA: *const ::std::os::raw::c_int,
                bsrSortedColIndA: *const ::std::os::raw::c_int,
                rowBlockDimA: ::std::os::raw::c_int,
                colBlockDimA: ::std::os::raw::c_int,
                descrC: cusparseMatDescr_t,
                bsrSortedValC: *mut cuDoubleComplex,
                bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
                bsrSortedColIndC: *mut ::std::os::raw::c_int,
                rowBlockDimC: ::std::os::raw::c_int,
                colBlockDimC: ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZgebsr2gebsr = val;
        self
    }
    pub fn cusparseCreateIdentityPermutation(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                n: ::std::os::raw::c_int,
                p: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateIdentityPermutation = val;
        self
    }
    pub fn cusparseXcoosort_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                cooRowsA: *const ::std::os::raw::c_int,
                cooColsA: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcoosort_bufferSizeExt = val;
        self
    }
    pub fn cusparseXcoosortByRow(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                cooRowsA: *mut ::std::os::raw::c_int,
                cooColsA: *mut ::std::os::raw::c_int,
                P: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcoosortByRow = val;
        self
    }
    pub fn cusparseXcoosortByColumn(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                cooRowsA: *mut ::std::os::raw::c_int,
                cooColsA: *mut ::std::os::raw::c_int,
                P: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcoosortByColumn = val;
        self
    }
    pub fn cusparseXcsrsort_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                csrRowPtrA: *const ::std::os::raw::c_int,
                csrColIndA: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcsrsort_bufferSizeExt = val;
        self
    }
    pub fn cusparseXcsrsort(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrRowPtrA: *const ::std::os::raw::c_int,
                csrColIndA: *mut ::std::os::raw::c_int,
                P: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcsrsort = val;
        self
    }
    pub fn cusparseXcscsort_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                cscColPtrA: *const ::std::os::raw::c_int,
                cscRowIndA: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcscsort_bufferSizeExt = val;
        self
    }
    pub fn cusparseXcscsort(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                cscColPtrA: *const ::std::os::raw::c_int,
                cscRowIndA: *mut ::std::os::raw::c_int,
                P: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseXcscsort = val;
        self
    }
    pub fn cusparseScsru2csr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                csrVal: *mut f32,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsru2csr_bufferSizeExt = val;
        self
    }
    pub fn cusparseDcsru2csr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                csrVal: *mut f64,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsru2csr_bufferSizeExt = val;
        self
    }
    pub fn cusparseCcsru2csr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                csrVal: *mut cuComplex,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsru2csr_bufferSizeExt = val;
        self
    }
    pub fn cusparseZcsru2csr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                csrVal: *mut cuDoubleComplex,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsru2csr_bufferSizeExt = val;
        self
    }
    pub fn cusparseScsru2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrVal: *mut f32,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsru2csr = val;
        self
    }
    pub fn cusparseDcsru2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrVal: *mut f64,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsru2csr = val;
        self
    }
    pub fn cusparseCcsru2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrVal: *mut cuComplex,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsru2csr = val;
        self
    }
    pub fn cusparseZcsru2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrVal: *mut cuDoubleComplex,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsru2csr = val;
        self
    }
    pub fn cusparseScsr2csru(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrVal: *mut f32,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScsr2csru = val;
        self
    }
    pub fn cusparseDcsr2csru(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrVal: *mut f64,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDcsr2csru = val;
        self
    }
    pub fn cusparseCcsr2csru(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrVal: *mut cuComplex,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCcsr2csru = val;
        self
    }
    pub fn cusparseZcsr2csru(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrVal: *mut cuDoubleComplex,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *mut ::std::os::raw::c_int,
                info: csru2csrInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseZcsr2csru = val;
        self
    }
    pub fn cusparseSpruneDense2csr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                threshold: *const f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f32,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneDense2csr_bufferSizeExt = val;
        self
    }
    pub fn cusparseDpruneDense2csr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                threshold: *const f64,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f64,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneDense2csr_bufferSizeExt = val;
        self
    }
    pub fn cusparseSpruneDense2csrNnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                threshold: *const f32,
                descrC: cusparseMatDescr_t,
                csrRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneDense2csrNnz = val;
        self
    }
    pub fn cusparseDpruneDense2csrNnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                threshold: *const f64,
                descrC: cusparseMatDescr_t,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneDense2csrNnz = val;
        self
    }
    pub fn cusparseSpruneDense2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                threshold: *const f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f32,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneDense2csr = val;
        self
    }
    pub fn cusparseDpruneDense2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                threshold: *const f64,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f64,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneDense2csr = val;
        self
    }
    pub fn cusparseSpruneCsr2csr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                threshold: *const f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f32,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneCsr2csr_bufferSizeExt = val;
        self
    }
    pub fn cusparseDpruneCsr2csr_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                threshold: *const f64,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f64,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneCsr2csr_bufferSizeExt = val;
        self
    }
    pub fn cusparseSpruneCsr2csrNnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                threshold: *const f32,
                descrC: cusparseMatDescr_t,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneCsr2csrNnz = val;
        self
    }
    pub fn cusparseDpruneCsr2csrNnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                threshold: *const f64,
                descrC: cusparseMatDescr_t,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneCsr2csrNnz = val;
        self
    }
    pub fn cusparseSpruneCsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                threshold: *const f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f32,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneCsr2csr = val;
        self
    }
    pub fn cusparseDpruneCsr2csr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                threshold: *const f64,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f64,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneCsr2csr = val;
        self
    }
    pub fn cusparseSpruneDense2csrByPercentage_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f32,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneDense2csrByPercentage_bufferSizeExt = val;
        self
    }
    pub fn cusparseDpruneDense2csrByPercentage_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f64,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneDense2csrByPercentage_bufferSizeExt = val;
        self
    }
    pub fn cusparseSpruneDense2csrNnzByPercentage(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneDense2csrNnzByPercentage = val;
        self
    }
    pub fn cusparseDpruneDense2csrNnzByPercentage(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneDense2csrNnzByPercentage = val;
        self
    }
    pub fn cusparseSpruneDense2csrByPercentage(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f32,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneDense2csrByPercentage = val;
        self
    }
    pub fn cusparseDpruneDense2csrByPercentage(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f64,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneDense2csrByPercentage = val;
        self
    }
    pub fn cusparseSpruneCsr2csrByPercentage_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f32,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneCsr2csrByPercentage_bufferSizeExt = val;
        self
    }
    pub fn cusparseDpruneCsr2csrByPercentage_bufferSizeExt(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *const f64,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *const ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBufferSizeInBytes: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneCsr2csrByPercentage_bufferSizeExt = val;
        self
    }
    pub fn cusparseSpruneCsr2csrNnzByPercentage(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneCsr2csrNnzByPercentage = val;
        self
    }
    pub fn cusparseDpruneCsr2csrNnzByPercentage(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedRowPtrC: *mut ::std::os::raw::c_int,
                nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneCsr2csrNnzByPercentage = val;
        self
    }
    pub fn cusparseSpruneCsr2csrByPercentage(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f32,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f32,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpruneCsr2csrByPercentage = val;
        self
    }
    pub fn cusparseDpruneCsr2csrByPercentage(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrSortedValA: *const f64,
                csrSortedRowPtrA: *const ::std::os::raw::c_int,
                csrSortedColIndA: *const ::std::os::raw::c_int,
                percentage: f32,
                descrC: cusparseMatDescr_t,
                csrSortedValC: *mut f64,
                csrSortedRowPtrC: *const ::std::os::raw::c_int,
                csrSortedColIndC: *mut ::std::os::raw::c_int,
                info: pruneInfo_t,
                pBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDpruneCsr2csrByPercentage = val;
        self
    }
    pub fn cusparseCsr2cscEx2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                csrVal: *const ::std::os::raw::c_void,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *const ::std::os::raw::c_int,
                cscVal: *mut ::std::os::raw::c_void,
                cscColPtr: *mut ::std::os::raw::c_int,
                cscRowInd: *mut ::std::os::raw::c_int,
                valType: cudaDataType,
                copyValues: cusparseAction_t,
                idxBase: cusparseIndexBase_t,
                alg: cusparseCsr2CscAlg_t,
                buffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCsr2cscEx2 = val;
        self
    }
    pub fn cusparseCsr2cscEx2_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnz: ::std::os::raw::c_int,
                csrVal: *const ::std::os::raw::c_void,
                csrRowPtr: *const ::std::os::raw::c_int,
                csrColInd: *const ::std::os::raw::c_int,
                cscVal: *mut ::std::os::raw::c_void,
                cscColPtr: *mut ::std::os::raw::c_int,
                cscRowInd: *mut ::std::os::raw::c_int,
                valType: cudaDataType,
                copyValues: cusparseAction_t,
                idxBase: cusparseIndexBase_t,
                alg: cusparseCsr2CscAlg_t,
                bufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCsr2cscEx2_bufferSize = val;
        self
    }
    pub fn cusparseCreateSpVec(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spVecDescr: *mut cusparseSpVecDescr_t,
                size: i64,
                nnz: i64,
                indices: *mut ::std::os::raw::c_void,
                values: *mut ::std::os::raw::c_void,
                idxType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateSpVec = val;
        self
    }
    pub fn cusparseCreateConstSpVec(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spVecDescr: *mut cusparseConstSpVecDescr_t,
                size: i64,
                nnz: i64,
                indices: *const ::std::os::raw::c_void,
                values: *const ::std::os::raw::c_void,
                idxType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateConstSpVec = val;
        self
    }
    pub fn cusparseDestroySpVec(
        mut self,
        val: Option<unsafe extern "C" fn(spVecDescr: cusparseConstSpVecDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroySpVec = val;
        self
    }
    pub fn cusparseSpVecGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spVecDescr: cusparseSpVecDescr_t,
                size: *mut i64,
                nnz: *mut i64,
                indices: *mut *mut ::std::os::raw::c_void,
                values: *mut *mut ::std::os::raw::c_void,
                idxType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpVecGet = val;
        self
    }
    pub fn cusparseConstSpVecGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spVecDescr: cusparseConstSpVecDescr_t,
                size: *mut i64,
                nnz: *mut i64,
                indices: *mut *const ::std::os::raw::c_void,
                values: *mut *const ::std::os::raw::c_void,
                idxType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstSpVecGet = val;
        self
    }
    pub fn cusparseSpVecGetIndexBase(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spVecDescr: cusparseConstSpVecDescr_t,
                idxBase: *mut cusparseIndexBase_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpVecGetIndexBase = val;
        self
    }
    pub fn cusparseSpVecGetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spVecDescr: cusparseSpVecDescr_t,
                values: *mut *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpVecGetValues = val;
        self
    }
    pub fn cusparseConstSpVecGetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spVecDescr: cusparseConstSpVecDescr_t,
                values: *mut *const ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstSpVecGetValues = val;
        self
    }
    pub fn cusparseSpVecSetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spVecDescr: cusparseSpVecDescr_t,
                values: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpVecSetValues = val;
        self
    }
    pub fn cusparseCreateDnVec(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnVecDescr: *mut cusparseDnVecDescr_t,
                size: i64,
                values: *mut ::std::os::raw::c_void,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateDnVec = val;
        self
    }
    pub fn cusparseCreateConstDnVec(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnVecDescr: *mut cusparseConstDnVecDescr_t,
                size: i64,
                values: *const ::std::os::raw::c_void,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateConstDnVec = val;
        self
    }
    pub fn cusparseDestroyDnVec(
        mut self,
        val: Option<unsafe extern "C" fn(dnVecDescr: cusparseConstDnVecDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyDnVec = val;
        self
    }
    pub fn cusparseDnVecGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnVecDescr: cusparseDnVecDescr_t,
                size: *mut i64,
                values: *mut *mut ::std::os::raw::c_void,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnVecGet = val;
        self
    }
    pub fn cusparseConstDnVecGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnVecDescr: cusparseConstDnVecDescr_t,
                size: *mut i64,
                values: *mut *const ::std::os::raw::c_void,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstDnVecGet = val;
        self
    }
    pub fn cusparseDnVecGetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnVecDescr: cusparseDnVecDescr_t,
                values: *mut *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnVecGetValues = val;
        self
    }
    pub fn cusparseConstDnVecGetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnVecDescr: cusparseConstDnVecDescr_t,
                values: *mut *const ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstDnVecGetValues = val;
        self
    }
    pub fn cusparseDnVecSetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnVecDescr: cusparseDnVecDescr_t,
                values: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnVecSetValues = val;
        self
    }
    pub fn cusparseDestroySpMat(
        mut self,
        val: Option<unsafe extern "C" fn(spMatDescr: cusparseConstSpMatDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroySpMat = val;
        self
    }
    pub fn cusparseSpMatGetFormat(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                format: *mut cusparseFormat_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMatGetFormat = val;
        self
    }
    pub fn cusparseSpMatGetIndexBase(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                idxBase: *mut cusparseIndexBase_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMatGetIndexBase = val;
        self
    }
    pub fn cusparseSpMatGetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                values: *mut *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMatGetValues = val;
        self
    }
    pub fn cusparseConstSpMatGetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                values: *mut *const ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstSpMatGetValues = val;
        self
    }
    pub fn cusparseSpMatSetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                values: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMatSetValues = val;
        self
    }
    pub fn cusparseSpMatGetSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                nnz: *mut i64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMatGetSize = val;
        self
    }
    pub fn cusparseSpMatGetStridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                batchCount: *mut ::std::os::raw::c_int,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMatGetStridedBatch = val;
        self
    }
    pub fn cusparseCooSetStridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                batchCount: ::std::os::raw::c_int,
                batchStride: i64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCooSetStridedBatch = val;
        self
    }
    pub fn cusparseCsrSetStridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                batchCount: ::std::os::raw::c_int,
                offsetsBatchStride: i64,
                columnsValuesBatchStride: i64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCsrSetStridedBatch = val;
        self
    }
    pub fn cusparseBsrSetStridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                batchCount: ::std::os::raw::c_int,
                offsetsBatchStride: i64,
                columnsBatchStride: i64,
                ValuesBatchStride: i64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseBsrSetStridedBatch = val;
        self
    }
    pub fn cusparseSpMatGetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                attribute: cusparseSpMatAttribute_t,
                data: *mut ::std::os::raw::c_void,
                dataSize: usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMatGetAttribute = val;
        self
    }
    pub fn cusparseSpMatSetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                attribute: cusparseSpMatAttribute_t,
                data: *mut ::std::os::raw::c_void,
                dataSize: usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMatSetAttribute = val;
        self
    }
    pub fn cusparseCreateCsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseSpMatDescr_t,
                rows: i64,
                cols: i64,
                nnz: i64,
                csrRowOffsets: *mut ::std::os::raw::c_void,
                csrColInd: *mut ::std::os::raw::c_void,
                csrValues: *mut ::std::os::raw::c_void,
                csrRowOffsetsType: cusparseIndexType_t,
                csrColIndType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateCsr = val;
        self
    }
    pub fn cusparseCreateConstCsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseConstSpMatDescr_t,
                rows: i64,
                cols: i64,
                nnz: i64,
                csrRowOffsets: *const ::std::os::raw::c_void,
                csrColInd: *const ::std::os::raw::c_void,
                csrValues: *const ::std::os::raw::c_void,
                csrRowOffsetsType: cusparseIndexType_t,
                csrColIndType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateConstCsr = val;
        self
    }
    pub fn cusparseCreateCsc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseSpMatDescr_t,
                rows: i64,
                cols: i64,
                nnz: i64,
                cscColOffsets: *mut ::std::os::raw::c_void,
                cscRowInd: *mut ::std::os::raw::c_void,
                cscValues: *mut ::std::os::raw::c_void,
                cscColOffsetsType: cusparseIndexType_t,
                cscRowIndType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateCsc = val;
        self
    }
    pub fn cusparseCreateConstCsc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseConstSpMatDescr_t,
                rows: i64,
                cols: i64,
                nnz: i64,
                cscColOffsets: *const ::std::os::raw::c_void,
                cscRowInd: *const ::std::os::raw::c_void,
                cscValues: *const ::std::os::raw::c_void,
                cscColOffsetsType: cusparseIndexType_t,
                cscRowIndType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateConstCsc = val;
        self
    }
    pub fn cusparseCsrGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                nnz: *mut i64,
                csrRowOffsets: *mut *mut ::std::os::raw::c_void,
                csrColInd: *mut *mut ::std::os::raw::c_void,
                csrValues: *mut *mut ::std::os::raw::c_void,
                csrRowOffsetsType: *mut cusparseIndexType_t,
                csrColIndType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCsrGet = val;
        self
    }
    pub fn cusparseConstCsrGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                nnz: *mut i64,
                csrRowOffsets: *mut *const ::std::os::raw::c_void,
                csrColInd: *mut *const ::std::os::raw::c_void,
                csrValues: *mut *const ::std::os::raw::c_void,
                csrRowOffsetsType: *mut cusparseIndexType_t,
                csrColIndType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstCsrGet = val;
        self
    }
    pub fn cusparseCscGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                nnz: *mut i64,
                cscColOffsets: *mut *mut ::std::os::raw::c_void,
                cscRowInd: *mut *mut ::std::os::raw::c_void,
                cscValues: *mut *mut ::std::os::raw::c_void,
                cscColOffsetsType: *mut cusparseIndexType_t,
                cscRowIndType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCscGet = val;
        self
    }
    pub fn cusparseConstCscGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                nnz: *mut i64,
                cscColOffsets: *mut *const ::std::os::raw::c_void,
                cscRowInd: *mut *const ::std::os::raw::c_void,
                cscValues: *mut *const ::std::os::raw::c_void,
                cscColOffsetsType: *mut cusparseIndexType_t,
                cscRowIndType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstCscGet = val;
        self
    }
    pub fn cusparseCsrSetPointers(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                csrRowOffsets: *mut ::std::os::raw::c_void,
                csrColInd: *mut ::std::os::raw::c_void,
                csrValues: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCsrSetPointers = val;
        self
    }
    pub fn cusparseCscSetPointers(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                cscColOffsets: *mut ::std::os::raw::c_void,
                cscRowInd: *mut ::std::os::raw::c_void,
                cscValues: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCscSetPointers = val;
        self
    }
    pub fn cusparseCreateBsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseSpMatDescr_t,
                brows: i64,
                bcols: i64,
                bnnz: i64,
                rowBlockSize: i64,
                colBlockSize: i64,
                bsrRowOffsets: *mut ::std::os::raw::c_void,
                bsrColInd: *mut ::std::os::raw::c_void,
                bsrValues: *mut ::std::os::raw::c_void,
                bsrRowOffsetsType: cusparseIndexType_t,
                bsrColIndType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
                order: cusparseOrder_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateBsr = val;
        self
    }
    pub fn cusparseCreateConstBsr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseConstSpMatDescr_t,
                brows: i64,
                bcols: i64,
                bnnz: i64,
                rowBlockDim: i64,
                colBlockDim: i64,
                bsrRowOffsets: *const ::std::os::raw::c_void,
                bsrColInd: *const ::std::os::raw::c_void,
                bsrValues: *const ::std::os::raw::c_void,
                bsrRowOffsetsType: cusparseIndexType_t,
                bsrColIndType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
                order: cusparseOrder_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateConstBsr = val;
        self
    }
    pub fn cusparseCreateCoo(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseSpMatDescr_t,
                rows: i64,
                cols: i64,
                nnz: i64,
                cooRowInd: *mut ::std::os::raw::c_void,
                cooColInd: *mut ::std::os::raw::c_void,
                cooValues: *mut ::std::os::raw::c_void,
                cooIdxType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateCoo = val;
        self
    }
    pub fn cusparseCreateConstCoo(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseConstSpMatDescr_t,
                rows: i64,
                cols: i64,
                nnz: i64,
                cooRowInd: *const ::std::os::raw::c_void,
                cooColInd: *const ::std::os::raw::c_void,
                cooValues: *const ::std::os::raw::c_void,
                cooIdxType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateConstCoo = val;
        self
    }
    pub fn cusparseCooGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                nnz: *mut i64,
                cooRowInd: *mut *mut ::std::os::raw::c_void,
                cooColInd: *mut *mut ::std::os::raw::c_void,
                cooValues: *mut *mut ::std::os::raw::c_void,
                idxType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCooGet = val;
        self
    }
    pub fn cusparseConstCooGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                nnz: *mut i64,
                cooRowInd: *mut *const ::std::os::raw::c_void,
                cooColInd: *mut *const ::std::os::raw::c_void,
                cooValues: *mut *const ::std::os::raw::c_void,
                idxType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstCooGet = val;
        self
    }
    pub fn cusparseCooSetPointers(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                cooRows: *mut ::std::os::raw::c_void,
                cooColumns: *mut ::std::os::raw::c_void,
                cooValues: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCooSetPointers = val;
        self
    }
    pub fn cusparseCreateBlockedEll(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseSpMatDescr_t,
                rows: i64,
                cols: i64,
                ellBlockSize: i64,
                ellCols: i64,
                ellColInd: *mut ::std::os::raw::c_void,
                ellValue: *mut ::std::os::raw::c_void,
                ellIdxType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateBlockedEll = val;
        self
    }
    pub fn cusparseCreateConstBlockedEll(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseConstSpMatDescr_t,
                rows: i64,
                cols: i64,
                ellBlockSize: i64,
                ellCols: i64,
                ellColInd: *const ::std::os::raw::c_void,
                ellValue: *const ::std::os::raw::c_void,
                ellIdxType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateConstBlockedEll = val;
        self
    }
    pub fn cusparseBlockedEllGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseSpMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                ellBlockSize: *mut i64,
                ellCols: *mut i64,
                ellColInd: *mut *mut ::std::os::raw::c_void,
                ellValue: *mut *mut ::std::os::raw::c_void,
                ellIdxType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseBlockedEllGet = val;
        self
    }
    pub fn cusparseConstBlockedEllGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: cusparseConstSpMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                ellBlockSize: *mut i64,
                ellCols: *mut i64,
                ellColInd: *mut *const ::std::os::raw::c_void,
                ellValue: *mut *const ::std::os::raw::c_void,
                ellIdxType: *mut cusparseIndexType_t,
                idxBase: *mut cusparseIndexBase_t,
                valueType: *mut cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstBlockedEllGet = val;
        self
    }
    pub fn cusparseCreateSlicedEll(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseSpMatDescr_t,
                rows: i64,
                cols: i64,
                nnz: i64,
                sellValuesSize: i64,
                sliceSize: i64,
                sellSliceOffsets: *mut ::std::os::raw::c_void,
                sellColInd: *mut ::std::os::raw::c_void,
                sellValues: *mut ::std::os::raw::c_void,
                sellSliceOffsetsType: cusparseIndexType_t,
                sellColIndType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateSlicedEll = val;
        self
    }
    pub fn cusparseCreateConstSlicedEll(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                spMatDescr: *mut cusparseConstSpMatDescr_t,
                rows: i64,
                cols: i64,
                nnz: i64,
                sellValuesSize: i64,
                sliceSize: i64,
                sellSliceOffsets: *const ::std::os::raw::c_void,
                sellColInd: *const ::std::os::raw::c_void,
                sellValues: *const ::std::os::raw::c_void,
                sellSliceOffsetsType: cusparseIndexType_t,
                sellColIndType: cusparseIndexType_t,
                idxBase: cusparseIndexBase_t,
                valueType: cudaDataType,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateConstSlicedEll = val;
        self
    }
    pub fn cusparseCreateDnMat(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnMatDescr: *mut cusparseDnMatDescr_t,
                rows: i64,
                cols: i64,
                ld: i64,
                values: *mut ::std::os::raw::c_void,
                valueType: cudaDataType,
                order: cusparseOrder_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateDnMat = val;
        self
    }
    pub fn cusparseCreateConstDnMat(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnMatDescr: *mut cusparseConstDnMatDescr_t,
                rows: i64,
                cols: i64,
                ld: i64,
                values: *const ::std::os::raw::c_void,
                valueType: cudaDataType,
                order: cusparseOrder_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseCreateConstDnMat = val;
        self
    }
    pub fn cusparseDestroyDnMat(
        mut self,
        val: Option<unsafe extern "C" fn(dnMatDescr: cusparseConstDnMatDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseDestroyDnMat = val;
        self
    }
    pub fn cusparseDnMatGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnMatDescr: cusparseDnMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                ld: *mut i64,
                values: *mut *mut ::std::os::raw::c_void,
                type_: *mut cudaDataType,
                order: *mut cusparseOrder_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnMatGet = val;
        self
    }
    pub fn cusparseConstDnMatGet(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnMatDescr: cusparseConstDnMatDescr_t,
                rows: *mut i64,
                cols: *mut i64,
                ld: *mut i64,
                values: *mut *const ::std::os::raw::c_void,
                type_: *mut cudaDataType,
                order: *mut cusparseOrder_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstDnMatGet = val;
        self
    }
    pub fn cusparseDnMatGetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnMatDescr: cusparseDnMatDescr_t,
                values: *mut *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnMatGetValues = val;
        self
    }
    pub fn cusparseConstDnMatGetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnMatDescr: cusparseConstDnMatDescr_t,
                values: *mut *const ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseConstDnMatGetValues = val;
        self
    }
    pub fn cusparseDnMatSetValues(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnMatDescr: cusparseDnMatDescr_t,
                values: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnMatSetValues = val;
        self
    }
    pub fn cusparseDnMatSetStridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnMatDescr: cusparseDnMatDescr_t,
                batchCount: ::std::os::raw::c_int,
                batchStride: i64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnMatSetStridedBatch = val;
        self
    }
    pub fn cusparseDnMatGetStridedBatch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dnMatDescr: cusparseConstDnMatDescr_t,
                batchCount: *mut ::std::os::raw::c_int,
                batchStride: *mut i64,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDnMatGetStridedBatch = val;
        self
    }
    pub fn cusparseAxpby(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                alpha: *const ::std::os::raw::c_void,
                vecX: cusparseConstSpVecDescr_t,
                beta: *const ::std::os::raw::c_void,
                vecY: cusparseDnVecDescr_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseAxpby = val;
        self
    }
    pub fn cusparseGather(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                vecY: cusparseConstDnVecDescr_t,
                vecX: cusparseSpVecDescr_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseGather = val;
        self
    }
    pub fn cusparseScatter(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                vecX: cusparseConstSpVecDescr_t,
                vecY: cusparseDnVecDescr_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseScatter = val;
        self
    }
    pub fn cusparseRot(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                c_coeff: *const ::std::os::raw::c_void,
                s_coeff: *const ::std::os::raw::c_void,
                vecX: cusparseSpVecDescr_t,
                vecY: cusparseDnVecDescr_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseRot = val;
        self
    }
    pub fn cusparseSpVV_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opX: cusparseOperation_t,
                vecX: cusparseConstSpVecDescr_t,
                vecY: cusparseConstDnVecDescr_t,
                result: *const ::std::os::raw::c_void,
                computeType: cudaDataType,
                bufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpVV_bufferSize = val;
        self
    }
    pub fn cusparseSpVV(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opX: cusparseOperation_t,
                vecX: cusparseConstSpVecDescr_t,
                vecY: cusparseConstDnVecDescr_t,
                result: *mut ::std::os::raw::c_void,
                computeType: cudaDataType,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpVV = val;
        self
    }
    pub fn cusparseSparseToDense_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseDnMatDescr_t,
                alg: cusparseSparseToDenseAlg_t,
                bufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSparseToDense_bufferSize = val;
        self
    }
    pub fn cusparseSparseToDense(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseDnMatDescr_t,
                alg: cusparseSparseToDenseAlg_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSparseToDense = val;
        self
    }
    pub fn cusparseDenseToSparse_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                matA: cusparseConstDnMatDescr_t,
                matB: cusparseSpMatDescr_t,
                alg: cusparseDenseToSparseAlg_t,
                bufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDenseToSparse_bufferSize = val;
        self
    }
    pub fn cusparseDenseToSparse_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                matA: cusparseConstDnMatDescr_t,
                matB: cusparseSpMatDescr_t,
                alg: cusparseDenseToSparseAlg_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDenseToSparse_analysis = val;
        self
    }
    pub fn cusparseDenseToSparse_convert(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                matA: cusparseConstDnMatDescr_t,
                matB: cusparseSpMatDescr_t,
                alg: cusparseDenseToSparseAlg_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseDenseToSparse_convert = val;
        self
    }
    pub fn cusparseSpMV(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                vecX: cusparseConstDnVecDescr_t,
                beta: *const ::std::os::raw::c_void,
                vecY: cusparseDnVecDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpMVAlg_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMV = val;
        self
    }
    pub fn cusparseSpMV_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                vecX: cusparseConstDnVecDescr_t,
                beta: *const ::std::os::raw::c_void,
                vecY: cusparseDnVecDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpMVAlg_t,
                bufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMV_bufferSize = val;
        self
    }
    pub fn cusparseSpMV_preprocess(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                vecX: cusparseConstDnVecDescr_t,
                beta: *const ::std::os::raw::c_void,
                vecY: cusparseDnVecDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpMVAlg_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMV_preprocess = val;
        self
    }
    pub fn cusparseSpSV_createDescr(
        mut self,
        val: Option<unsafe extern "C" fn(descr: *mut cusparseSpSVDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSpSV_createDescr = val;
        self
    }
    pub fn cusparseSpSV_destroyDescr(
        mut self,
        val: Option<unsafe extern "C" fn(descr: cusparseSpSVDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSpSV_destroyDescr = val;
        self
    }
    pub fn cusparseSpSV_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                vecX: cusparseConstDnVecDescr_t,
                vecY: cusparseDnVecDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpSVAlg_t,
                spsvDescr: cusparseSpSVDescr_t,
                bufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpSV_bufferSize = val;
        self
    }
    pub fn cusparseSpSV_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                vecX: cusparseConstDnVecDescr_t,
                vecY: cusparseDnVecDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpSVAlg_t,
                spsvDescr: cusparseSpSVDescr_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpSV_analysis = val;
        self
    }
    pub fn cusparseSpSV_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                vecX: cusparseConstDnVecDescr_t,
                vecY: cusparseDnVecDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpSVAlg_t,
                spsvDescr: cusparseSpSVDescr_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpSV_solve = val;
        self
    }
    pub fn cusparseSpSV_updateMatrix(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                spsvDescr: cusparseSpSVDescr_t,
                newValues: *mut ::std::os::raw::c_void,
                updatePart: cusparseSpSVUpdate_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpSV_updateMatrix = val;
        self
    }
    pub fn cusparseSpSM_createDescr(
        mut self,
        val: Option<unsafe extern "C" fn(descr: *mut cusparseSpSMDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSpSM_createDescr = val;
        self
    }
    pub fn cusparseSpSM_destroyDescr(
        mut self,
        val: Option<unsafe extern "C" fn(descr: cusparseSpSMDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSpSM_destroyDescr = val;
        self
    }
    pub fn cusparseSpSM_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                matC: cusparseDnMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpSMAlg_t,
                spsmDescr: cusparseSpSMDescr_t,
                bufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpSM_bufferSize = val;
        self
    }
    pub fn cusparseSpSM_analysis(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                matC: cusparseDnMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpSMAlg_t,
                spsmDescr: cusparseSpSMDescr_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpSM_analysis = val;
        self
    }
    pub fn cusparseSpSM_solve(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                matC: cusparseDnMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpSMAlg_t,
                spsmDescr: cusparseSpSMDescr_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpSM_solve = val;
        self
    }
    pub fn cusparseSpSM_updateMatrix(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                spsmDescr: cusparseSpSMDescr_t,
                newValues: *mut ::std::os::raw::c_void,
                updatePart: cusparseSpSMUpdate_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpSM_updateMatrix = val;
        self
    }
    pub fn cusparseSpMM_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseDnMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpMMAlg_t,
                bufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMM_bufferSize = val;
        self
    }
    pub fn cusparseSpMM_preprocess(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseDnMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpMMAlg_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMM_preprocess = val;
        self
    }
    pub fn cusparseSpMM(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseDnMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpMMAlg_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMM = val;
        self
    }
    pub fn cusparseSpGEMM_createDescr(
        mut self,
        val: Option<unsafe extern "C" fn(descr: *mut cusparseSpGEMMDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSpGEMM_createDescr = val;
        self
    }
    pub fn cusparseSpGEMM_destroyDescr(
        mut self,
        val: Option<unsafe extern "C" fn(descr: cusparseSpGEMMDescr_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSpGEMM_destroyDescr = val;
        self
    }
    pub fn cusparseSpGEMM_workEstimation(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstSpMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseSpMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpGEMMAlg_t,
                spgemmDescr: cusparseSpGEMMDescr_t,
                bufferSize1: *mut usize,
                externalBuffer1: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpGEMM_workEstimation = val;
        self
    }
    pub fn cusparseSpGEMM_getNumProducts(
        mut self,
        val: Option<unsafe extern "C" fn(spgemmDescr: cusparseSpGEMMDescr_t, num_prods: *mut i64) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSpGEMM_getNumProducts = val;
        self
    }
    pub fn cusparseSpGEMM_estimateMemory(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstSpMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseSpMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpGEMMAlg_t,
                spgemmDescr: cusparseSpGEMMDescr_t,
                chunk_fraction: f32,
                bufferSize3: *mut usize,
                externalBuffer3: *mut ::std::os::raw::c_void,
                bufferSize2: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpGEMM_estimateMemory = val;
        self
    }
    pub fn cusparseSpGEMM_compute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstSpMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseSpMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpGEMMAlg_t,
                spgemmDescr: cusparseSpGEMMDescr_t,
                bufferSize2: *mut usize,
                externalBuffer2: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpGEMM_compute = val;
        self
    }
    pub fn cusparseSpGEMM_copy(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstSpMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseSpMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpGEMMAlg_t,
                spgemmDescr: cusparseSpGEMMDescr_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpGEMM_copy = val;
        self
    }
    pub fn cusparseSpGEMMreuse_workEstimation(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstSpMatDescr_t,
                matC: cusparseSpMatDescr_t,
                alg: cusparseSpGEMMAlg_t,
                spgemmDescr: cusparseSpGEMMDescr_t,
                bufferSize1: *mut usize,
                externalBuffer1: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpGEMMreuse_workEstimation = val;
        self
    }
    pub fn cusparseSpGEMMreuse_nnz(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstSpMatDescr_t,
                matC: cusparseSpMatDescr_t,
                alg: cusparseSpGEMMAlg_t,
                spgemmDescr: cusparseSpGEMMDescr_t,
                bufferSize2: *mut usize,
                externalBuffer2: *mut ::std::os::raw::c_void,
                bufferSize3: *mut usize,
                externalBuffer3: *mut ::std::os::raw::c_void,
                bufferSize4: *mut usize,
                externalBuffer4: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpGEMMreuse_nnz = val;
        self
    }
    pub fn cusparseSpGEMMreuse_copy(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstSpMatDescr_t,
                matC: cusparseSpMatDescr_t,
                alg: cusparseSpGEMMAlg_t,
                spgemmDescr: cusparseSpGEMMDescr_t,
                bufferSize5: *mut usize,
                externalBuffer5: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpGEMMreuse_copy = val;
        self
    }
    pub fn cusparseSpGEMMreuse_compute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstSpMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseSpMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpGEMMAlg_t,
                spgemmDescr: cusparseSpGEMMDescr_t,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpGEMMreuse_compute = val;
        self
    }
    pub fn cusparseSDDMM_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstDnMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseSpMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSDDMMAlg_t,
                bufferSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSDDMM_bufferSize = val;
        self
    }
    pub fn cusparseSDDMM_preprocess(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstDnMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseSpMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSDDMMAlg_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSDDMM_preprocess = val;
        self
    }
    pub fn cusparseSDDMM(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                alpha: *const ::std::os::raw::c_void,
                matA: cusparseConstDnMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                beta: *const ::std::os::raw::c_void,
                matC: cusparseSpMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSDDMMAlg_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSDDMM = val;
        self
    }
    pub fn cusparseSpMMOp_createPlan(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusparseHandle_t,
                plan: *mut cusparseSpMMOpPlan_t,
                opA: cusparseOperation_t,
                opB: cusparseOperation_t,
                matA: cusparseConstSpMatDescr_t,
                matB: cusparseConstDnMatDescr_t,
                matC: cusparseDnMatDescr_t,
                computeType: cudaDataType,
                alg: cusparseSpMMOpAlg_t,
                addOperationLtoirBuffer: *const ::std::os::raw::c_void,
                addOperationBufferSize: usize,
                mulOperationLtoirBuffer: *const ::std::os::raw::c_void,
                mulOperationBufferSize: usize,
                epilogueLtoirBuffer: *const ::std::os::raw::c_void,
                epilogueBufferSize: usize,
                SpMMWorkspaceSize: *mut usize,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMMOp_createPlan = val;
        self
    }
    pub fn cusparseSpMMOp(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                plan: cusparseSpMMOpPlan_t,
                externalBuffer: *mut ::std::os::raw::c_void,
            ) -> cusparseStatus_t,
        >,
    ) -> Self {
        self.cusparseSpMMOp = val;
        self
    }
    pub fn cusparseSpMMOp_destroyPlan(
        mut self,
        val: Option<unsafe extern "C" fn(plan: cusparseSpMMOpPlan_t) -> cusparseStatus_t>,
    ) -> Self {
        self.cusparseSpMMOp_destroyPlan = val;
        self
    }
}
pub unsafe fn cusparseCreate() -> Result<cusparseHandle_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseHandle_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroy(handle: cusparseHandle_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroy(handle) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseGetVersion(handle: cusparseHandle_t) -> Result<i32, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseGetVersion(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseGetProperty(type_: libraryPropertyType) -> Result<i32, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseGetErrorName(status: cusparseStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cusparseGetErrorName(status) }
}
pub unsafe fn cusparseGetErrorString(status: cusparseStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cusparseGetErrorString(status) }
}
pub unsafe fn cusparseSetStream(
    handle: cusparseHandle_t,
    streamId: cudaStream_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSetStream(handle, streamId) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseGetStream(handle: cusparseHandle_t) -> Result<cudaStream_t, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseGetStream(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaStream_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseGetPointerMode(
    handle: cusparseHandle_t,
) -> Result<cusparsePointerMode_t, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusparsePointerMode_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseGetPointerMode(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusparsePointerMode_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseSetPointerMode(
    handle: cusparseHandle_t,
    mode: cusparsePointerMode_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSetPointerMode(handle, mode) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
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
pub unsafe fn cusparseLoggerSetFile<T: types::CudaAsPtr>(mut file: T) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseLoggerSetFile(file.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseLoggerOpenFile<T: types::CudaAsPtr>(logFile: T) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseLoggerOpenFile(logFile.as_const_ptr() as *const _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseLoggerSetLevel(level: i32) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseLoggerSetLevel(level as _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseLoggerSetMask(mask: i32) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseLoggerSetMask(mask as _) };
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
pub unsafe fn cusparseCreateMatDescr() -> Result<cusparseMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateMatDescr(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyMatDescr(descrA: cusparseMatDescr_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyMatDescr(descrA) };
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
pub unsafe fn cusparseCreateCsric02Info() -> Result<csric02Info_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<csric02Info_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateCsric02Info(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as csric02Info_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyCsric02Info(info: csric02Info_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyCsric02Info(info) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateBsric02Info() -> Result<bsric02Info_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<bsric02Info_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateBsric02Info(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as bsric02Info_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyBsric02Info(info: bsric02Info_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyBsric02Info(info) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateCsrilu02Info() -> Result<csrilu02Info_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<csrilu02Info_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateCsrilu02Info(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as csrilu02Info_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyCsrilu02Info(info: csrilu02Info_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyCsrilu02Info(info) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateBsrilu02Info() -> Result<bsrilu02Info_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<bsrilu02Info_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateBsrilu02Info(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as bsrilu02Info_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyBsrilu02Info(info: bsrilu02Info_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyBsrilu02Info(info) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateBsrsv2Info() -> Result<bsrsv2Info_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<bsrsv2Info_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateBsrsv2Info(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as bsrsv2Info_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyBsrsv2Info(info: bsrsv2Info_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyBsrsv2Info(info) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateBsrsm2Info() -> Result<bsrsm2Info_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<bsrsm2Info_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateBsrsm2Info(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as bsrsm2Info_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyBsrsm2Info(info: bsrsm2Info_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyBsrsm2Info(info) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateCsru2csrInfo() -> Result<csru2csrInfo_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<csru2csrInfo_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateCsru2csrInfo(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as csru2csrInfo_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyCsru2csrInfo(info: csru2csrInfo_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyCsru2csrInfo(info) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateColorInfo() -> Result<cusparseColorInfo_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseColorInfo_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateColorInfo(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseColorInfo_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyColorInfo(info: cusparseColorInfo_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyColorInfo(info) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreatePruneInfo() -> Result<pruneInfo_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<pruneInfo_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreatePruneInfo(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as pruneInfo_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyPruneInfo(info: pruneInfo_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyPruneInfo(info) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgemvi<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    nnz: i32,
    xVal: V,
    xInd: W,
    beta: X,
    mut y: Y,
    idxBase: cusparseIndexBase_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgemvi(
            handle,
            transA,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            nnz as _,
            xVal.as_const_ptr() as *const _,
            xInd.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            idxBase,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgemvi_bufferSize<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut pBufferSize: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgemvi_bufferSize(
            handle,
            transA,
            m as _,
            n as _,
            nnz as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgemvi<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    nnz: i32,
    xVal: V,
    xInd: W,
    beta: X,
    mut y: Y,
    idxBase: cusparseIndexBase_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgemvi(
            handle,
            transA,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            nnz as _,
            xVal.as_const_ptr() as *const _,
            xInd.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            idxBase,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgemvi_bufferSize<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut pBufferSize: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgemvi_bufferSize(
            handle,
            transA,
            m as _,
            n as _,
            nnz as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgemvi<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    nnz: i32,
    xVal: V,
    xInd: W,
    beta: X,
    mut y: Y,
    idxBase: cusparseIndexBase_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgemvi(
            handle,
            transA,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            nnz as _,
            xVal.as_const_ptr() as *const _,
            xInd.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            idxBase,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgemvi_bufferSize<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut pBufferSize: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgemvi_bufferSize(
            handle,
            transA,
            m as _,
            n as _,
            nnz as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgemvi<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: i32,
    n: i32,
    alpha: T,
    A: U,
    lda: i32,
    nnz: i32,
    xVal: V,
    xInd: W,
    beta: X,
    mut y: Y,
    idxBase: cusparseIndexBase_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgemvi(
            handle,
            transA,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            lda as _,
            nnz as _,
            xVal.as_const_ptr() as *const _,
            xInd.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            idxBase,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgemvi_bufferSize<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut pBufferSize: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgemvi_bufferSize(
            handle,
            transA,
            m as _,
            n as _,
            nnz as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrmv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockDim: i32,
    x: X,
    beta: Y,
    mut y: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrmv(
            handle,
            dirA,
            transA,
            mb as _,
            nb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrmv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockDim: i32,
    x: X,
    beta: Y,
    mut y: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrmv(
            handle,
            dirA,
            transA,
            mb as _,
            nb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrmv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockDim: i32,
    x: X,
    beta: Y,
    mut y: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrmv(
            handle,
            dirA,
            transA,
            mb as _,
            nb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrmv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockDim: i32,
    x: X,
    beta: Y,
    mut y: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrmv(
            handle,
            dirA,
            transA,
            mb as _,
            nb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrxmv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    sizeOfMask: i32,
    mb: i32,
    nb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedMaskPtrA: V,
    bsrSortedRowPtrA: W,
    bsrSortedEndPtrA: X,
    bsrSortedColIndA: Y,
    blockDim: i32,
    x: Z,
    beta: A,
    mut y: B,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrxmv(
            handle,
            dirA,
            transA,
            sizeOfMask as _,
            mb as _,
            nb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedMaskPtrA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedEndPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrxmv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    sizeOfMask: i32,
    mb: i32,
    nb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedMaskPtrA: V,
    bsrSortedRowPtrA: W,
    bsrSortedEndPtrA: X,
    bsrSortedColIndA: Y,
    blockDim: i32,
    x: Z,
    beta: A,
    mut y: B,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrxmv(
            handle,
            dirA,
            transA,
            sizeOfMask as _,
            mb as _,
            nb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedMaskPtrA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedEndPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrxmv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    sizeOfMask: i32,
    mb: i32,
    nb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedMaskPtrA: V,
    bsrSortedRowPtrA: W,
    bsrSortedEndPtrA: X,
    bsrSortedColIndA: Y,
    blockDim: i32,
    x: Z,
    beta: A,
    mut y: B,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrxmv(
            handle,
            dirA,
            transA,
            sizeOfMask as _,
            mb as _,
            nb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedMaskPtrA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedEndPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrxmv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
    B: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    sizeOfMask: i32,
    mb: i32,
    nb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedMaskPtrA: V,
    bsrSortedRowPtrA: W,
    bsrSortedEndPtrA: X,
    bsrSortedColIndA: Y,
    blockDim: i32,
    x: Z,
    beta: A,
    mut y: B,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrxmv(
            handle,
            dirA,
            transA,
            sizeOfMask as _,
            mb as _,
            nb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedMaskPtrA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedEndPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXbsrsv2_zeroPivot<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: bsrsv2Info_t,
    mut position: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseXbsrsv2_zeroPivot(handle, info, position.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrsv2_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    info: bsrsv2Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrsv2_bufferSize(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_mut_ptr() as *mut _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrsv2_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    info: bsrsv2Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrsv2_bufferSize(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_mut_ptr() as *mut _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrsv2_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    info: bsrsv2Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrsv2_bufferSize(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_mut_ptr() as *mut _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrsv2_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    info: bsrsv2Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrsv2_bufferSize(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_mut_ptr() as *mut _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrsv2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockSize: i32,
    info: bsrsv2Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrsv2_bufferSizeExt(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_mut_ptr() as *mut _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrsv2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockSize: i32,
    info: bsrsv2Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrsv2_bufferSizeExt(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_mut_ptr() as *mut _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrsv2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockSize: i32,
    info: bsrsv2Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrsv2_bufferSizeExt(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_mut_ptr() as *mut _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrsv2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockSize: i32,
    info: bsrsv2Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrsv2_bufferSizeExt(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_mut_ptr() as *mut _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrsv2_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    info: bsrsv2Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrsv2_analysis(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrsv2_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    info: bsrsv2Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrsv2_analysis(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrsv2_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    info: bsrsv2Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrsv2_analysis(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrsv2_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    info: bsrsv2Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrsv2_analysis(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrsv2_solve<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockDim: i32,
    info: bsrsv2Info_t,
    f: X,
    mut x: Y,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrsv2_solve(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            f.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrsv2_solve<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockDim: i32,
    info: bsrsv2Info_t,
    f: X,
    mut x: Y,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrsv2_solve(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            f.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrsv2_solve<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockDim: i32,
    info: bsrsv2Info_t,
    f: X,
    mut x: Y,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrsv2_solve(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            f.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrsv2_solve<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockDim: i32,
    info: bsrsv2Info_t,
    f: X,
    mut x: Y,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrsv2_solve(
            handle,
            dirA,
            transA,
            mb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            info,
            f.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrmm<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: i32,
    n: i32,
    kb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockSize: i32,
    B: X,
    ldb: i32,
    beta: Y,
    mut C: Z,
    ldc: i32,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrmm(
            handle,
            dirA,
            transA,
            transB,
            mb as _,
            n as _,
            kb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockSize as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrmm<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: i32,
    n: i32,
    kb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockSize: i32,
    B: X,
    ldb: i32,
    beta: Y,
    mut C: Z,
    ldc: i32,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrmm(
            handle,
            dirA,
            transA,
            transB,
            mb as _,
            n as _,
            kb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockSize as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrmm<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: i32,
    n: i32,
    kb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockSize: i32,
    B: X,
    ldb: i32,
    beta: Y,
    mut C: Z,
    ldc: i32,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrmm(
            handle,
            dirA,
            transA,
            transB,
            mb as _,
            n as _,
            kb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockSize as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrmm<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: i32,
    n: i32,
    kb: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: U,
    bsrSortedRowPtrA: V,
    bsrSortedColIndA: W,
    blockSize: i32,
    B: X,
    ldb: i32,
    beta: Y,
    mut C: Z,
    ldc: i32,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrmm(
            handle,
            dirA,
            transA,
            transB,
            mb as _,
            n as _,
            kb as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockSize as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            beta.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXbsrsm2_zeroPivot<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: bsrsm2Info_t,
    mut position: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseXbsrsm2_zeroPivot(handle, info, position.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrsm2_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrsm2_bufferSize(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrsm2_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrsm2_bufferSize(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrsm2_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrsm2_bufferSize(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrsm2_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrsm2_bufferSize(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrsm2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrsm2_bufferSizeExt(
            handle,
            dirA,
            transA,
            transB,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrsm2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrsm2_bufferSizeExt(
            handle,
            dirA,
            transA,
            transB,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrsm2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrsm2_bufferSizeExt(
            handle,
            dirA,
            transA,
            transB,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrsm2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrsm2_bufferSizeExt(
            handle,
            dirA,
            transA,
            transB,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrsm2_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrsm2_analysis(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrsm2_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrsm2_analysis(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrsm2_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrsm2_analysis(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrsm2_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrsm2Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrsm2_analysis(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrsm2_solve<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: U,
    bsrSortedRowPtr: V,
    bsrSortedColInd: W,
    blockSize: i32,
    info: bsrsm2Info_t,
    B: X,
    ldb: i32,
    mut X: Y,
    ldx: i32,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrsm2_solve(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            B.as_const_ptr() as *const _,
            ldb as _,
            X.as_mut_ptr() as *mut _,
            ldx as _,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrsm2_solve<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: U,
    bsrSortedRowPtr: V,
    bsrSortedColInd: W,
    blockSize: i32,
    info: bsrsm2Info_t,
    B: X,
    ldb: i32,
    mut X: Y,
    ldx: i32,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrsm2_solve(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            B.as_const_ptr() as *const _,
            ldb as _,
            X.as_mut_ptr() as *mut _,
            ldx as _,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrsm2_solve<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: U,
    bsrSortedRowPtr: V,
    bsrSortedColInd: W,
    blockSize: i32,
    info: bsrsm2Info_t,
    B: X,
    ldb: i32,
    mut X: Y,
    ldx: i32,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrsm2_solve(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            B.as_const_ptr() as *const _,
            ldb as _,
            X.as_mut_ptr() as *mut _,
            ldx as _,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrsm2_solve<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: i32,
    n: i32,
    nnzb: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: U,
    bsrSortedRowPtr: V,
    bsrSortedColInd: W,
    blockSize: i32,
    info: bsrsm2Info_t,
    B: X,
    ldb: i32,
    mut X: Y,
    ldx: i32,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrsm2_solve(
            handle,
            dirA,
            transA,
            transXY,
            mb as _,
            n as _,
            nnzb as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            B.as_const_ptr() as *const _,
            ldb as _,
            X.as_mut_ptr() as *mut _,
            ldx as _,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsrilu02_numericBoost<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    enable_boost: i32,
    mut tol: T,
    mut boost_val: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsrilu02_numericBoost(
            handle,
            info,
            enable_boost as _,
            tol.as_mut_ptr() as *mut _,
            boost_val.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsrilu02_numericBoost<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    enable_boost: i32,
    mut tol: T,
    mut boost_val: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsrilu02_numericBoost(
            handle,
            info,
            enable_boost as _,
            tol.as_mut_ptr() as *mut _,
            boost_val.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsrilu02_numericBoost<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    enable_boost: i32,
    mut tol: T,
    mut boost_val: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsrilu02_numericBoost(
            handle,
            info,
            enable_boost as _,
            tol.as_mut_ptr() as *mut _,
            boost_val.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsrilu02_numericBoost<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    enable_boost: i32,
    mut tol: T,
    mut boost_val: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsrilu02_numericBoost(
            handle,
            info,
            enable_boost as _,
            tol.as_mut_ptr() as *mut _,
            boost_val.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcsrilu02_zeroPivot<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    mut position: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseXcsrilu02_zeroPivot(handle, info, position.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsrilu02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    info: csrilu02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsrilu02_bufferSize(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsrilu02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    info: csrilu02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsrilu02_bufferSize(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsrilu02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    info: csrilu02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsrilu02_bufferSize(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsrilu02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    info: csrilu02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsrilu02_bufferSize(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsrilu02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedVal: T,
    csrSortedRowPtr: U,
    csrSortedColInd: V,
    info: csrilu02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsrilu02_bufferSizeExt(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedVal.as_mut_ptr() as *mut _,
            csrSortedRowPtr.as_const_ptr() as *const _,
            csrSortedColInd.as_const_ptr() as *const _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsrilu02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedVal: T,
    csrSortedRowPtr: U,
    csrSortedColInd: V,
    info: csrilu02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsrilu02_bufferSizeExt(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedVal.as_mut_ptr() as *mut _,
            csrSortedRowPtr.as_const_ptr() as *const _,
            csrSortedColInd.as_const_ptr() as *const _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsrilu02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedVal: T,
    csrSortedRowPtr: U,
    csrSortedColInd: V,
    info: csrilu02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsrilu02_bufferSizeExt(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedVal.as_mut_ptr() as *mut _,
            csrSortedRowPtr.as_const_ptr() as *const _,
            csrSortedColInd.as_const_ptr() as *const _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsrilu02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedVal: T,
    csrSortedRowPtr: U,
    csrSortedColInd: V,
    info: csrilu02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsrilu02_bufferSizeExt(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedVal.as_mut_ptr() as *mut _,
            csrSortedRowPtr.as_const_ptr() as *const _,
            csrSortedColInd.as_const_ptr() as *const _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsrilu02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsrilu02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsrilu02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsrilu02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsrilu02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA_valM.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsrilu02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA_valM.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsrilu02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA_valM.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsrilu02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA_valM.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrilu02_numericBoost<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    enable_boost: i32,
    mut tol: T,
    mut boost_val: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrilu02_numericBoost(
            handle,
            info,
            enable_boost as _,
            tol.as_mut_ptr() as *mut _,
            boost_val.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrilu02_numericBoost<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    enable_boost: i32,
    mut tol: T,
    mut boost_val: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrilu02_numericBoost(
            handle,
            info,
            enable_boost as _,
            tol.as_mut_ptr() as *mut _,
            boost_val.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrilu02_numericBoost<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    enable_boost: i32,
    mut tol: T,
    mut boost_val: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrilu02_numericBoost(
            handle,
            info,
            enable_boost as _,
            tol.as_mut_ptr() as *mut _,
            boost_val.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrilu02_numericBoost<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    enable_boost: i32,
    mut tol: T,
    mut boost_val: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrilu02_numericBoost(
            handle,
            info,
            enable_boost as _,
            tol.as_mut_ptr() as *mut _,
            boost_val.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXbsrilu02_zeroPivot<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    mut position: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseXbsrilu02_zeroPivot(handle, info, position.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrilu02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrilu02_bufferSize(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrilu02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrilu02_bufferSize(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrilu02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrilu02_bufferSize(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrilu02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrilu02_bufferSize(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrilu02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrilu02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrilu02_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrilu02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrilu02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrilu02_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrilu02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrilu02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrilu02_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrilu02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsrilu02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrilu02_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrilu02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrilu02_analysis(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrilu02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrilu02_analysis(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrilu02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrilu02_analysis(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrilu02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrilu02_analysis(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsrilu02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsrilu02(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsrilu02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsrilu02(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsrilu02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsrilu02(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsrilu02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsrilu02(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcsric02_zeroPivot<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: csric02Info_t,
    mut position: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseXcsric02_zeroPivot(handle, info, position.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsric02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    info: csric02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsric02_bufferSize(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsric02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    info: csric02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsric02_bufferSize(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsric02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    info: csric02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsric02_bufferSize(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsric02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    info: csric02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsric02_bufferSize(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsric02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedVal: T,
    csrSortedRowPtr: U,
    csrSortedColInd: V,
    info: csric02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsric02_bufferSizeExt(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedVal.as_mut_ptr() as *mut _,
            csrSortedRowPtr.as_const_ptr() as *const _,
            csrSortedColInd.as_const_ptr() as *const _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsric02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedVal: T,
    csrSortedRowPtr: U,
    csrSortedColInd: V,
    info: csric02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsric02_bufferSizeExt(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedVal.as_mut_ptr() as *mut _,
            csrSortedRowPtr.as_const_ptr() as *const _,
            csrSortedColInd.as_const_ptr() as *const _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsric02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedVal: T,
    csrSortedRowPtr: U,
    csrSortedColInd: V,
    info: csric02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsric02_bufferSizeExt(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedVal.as_mut_ptr() as *mut _,
            csrSortedRowPtr.as_const_ptr() as *const _,
            csrSortedColInd.as_const_ptr() as *const _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsric02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrSortedVal: T,
    csrSortedRowPtr: U,
    csrSortedColInd: V,
    info: csric02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsric02_bufferSizeExt(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedVal.as_mut_ptr() as *mut _,
            csrSortedRowPtr.as_const_ptr() as *const _,
            csrSortedColInd.as_const_ptr() as *const _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsric02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsric02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsric02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsric02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsric02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA_valM.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsric02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA_valM.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsric02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA_valM.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsric02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA_valM.as_mut_ptr() as *mut _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXbsric02_zeroPivot<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    info: bsric02Info_t,
    mut position: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseXbsric02_zeroPivot(handle, info, position.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsric02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsric02_bufferSize(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsric02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsric02_bufferSize(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsric02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsric02_bufferSize(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsric02_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsric02_bufferSize(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsric02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsric02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsric02_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsric02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsric02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsric02_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsric02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsric02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsric02_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsric02_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockSize: i32,
    info: bsric02Info_t,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsric02_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockSize as _,
            info,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsric02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pInputBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsric02_analysis(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pInputBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsric02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pInputBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsric02_analysis(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pInputBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsric02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pInputBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsric02_analysis(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pInputBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsric02_analysis<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pInputBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsric02_analysis(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pInputBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsric02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsric02(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsric02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsric02(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsric02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsric02(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsric02<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    mut bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    blockDim: i32,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsric02(
            handle,
            dirA,
            mb as _,
            nnzb as _,
            descrA,
            bsrSortedVal.as_mut_ptr() as *mut _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            blockDim as _,
            info,
            policy,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgtsv2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    B: W,
    ldb: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgtsv2_bufferSizeExt(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgtsv2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    B: W,
    ldb: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgtsv2_bufferSizeExt(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgtsv2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    B: W,
    ldb: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgtsv2_bufferSizeExt(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgtsv2_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    B: W,
    ldb: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgtsv2_bufferSizeExt(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgtsv2<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    mut B: W,
    ldb: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgtsv2(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgtsv2<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    mut B: W,
    ldb: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgtsv2(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgtsv2<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    mut B: W,
    ldb: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgtsv2(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgtsv2<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    mut B: W,
    ldb: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgtsv2(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgtsv2_nopivot_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    B: W,
    ldb: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgtsv2_nopivot_bufferSizeExt(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgtsv2_nopivot_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    B: W,
    ldb: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgtsv2_nopivot_bufferSizeExt(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgtsv2_nopivot_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    B: W,
    ldb: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgtsv2_nopivot_bufferSizeExt(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgtsv2_nopivot_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    B: W,
    ldb: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgtsv2_nopivot_bufferSizeExt(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            ldb as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgtsv2_nopivot<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    mut B: W,
    ldb: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgtsv2_nopivot(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgtsv2_nopivot<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    mut B: W,
    ldb: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgtsv2_nopivot(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgtsv2_nopivot<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    mut B: W,
    ldb: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgtsv2_nopivot(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgtsv2_nopivot<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    dl: T,
    d: U,
    du: V,
    mut B: W,
    ldb: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgtsv2_nopivot(
            handle,
            m as _,
            n as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgtsv2StridedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    dl: T,
    d: U,
    du: V,
    x: W,
    batchCount: i32,
    batchStride: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgtsv2StridedBatch_bufferSizeExt(
            handle,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            batchStride as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgtsv2StridedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    dl: T,
    d: U,
    du: V,
    x: W,
    batchCount: i32,
    batchStride: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgtsv2StridedBatch_bufferSizeExt(
            handle,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            batchStride as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgtsv2StridedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    dl: T,
    d: U,
    du: V,
    x: W,
    batchCount: i32,
    batchStride: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgtsv2StridedBatch_bufferSizeExt(
            handle,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            batchStride as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgtsv2StridedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    dl: T,
    d: U,
    du: V,
    x: W,
    batchCount: i32,
    batchStride: i32,
    mut bufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgtsv2StridedBatch_bufferSizeExt(
            handle,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            batchStride as _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgtsv2StridedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    dl: T,
    d: U,
    du: V,
    mut x: W,
    batchCount: i32,
    batchStride: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgtsv2StridedBatch(
            handle,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            batchStride as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgtsv2StridedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    dl: T,
    d: U,
    du: V,
    mut x: W,
    batchCount: i32,
    batchStride: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgtsv2StridedBatch(
            handle,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            batchStride as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgtsv2StridedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    dl: T,
    d: U,
    du: V,
    mut x: W,
    batchCount: i32,
    batchStride: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgtsv2StridedBatch(
            handle,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            batchStride as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgtsv2StridedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    dl: T,
    d: U,
    du: V,
    mut x: W,
    batchCount: i32,
    batchStride: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgtsv2StridedBatch(
            handle,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            batchStride as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgtsvInterleavedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    dl: T,
    d: U,
    du: V,
    x: W,
    batchCount: i32,
    mut pBufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgtsvInterleavedBatch_bufferSizeExt(
            handle,
            algo as _,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgtsvInterleavedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    dl: T,
    d: U,
    du: V,
    x: W,
    batchCount: i32,
    mut pBufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgtsvInterleavedBatch_bufferSizeExt(
            handle,
            algo as _,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgtsvInterleavedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    dl: T,
    d: U,
    du: V,
    x: W,
    batchCount: i32,
    mut pBufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgtsvInterleavedBatch_bufferSizeExt(
            handle,
            algo as _,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgtsvInterleavedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    dl: T,
    d: U,
    du: V,
    x: W,
    batchCount: i32,
    mut pBufferSizeInBytes: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgtsvInterleavedBatch_bufferSizeExt(
            handle,
            algo as _,
            m as _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgtsvInterleavedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    mut dl: T,
    mut d: U,
    mut du: V,
    mut x: W,
    batchCount: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgtsvInterleavedBatch(
            handle,
            algo as _,
            m as _,
            dl.as_mut_ptr() as *mut _,
            d.as_mut_ptr() as *mut _,
            du.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgtsvInterleavedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    mut dl: T,
    mut d: U,
    mut du: V,
    mut x: W,
    batchCount: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgtsvInterleavedBatch(
            handle,
            algo as _,
            m as _,
            dl.as_mut_ptr() as *mut _,
            d.as_mut_ptr() as *mut _,
            du.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgtsvInterleavedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    mut dl: T,
    mut d: U,
    mut du: V,
    mut x: W,
    batchCount: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgtsvInterleavedBatch(
            handle,
            algo as _,
            m as _,
            dl.as_mut_ptr() as *mut _,
            d.as_mut_ptr() as *mut _,
            du.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgtsvInterleavedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    mut dl: T,
    mut d: U,
    mut du: V,
    mut x: W,
    batchCount: i32,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgtsvInterleavedBatch(
            handle,
            algo as _,
            m as _,
            dl.as_mut_ptr() as *mut _,
            d.as_mut_ptr() as *mut _,
            du.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgpsvInterleavedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    ds: T,
    dl: U,
    d: V,
    du: W,
    dw: X,
    x: Y,
    batchCount: i32,
    mut pBufferSizeInBytes: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgpsvInterleavedBatch_bufferSizeExt(
            handle,
            algo as _,
            m as _,
            ds.as_const_ptr() as *const _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            dw.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgpsvInterleavedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    ds: T,
    dl: U,
    d: V,
    du: W,
    dw: X,
    x: Y,
    batchCount: i32,
    mut pBufferSizeInBytes: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgpsvInterleavedBatch_bufferSizeExt(
            handle,
            algo as _,
            m as _,
            ds.as_const_ptr() as *const _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            dw.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgpsvInterleavedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    ds: T,
    dl: U,
    d: V,
    du: W,
    dw: X,
    x: Y,
    batchCount: i32,
    mut pBufferSizeInBytes: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgpsvInterleavedBatch_bufferSizeExt(
            handle,
            algo as _,
            m as _,
            ds.as_const_ptr() as *const _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            dw.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgpsvInterleavedBatch_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    ds: T,
    dl: U,
    d: V,
    du: W,
    dw: X,
    x: Y,
    batchCount: i32,
    mut pBufferSizeInBytes: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgpsvInterleavedBatch_bufferSizeExt(
            handle,
            algo as _,
            m as _,
            ds.as_const_ptr() as *const _,
            dl.as_const_ptr() as *const _,
            d.as_const_ptr() as *const _,
            du.as_const_ptr() as *const _,
            dw.as_const_ptr() as *const _,
            x.as_const_ptr() as *const _,
            batchCount as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgpsvInterleavedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    mut ds: T,
    mut dl: U,
    mut d: V,
    mut du: W,
    mut dw: X,
    mut x: Y,
    batchCount: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgpsvInterleavedBatch(
            handle,
            algo as _,
            m as _,
            ds.as_mut_ptr() as *mut _,
            dl.as_mut_ptr() as *mut _,
            d.as_mut_ptr() as *mut _,
            du.as_mut_ptr() as *mut _,
            dw.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgpsvInterleavedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    mut ds: T,
    mut dl: U,
    mut d: V,
    mut du: W,
    mut dw: X,
    mut x: Y,
    batchCount: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgpsvInterleavedBatch(
            handle,
            algo as _,
            m as _,
            ds.as_mut_ptr() as *mut _,
            dl.as_mut_ptr() as *mut _,
            d.as_mut_ptr() as *mut _,
            du.as_mut_ptr() as *mut _,
            dw.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgpsvInterleavedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    mut ds: T,
    mut dl: U,
    mut d: V,
    mut du: W,
    mut dw: X,
    mut x: Y,
    batchCount: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgpsvInterleavedBatch(
            handle,
            algo as _,
            m as _,
            ds.as_mut_ptr() as *mut _,
            dl.as_mut_ptr() as *mut _,
            d.as_mut_ptr() as *mut _,
            du.as_mut_ptr() as *mut _,
            dw.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgpsvInterleavedBatch<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    algo: i32,
    m: i32,
    mut ds: T,
    mut dl: U,
    mut d: V,
    mut du: W,
    mut dw: X,
    mut x: Y,
    batchCount: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgpsvInterleavedBatch(
            handle,
            algo as _,
            m as _,
            ds.as_mut_ptr() as *mut _,
            dl.as_mut_ptr() as *mut _,
            d.as_mut_ptr() as *mut _,
            du.as_mut_ptr() as *mut _,
            dw.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            batchCount as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsrgeam2_bufferSizeExt<
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
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    nnzA: i32,
    csrSortedValA: U,
    csrSortedRowPtrA: V,
    csrSortedColIndA: W,
    beta: X,
    descrB: cusparseMatDescr_t,
    nnzB: i32,
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
            handle,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            nnzA as _,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            descrB,
            nnzB as _,
            csrSortedValB.as_const_ptr() as *const _,
            csrSortedRowPtrB.as_const_ptr() as *const _,
            csrSortedColIndB.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsrgeam2_bufferSizeExt<
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
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    nnzA: i32,
    csrSortedValA: U,
    csrSortedRowPtrA: V,
    csrSortedColIndA: W,
    beta: X,
    descrB: cusparseMatDescr_t,
    nnzB: i32,
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
            handle,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            nnzA as _,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            descrB,
            nnzB as _,
            csrSortedValB.as_const_ptr() as *const _,
            csrSortedRowPtrB.as_const_ptr() as *const _,
            csrSortedColIndB.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsrgeam2_bufferSizeExt<
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
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    nnzA: i32,
    csrSortedValA: U,
    csrSortedRowPtrA: V,
    csrSortedColIndA: W,
    beta: X,
    descrB: cusparseMatDescr_t,
    nnzB: i32,
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
            handle,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            nnzA as _,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            descrB,
            nnzB as _,
            csrSortedValB.as_const_ptr() as *const _,
            csrSortedRowPtrB.as_const_ptr() as *const _,
            csrSortedColIndB.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsrgeam2_bufferSizeExt<
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
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    nnzA: i32,
    csrSortedValA: U,
    csrSortedRowPtrA: V,
    csrSortedColIndA: W,
    beta: X,
    descrB: cusparseMatDescr_t,
    nnzB: i32,
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
            handle,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            nnzA as _,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            descrB,
            nnzB as _,
            csrSortedValB.as_const_ptr() as *const _,
            csrSortedRowPtrB.as_const_ptr() as *const _,
            csrSortedColIndB.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcsrgeam2Nnz<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    nnzA: i32,
    csrSortedRowPtrA: T,
    csrSortedColIndA: U,
    descrB: cusparseMatDescr_t,
    nnzB: i32,
    csrSortedRowPtrB: V,
    csrSortedColIndB: W,
    descrC: cusparseMatDescr_t,
    mut csrSortedRowPtrC: X,
    mut nnzTotalDevHostPtr: Y,
    mut workspace: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcsrgeam2Nnz(
            handle,
            m as _,
            n as _,
            descrA,
            nnzA as _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            descrB,
            nnzB as _,
            csrSortedRowPtrB.as_const_ptr() as *const _,
            csrSortedColIndB.as_const_ptr() as *const _,
            descrC,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            workspace.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsrgeam2<
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
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    nnzA: i32,
    csrSortedValA: U,
    csrSortedRowPtrA: V,
    csrSortedColIndA: W,
    beta: X,
    descrB: cusparseMatDescr_t,
    nnzB: i32,
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
            handle,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            nnzA as _,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            descrB,
            nnzB as _,
            csrSortedValB.as_const_ptr() as *const _,
            csrSortedRowPtrB.as_const_ptr() as *const _,
            csrSortedColIndB.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsrgeam2<
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
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    nnzA: i32,
    csrSortedValA: U,
    csrSortedRowPtrA: V,
    csrSortedColIndA: W,
    beta: X,
    descrB: cusparseMatDescr_t,
    nnzB: i32,
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
            handle,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            nnzA as _,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            descrB,
            nnzB as _,
            csrSortedValB.as_const_ptr() as *const _,
            csrSortedRowPtrB.as_const_ptr() as *const _,
            csrSortedColIndB.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsrgeam2<
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
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    nnzA: i32,
    csrSortedValA: U,
    csrSortedRowPtrA: V,
    csrSortedColIndA: W,
    beta: X,
    descrB: cusparseMatDescr_t,
    nnzB: i32,
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
            handle,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            nnzA as _,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            descrB,
            nnzB as _,
            csrSortedValB.as_const_ptr() as *const _,
            csrSortedRowPtrB.as_const_ptr() as *const _,
            csrSortedColIndB.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsrgeam2<
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
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    alpha: T,
    descrA: cusparseMatDescr_t,
    nnzA: i32,
    csrSortedValA: U,
    csrSortedRowPtrA: V,
    csrSortedColIndA: W,
    beta: X,
    descrB: cusparseMatDescr_t,
    nnzB: i32,
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
            handle,
            m as _,
            n as _,
            alpha.as_const_ptr() as *const _,
            descrA,
            nnzA as _,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            descrB,
            nnzB as _,
            csrSortedValB.as_const_ptr() as *const _,
            csrSortedRowPtrB.as_const_ptr() as *const _,
            csrSortedColIndB.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsrcolor<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            fractionToColor.as_const_ptr() as *const _,
            ncolors.as_mut_ptr() as *mut _,
            coloring.as_mut_ptr() as *mut _,
            reordering.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            fractionToColor.as_const_ptr() as *const _,
            ncolors.as_mut_ptr() as *mut _,
            coloring.as_mut_ptr() as *mut _,
            reordering.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            fractionToColor.as_const_ptr() as *const _,
            ncolors.as_mut_ptr() as *mut _,
            coloring.as_mut_ptr() as *mut _,
            reordering.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    nnz: i32,
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
            handle,
            m as _,
            nnz as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            fractionToColor.as_const_ptr() as *const _,
            ncolors.as_mut_ptr() as *mut _,
            coloring.as_mut_ptr() as *mut _,
            reordering.as_mut_ptr() as *mut _,
            info,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSnnz<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    A: T,
    lda: i32,
    mut nnzPerRowCol: U,
    mut nnzTotalDevHostPtr: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSnnz(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            A.as_const_ptr() as *const _,
            lda as _,
            nnzPerRowCol.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnnz<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    A: T,
    lda: i32,
    mut nnzPerRowCol: U,
    mut nnzTotalDevHostPtr: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDnnz(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            A.as_const_ptr() as *const _,
            lda as _,
            nnzPerRowCol.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCnnz<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    A: T,
    lda: i32,
    mut nnzPerRowCol: U,
    mut nnzTotalDevHostPtr: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCnnz(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            A.as_const_ptr() as *const _,
            lda as _,
            nnzPerRowCol.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZnnz<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    A: T,
    lda: i32,
    mut nnzPerRowCol: U,
    mut nnzTotalDevHostPtr: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZnnz(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            A.as_const_ptr() as *const _,
            lda as _,
            nnzPerRowCol.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSnnz_compress<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    descr: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    mut nnzPerRow: V,
    mut nnzC: W,
    tol: f32,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSnnz_compress(
            handle,
            m as _,
            descr,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            nnzPerRow.as_mut_ptr() as *mut _,
            nnzC.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    descr: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    mut nnzPerRow: V,
    mut nnzC: W,
    tol: f64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDnnz_compress(
            handle,
            m as _,
            descr,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            nnzPerRow.as_mut_ptr() as *mut _,
            nnzC.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    descr: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    mut nnzPerRow: V,
    mut nnzC: W,
    tol: cuComplex,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCnnz_compress(
            handle,
            m as _,
            descr,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            nnzPerRow.as_mut_ptr() as *mut _,
            nnzC.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    descr: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    mut nnzPerRow: V,
    mut nnzC: W,
    tol: cuDoubleComplex,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZnnz_compress(
            handle,
            m as _,
            descr,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            nnzPerRow.as_mut_ptr() as *mut _,
            nnzC.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedColIndA: U,
    csrSortedRowPtrA: V,
    nnzA: i32,
    nnzPerRow: W,
    mut csrSortedValC: X,
    mut csrSortedColIndC: Y,
    mut csrSortedRowPtrC: Z,
    tol: f32,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsr2csr_compress(
            handle,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            nnzA as _,
            nnzPerRow.as_const_ptr() as *const _,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedColIndA: U,
    csrSortedRowPtrA: V,
    nnzA: i32,
    nnzPerRow: W,
    mut csrSortedValC: X,
    mut csrSortedColIndC: Y,
    mut csrSortedRowPtrC: Z,
    tol: f64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsr2csr_compress(
            handle,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            nnzA as _,
            nnzPerRow.as_const_ptr() as *const _,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedColIndA: U,
    csrSortedRowPtrA: V,
    nnzA: i32,
    nnzPerRow: W,
    mut csrSortedValC: X,
    mut csrSortedColIndC: Y,
    mut csrSortedRowPtrC: Z,
    tol: cuComplex,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsr2csr_compress(
            handle,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            nnzA as _,
            nnzPerRow.as_const_ptr() as *const _,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedColIndA: U,
    csrSortedRowPtrA: V,
    nnzA: i32,
    nnzPerRow: W,
    mut csrSortedValC: X,
    mut csrSortedColIndC: Y,
    mut csrSortedRowPtrC: Z,
    tol: cuDoubleComplex,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsr2csr_compress(
            handle,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            nnzA as _,
            nnzPerRow.as_const_ptr() as *const _,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            tol,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcoo2csr<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    cooRowInd: T,
    nnz: i32,
    m: i32,
    mut csrSortedRowPtr: U,
    idxBase: cusparseIndexBase_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcoo2csr(
            handle,
            cooRowInd.as_const_ptr() as *const _,
            nnz as _,
            m as _,
            csrSortedRowPtr.as_mut_ptr() as *mut _,
            idxBase,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcsr2coo<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    csrSortedRowPtr: T,
    nnz: i32,
    m: i32,
    mut cooRowInd: U,
    idxBase: cusparseIndexBase_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcsr2coo(
            handle,
            csrSortedRowPtr.as_const_ptr() as *const _,
            nnz as _,
            m as _,
            cooRowInd.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedRowPtrA: T,
    csrSortedColIndA: U,
    blockDim: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedRowPtrC: V,
    mut nnzTotalDevHostPtr: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcsr2bsrNnz(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            descrC,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsr2bsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    blockDim: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsr2bsr(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsr2bsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    blockDim: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsr2bsr(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsr2bsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    blockDim: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsr2bsr(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsr2bsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    blockDim: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsr2bsr(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSbsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: W,
    mut csrSortedRowPtrC: X,
    mut csrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSbsr2csr(
            handle,
            dirA,
            mb as _,
            nb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDbsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: W,
    mut csrSortedRowPtrC: X,
    mut csrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDbsr2csr(
            handle,
            dirA,
            mb as _,
            nb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCbsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: W,
    mut csrSortedRowPtrC: X,
    mut csrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCbsr2csr(
            handle,
            dirA,
            mb as _,
            nb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZbsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    blockDim: i32,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: W,
    mut csrSortedRowPtrC: X,
    mut csrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZbsr2csr(
            handle,
            dirA,
            mb as _,
            nb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            blockDim as _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgebsr2gebsc_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgebsr2gebsc_bufferSize(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgebsr2gebsc_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgebsr2gebsc_bufferSize(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgebsr2gebsc_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgebsr2gebsc_bufferSize(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgebsr2gebsc_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgebsr2gebsc_bufferSize(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgebsr2gebsc_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgebsr2gebsc_bufferSizeExt(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgebsr2gebsc_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgebsr2gebsc_bufferSizeExt(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgebsr2gebsc_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgebsr2gebsc_bufferSizeExt(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgebsr2gebsc_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgebsr2gebsc_bufferSizeExt(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgebsr2gebsc<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut bscVal: W,
    mut bscRowInd: X,
    mut bscColPtr: Y,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgebsr2gebsc(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            bscVal.as_mut_ptr() as *mut _,
            bscRowInd.as_mut_ptr() as *mut _,
            bscColPtr.as_mut_ptr() as *mut _,
            copyValues,
            idxBase,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgebsr2gebsc<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut bscVal: W,
    mut bscRowInd: X,
    mut bscColPtr: Y,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgebsr2gebsc(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            bscVal.as_mut_ptr() as *mut _,
            bscRowInd.as_mut_ptr() as *mut _,
            bscColPtr.as_mut_ptr() as *mut _,
            copyValues,
            idxBase,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgebsr2gebsc<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut bscVal: W,
    mut bscRowInd: X,
    mut bscColPtr: Y,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgebsr2gebsc(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            bscVal.as_mut_ptr() as *mut _,
            bscRowInd.as_mut_ptr() as *mut _,
            bscColPtr.as_mut_ptr() as *mut _,
            copyValues,
            idxBase,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgebsr2gebsc<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    bsrSortedVal: T,
    bsrSortedRowPtr: U,
    bsrSortedColInd: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut bscVal: W,
    mut bscRowInd: X,
    mut bscColPtr: Y,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgebsr2gebsc(
            handle,
            mb as _,
            nb as _,
            nnzb as _,
            bsrSortedVal.as_const_ptr() as *const _,
            bsrSortedRowPtr.as_const_ptr() as *const _,
            bsrSortedColInd.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            bscVal.as_mut_ptr() as *mut _,
            bscRowInd.as_mut_ptr() as *mut _,
            bscColPtr.as_mut_ptr() as *mut _,
            copyValues,
            idxBase,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXgebsr2csr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedRowPtrA: T,
    bsrSortedColIndA: U,
    rowBlockDim: i32,
    colBlockDim: i32,
    descrC: cusparseMatDescr_t,
    mut csrSortedRowPtrC: V,
    mut csrSortedColIndC: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXgebsr2csr(
            handle,
            dirA,
            mb as _,
            nb as _,
            descrA,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            descrC,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgebsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: W,
    mut csrSortedRowPtrC: X,
    mut csrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgebsr2csr(
            handle,
            dirA,
            mb as _,
            nb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgebsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: W,
    mut csrSortedRowPtrC: X,
    mut csrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgebsr2csr(
            handle,
            dirA,
            mb as _,
            nb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgebsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: W,
    mut csrSortedRowPtrC: X,
    mut csrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgebsr2csr(
            handle,
            dirA,
            mb as _,
            nb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgebsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: W,
    mut csrSortedRowPtrC: X,
    mut csrSortedColIndC: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgebsr2csr(
            handle,
            dirA,
            mb as _,
            nb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsr2gebsr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsr2gebsr_bufferSize(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsr2gebsr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsr2gebsr_bufferSize(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsr2gebsr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsr2gebsr_bufferSize(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsr2gebsr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsr2gebsr_bufferSize(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsr2gebsr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsr2gebsr_bufferSizeExt(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsr2gebsr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsr2gebsr_bufferSizeExt(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsr2gebsr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsr2gebsr_bufferSizeExt(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsr2gebsr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsr2gebsr_bufferSizeExt(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDim as _,
            colBlockDim as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcsr2gebsrNnz<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedRowPtrA: T,
    csrSortedColIndA: U,
    descrC: cusparseMatDescr_t,
    mut bsrSortedRowPtrC: V,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut nnzTotalDevHostPtr: W,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcsr2gebsrNnz(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            descrC,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            rowBlockDim as _,
            colBlockDim as _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsr2gebsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsr2gebsr(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
            rowBlockDim as _,
            colBlockDim as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsr2gebsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsr2gebsr(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
            rowBlockDim as _,
            colBlockDim as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsr2gebsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsr2gebsr(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
            rowBlockDim as _,
            colBlockDim as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsr2gebsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: i32,
    n: i32,
    descrA: cusparseMatDescr_t,
    csrSortedValA: T,
    csrSortedRowPtrA: U,
    csrSortedColIndA: V,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
    rowBlockDim: i32,
    colBlockDim: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsr2gebsr(
            handle,
            dirA,
            m as _,
            n as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
            rowBlockDim as _,
            colBlockDim as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgebsr2gebsr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgebsr2gebsr_bufferSize(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgebsr2gebsr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgebsr2gebsr_bufferSize(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgebsr2gebsr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgebsr2gebsr_bufferSize(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgebsr2gebsr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgebsr2gebsr_bufferSize(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgebsr2gebsr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgebsr2gebsr_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgebsr2gebsr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgebsr2gebsr_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgebsr2gebsr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgebsr2gebsr_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgebsr2gebsr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBufferSize: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgebsr2gebsr_bufferSizeExt(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXgebsr2gebsrNnz<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedRowPtrA: T,
    bsrSortedColIndA: U,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedRowPtrC: V,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut nnzTotalDevHostPtr: W,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXgebsr2gebsrNnz(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            descrC,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            rowBlockDimC as _,
            colBlockDimC as _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSgebsr2gebsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSgebsr2gebsr(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDgebsr2gebsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDgebsr2gebsr(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCgebsr2gebsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCgebsr2gebsr(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZgebsr2gebsr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: i32,
    nb: i32,
    nnzb: i32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: T,
    bsrSortedRowPtrA: U,
    bsrSortedColIndA: V,
    rowBlockDimA: i32,
    colBlockDimA: i32,
    descrC: cusparseMatDescr_t,
    mut bsrSortedValC: W,
    mut bsrSortedRowPtrC: X,
    mut bsrSortedColIndC: Y,
    rowBlockDimC: i32,
    colBlockDimC: i32,
    mut pBuffer: Z,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZgebsr2gebsr(
            handle,
            dirA,
            mb as _,
            nb as _,
            nnzb as _,
            descrA,
            bsrSortedValA.as_const_ptr() as *const _,
            bsrSortedRowPtrA.as_const_ptr() as *const _,
            bsrSortedColIndA.as_const_ptr() as *const _,
            rowBlockDimA as _,
            colBlockDimA as _,
            descrC,
            bsrSortedValC.as_mut_ptr() as *mut _,
            bsrSortedRowPtrC.as_mut_ptr() as *mut _,
            bsrSortedColIndC.as_mut_ptr() as *mut _,
            rowBlockDimC as _,
            colBlockDimC as _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateIdentityPermutation(
    handle: cusparseHandle_t,
    n: i32,
) -> Result<i32, crate::sys::cusparseStatus_t> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateIdentityPermutation(handle, n as _, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseXcoosort_bufferSizeExt<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    cooRowsA: T,
    cooColsA: U,
    mut pBufferSizeInBytes: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcoosort_bufferSizeExt(
            handle,
            m as _,
            n as _,
            nnz as _,
            cooRowsA.as_const_ptr() as *const _,
            cooColsA.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcoosortByRow<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut cooRowsA: T,
    mut cooColsA: U,
    mut P: V,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcoosortByRow(
            handle,
            m as _,
            n as _,
            nnz as _,
            cooRowsA.as_mut_ptr() as *mut _,
            cooColsA.as_mut_ptr() as *mut _,
            P.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcoosortByColumn<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut cooRowsA: T,
    mut cooColsA: U,
    mut P: V,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcoosortByColumn(
            handle,
            m as _,
            n as _,
            nnz as _,
            cooRowsA.as_mut_ptr() as *mut _,
            cooColsA.as_mut_ptr() as *mut _,
            P.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcsrsort_bufferSizeExt<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    csrRowPtrA: T,
    csrColIndA: U,
    mut pBufferSizeInBytes: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcsrsort_bufferSizeExt(
            handle,
            m as _,
            n as _,
            nnz as _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcsrsort<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    mut csrColIndA: U,
    mut P: V,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcsrsort(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_mut_ptr() as *mut _,
            P.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcscsort_bufferSizeExt<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    cscColPtrA: T,
    cscRowIndA: U,
    mut pBufferSizeInBytes: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcscsort_bufferSizeExt(
            handle,
            m as _,
            n as _,
            nnz as _,
            cscColPtrA.as_const_ptr() as *const _,
            cscRowIndA.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseXcscsort<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    cscColPtrA: T,
    mut cscRowIndA: U,
    mut P: V,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseXcscsort(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            cscColPtrA.as_const_ptr() as *const _,
            cscRowIndA.as_mut_ptr() as *mut _,
            P.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsru2csr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsru2csr_bufferSizeExt(
            handle,
            m as _,
            n as _,
            nnz as _,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsru2csr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsru2csr_bufferSizeExt(
            handle,
            m as _,
            n as _,
            nnz as _,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsru2csr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsru2csr_bufferSizeExt(
            handle,
            m as _,
            n as _,
            nnz as _,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsru2csr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBufferSizeInBytes: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsru2csr_bufferSizeExt(
            handle,
            m as _,
            n as _,
            nnz as _,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsru2csr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsru2csr(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsru2csr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsru2csr(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsru2csr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsru2csr(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsru2csr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsru2csr(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScsr2csru<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseScsr2csru(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDcsr2csru<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDcsr2csru(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCcsr2csru<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCcsr2csru(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseZcsr2csru<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    mut csrVal: T,
    csrRowPtr: U,
    mut csrColInd: V,
    info: csru2csrInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseZcsr2csru(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_mut_ptr() as *mut _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneDense2csr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    threshold: U,
    descrC: cusparseMatDescr_t,
    csrSortedValC: V,
    csrSortedRowPtrC: W,
    csrSortedColIndC: X,
    mut pBufferSizeInBytes: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpruneDense2csr_bufferSizeExt(
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneDense2csr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    threshold: U,
    descrC: cusparseMatDescr_t,
    csrSortedValC: V,
    csrSortedRowPtrC: W,
    csrSortedColIndC: X,
    mut pBufferSizeInBytes: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDpruneDense2csr_bufferSizeExt(
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneDense2csrNnz<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    threshold: U,
    descrC: cusparseMatDescr_t,
    mut csrRowPtrC: V,
    mut nnzTotalDevHostPtr: W,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpruneDense2csrNnz(
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneDense2csrNnz<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    threshold: U,
    descrC: cusparseMatDescr_t,
    mut csrSortedRowPtrC: V,
    mut nnzTotalDevHostPtr: W,
    mut pBuffer: X,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDpruneDense2csrNnz(
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneDense2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    threshold: U,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: V,
    csrSortedRowPtrC: W,
    mut csrSortedColIndC: X,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpruneDense2csr(
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneDense2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    threshold: U,
    descrC: cusparseMatDescr_t,
    mut csrSortedValC: V,
    csrSortedRowPtrC: W,
    mut csrSortedColIndC: X,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDpruneDense2csr(
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneCsr2csr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneCsr2csr_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneCsr2csrNnz<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneCsr2csrNnz<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneCsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneCsr2csr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            threshold.as_const_ptr() as *const _,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneDense2csrByPercentage_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
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
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            percentage,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneDense2csrByPercentage_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
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
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            percentage,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneDense2csrNnzByPercentage<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    mut csrRowPtrC: U,
    mut nnzTotalDevHostPtr: V,
    info: pruneInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpruneDense2csrNnzByPercentage(
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            percentage,
            descrC,
            csrRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneDense2csrNnzByPercentage<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    mut csrRowPtrC: U,
    mut nnzTotalDevHostPtr: V,
    info: pruneInfo_t,
    mut pBuffer: W,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDpruneDense2csrNnzByPercentage(
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            percentage,
            descrC,
            csrRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneDense2csrByPercentage<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
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
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            percentage,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneDense2csrByPercentage<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
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
            handle,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            percentage,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneCsr2csrByPercentage_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            percentage,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneCsr2csrByPercentage_bufferSizeExt<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            percentage,
            descrC,
            csrSortedValC.as_const_ptr() as *const _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_const_ptr() as *const _,
            info,
            pBufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneCsr2csrNnzByPercentage<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            percentage,
            descrC,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneCsr2csrNnzByPercentage<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            percentage,
            descrC,
            csrSortedRowPtrC.as_mut_ptr() as *mut _,
            nnzTotalDevHostPtr.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpruneCsr2csrByPercentage<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            percentage,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDpruneCsr2csrByPercentage<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrSortedValA.as_const_ptr() as *const _,
            csrSortedRowPtrA.as_const_ptr() as *const _,
            csrSortedColIndA.as_const_ptr() as *const _,
            percentage,
            descrC,
            csrSortedValC.as_mut_ptr() as *mut _,
            csrSortedRowPtrC.as_const_ptr() as *const _,
            csrSortedColIndC.as_mut_ptr() as *mut _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCsr2cscEx2<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
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
            handle,
            m as _,
            n as _,
            nnz as _,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            cscVal.as_mut_ptr() as *mut _,
            cscColPtr.as_mut_ptr() as *mut _,
            cscRowInd.as_mut_ptr() as *mut _,
            valType,
            copyValues,
            idxBase,
            alg,
            buffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCsr2cscEx2_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
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
            handle,
            m as _,
            n as _,
            nnz as _,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            cscVal.as_mut_ptr() as *mut _,
            cscColPtr.as_mut_ptr() as *mut _,
            cscRowInd.as_mut_ptr() as *mut _,
            valType,
            copyValues,
            idxBase,
            alg,
            bufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateSpVec(
    size: i64,
    nnz: i64,
    indices: *mut ::std::os::raw::c_void,
    values: *mut ::std::os::raw::c_void,
    idxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseSpVecDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseSpVecDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateSpVec(
            out_0.as_mut_ptr() as *mut _,
            size,
            nnz,
            indices,
            values,
            idxType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseSpVecDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateConstSpVec(
    size: i64,
    nnz: i64,
    indices: *const ::std::os::raw::c_void,
    values: *const ::std::os::raw::c_void,
    idxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseConstSpVecDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseConstSpVecDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateConstSpVec(
            out_0.as_mut_ptr() as *mut _,
            size,
            nnz,
            indices,
            values,
            idxType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseConstSpVecDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroySpVec(spVecDescr: cusparseConstSpVecDescr_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroySpVec(spVecDescr) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpVecGet(
    spVecDescr: cusparseSpVecDescr_t,
    indices: *mut *mut ::std::os::raw::c_void,
    values: *mut *mut ::std::os::raw::c_void,
) -> Result<(i64, i64, cusparseIndexType_t, cusparseIndexBase_t, cudaDataType), crate::sys::cusparseStatus_t> {
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_5.assume_init() as cusparseIndexType_t,
                out_6.assume_init() as cusparseIndexBase_t,
                out_7.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseConstSpVecGet(
    spVecDescr: cusparseConstSpVecDescr_t,
    indices: *mut *const ::std::os::raw::c_void,
    values: *mut *const ::std::os::raw::c_void,
) -> Result<(i64, i64, cusparseIndexType_t, cusparseIndexBase_t, cudaDataType), crate::sys::cusparseStatus_t> {
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_5.assume_init() as cusparseIndexType_t,
                out_6.assume_init() as cusparseIndexBase_t,
                out_7.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseSpVecGetIndexBase(
    spVecDescr: cusparseConstSpVecDescr_t,
) -> Result<cusparseIndexBase_t, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseSpVecGetIndexBase(spVecDescr, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusparseIndexBase_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseSpVecGetValues<T: types::CudaAsPtr>(
    spVecDescr: cusparseSpVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpVecGetValues(spVecDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstSpVecGetValues<T: types::CudaAsPtr>(
    spVecDescr: cusparseConstSpVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseConstSpVecGetValues(spVecDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpVecSetValues<T: types::CudaAsPtr>(
    spVecDescr: cusparseSpVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpVecSetValues(spVecDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateDnVec(
    size: i64,
    values: *mut ::std::os::raw::c_void,
    valueType: cudaDataType,
) -> Result<cusparseDnVecDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseDnVecDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateDnVec(out_0.as_mut_ptr() as *mut _, size, values, valueType) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseDnVecDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateConstDnVec(
    size: i64,
    values: *const ::std::os::raw::c_void,
    valueType: cudaDataType,
) -> Result<cusparseConstDnVecDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseConstDnVecDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseCreateConstDnVec(out_0.as_mut_ptr() as *mut _, size, values, valueType) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseConstDnVecDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyDnVec(dnVecDescr: cusparseConstDnVecDescr_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyDnVec(dnVecDescr) };
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as i64, out_3.assume_init() as cudaDataType)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as i64, out_3.assume_init() as cudaDataType)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDnVecGetValues<T: types::CudaAsPtr>(
    dnVecDescr: cusparseDnVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDnVecGetValues(dnVecDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstDnVecGetValues<T: types::CudaAsPtr>(
    dnVecDescr: cusparseConstDnVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseConstDnVecGetValues(dnVecDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnVecSetValues<T: types::CudaAsPtr>(
    dnVecDescr: cusparseDnVecDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDnVecSetValues(dnVecDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDestroySpMat(spMatDescr: cusparseConstSpMatDescr_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroySpMat(spMatDescr) };
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
    let status = unsafe { crate::sys::cusparseSpMatGetFormat(spMatDescr, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusparseFormat_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseSpMatGetIndexBase(
    spMatDescr: cusparseConstSpMatDescr_t,
) -> Result<cusparseIndexBase_t, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusparseIndexBase_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseSpMatGetIndexBase(spMatDescr, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusparseIndexBase_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseSpMatGetValues<T: types::CudaAsPtr>(
    spMatDescr: cusparseSpMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpMatGetValues(spMatDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstSpMatGetValues<T: types::CudaAsPtr>(
    spMatDescr: cusparseConstSpMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseConstSpMatGetValues(spMatDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatSetValues<T: types::CudaAsPtr>(
    spMatDescr: cusparseSpMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpMatSetValues(spMatDescr, values.as_mut_ptr() as *mut _) };
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseSpMatGetStridedBatch(
    spMatDescr: cusparseConstSpMatDescr_t,
) -> Result<i32, crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusparseSpMatGetStridedBatch(spMatDescr, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCooSetStridedBatch(
    spMatDescr: cusparseSpMatDescr_t,
    batchCount: i32,
    batchStride: i64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseCooSetStridedBatch(spMatDescr, batchCount as _, batchStride) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCsrSetStridedBatch(
    spMatDescr: cusparseSpMatDescr_t,
    batchCount: i32,
    offsetsBatchStride: i64,
    columnsValuesBatchStride: i64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCsrSetStridedBatch(
            spMatDescr,
            batchCount as _,
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
    batchCount: i32,
    offsetsBatchStride: i64,
    columnsBatchStride: i64,
    ValuesBatchStride: i64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseBsrSetStridedBatch(
            spMatDescr,
            batchCount as _,
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
pub unsafe fn cusparseSpMatGetAttribute<T: types::CudaAsPtr>(
    spMatDescr: cusparseConstSpMatDescr_t,
    attribute: cusparseSpMatAttribute_t,
    mut data: T,
    dataSize: usize,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status =
        unsafe { crate::sys::cusparseSpMatGetAttribute(spMatDescr, attribute, data.as_mut_ptr() as *mut _, dataSize) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMatSetAttribute<T: types::CudaAsPtr>(
    spMatDescr: cusparseSpMatDescr_t,
    attribute: cusparseSpMatAttribute_t,
    mut data: T,
    dataSize: usize,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status =
        unsafe { crate::sys::cusparseSpMatSetAttribute(spMatDescr, attribute, data.as_mut_ptr() as *mut _, dataSize) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateCsr(
    rows: i64,
    cols: i64,
    nnz: i64,
    csrRowOffsets: *mut ::std::os::raw::c_void,
    csrColInd: *mut ::std::os::raw::c_void,
    csrValues: *mut ::std::os::raw::c_void,
    csrRowOffsetsType: cusparseIndexType_t,
    csrColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateCsr(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            nnz,
            csrRowOffsets,
            csrColInd,
            csrValues,
            csrRowOffsetsType,
            csrColIndType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateConstCsr(
    rows: i64,
    cols: i64,
    nnz: i64,
    csrRowOffsets: *const ::std::os::raw::c_void,
    csrColInd: *const ::std::os::raw::c_void,
    csrValues: *const ::std::os::raw::c_void,
    csrRowOffsetsType: cusparseIndexType_t,
    csrColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseConstSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseConstSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateConstCsr(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            nnz,
            csrRowOffsets,
            csrColInd,
            csrValues,
            csrRowOffsetsType,
            csrColIndType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseConstSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateCsc(
    rows: i64,
    cols: i64,
    nnz: i64,
    cscColOffsets: *mut ::std::os::raw::c_void,
    cscRowInd: *mut ::std::os::raw::c_void,
    cscValues: *mut ::std::os::raw::c_void,
    cscColOffsetsType: cusparseIndexType_t,
    cscRowIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateCsc(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            nnz,
            cscColOffsets,
            cscRowInd,
            cscValues,
            cscColOffsetsType,
            cscRowIndType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateConstCsc(
    rows: i64,
    cols: i64,
    nnz: i64,
    cscColOffsets: *const ::std::os::raw::c_void,
    cscRowInd: *const ::std::os::raw::c_void,
    cscValues: *const ::std::os::raw::c_void,
    cscColOffsetsType: cusparseIndexType_t,
    cscRowIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseConstSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseConstSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateConstCsc(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            nnz,
            cscColOffsets,
            cscRowInd,
            cscValues,
            cscColOffsetsType,
            cscRowIndType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseConstSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_7.assume_init() as cusparseIndexType_t,
                out_8.assume_init() as cusparseIndexType_t,
                out_9.assume_init() as cusparseIndexBase_t,
                out_10.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_7.assume_init() as cusparseIndexType_t,
                out_8.assume_init() as cusparseIndexType_t,
                out_9.assume_init() as cusparseIndexBase_t,
                out_10.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_7.assume_init() as cusparseIndexType_t,
                out_8.assume_init() as cusparseIndexType_t,
                out_9.assume_init() as cusparseIndexBase_t,
                out_10.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_7.assume_init() as cusparseIndexType_t,
                out_8.assume_init() as cusparseIndexType_t,
                out_9.assume_init() as cusparseIndexBase_t,
                out_10.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCsrSetPointers<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    spMatDescr: cusparseSpMatDescr_t,
    mut csrRowOffsets: T,
    mut csrColInd: U,
    mut csrValues: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCsrSetPointers(
            spMatDescr,
            csrRowOffsets.as_mut_ptr() as *mut _,
            csrColInd.as_mut_ptr() as *mut _,
            csrValues.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCscSetPointers<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    spMatDescr: cusparseSpMatDescr_t,
    mut cscColOffsets: T,
    mut cscRowInd: U,
    mut cscValues: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCscSetPointers(
            spMatDescr,
            cscColOffsets.as_mut_ptr() as *mut _,
            cscRowInd.as_mut_ptr() as *mut _,
            cscValues.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateBsr(
    brows: i64,
    bcols: i64,
    bnnz: i64,
    rowBlockSize: i64,
    colBlockSize: i64,
    bsrRowOffsets: *mut ::std::os::raw::c_void,
    bsrColInd: *mut ::std::os::raw::c_void,
    bsrValues: *mut ::std::os::raw::c_void,
    bsrRowOffsetsType: cusparseIndexType_t,
    bsrColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
    order: cusparseOrder_t,
) -> Result<cusparseSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateBsr(
            out_0.as_mut_ptr() as *mut _,
            brows,
            bcols,
            bnnz,
            rowBlockSize,
            colBlockSize,
            bsrRowOffsets,
            bsrColInd,
            bsrValues,
            bsrRowOffsetsType,
            bsrColIndType,
            idxBase,
            valueType,
            order,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateConstBsr(
    brows: i64,
    bcols: i64,
    bnnz: i64,
    rowBlockDim: i64,
    colBlockDim: i64,
    bsrRowOffsets: *const ::std::os::raw::c_void,
    bsrColInd: *const ::std::os::raw::c_void,
    bsrValues: *const ::std::os::raw::c_void,
    bsrRowOffsetsType: cusparseIndexType_t,
    bsrColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
    order: cusparseOrder_t,
) -> Result<cusparseConstSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseConstSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateConstBsr(
            out_0.as_mut_ptr() as *mut _,
            brows,
            bcols,
            bnnz,
            rowBlockDim,
            colBlockDim,
            bsrRowOffsets,
            bsrColInd,
            bsrValues,
            bsrRowOffsetsType,
            bsrColIndType,
            idxBase,
            valueType,
            order,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseConstSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateCoo(
    rows: i64,
    cols: i64,
    nnz: i64,
    cooRowInd: *mut ::std::os::raw::c_void,
    cooColInd: *mut ::std::os::raw::c_void,
    cooValues: *mut ::std::os::raw::c_void,
    cooIdxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateCoo(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            nnz,
            cooRowInd,
            cooColInd,
            cooValues,
            cooIdxType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateConstCoo(
    rows: i64,
    cols: i64,
    nnz: i64,
    cooRowInd: *const ::std::os::raw::c_void,
    cooColInd: *const ::std::os::raw::c_void,
    cooValues: *const ::std::os::raw::c_void,
    cooIdxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseConstSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseConstSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateConstCoo(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            nnz,
            cooRowInd,
            cooColInd,
            cooValues,
            cooIdxType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseConstSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCooGet(
    spMatDescr: cusparseSpMatDescr_t,
    cooRowInd: *mut *mut ::std::os::raw::c_void,
    cooColInd: *mut *mut ::std::os::raw::c_void,
    cooValues: *mut *mut ::std::os::raw::c_void,
) -> Result<(i64, i64, i64, cusparseIndexType_t, cusparseIndexBase_t, cudaDataType), crate::sys::cusparseStatus_t> {
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_7.assume_init() as cusparseIndexType_t,
                out_8.assume_init() as cusparseIndexBase_t,
                out_9.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseConstCooGet(
    spMatDescr: cusparseConstSpMatDescr_t,
    cooRowInd: *mut *const ::std::os::raw::c_void,
    cooColInd: *mut *const ::std::os::raw::c_void,
    cooValues: *mut *const ::std::os::raw::c_void,
) -> Result<(i64, i64, i64, cusparseIndexType_t, cusparseIndexBase_t, cudaDataType), crate::sys::cusparseStatus_t> {
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_7.assume_init() as cusparseIndexType_t,
                out_8.assume_init() as cusparseIndexBase_t,
                out_9.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCooSetPointers<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    spMatDescr: cusparseSpMatDescr_t,
    mut cooRows: T,
    mut cooColumns: U,
    mut cooValues: V,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseCooSetPointers(
            spMatDescr,
            cooRows.as_mut_ptr() as *mut _,
            cooColumns.as_mut_ptr() as *mut _,
            cooValues.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseCreateBlockedEll(
    rows: i64,
    cols: i64,
    ellBlockSize: i64,
    ellCols: i64,
    ellColInd: *mut ::std::os::raw::c_void,
    ellValue: *mut ::std::os::raw::c_void,
    ellIdxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateBlockedEll(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            ellBlockSize,
            ellCols,
            ellColInd,
            ellValue,
            ellIdxType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateConstBlockedEll(
    rows: i64,
    cols: i64,
    ellBlockSize: i64,
    ellCols: i64,
    ellColInd: *const ::std::os::raw::c_void,
    ellValue: *const ::std::os::raw::c_void,
    ellIdxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseConstSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseConstSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateConstBlockedEll(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            ellBlockSize,
            ellCols,
            ellColInd,
            ellValue,
            ellIdxType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseConstSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_4.assume_init() as i64,
                out_7.assume_init() as cusparseIndexType_t,
                out_8.assume_init() as cusparseIndexBase_t,
                out_9.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_4.assume_init() as i64,
                out_7.assume_init() as cusparseIndexType_t,
                out_8.assume_init() as cusparseIndexBase_t,
                out_9.assume_init() as cudaDataType,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateSlicedEll(
    rows: i64,
    cols: i64,
    nnz: i64,
    sellValuesSize: i64,
    sliceSize: i64,
    sellSliceOffsets: *mut ::std::os::raw::c_void,
    sellColInd: *mut ::std::os::raw::c_void,
    sellValues: *mut ::std::os::raw::c_void,
    sellSliceOffsetsType: cusparseIndexType_t,
    sellColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateSlicedEll(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            nnz,
            sellValuesSize,
            sliceSize,
            sellSliceOffsets,
            sellColInd,
            sellValues,
            sellSliceOffsetsType,
            sellColIndType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateConstSlicedEll(
    rows: i64,
    cols: i64,
    nnz: i64,
    sellValuesSize: i64,
    sliceSize: i64,
    sellSliceOffsets: *const ::std::os::raw::c_void,
    sellColInd: *const ::std::os::raw::c_void,
    sellValues: *const ::std::os::raw::c_void,
    sellSliceOffsetsType: cusparseIndexType_t,
    sellColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> Result<cusparseConstSpMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseConstSpMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateConstSlicedEll(
            out_0.as_mut_ptr() as *mut _,
            rows,
            cols,
            nnz,
            sellValuesSize,
            sliceSize,
            sellSliceOffsets,
            sellColInd,
            sellValues,
            sellSliceOffsetsType,
            sellColIndType,
            idxBase,
            valueType,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseConstSpMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateDnMat(
    rows: i64,
    cols: i64,
    ld: i64,
    values: *mut ::std::os::raw::c_void,
    valueType: cudaDataType,
    order: cusparseOrder_t,
) -> Result<cusparseDnMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseDnMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateDnMat(out_0.as_mut_ptr() as *mut _, rows, cols, ld, values, valueType, order)
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseDnMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseCreateConstDnMat(
    rows: i64,
    cols: i64,
    ld: i64,
    values: *const ::std::os::raw::c_void,
    valueType: cudaDataType,
    order: cusparseOrder_t,
) -> Result<cusparseConstDnMatDescr_t, crate::sys::cusparseStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusparseConstDnMatDescr_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseCreateConstDnMat(out_0.as_mut_ptr() as *mut _, rows, cols, ld, values, valueType, order)
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusparseConstDnMatDescr_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDestroyDnMat(dnMatDescr: cusparseConstDnMatDescr_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDestroyDnMat(dnMatDescr) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_5.assume_init() as cudaDataType,
                out_6.assume_init() as cusparseOrder_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i64,
                out_2.assume_init() as i64,
                out_3.assume_init() as i64,
                out_5.assume_init() as cudaDataType,
                out_6.assume_init() as cusparseOrder_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseDnMatGetValues<T: types::CudaAsPtr>(
    dnMatDescr: cusparseDnMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDnMatGetValues(dnMatDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseConstDnMatGetValues<T: types::CudaAsPtr>(
    dnMatDescr: cusparseConstDnMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseConstDnMatGetValues(dnMatDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnMatSetValues<T: types::CudaAsPtr>(
    dnMatDescr: cusparseDnMatDescr_t,
    mut values: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDnMatSetValues(dnMatDescr, values.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnMatSetStridedBatch(
    dnMatDescr: cusparseDnMatDescr_t,
    batchCount: i32,
    batchStride: i64,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseDnMatSetStridedBatch(dnMatDescr, batchCount as _, batchStride) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDnMatGetStridedBatch(
    dnMatDescr: cusparseConstDnMatDescr_t,
) -> Result<(i32, i64), crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseDnMatGetStridedBatch(dnMatDescr, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _)
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as i32, out_2.assume_init() as i64)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseAxpby<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    alpha: T,
    vecX: cusparseConstSpVecDescr_t,
    beta: U,
    vecY: cusparseDnVecDescr_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseAxpby(
            handle,
            alpha.as_const_ptr() as *const _,
            vecX,
            beta.as_const_ptr() as *const _,
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
    handle: cusparseHandle_t,
    vecY: cusparseConstDnVecDescr_t,
    vecX: cusparseSpVecDescr_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseGather(handle, vecY, vecX) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseScatter(
    handle: cusparseHandle_t,
    vecX: cusparseConstSpVecDescr_t,
    vecY: cusparseDnVecDescr_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseScatter(handle, vecX, vecY) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseRot<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    c_coeff: T,
    s_coeff: U,
    vecX: cusparseSpVecDescr_t,
    vecY: cusparseDnVecDescr_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseRot(
            handle,
            c_coeff.as_const_ptr() as *const _,
            s_coeff.as_const_ptr() as *const _,
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
pub unsafe fn cusparseSpVV_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    opX: cusparseOperation_t,
    vecX: cusparseConstSpVecDescr_t,
    vecY: cusparseConstDnVecDescr_t,
    result: T,
    computeType: cudaDataType,
    mut bufferSize: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpVV_bufferSize(
            handle,
            opX,
            vecX,
            vecY,
            result.as_const_ptr() as *const _,
            computeType,
            bufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpVV<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    opX: cusparseOperation_t,
    vecX: cusparseConstSpVecDescr_t,
    vecY: cusparseConstDnVecDescr_t,
    mut result: T,
    computeType: cudaDataType,
    mut externalBuffer: U,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpVV(
            handle,
            opX,
            vecX,
            vecY,
            result.as_mut_ptr() as *mut _,
            computeType,
            externalBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSparseToDense_bufferSize<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseDnMatDescr_t,
    alg: cusparseSparseToDenseAlg_t,
    mut bufferSize: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSparseToDense_bufferSize(handle, matA, matB, alg, bufferSize.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSparseToDense<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseDnMatDescr_t,
    alg: cusparseSparseToDenseAlg_t,
    mut externalBuffer: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status =
        unsafe { crate::sys::cusparseSparseToDense(handle, matA, matB, alg, externalBuffer.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDenseToSparse_bufferSize<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    matA: cusparseConstDnMatDescr_t,
    matB: cusparseSpMatDescr_t,
    alg: cusparseDenseToSparseAlg_t,
    mut bufferSize: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDenseToSparse_bufferSize(handle, matA, matB, alg, bufferSize.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDenseToSparse_analysis<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    matA: cusparseConstDnMatDescr_t,
    matB: cusparseSpMatDescr_t,
    alg: cusparseDenseToSparseAlg_t,
    mut externalBuffer: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDenseToSparse_analysis(handle, matA, matB, alg, externalBuffer.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseDenseToSparse_convert<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    matA: cusparseConstDnMatDescr_t,
    matB: cusparseSpMatDescr_t,
    alg: cusparseDenseToSparseAlg_t,
    mut externalBuffer: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseDenseToSparse_convert(handle, matA, matB, alg, externalBuffer.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMV<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            alpha.as_const_ptr() as *const _,
            matA,
            vecX,
            beta.as_const_ptr() as *const _,
            vecY,
            computeType,
            alg,
            externalBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMV_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            alpha.as_const_ptr() as *const _,
            matA,
            vecX,
            beta.as_const_ptr() as *const _,
            vecY,
            computeType,
            alg,
            bufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMV_preprocess<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            alpha.as_const_ptr() as *const _,
            matA,
            vecX,
            beta.as_const_ptr() as *const _,
            vecY,
            computeType,
            alg,
            externalBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSV_createDescr<T: types::CudaAsPtr>(mut descr: T) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpSV_createDescr(descr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSV_destroyDescr(descr: cusparseSpSVDescr_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpSV_destroyDescr(descr) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSV_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            alpha.as_const_ptr() as *const _,
            matA,
            vecX,
            vecY,
            computeType,
            alg,
            spsvDescr,
            bufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSV_analysis<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            alpha.as_const_ptr() as *const _,
            matA,
            vecX,
            vecY,
            computeType,
            alg,
            spsvDescr,
            externalBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSV_solve<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            alpha.as_const_ptr() as *const _,
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
pub unsafe fn cusparseSpSV_updateMatrix<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    spsvDescr: cusparseSpSVDescr_t,
    mut newValues: T,
    updatePart: cusparseSpSVUpdate_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpSV_updateMatrix(handle, spsvDescr, newValues.as_mut_ptr() as *mut _, updatePart)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSM_createDescr<T: types::CudaAsPtr>(mut descr: T) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpSM_createDescr(descr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSM_destroyDescr(descr: cusparseSpSMDescr_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpSM_destroyDescr(descr) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSM_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            matC,
            computeType,
            alg,
            spsmDescr,
            bufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSM_analysis<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            matC,
            computeType,
            alg,
            spsmDescr,
            externalBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpSM_solve<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
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
pub unsafe fn cusparseSpSM_updateMatrix<T: types::CudaAsPtr>(
    handle: cusparseHandle_t,
    spsmDescr: cusparseSpSMDescr_t,
    mut newValues: T,
    updatePart: cusparseSpSMUpdate_t,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe {
        crate::sys::cusparseSpSM_updateMatrix(handle, spsmDescr, newValues.as_mut_ptr() as *mut _, updatePart)
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMM_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
            matC,
            computeType,
            alg,
            bufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMM_preprocess<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
            matC,
            computeType,
            alg,
            externalBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMM<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
            matC,
            computeType,
            alg,
            externalBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_createDescr<T: types::CudaAsPtr>(
    mut descr: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpGEMM_createDescr(descr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_destroyDescr(descr: cusparseSpGEMMDescr_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpGEMM_destroyDescr(descr) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_workEstimation<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
            matC,
            computeType,
            alg,
            spgemmDescr,
            bufferSize1.as_mut_ptr() as *mut _,
            externalBuffer1.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_getNumProducts<T: types::CudaAsPtr>(
    spgemmDescr: cusparseSpGEMMDescr_t,
    mut num_prods: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpGEMM_getNumProducts(spgemmDescr, num_prods.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_estimateMemory<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
            matC,
            computeType,
            alg,
            spgemmDescr,
            chunk_fraction,
            bufferSize3.as_mut_ptr() as *mut _,
            externalBuffer3.as_mut_ptr() as *mut _,
            bufferSize2.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_compute<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
            matC,
            computeType,
            alg,
            spgemmDescr,
            bufferSize2.as_mut_ptr() as *mut _,
            externalBuffer2.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMM_copy<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
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
pub unsafe fn cusparseSpGEMMreuse_workEstimation<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            matA,
            matB,
            matC,
            alg,
            spgemmDescr,
            bufferSize1.as_mut_ptr() as *mut _,
            externalBuffer1.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMMreuse_nnz<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            matA,
            matB,
            matC,
            alg,
            spgemmDescr,
            bufferSize2.as_mut_ptr() as *mut _,
            externalBuffer2.as_mut_ptr() as *mut _,
            bufferSize3.as_mut_ptr() as *mut _,
            externalBuffer3.as_mut_ptr() as *mut _,
            bufferSize4.as_mut_ptr() as *mut _,
            externalBuffer4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMMreuse_copy<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            matA,
            matB,
            matC,
            alg,
            spgemmDescr,
            bufferSize5.as_mut_ptr() as *mut _,
            externalBuffer5.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpGEMMreuse_compute<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
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
pub unsafe fn cusparseSDDMM_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
            matC,
            computeType,
            alg,
            bufferSize.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSDDMM_preprocess<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
            matC,
            computeType,
            alg,
            externalBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSDDMM<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusparseHandle_t,
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
            handle,
            opA,
            opB,
            alpha.as_const_ptr() as *const _,
            matA,
            matB,
            beta.as_const_ptr() as *const _,
            matC,
            computeType,
            alg,
            externalBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMMOp_createPlan(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    matC: cusparseDnMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpMMOpAlg_t,
    addOperationLtoirBuffer: *const ::std::os::raw::c_void,
    addOperationBufferSize: usize,
    mulOperationLtoirBuffer: *const ::std::os::raw::c_void,
    mulOperationBufferSize: usize,
    epilogueLtoirBuffer: *const ::std::os::raw::c_void,
    epilogueBufferSize: usize,
) -> Result<(cusparseSpMMOpPlan_t, usize), crate::sys::cusparseStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusparseSpMMOpPlan_t> = std::mem::MaybeUninit::uninit();
    let mut out_15: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusparseSpMMOp_createPlan(
            handle,
            out_1.as_mut_ptr() as *mut _,
            opA,
            opB,
            matA,
            matB,
            matC,
            computeType,
            alg,
            addOperationLtoirBuffer,
            addOperationBufferSize,
            mulOperationLtoirBuffer,
            mulOperationBufferSize,
            epilogueLtoirBuffer,
            epilogueBufferSize,
            out_15.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cusparseSpMMOpPlan_t,
                out_15.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusparseSpMMOp<T: types::CudaAsPtr>(
    plan: cusparseSpMMOpPlan_t,
    mut externalBuffer: T,
) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpMMOp(plan, externalBuffer.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusparseSpMMOp_destroyPlan(plan: cusparseSpMMOpPlan_t) -> Result<(), crate::sys::cusparseStatus_t> {
    let status = unsafe { crate::sys::cusparseSpMMOp_destroyPlan(plan) };
    if status == crate::sys::cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
