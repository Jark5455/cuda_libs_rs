pub use crate::sys::cublasStatus_t as CudaTargetStatus;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart::sys::*;
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
impl crate::sys::cublasLtMatrixLayoutOpaque_t {
    pub fn data(mut self, val: [u64; 14usize]) -> Self {
        self.data = val;
        self
    }
}
impl crate::sys::cublasLtMatmulAlgo_t {
    pub fn data(mut self, val: [u64; 8usize]) -> Self {
        self.data = val;
        self
    }
}
impl crate::sys::cublasLtMatmulDescOpaque_t {
    pub fn data(mut self, val: [u64; 32usize]) -> Self {
        self.data = val;
        self
    }
}
impl crate::sys::cublasLtMatrixTransformDescOpaque_t {
    pub fn data(mut self, val: [u64; 8usize]) -> Self {
        self.data = val;
        self
    }
}
impl crate::sys::cublasLtMatmulPreferenceOpaque_t {
    pub fn data(mut self, val: [u64; 12usize]) -> Self {
        self.data = val;
        self
    }
}
impl crate::sys::cublasLtEmulationDescOpaque_t {
    pub fn data(mut self, val: [u64; 8usize]) -> Self {
        self.data = val;
        self
    }
}
impl crate::sys::cublasLtMatmulHeuristicResult_t {
    pub fn algo(mut self, val: cublasLtMatmulAlgo_t) -> Self {
        self.algo = val;
        self
    }
    pub fn workspaceSize(mut self, val: usize) -> Self {
        self.workspaceSize = val;
        self
    }
    pub fn state(mut self, val: cublasStatus_t) -> Self {
        self.state = val;
        self
    }
    pub fn wavesCount(mut self, val: f32) -> Self {
        self.wavesCount = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 4usize]) -> Self {
        self.reserved = val;
        self
    }
}
pub struct CublasLtHandle {
    pub(crate) handle: crate::sys::cublasLtHandle_t,
}
impl CublasLtHandle {
    #[doc = " Execute matrix multiplication (D = alpha * op(A) * op(B) + beta * C).\n\n \\retval     CUBLAS_STATUS_NOT_INITIALIZED   if cuBLASLt handle has not been initialized\n \\retval     CUBLAS_STATUS_INVALID_VALUE     if parameters are in conflict or in an impossible configuration; e.g.\n                                             when workspaceSizeInBytes is less than workspace required by configured\n                                             algo\n \\retval     CUBLAS_STATUS_NOT_SUPPORTED     if current implementation on selected device doesn't support configured\n                                             operation\n \\retval     CUBLAS_STATUS_ARCH_MISMATCH     if configured operation cannot be run using selected device\n \\retval     CUBLAS_STATUS_EXECUTION_FAILED  if cuda reported execution error from the device\n \\retval     CUBLAS_STATUS_SUCCESS           if the operation completed successfully"]
    pub unsafe fn cublasLtMatmul<
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
        computeDesc: cublasLtMatmulDesc_t,
        alpha: T,
        A: U,
        Adesc: cublasLtMatrixLayout_t,
        B: V,
        Bdesc: cublasLtMatrixLayout_t,
        beta: W,
        C: X,
        Cdesc: cublasLtMatrixLayout_t,
        mut D: Y,
        Ddesc: cublasLtMatrixLayout_t,
        algo: Z,
        mut workspace: A,
        workspaceSizeInBytes: usize,
        stream: cudaStream_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasLtMatmul(
                self.handle,
                computeDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Adesc,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Bdesc,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                C.as_const_ptr() as *const ::std::os::raw::c_void,
                Cdesc,
                D.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Ddesc,
                algo.as_const_ptr() as *const cublasLtMatmulAlgo_t,
                workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceSizeInBytes,
                stream,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    #[doc = " Matrix layout conversion helper (C = alpha * op(A) + beta * op(B))\n\n Can be used to change memory order of data or to scale and shift the values.\n\n \\retval     CUBLAS_STATUS_NOT_INITIALIZED   if cuBLASLt handle has not been initialized\n \\retval     CUBLAS_STATUS_INVALID_VALUE     if parameters are in conflict or in an impossible configuration; e.g.\n                                             when A is not NULL, but Adesc is NULL\n \\retval     CUBLAS_STATUS_NOT_SUPPORTED     if current implementation on selected device doesn't support configured\n                                             operation\n \\retval     CUBLAS_STATUS_ARCH_MISMATCH     if configured operation cannot be run using selected device\n \\retval     CUBLAS_STATUS_EXECUTION_FAILED  if cuda reported execution error from the device\n \\retval     CUBLAS_STATUS_SUCCESS           if the operation completed successfully"]
    pub unsafe fn cublasLtMatrixTransform<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transformDesc: cublasLtMatrixTransformDesc_t,
        alpha: T,
        A: U,
        Adesc: cublasLtMatrixLayout_t,
        beta: V,
        B: W,
        Bdesc: cublasLtMatrixLayout_t,
        mut C: X,
        Cdesc: cublasLtMatrixLayout_t,
        stream: cudaStream_t,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasLtMatrixTransform(
                self.handle,
                transformDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                Adesc,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                Bdesc,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
                Cdesc,
                stream,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    #[doc = " Query cublasLt heuristic for algorithm appropriate for given use case.\n\n \\param[in]      lightHandle            Pointer to the allocated cuBLASLt handle for the cuBLASLt\n                                        context. See cublasLtHandle_t.\n \\param[in]      operationDesc          Handle to the matrix multiplication descriptor.\n \\param[in]      Adesc                  Handle to the layout descriptors for matrix A.\n \\param[in]      Bdesc                  Handle to the layout descriptors for matrix B.\n \\param[in]      Cdesc                  Handle to the layout descriptors for matrix C.\n \\param[in]      Ddesc                  Handle to the layout descriptors for matrix D.\n \\param[in]      preference             Pointer to the structure holding the heuristic search\n                                        preferences descriptor. See cublasLtMatrixLayout_t.\n \\param[in]      requestedAlgoCount     Size of heuristicResultsArray (in elements) and requested\n                                        maximum number of algorithms to return.\n \\param[in, out] heuristicResultsArray  Output algorithms and associated runtime characteristics,\n                                        ordered in increasing estimated compute time.\n \\param[out]     returnAlgoCount        The number of heuristicResultsArray elements written.\n\n \\retval  CUBLAS_STATUS_INVALID_VALUE   if requestedAlgoCount is less or equal to zero\n \\retval  CUBLAS_STATUS_NOT_SUPPORTED   if no heuristic function available for current configuration\n \\retval  CUBLAS_STATUS_SUCCESS         if query was successful, inspect\n                                        heuristicResultsArray[0 to (returnAlgoCount - 1)].state\n                                        for detail status of results"]
    pub unsafe fn cublasLtMatmulAlgoGetHeuristic(
        &self,
        operationDesc: cublasLtMatmulDesc_t,
        Adesc: cublasLtMatrixLayout_t,
        Bdesc: cublasLtMatrixLayout_t,
        Cdesc: cublasLtMatrixLayout_t,
        Ddesc: cublasLtMatrixLayout_t,
        preference: cublasLtMatmulPreference_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        heuristicResultsArray: *mut cublasLtMatmulHeuristicResult_t,
    ) -> Result<::std::os::raw::c_int, crate::sys::cublasStatus_t> {
        let mut out_9: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasLtMatmulAlgoGetHeuristic(
                self.handle,
                operationDesc,
                Adesc,
                Bdesc,
                Cdesc,
                Ddesc,
                preference,
                requestedAlgoCount,
                heuristicResultsArray,
                out_9.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_9.assume_init()) }
        } else {
            Err(status)
        }
    }
    #[doc = " Routine to get all algo IDs that can potentially run\n\n \\param[in]  int              requestedAlgoCount requested number of algos (must be less or equal to size of algoIdsA\n (in elements)) \\param[out] algoIdsA         array to write algoIds to \\param[out] returnAlgoCount  number of algoIds\n actually written\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if requestedAlgoCount is less or equal to zero\n \\retval     CUBLAS_STATUS_SUCCESS        if query was successful, inspect returnAlgoCount to get actual number of IDs\n                                          available"]
    pub unsafe fn cublasLtMatmulAlgoGetIds(
        &self,
        computeType: cublasComputeType_t,
        scaleType: cudaDataType_t,
        Atype: cudaDataType_t,
        Btype: cudaDataType_t,
        Ctype: cudaDataType_t,
        Dtype: cudaDataType_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        algoIdsArray: *mut ::std::os::raw::c_int,
    ) -> Result<::std::os::raw::c_int, crate::sys::cublasStatus_t> {
        let mut out_9: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cublasLtMatmulAlgoGetIds(
                self.handle,
                computeType,
                scaleType,
                Atype,
                Btype,
                Ctype,
                Dtype,
                requestedAlgoCount,
                algoIdsArray,
                out_9.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            unsafe { Ok(out_9.assume_init()) }
        } else {
            Err(status)
        }
    }
    #[doc = " Initialize algo structure\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if algo is NULL or algoId is outside of recognized range\n \\retval     CUBLAS_STATUS_NOT_SUPPORTED  if algoId is not supported for given combination of data types\n \\retval     CUBLAS_STATUS_SUCCESS        if the structure was successfully initialized"]
    pub unsafe fn cublasLtMatmulAlgoInit<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        computeType: cublasComputeType_t,
        scaleType: cudaDataType_t,
        Atype: cudaDataType_t,
        Btype: cudaDataType_t,
        Ctype: cudaDataType_t,
        Dtype: cudaDataType_t,
        algoId: ::std::os::raw::c_int,
        mut algo: T,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasLtMatmulAlgoInit(
                self.handle,
                computeType,
                scaleType,
                Atype,
                Btype,
                Ctype,
                Dtype,
                algoId,
                algo.as_mut_ptr() as *mut cublasLtMatmulAlgo_t,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    #[doc = " Check configured algo descriptor for correctness and support on current device.\n\n Result includes required workspace size and calculated wave count.\n\n CUBLAS_STATUS_SUCCESS doesn't fully guarantee algo will run (will fail if e.g. buffers are not correctly aligned);\n but if cublasLtMatmulAlgoCheck fails, the algo will not run.\n\n \\param[in]  algo    algo configuration to check\n \\param[out] result  result structure to report algo runtime characteristics; algo field is never updated\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if matrix layout descriptors or operation descriptor don't match algo\n                                          descriptor\n \\retval     CUBLAS_STATUS_NOT_SUPPORTED  if algo configuration or data type combination is not currently supported on\n                                          given device\n \\retval     CUBLAS_STATUS_ARCH_MISMATCH  if algo configuration cannot be run using the selected device\n \\retval     CUBLAS_STATUS_SUCCESS        if check was successful"]
    pub unsafe fn cublasLtMatmulAlgoCheck<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        operationDesc: cublasLtMatmulDesc_t,
        Adesc: cublasLtMatrixLayout_t,
        Bdesc: cublasLtMatrixLayout_t,
        Cdesc: cublasLtMatrixLayout_t,
        Ddesc: cublasLtMatrixLayout_t,
        algo: T,
        mut result: U,
    ) -> Result<(), crate::sys::cublasStatus_t> {
        let status = unsafe {
            crate::sys::cublasLtMatmulAlgoCheck(
                self.handle,
                operationDesc,
                Adesc,
                Bdesc,
                Cdesc,
                Ddesc,
                algo.as_const_ptr() as *const cublasLtMatmulAlgo_t,
                result.as_mut_ptr() as *mut cublasLtMatmulHeuristicResult_t,
            )
        };
        if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
pub unsafe fn cublasLtGetStatusName(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cublasLtGetStatusName(status) }
}
pub unsafe fn cublasLtGetStatusString(status: cublasStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cublasLtGetStatusString(status) }
}
pub unsafe fn cublasLtGetVersion() -> usize {
    unsafe { crate::sys::cublasLtGetVersion() }
}
pub unsafe fn cublasLtGetCudartVersion() -> usize {
    unsafe { crate::sys::cublasLtGetCudartVersion() }
}
pub unsafe fn cublasLtGetProperty(
    type_: libraryPropertyType,
) -> Result<::std::os::raw::c_int, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cublasLtGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cublasLtHeuristicsCacheGetCapacity() -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cublasLtHeuristicsCacheGetCapacity(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cublasLtHeuristicsCacheSetCapacity(
    capacity: usize,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtHeuristicsCacheSetCapacity(capacity) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Restricts usage of CPU instructions (ISA) specified by the flags in the mask.\n\n Flags can be combined with bitwise OR(|) operator. Supported flags:\n - 0x1 -- x86-64 AVX512 ISA\n\n Default mask: 0 (any applicable ISA is allowed).\n\n The function returns the previous value of the mask.\n The function takes precedence over the environment variable CUBLASLT_DISABLE_CPU_INSTRUCTIONS_MASK."]
pub unsafe fn cublasLtDisableCpuInstructionsSetMask(
    mask: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_uint {
    unsafe { crate::sys::cublasLtDisableCpuInstructionsSetMask(mask) }
}
#[doc = " Internal. Do not use directly."]
pub unsafe fn cublasLtMatrixLayoutInit_internal(
    matLayout: cublasLtMatrixLayout_t,
    size: usize,
    type_: cudaDataType,
    rows: u64,
    cols: u64,
    ld: i64,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatrixLayoutInit_internal(matLayout, size, type_, rows, cols, ld)
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Experimental: Internal. Do not use directly."]
pub unsafe fn cublasLtGroupedMatrixLayoutInit_internal<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    matLayout: cublasLtMatrixLayout_t,
    size: usize,
    type_: cudaDataType,
    groupCount: ::std::os::raw::c_int,
    rows_array: T,
    cols_array: U,
    ld_array: V,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtGroupedMatrixLayoutInit_internal(
            matLayout,
            size,
            type_,
            groupCount,
            rows_array.as_const_ptr() as *const ::std::os::raw::c_void,
            cols_array.as_const_ptr() as *const ::std::os::raw::c_void,
            ld_array.as_const_ptr() as *const ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Set matrix layout descriptor attribute.\n\n \\param[in]  matLayout    The descriptor\n \\param[in]  attr         The attribute\n \\param[in]  buf          memory address containing the new value\n \\param[in]  sizeInBytes  size of buf buffer for verification (in bytes)\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatrixLayoutSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    matLayout: cublasLtMatrixLayout_t,
    attr: cublasLtMatrixLayoutAttribute_t,
    buf: T,
    sizeInBytes: usize,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatrixLayoutSetAttribute(
            matLayout,
            attr,
            buf.as_const_ptr() as *const ::std::os::raw::c_void,
            sizeInBytes,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Get matrix layout descriptor attribute.\n\n \\param[in]  matLayout    The descriptor\n \\param[in]  attr         The attribute\n \\param[out] buf          memory address containing the new value\n \\param[in]  sizeInBytes  size of buf buffer for verification (in bytes)\n \\param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\n                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\n                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatrixLayoutGetAttribute<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    matLayout: cublasLtMatrixLayout_t,
    attr: cublasLtMatrixLayoutAttribute_t,
    mut buf: T,
    sizeInBytes: usize,
    mut sizeWritten: U,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatrixLayoutGetAttribute(
            matLayout,
            attr,
            buf.as_mut_ptr() as *mut ::std::os::raw::c_void,
            sizeInBytes,
            sizeWritten.as_mut_ptr() as *mut usize,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Internal. Do not use directly."]
pub unsafe fn cublasLtMatmulDescInit_internal(
    matmulDesc: cublasLtMatmulDesc_t,
    size: usize,
    computeType: cublasComputeType_t,
    scaleType: cudaDataType_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatmulDescInit_internal(matmulDesc, size, computeType, scaleType)
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Set matmul operation descriptor attribute.\n\n \\param[in]  matmulDesc   The descriptor\n \\param[in]  attr         The attribute\n \\param[in]  buf          memory address containing the new value\n \\param[in]  sizeInBytes  size of buf buffer for verification (in bytes)\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatmulDescSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    matmulDesc: cublasLtMatmulDesc_t,
    attr: cublasLtMatmulDescAttributes_t,
    buf: T,
    sizeInBytes: usize,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatmulDescSetAttribute(
            matmulDesc,
            attr,
            buf.as_const_ptr() as *const ::std::os::raw::c_void,
            sizeInBytes,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Get matmul operation descriptor attribute.\n\n \\param[in]  matmulDesc   The descriptor\n \\param[in]  attr         The attribute\n \\param[out] buf          memory address containing the new value\n \\param[in]  sizeInBytes  size of buf buffer for verification (in bytes)\n \\param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\n                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\n                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatmulDescGetAttribute(
    matmulDesc: cublasLtMatmulDesc_t,
    attr: cublasLtMatmulDescAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cublasLtMatmulDescGetAttribute(
            matmulDesc,
            attr,
            buf,
            sizeInBytes,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        unsafe { Ok(out_4.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " Internal. Do not use directly."]
pub unsafe fn cublasLtMatrixTransformDescInit_internal(
    transformDesc: cublasLtMatrixTransformDesc_t,
    size: usize,
    scaleType: cudaDataType,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatrixTransformDescInit_internal(transformDesc, size, scaleType)
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Set matrix transform operation descriptor attribute.\n\n \\param[in]  transformDesc  The descriptor\n \\param[in]  attr           The attribute\n \\param[in]  buf            memory address containing the new value\n \\param[in]  sizeInBytes    size of buf buffer for verification (in bytes)\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatrixTransformDescSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    transformDesc: cublasLtMatrixTransformDesc_t,
    attr: cublasLtMatrixTransformDescAttributes_t,
    buf: T,
    sizeInBytes: usize,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatrixTransformDescSetAttribute(
            transformDesc,
            attr,
            buf.as_const_ptr() as *const ::std::os::raw::c_void,
            sizeInBytes,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Get matrix transform operation descriptor attribute.\n\n \\param[in]  transformDesc  The descriptor\n \\param[in]  attr           The attribute\n \\param[out] buf            memory address containing the new value\n \\param[in]  sizeInBytes    size of buf buffer for verification (in bytes)\n \\param[out] sizeWritten    only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number\n of bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\n                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatrixTransformDescGetAttribute<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    transformDesc: cublasLtMatrixTransformDesc_t,
    attr: cublasLtMatrixTransformDescAttributes_t,
    mut buf: T,
    sizeInBytes: usize,
    mut sizeWritten: U,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatrixTransformDescGetAttribute(
            transformDesc,
            attr,
            buf.as_mut_ptr() as *mut ::std::os::raw::c_void,
            sizeInBytes,
            sizeWritten.as_mut_ptr() as *mut usize,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Internal. Do not use directly."]
pub unsafe fn cublasLtEmulationDescInit_internal(
    emulationDesc: cublasLtEmulationDesc_t,
    size: usize,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtEmulationDescInit_internal(emulationDesc, size) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Set emulation descriptor attribute.\n\n \\param[in]  emulationDesc  The descriptor\n \\param[in]  attr           The attribute\n \\param[in]  buf            memory address containing the new value\n \\param[in]  sizeInBytes    size of buf buffer for verification (in bytes)\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtEmulationDescSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    emulationDesc: cublasLtEmulationDesc_t,
    attr: cublasLtEmulationDescAttributes_t,
    buf: T,
    sizeInBytes: usize,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtEmulationDescSetAttribute(
            emulationDesc,
            attr,
            buf.as_const_ptr() as *const ::std::os::raw::c_void,
            sizeInBytes,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Get emulation descriptor attribute.\n\n \\param[in]  emulationDesc  The descriptor\n \\param[in]  attr           The attribute\n \\param[out] buf            memory address containing the new value\n \\param[in]  sizeInBytes    size of buf buffer for verification (in bytes)\n \\param[out] sizeWritten    only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number\n of bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\n                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtEmulationDescGetAttribute(
    emulationDesc: cublasLtEmulationDesc_t,
    attr: cublasLtEmulationDescAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cublasLtEmulationDescGetAttribute(
            emulationDesc,
            attr,
            buf,
            sizeInBytes,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        unsafe { Ok(out_4.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " Internal. Do not use directly."]
pub unsafe fn cublasLtMatmulPreferenceInit_internal(
    pref: cublasLtMatmulPreference_t,
    size: usize,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulPreferenceInit_internal(pref, size) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Set matmul heuristic search preference descriptor attribute.\n\n \\param[in]  pref         The descriptor\n \\param[in]  attr         The attribute\n \\param[in]  buf          memory address containing the new value\n \\param[in]  sizeInBytes  size of buf buffer for verification (in bytes)\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatmulPreferenceSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    pref: cublasLtMatmulPreference_t,
    attr: cublasLtMatmulPreferenceAttributes_t,
    buf: T,
    sizeInBytes: usize,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatmulPreferenceSetAttribute(
            pref,
            attr,
            buf.as_const_ptr() as *const ::std::os::raw::c_void,
            sizeInBytes,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Get matmul heuristic search preference descriptor attribute.\n\n \\param[in]  pref         The descriptor\n \\param[in]  attr         The attribute\n \\param[out] buf          memory address containing the new value\n \\param[in]  sizeInBytes  size of buf buffer for verification (in bytes)\n \\param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\n                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\n                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatmulPreferenceGetAttribute(
    pref: cublasLtMatmulPreference_t,
    attr: cublasLtMatmulPreferenceAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cublasLtMatmulPreferenceGetAttribute(
            pref,
            attr,
            buf,
            sizeInBytes,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        unsafe { Ok(out_4.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " Get algo capability attribute.\n\n E.g. to get list of supported Tile IDs:\n      cublasLtMatmulTile_t tiles[CUBLASLT_MATMUL_TILE_END];\n      size_t num_tiles, size_written;\n      if (cublasLtMatmulAlgoCapGetAttribute(algo, CUBLASLT_ALGO_CAP_TILE_IDS, tiles, sizeof(tiles), size_written) ==\n CUBLAS_STATUS_SUCCESS) { num_tiles = size_written / sizeof(tiles[0]);\n      }\n\n \\param[in]  algo         The algo descriptor\n \\param[in]  attr         The attribute\n \\param[out] buf          memory address containing the new value\n \\param[in]  sizeInBytes  size of buf buffer for verification (in bytes)\n \\param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\n                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\n                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatmulAlgoCapGetAttribute(
    algo: *const cublasLtMatmulAlgo_t,
    attr: cublasLtMatmulAlgoCapAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cublasLtMatmulAlgoCapGetAttribute(
            algo,
            attr,
            buf,
            sizeInBytes,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        unsafe { Ok(out_4.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " Set algo configuration attribute.\n\n \\param[in]  algo         The algo descriptor\n \\param[in]  attr         The attribute\n \\param[in]  buf          memory address containing the new value\n \\param[in]  sizeInBytes  size of buf buffer for verification (in bytes)\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatmulAlgoConfigSetAttribute<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut algo: T,
    attr: cublasLtMatmulAlgoConfigAttributes_t,
    buf: U,
    sizeInBytes: usize,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatmulAlgoConfigSetAttribute(
            algo.as_mut_ptr() as *mut cublasLtMatmulAlgo_t,
            attr,
            buf.as_const_ptr() as *const ::std::os::raw::c_void,
            sizeInBytes,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Get algo configuration attribute.\n\n \\param[in]  algo         The algo descriptor\n \\param[in]  attr         The attribute\n \\param[out] buf          memory address containing the new value\n \\param[in]  sizeInBytes  size of buf buffer for verification (in bytes)\n \\param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\n                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\n                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for\n                                          selected attribute\n \\retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatmulAlgoConfigGetAttribute(
    algo: *const cublasLtMatmulAlgo_t,
    attr: cublasLtMatmulAlgoConfigAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cublasLtMatmulAlgoConfigGetAttribute(
            algo,
            attr,
            buf,
            sizeInBytes,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        unsafe { Ok(out_4.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " Experimental: Logger callback setter.\n\n \\param[in]  callback                     a user defined callback function to be called by the logger\n\n \\retval     CUBLAS_STATUS_SUCCESS        if callback was set successfully"]
pub unsafe fn cublasLtLoggerSetCallback(
    callback: cublasLtLoggerCallback_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerSetCallback(callback) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Experimental: Log file setter.\n\n \\param[in]  file                         an open file with write permissions\n\n \\retval     CUBLAS_STATUS_SUCCESS        if log file was set successfully"]
pub unsafe fn cublasLtLoggerSetFile<T: ::cuda_libs::types::CudaAsPtr>(
    mut file: T,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerSetFile(file.as_mut_ptr() as *mut FILE) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Experimental: Open log file.\n\n \\param[in]  logFile                      log file path. if the log file does not exist, it will be created\n\n \\retval     CUBLAS_STATUS_SUCCESS        if log file was created successfully"]
pub unsafe fn cublasLtLoggerOpenFile<T: ::cuda_libs::types::CudaAsPtr>(
    logFile: T,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtLoggerOpenFile(logFile.as_const_ptr() as *const ::std::os::raw::c_char)
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Experimental: Log level setter.\n\n \\param[in]  level                        log level, should be one of the following:\n                                          0. Off\n                                          1. Errors\n                                          2. Performance Trace\n                                          3. Performance Hints\n                                          4. Heuristics Trace\n                                          5. API Trace\n\n \\retval     CUBLAS_STATUS_INVALID_VALUE  if log level is not one of the above levels\n\n \\retval     CUBLAS_STATUS_SUCCESS        if log level was set successfully"]
pub unsafe fn cublasLtLoggerSetLevel(
    level: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerSetLevel(level) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Experimental: Log mask setter.\n\n \\param[in]  mask                         log mask, should be a combination of the following masks:\n                                          0.  Off\n                                          1.  Errors\n                                          2.  Performance Trace\n                                          4.  Performance Hints\n                                          8.  Heuristics Trace\n                                          16. API Trace\n\n \\retval     CUBLAS_STATUS_SUCCESS        if log mask was set successfully"]
pub unsafe fn cublasLtLoggerSetMask(
    mask: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerSetMask(mask) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Experimental: Disable logging for the entire session.\n\n \\retval     CUBLAS_STATUS_SUCCESS        if disabled logging"]
pub unsafe fn cublasLtLoggerForceDisable() -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerForceDisable() };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
impl CublasLtHandle {
    pub fn new() -> Result<Self, crate::sys::cublasStatus_t> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let status = crate::sys::cublasLtCreate(&mut handle);
            if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS {
                Ok(Self { handle })
            } else {
                Err(status)
            }
        }
    }
}
impl Drop for CublasLtHandle {
    fn drop(&mut self) {
        unsafe {
            crate::sys::cublasLtDestroy(self.handle);
        }
    }
}
