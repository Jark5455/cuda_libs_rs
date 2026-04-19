#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
pub use crate::sys::__BindgenBitfieldUnit;
pub use crate::sys::__off_t;
pub use crate::sys::__off64_t;
pub use crate::sys::__uint64_t;
pub use crate::sys::_IO_FILE;
pub use crate::sys::_IO_codecvt;
pub use crate::sys::_IO_lock_t;
pub use crate::sys::_IO_marker;
pub use crate::sys::_IO_wide_data;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_16F;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_32F;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_32I;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_64F;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_TYPE_MASK;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_DMMA;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_FMA;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_GAUSSIAN;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_HMMA;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_IMMA;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_8F_E4M3;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_8F_E5M2;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_8I;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_16BF;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_16F;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_32F;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_64F;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_TF32;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_OP_INPUT_TYPE_MASK;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_OP_TYPE_MASK;
pub use crate::sys::CUBLASLT_NUMERICAL_IMPL_FLAGS_TENSOR_OP_MASK;
pub use crate::sys::FILE;
pub use crate::sys::cublasComputeType_t;
pub use crate::sys::cublasLtBatchMode_t;
pub use crate::sys::cublasLtClusterShape_t;
pub use crate::sys::cublasLtContext;
pub use crate::sys::cublasLtEmulationDesc_t;
pub use crate::sys::cublasLtEmulationDescAttributes_t;
pub use crate::sys::cublasLtEmulationDescOpaque_t;
pub use crate::sys::cublasLtEpilogue_t;
pub use crate::sys::cublasLtHandle_t;
pub use crate::sys::cublasLtIntegerWidth_t;
pub use crate::sys::cublasLtLoggerCallback_t;
pub use crate::sys::cublasLtMatmulAlgo_t;
pub use crate::sys::cublasLtMatmulAlgoCapAttributes_t;
pub use crate::sys::cublasLtMatmulAlgoConfigAttributes_t;
pub use crate::sys::cublasLtMatmulDesc_t;
pub use crate::sys::cublasLtMatmulDescAttributes_t;
pub use crate::sys::cublasLtMatmulDescOpaque_t;
pub use crate::sys::cublasLtMatmulHeuristicResult_t;
pub use crate::sys::cublasLtMatmulInnerShape_t;
pub use crate::sys::cublasLtMatmulMatrixScale_t;
pub use crate::sys::cublasLtMatmulPreference_t;
pub use crate::sys::cublasLtMatmulPreferenceAttributes_t;
pub use crate::sys::cublasLtMatmulPreferenceOpaque_t;
pub use crate::sys::cublasLtMatmulSearch_t;
pub use crate::sys::cublasLtMatmulStages_t;
pub use crate::sys::cublasLtMatmulTile_t;
pub use crate::sys::cublasLtMatrixLayout_t;
pub use crate::sys::cublasLtMatrixLayoutAttribute_t;
pub use crate::sys::cublasLtMatrixLayoutOpaque_t;
pub use crate::sys::cublasLtMatrixTransformDesc_t;
pub use crate::sys::cublasLtMatrixTransformDescAttributes_t;
pub use crate::sys::cublasLtMatrixTransformDescOpaque_t;
pub use crate::sys::cublasLtNumericalImplFlags_t;
pub use crate::sys::cublasLtOrder_t;
pub use crate::sys::cublasLtPointerMode_t;
pub use crate::sys::cublasLtPointerModeMask_t;
pub use crate::sys::cublasLtReductionScheme_t;
pub use crate::sys::cublasStatus_t as CudaTargetStatus;
pub use crate::sys::cublasStatus_t;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart;
#[allow(unused_imports)]
use cuda_libs_cudart::sys::*;
#[allow(unused_imports)]
use cuda_libs_cudart::types;
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
impl crate::sys::cublasLtMatrixLayoutOpaque_t {
    pub fn data(mut self, val: [u64; 14usize]) -> Self {
        self.data = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cublasLtMatmulAlgo_t {
    pub fn data(mut self, val: [u64; 8usize]) -> Self {
        self.data = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cublasLtMatmulDescOpaque_t {
    pub fn data(mut self, val: [u64; 32usize]) -> Self {
        self.data = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cublasLtMatrixTransformDescOpaque_t {
    pub fn data(mut self, val: [u64; 8usize]) -> Self {
        self.data = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cublasLtMatmulPreferenceOpaque_t {
    pub fn data(mut self, val: [u64; 12usize]) -> Self {
        self.data = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cublasLtEmulationDescOpaque_t {
    pub fn data(mut self, val: [u64; 8usize]) -> Self {
        self.data = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
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
#[cfg(feature = "runtime-link")]
impl crate::sys::DynamicBindings {
    pub fn cublasLtCreate(mut self, val: Option<unsafe extern "C" fn(*mut cublasLtHandle_t) -> cublasStatus_t>) -> Self {
        self.cublasLtCreate = val;
        self
    }
    pub fn cublasLtDestroy(mut self, val: Option<unsafe extern "C" fn(cublasLtHandle_t) -> cublasStatus_t>) -> Self {
        self.cublasLtDestroy = val;
        self
    }
    pub fn cublasLtGetStatusName(mut self, val: Option<unsafe extern "C" fn(cublasStatus_t) -> *const ::std::os::raw::c_char>) -> Self {
        self.cublasLtGetStatusName = val;
        self
    }
    pub fn cublasLtGetStatusString(mut self, val: Option<unsafe extern "C" fn(cublasStatus_t) -> *const ::std::os::raw::c_char>) -> Self {
        self.cublasLtGetStatusString = val;
        self
    }
    pub fn cublasLtGetVersion(mut self, val: Option<unsafe extern "C" fn() -> usize>) -> Self {
        self.cublasLtGetVersion = val;
        self
    }
    pub fn cublasLtGetCudartVersion(mut self, val: Option<unsafe extern "C" fn() -> usize>) -> Self {
        self.cublasLtGetCudartVersion = val;
        self
    }
    pub fn cublasLtGetProperty(mut self, val: Option<unsafe extern "C" fn(libraryPropertyType, *mut ::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasLtGetProperty = val;
        self
    }
    pub fn cublasLtHeuristicsCacheGetCapacity(mut self, val: Option<unsafe extern "C" fn(*mut usize) -> cublasStatus_t>) -> Self {
        self.cublasLtHeuristicsCacheGetCapacity = val;
        self
    }
    pub fn cublasLtHeuristicsCacheSetCapacity(mut self, val: Option<unsafe extern "C" fn(usize) -> cublasStatus_t>) -> Self {
        self.cublasLtHeuristicsCacheSetCapacity = val;
        self
    }
    pub fn cublasLtDisableCpuInstructionsSetMask(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_uint) -> ::std::os::raw::c_uint>) -> Self {
        self.cublasLtDisableCpuInstructionsSetMask = val;
        self
    }
    pub fn cublasLtMatmul(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasLtHandle_t,
                cublasLtMatmulDesc_t,
                *const ::std::os::raw::c_void,
                *const ::std::os::raw::c_void,
                cublasLtMatrixLayout_t,
                *const ::std::os::raw::c_void,
                cublasLtMatrixLayout_t,
                *const ::std::os::raw::c_void,
                *const ::std::os::raw::c_void,
                cublasLtMatrixLayout_t,
                *mut ::std::os::raw::c_void,
                cublasLtMatrixLayout_t,
                *const cublasLtMatmulAlgo_t,
                *mut ::std::os::raw::c_void,
                usize,
                cudaStream_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasLtMatmul = val;
        self
    }
    pub fn cublasLtMatrixTransform(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasLtHandle_t,
                cublasLtMatrixTransformDesc_t,
                *const ::std::os::raw::c_void,
                *const ::std::os::raw::c_void,
                cublasLtMatrixLayout_t,
                *const ::std::os::raw::c_void,
                *const ::std::os::raw::c_void,
                cublasLtMatrixLayout_t,
                *mut ::std::os::raw::c_void,
                cublasLtMatrixLayout_t,
                cudaStream_t,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasLtMatrixTransform = val;
        self
    }
    pub fn cublasLtMatrixLayoutInit_internal(mut self, val: Option<unsafe extern "C" fn(cublasLtMatrixLayout_t, usize, cudaDataType, u64, u64, i64) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixLayoutInit_internal = val;
        self
    }
    pub fn cublasLtGroupedMatrixLayoutInit_internal(mut self, val: Option<unsafe extern "C" fn(cublasLtMatrixLayout_t, usize, cudaDataType, ::std::os::raw::c_int, *const ::std::os::raw::c_void, *const ::std::os::raw::c_void, *const ::std::os::raw::c_void) -> cublasStatus_t>) -> Self {
        self.cublasLtGroupedMatrixLayoutInit_internal = val;
        self
    }
    pub fn cublasLtMatrixLayoutCreate(mut self, val: Option<unsafe extern "C" fn(*mut cublasLtMatrixLayout_t, cudaDataType, u64, u64, i64) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixLayoutCreate = val;
        self
    }
    pub fn cublasLtGroupedMatrixLayoutCreate(mut self, val: Option<unsafe extern "C" fn(*mut cublasLtMatrixLayout_t, cudaDataType, ::std::os::raw::c_int, *const ::std::os::raw::c_void, *const ::std::os::raw::c_void, *const ::std::os::raw::c_void) -> cublasStatus_t>) -> Self {
        self.cublasLtGroupedMatrixLayoutCreate = val;
        self
    }
    pub fn cublasLtMatrixLayoutDestroy(mut self, val: Option<unsafe extern "C" fn(cublasLtMatrixLayout_t) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixLayoutDestroy = val;
        self
    }
    pub fn cublasLtMatrixLayoutSetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtMatrixLayout_t, cublasLtMatrixLayoutAttribute_t, *const ::std::os::raw::c_void, usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixLayoutSetAttribute = val;
        self
    }
    pub fn cublasLtMatrixLayoutGetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtMatrixLayout_t, cublasLtMatrixLayoutAttribute_t, *mut ::std::os::raw::c_void, usize, *mut usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixLayoutGetAttribute = val;
        self
    }
    pub fn cublasLtMatmulDescInit_internal(mut self, val: Option<unsafe extern "C" fn(cublasLtMatmulDesc_t, usize, cublasComputeType_t, cudaDataType_t) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulDescInit_internal = val;
        self
    }
    pub fn cublasLtMatmulDescCreate(mut self, val: Option<unsafe extern "C" fn(*mut cublasLtMatmulDesc_t, cublasComputeType_t, cudaDataType_t) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulDescCreate = val;
        self
    }
    pub fn cublasLtMatmulDescDestroy(mut self, val: Option<unsafe extern "C" fn(cublasLtMatmulDesc_t) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulDescDestroy = val;
        self
    }
    pub fn cublasLtMatmulDescSetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtMatmulDesc_t, cublasLtMatmulDescAttributes_t, *const ::std::os::raw::c_void, usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulDescSetAttribute = val;
        self
    }
    pub fn cublasLtMatmulDescGetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtMatmulDesc_t, cublasLtMatmulDescAttributes_t, *mut ::std::os::raw::c_void, usize, *mut usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulDescGetAttribute = val;
        self
    }
    pub fn cublasLtMatrixTransformDescInit_internal(mut self, val: Option<unsafe extern "C" fn(cublasLtMatrixTransformDesc_t, usize, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixTransformDescInit_internal = val;
        self
    }
    pub fn cublasLtMatrixTransformDescCreate(mut self, val: Option<unsafe extern "C" fn(*mut cublasLtMatrixTransformDesc_t, cudaDataType) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixTransformDescCreate = val;
        self
    }
    pub fn cublasLtMatrixTransformDescDestroy(mut self, val: Option<unsafe extern "C" fn(cublasLtMatrixTransformDesc_t) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixTransformDescDestroy = val;
        self
    }
    pub fn cublasLtMatrixTransformDescSetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtMatrixTransformDesc_t, cublasLtMatrixTransformDescAttributes_t, *const ::std::os::raw::c_void, usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixTransformDescSetAttribute = val;
        self
    }
    pub fn cublasLtMatrixTransformDescGetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtMatrixTransformDesc_t, cublasLtMatrixTransformDescAttributes_t, *mut ::std::os::raw::c_void, usize, *mut usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatrixTransformDescGetAttribute = val;
        self
    }
    pub fn cublasLtEmulationDescInit_internal(mut self, val: Option<unsafe extern "C" fn(cublasLtEmulationDesc_t, usize) -> cublasStatus_t>) -> Self {
        self.cublasLtEmulationDescInit_internal = val;
        self
    }
    pub fn cublasLtEmulationDescCreate(mut self, val: Option<unsafe extern "C" fn(*mut cublasLtEmulationDesc_t) -> cublasStatus_t>) -> Self {
        self.cublasLtEmulationDescCreate = val;
        self
    }
    pub fn cublasLtEmulationDescDestroy(mut self, val: Option<unsafe extern "C" fn(cublasLtEmulationDesc_t) -> cublasStatus_t>) -> Self {
        self.cublasLtEmulationDescDestroy = val;
        self
    }
    pub fn cublasLtEmulationDescSetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtEmulationDesc_t, cublasLtEmulationDescAttributes_t, *const ::std::os::raw::c_void, usize) -> cublasStatus_t>) -> Self {
        self.cublasLtEmulationDescSetAttribute = val;
        self
    }
    pub fn cublasLtEmulationDescGetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtEmulationDesc_t, cublasLtEmulationDescAttributes_t, *mut ::std::os::raw::c_void, usize, *mut usize) -> cublasStatus_t>) -> Self {
        self.cublasLtEmulationDescGetAttribute = val;
        self
    }
    pub fn cublasLtMatmulPreferenceInit_internal(mut self, val: Option<unsafe extern "C" fn(cublasLtMatmulPreference_t, usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulPreferenceInit_internal = val;
        self
    }
    pub fn cublasLtMatmulPreferenceCreate(mut self, val: Option<unsafe extern "C" fn(*mut cublasLtMatmulPreference_t) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulPreferenceCreate = val;
        self
    }
    pub fn cublasLtMatmulPreferenceDestroy(mut self, val: Option<unsafe extern "C" fn(cublasLtMatmulPreference_t) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulPreferenceDestroy = val;
        self
    }
    pub fn cublasLtMatmulPreferenceSetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtMatmulPreference_t, cublasLtMatmulPreferenceAttributes_t, *const ::std::os::raw::c_void, usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulPreferenceSetAttribute = val;
        self
    }
    pub fn cublasLtMatmulPreferenceGetAttribute(mut self, val: Option<unsafe extern "C" fn(cublasLtMatmulPreference_t, cublasLtMatmulPreferenceAttributes_t, *mut ::std::os::raw::c_void, usize, *mut usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulPreferenceGetAttribute = val;
        self
    }
    pub fn cublasLtMatmulAlgoGetHeuristic(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cublasLtHandle_t,
                cublasLtMatmulDesc_t,
                cublasLtMatrixLayout_t,
                cublasLtMatrixLayout_t,
                cublasLtMatrixLayout_t,
                cublasLtMatrixLayout_t,
                cublasLtMatmulPreference_t,
                ::std::os::raw::c_int,
                *mut cublasLtMatmulHeuristicResult_t,
                *mut ::std::os::raw::c_int,
            ) -> cublasStatus_t,
        >,
    ) -> Self {
        self.cublasLtMatmulAlgoGetHeuristic = val;
        self
    }
    pub fn cublasLtMatmulAlgoGetIds(
        mut self,
        val: Option<unsafe extern "C" fn(cublasLtHandle_t, cublasComputeType_t, cudaDataType_t, cudaDataType_t, cudaDataType_t, cudaDataType_t, cudaDataType_t, ::std::os::raw::c_int, *mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> cublasStatus_t>,
    ) -> Self {
        self.cublasLtMatmulAlgoGetIds = val;
        self
    }
    pub fn cublasLtMatmulAlgoInit(mut self, val: Option<unsafe extern "C" fn(cublasLtHandle_t, cublasComputeType_t, cudaDataType_t, cudaDataType_t, cudaDataType_t, cudaDataType_t, cudaDataType_t, ::std::os::raw::c_int, *mut cublasLtMatmulAlgo_t) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulAlgoInit = val;
        self
    }
    pub fn cublasLtMatmulAlgoCheck(
        mut self,
        val: Option<unsafe extern "C" fn(cublasLtHandle_t, cublasLtMatmulDesc_t, cublasLtMatrixLayout_t, cublasLtMatrixLayout_t, cublasLtMatrixLayout_t, cublasLtMatrixLayout_t, *const cublasLtMatmulAlgo_t, *mut cublasLtMatmulHeuristicResult_t) -> cublasStatus_t>,
    ) -> Self {
        self.cublasLtMatmulAlgoCheck = val;
        self
    }
    pub fn cublasLtMatmulAlgoCapGetAttribute(mut self, val: Option<unsafe extern "C" fn(*const cublasLtMatmulAlgo_t, cublasLtMatmulAlgoCapAttributes_t, *mut ::std::os::raw::c_void, usize, *mut usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulAlgoCapGetAttribute = val;
        self
    }
    pub fn cublasLtMatmulAlgoConfigSetAttribute(mut self, val: Option<unsafe extern "C" fn(*mut cublasLtMatmulAlgo_t, cublasLtMatmulAlgoConfigAttributes_t, *const ::std::os::raw::c_void, usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulAlgoConfigSetAttribute = val;
        self
    }
    pub fn cublasLtMatmulAlgoConfigGetAttribute(mut self, val: Option<unsafe extern "C" fn(*const cublasLtMatmulAlgo_t, cublasLtMatmulAlgoConfigAttributes_t, *mut ::std::os::raw::c_void, usize, *mut usize) -> cublasStatus_t>) -> Self {
        self.cublasLtMatmulAlgoConfigGetAttribute = val;
        self
    }
    pub fn cublasLtLoggerSetCallback(mut self, val: Option<unsafe extern "C" fn(cublasLtLoggerCallback_t) -> cublasStatus_t>) -> Self {
        self.cublasLtLoggerSetCallback = val;
        self
    }
    pub fn cublasLtLoggerSetFile(mut self, val: Option<unsafe extern "C" fn(*mut FILE) -> cublasStatus_t>) -> Self {
        self.cublasLtLoggerSetFile = val;
        self
    }
    pub fn cublasLtLoggerOpenFile(mut self, val: Option<unsafe extern "C" fn(*const ::std::os::raw::c_char) -> cublasStatus_t>) -> Self {
        self.cublasLtLoggerOpenFile = val;
        self
    }
    pub fn cublasLtLoggerSetLevel(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasLtLoggerSetLevel = val;
        self
    }
    pub fn cublasLtLoggerSetMask(mut self, val: Option<unsafe extern "C" fn(::std::os::raw::c_int) -> cublasStatus_t>) -> Self {
        self.cublasLtLoggerSetMask = val;
        self
    }
    pub fn cublasLtLoggerForceDisable(mut self, val: Option<unsafe extern "C" fn() -> cublasStatus_t>) -> Self {
        self.cublasLtLoggerForceDisable = val;
        self
    }
}
pub unsafe fn cublasLtCreate() -> Result<cublasLtHandle_t, crate::sys::cublasStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cublasLtHandle_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cublasLtHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasLtDestroy(lightHandle: cublasLtHandle_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtDestroy(lightHandle) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
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
pub unsafe fn cublasLtGetProperty(type_: libraryPropertyType) -> Result<i32, crate::sys::cublasStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasLtHeuristicsCacheGetCapacity() -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtHeuristicsCacheGetCapacity(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cublasLtHeuristicsCacheSetCapacity(capacity: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtHeuristicsCacheSetCapacity(capacity) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Restricts usage of CPU instructions (ISA) specified by the flags in the mask.\nFlags can be combined with bitwise OR(|) operator. Supported flags:\n- 0x1 -- x86-64 AVX512 ISA\nDefault mask: 0 (any applicable ISA is allowed).\nThe function returns the previous value of the mask.\nThe function takes precedence over the environment variable CUBLASLT_DISABLE_CPU_INSTRUCTIONS_MASK."]
pub unsafe fn cublasLtDisableCpuInstructionsSetMask(mask: u32) -> u32 {
    (unsafe { crate::sys::cublasLtDisableCpuInstructionsSetMask(mask as _) }) as u32
}
#[doc = "Execute matrix multiplication (D = alpha * op(A) * op(B) + beta * C).\n\\retval CUBLAS_STATUS_NOT_INITIALIZED   if cuBLASLt handle has not been initialized\n\\retval CUBLAS_STATUS_INVALID_VALUE     if parameters are in conflict or in an impossible configuration; e.g.\nwhen workspaceSizeInBytes is less than workspace required by configured\nalgo\n\\retval CUBLAS_STATUS_NOT_SUPPORTED     if current implementation on selected device doesn't support configured\noperation\n\\retval CUBLAS_STATUS_ARCH_MISMATCH     if configured operation cannot be run using selected device\n\\retval CUBLAS_STATUS_EXECUTION_FAILED  if cuda reported execution error from the device\n\\retval CUBLAS_STATUS_SUCCESS           if the operation completed successfully"]
pub unsafe fn cublasLtMatmul<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsPtr, T7: types::CudaAsMutPtr>(
    lightHandle: cublasLtHandle_t,
    computeDesc: cublasLtMatmulDesc_t,
    alpha: T0,
    A: T1,
    Adesc: cublasLtMatrixLayout_t,
    B: T2,
    Bdesc: cublasLtMatrixLayout_t,
    beta: T3,
    C: T4,
    Cdesc: cublasLtMatrixLayout_t,
    mut D: T5,
    Ddesc: cublasLtMatrixLayout_t,
    algo: T6,
    mut workspace: T7,
    workspaceSizeInBytes: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatmul(
            lightHandle,
            computeDesc,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Adesc,
            B.as_const_ptr() as *const _,
            Bdesc,
            beta.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            Cdesc,
            D.as_mut_ptr() as *mut _,
            Ddesc,
            algo.as_const_ptr() as *const _,
            workspace.as_mut_ptr() as *mut _,
            workspaceSizeInBytes,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Matrix layout conversion helper (C = alpha * op(A) + beta * op(B))\nCan be used to change memory order of data or to scale and shift the values.\n\\retval CUBLAS_STATUS_NOT_INITIALIZED   if cuBLASLt handle has not been initialized\n\\retval CUBLAS_STATUS_INVALID_VALUE     if parameters are in conflict or in an impossible configuration; e.g.\nwhen A is not NULL, but Adesc is NULL\n\\retval CUBLAS_STATUS_NOT_SUPPORTED     if current implementation on selected device doesn't support configured\noperation\n\\retval CUBLAS_STATUS_ARCH_MISMATCH     if configured operation cannot be run using selected device\n\\retval CUBLAS_STATUS_EXECUTION_FAILED  if cuda reported execution error from the device\n\\retval CUBLAS_STATUS_SUCCESS           if the operation completed successfully"]
pub unsafe fn cublasLtMatrixTransform<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    lightHandle: cublasLtHandle_t,
    transformDesc: cublasLtMatrixTransformDesc_t,
    alpha: T0,
    A: T1,
    Adesc: cublasLtMatrixLayout_t,
    beta: T2,
    B: T3,
    Bdesc: cublasLtMatrixLayout_t,
    mut C: T4,
    Cdesc: cublasLtMatrixLayout_t,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe {
        crate::sys::cublasLtMatrixTransform(
            lightHandle,
            transformDesc,
            alpha.as_const_ptr() as *const _,
            A.as_const_ptr() as *const _,
            Adesc,
            beta.as_const_ptr() as *const _,
            B.as_const_ptr() as *const _,
            Bdesc,
            C.as_mut_ptr() as *mut _,
            Cdesc,
            stream,
        )
    };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Internal. Do not use directly."]
pub unsafe fn cublasLtMatrixLayoutInit_internal(matLayout: cublasLtMatrixLayout_t, size: usize, type_: cudaDataType, rows: u64, cols: u64, ld: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixLayoutInit_internal(matLayout, size, type_, rows, cols, ld) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Experimental: Internal. Do not use directly."]
pub unsafe fn cublasLtGroupedMatrixLayoutInit_internal<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(matLayout: cublasLtMatrixLayout_t, size: usize, type_: cudaDataType, groupCount: i32, rows_array: T0, cols_array: T1, ld_array: T2) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtGroupedMatrixLayoutInit_internal(matLayout, size, type_, groupCount as _, rows_array.as_const_ptr() as *const _, cols_array.as_const_ptr() as *const _, ld_array.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Create new matrix layout descriptor.\n\\retval CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated\n\\retval CUBLAS_STATUS_SUCCESS       if desciptor was created successfully"]
pub unsafe fn cublasLtMatrixLayoutCreate<T0: types::CudaAsMutPtr>(mut matLayout: T0, type_: cudaDataType, rows: u64, cols: u64, ld: i64) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixLayoutCreate(matLayout.as_mut_ptr() as *mut _, type_, rows, cols, ld) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Experimental: Create new grouped matrix layout descriptor.\n\\retval CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated\n\\retval CUBLAS_STATUS_SUCCESS       if desciptor was created successfully"]
pub unsafe fn cublasLtGroupedMatrixLayoutCreate<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr>(mut matLayout: T0, type_: cudaDataType, groupCount: i32, rows_array: T1, cols_array: T2, ld_array: T3) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtGroupedMatrixLayoutCreate(matLayout.as_mut_ptr() as *mut _, type_, groupCount as _, rows_array.as_const_ptr() as *const _, cols_array.as_const_ptr() as *const _, ld_array.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Destroy matrix layout descriptor.\n\\retval CUBLAS_STATUS_SUCCESS  if operation was successful"]
pub unsafe fn cublasLtMatrixLayoutDestroy(matLayout: cublasLtMatrixLayout_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixLayoutDestroy(matLayout) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set matrix layout descriptor attribute.\n\n# Arguments\n\n* `matLayout` [in]  -    The descriptor\n* `attr` [in]  -         The attribute\n* `buf` [in]  -          memory address containing the new value\n* `sizeInBytes` [in]  -  size of buf buffer for verification (in bytes)\n\\retval CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatrixLayoutSetAttribute<T0: types::CudaAsPtr>(matLayout: cublasLtMatrixLayout_t, attr: cublasLtMatrixLayoutAttribute_t, buf: T0, sizeInBytes: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixLayoutSetAttribute(matLayout, attr, buf.as_const_ptr() as *const _, sizeInBytes) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get matrix layout descriptor attribute.\n\n# Arguments\n\n* `matLayout` [in]  -    The descriptor\n* `attr` [in]  -         The attribute\n* `buf` [out]  -          memory address containing the new value\n* `sizeInBytes` [in]  -  size of buf buffer for verification (in bytes)\n* `sizeWritten` [out]  -  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\nbytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\\retval CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\nand buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatrixLayoutGetAttribute<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(matLayout: cublasLtMatrixLayout_t, attr: cublasLtMatrixLayoutAttribute_t, mut buf: T0, sizeInBytes: usize, mut sizeWritten: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixLayoutGetAttribute(matLayout, attr, buf.as_mut_ptr() as *mut _, sizeInBytes, sizeWritten.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Internal. Do not use directly."]
pub unsafe fn cublasLtMatmulDescInit_internal(matmulDesc: cublasLtMatmulDesc_t, size: usize, computeType: cublasComputeType_t, scaleType: cudaDataType_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulDescInit_internal(matmulDesc, size, computeType, scaleType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Create new matmul operation descriptor.\n\\retval CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated\n\\retval CUBLAS_STATUS_SUCCESS       if desciptor was created successfully"]
pub unsafe fn cublasLtMatmulDescCreate(computeType: cublasComputeType_t, scaleType: cudaDataType_t) -> Result<cublasLtMatmulDesc_t, crate::sys::cublasStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cublasLtMatmulDesc_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtMatmulDescCreate(out_0.as_mut_ptr() as *mut _, computeType, scaleType) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cublasLtMatmulDesc_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroy matmul operation descriptor.\n\\retval CUBLAS_STATUS_SUCCESS  if operation was successful"]
pub unsafe fn cublasLtMatmulDescDestroy(matmulDesc: cublasLtMatmulDesc_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulDescDestroy(matmulDesc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set matmul operation descriptor attribute.\n\n# Arguments\n\n* `matmulDesc` [in]  -   The descriptor\n* `attr` [in]  -         The attribute\n* `buf` [in]  -          memory address containing the new value\n* `sizeInBytes` [in]  -  size of buf buffer for verification (in bytes)\n\\retval CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatmulDescSetAttribute<T0: types::CudaAsPtr>(matmulDesc: cublasLtMatmulDesc_t, attr: cublasLtMatmulDescAttributes_t, buf: T0, sizeInBytes: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulDescSetAttribute(matmulDesc, attr, buf.as_const_ptr() as *const _, sizeInBytes) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get matmul operation descriptor attribute.\n\n# Arguments\n\n* `matmulDesc` [in]  -   The descriptor\n* `attr` [in]  -         The attribute\n* `buf` [out]  -          memory address containing the new value\n* `sizeInBytes` [in]  -  size of buf buffer for verification (in bytes)\n* `sizeWritten` [out]  -  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\nbytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\\retval CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\nand buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatmulDescGetAttribute(matmulDesc: cublasLtMatmulDesc_t, attr: cublasLtMatmulDescAttributes_t, buf: *mut ::std::os::raw::c_void, sizeInBytes: usize) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtMatmulDescGetAttribute(matmulDesc, attr, buf, sizeInBytes, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Internal. Do not use directly."]
pub unsafe fn cublasLtMatrixTransformDescInit_internal(transformDesc: cublasLtMatrixTransformDesc_t, size: usize, scaleType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixTransformDescInit_internal(transformDesc, size, scaleType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Create new matrix transform operation descriptor.\n\\retval CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated\n\\retval CUBLAS_STATUS_SUCCESS       if desciptor was created successfully"]
pub unsafe fn cublasLtMatrixTransformDescCreate<T0: types::CudaAsMutPtr>(mut transformDesc: T0, scaleType: cudaDataType) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixTransformDescCreate(transformDesc.as_mut_ptr() as *mut _, scaleType) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Destroy matrix transform operation descriptor.\n\\retval CUBLAS_STATUS_SUCCESS  if operation was successful"]
pub unsafe fn cublasLtMatrixTransformDescDestroy(transformDesc: cublasLtMatrixTransformDesc_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixTransformDescDestroy(transformDesc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set matrix transform operation descriptor attribute.\n\n# Arguments\n\n* `transformDesc` [in]  -  The descriptor\n* `attr` [in]  -           The attribute\n* `buf` [in]  -            memory address containing the new value\n* `sizeInBytes` [in]  -    size of buf buffer for verification (in bytes)\n\\retval CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatrixTransformDescSetAttribute<T0: types::CudaAsPtr>(transformDesc: cublasLtMatrixTransformDesc_t, attr: cublasLtMatrixTransformDescAttributes_t, buf: T0, sizeInBytes: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixTransformDescSetAttribute(transformDesc, attr, buf.as_const_ptr() as *const _, sizeInBytes) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get matrix transform operation descriptor attribute.\n\n# Arguments\n\n* `transformDesc` [in]  -  The descriptor\n* `attr` [in]  -           The attribute\n* `buf` [out]  -            memory address containing the new value\n* `sizeInBytes` [in]  -    size of buf buffer for verification (in bytes)\n* `sizeWritten` [out]  -    only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number\nof bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\\retval CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\nand buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatrixTransformDescGetAttribute<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(transformDesc: cublasLtMatrixTransformDesc_t, attr: cublasLtMatrixTransformDescAttributes_t, mut buf: T0, sizeInBytes: usize, mut sizeWritten: T1) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatrixTransformDescGetAttribute(transformDesc, attr, buf.as_mut_ptr() as *mut _, sizeInBytes, sizeWritten.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Internal. Do not use directly."]
pub unsafe fn cublasLtEmulationDescInit_internal(emulationDesc: cublasLtEmulationDesc_t, size: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtEmulationDescInit_internal(emulationDesc, size) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Create new emulation descriptor.\n\\retval CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated\n\\retval CUBLAS_STATUS_SUCCESS       if desciptor was created successfully"]
pub unsafe fn cublasLtEmulationDescCreate() -> Result<cublasLtEmulationDesc_t, crate::sys::cublasStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cublasLtEmulationDesc_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtEmulationDescCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cublasLtEmulationDesc_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroy emulation descriptor.\n\\retval CUBLAS_STATUS_SUCCESS  if operation was successful"]
pub unsafe fn cublasLtEmulationDescDestroy(emulationDesc: cublasLtEmulationDesc_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtEmulationDescDestroy(emulationDesc) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set emulation descriptor attribute.\n\n# Arguments\n\n* `emulationDesc` [in]  -  The descriptor\n* `attr` [in]  -           The attribute\n* `buf` [in]  -            memory address containing the new value\n* `sizeInBytes` [in]  -    size of buf buffer for verification (in bytes)\n\\retval CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtEmulationDescSetAttribute<T0: types::CudaAsPtr>(emulationDesc: cublasLtEmulationDesc_t, attr: cublasLtEmulationDescAttributes_t, buf: T0, sizeInBytes: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtEmulationDescSetAttribute(emulationDesc, attr, buf.as_const_ptr() as *const _, sizeInBytes) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get emulation descriptor attribute.\n\n# Arguments\n\n* `emulationDesc` [in]  -  The descriptor\n* `attr` [in]  -           The attribute\n* `buf` [out]  -            memory address containing the new value\n* `sizeInBytes` [in]  -    size of buf buffer for verification (in bytes)\n* `sizeWritten` [out]  -    only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number\nof bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\\retval CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\nand buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtEmulationDescGetAttribute(emulationDesc: cublasLtEmulationDesc_t, attr: cublasLtEmulationDescAttributes_t, buf: *mut ::std::os::raw::c_void, sizeInBytes: usize) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtEmulationDescGetAttribute(emulationDesc, attr, buf, sizeInBytes, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Internal. Do not use directly."]
pub unsafe fn cublasLtMatmulPreferenceInit_internal(pref: cublasLtMatmulPreference_t, size: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulPreferenceInit_internal(pref, size) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Create new matmul heuristic search preference descriptor.\n\\retval CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated\n\\retval CUBLAS_STATUS_SUCCESS       if desciptor was created successfully"]
pub unsafe fn cublasLtMatmulPreferenceCreate() -> Result<cublasLtMatmulPreference_t, crate::sys::cublasStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cublasLtMatmulPreference_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtMatmulPreferenceCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cublasLtMatmulPreference_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroy matmul heuristic search preference descriptor.\n\\retval CUBLAS_STATUS_SUCCESS  if operation was successful"]
pub unsafe fn cublasLtMatmulPreferenceDestroy(pref: cublasLtMatmulPreference_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulPreferenceDestroy(pref) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set matmul heuristic search preference descriptor attribute.\n\n# Arguments\n\n* `pref` [in]  -         The descriptor\n* `attr` [in]  -         The attribute\n* `buf` [in]  -          memory address containing the new value\n* `sizeInBytes` [in]  -  size of buf buffer for verification (in bytes)\n\\retval CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatmulPreferenceSetAttribute<T0: types::CudaAsPtr>(pref: cublasLtMatmulPreference_t, attr: cublasLtMatmulPreferenceAttributes_t, buf: T0, sizeInBytes: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulPreferenceSetAttribute(pref, attr, buf.as_const_ptr() as *const _, sizeInBytes) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get matmul heuristic search preference descriptor attribute.\n\n# Arguments\n\n* `pref` [in]  -         The descriptor\n* `attr` [in]  -         The attribute\n* `buf` [out]  -          memory address containing the new value\n* `sizeInBytes` [in]  -  size of buf buffer for verification (in bytes)\n* `sizeWritten` [out]  -  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\nbytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\\retval CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\nand buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatmulPreferenceGetAttribute(pref: cublasLtMatmulPreference_t, attr: cublasLtMatmulPreferenceAttributes_t, buf: *mut ::std::os::raw::c_void, sizeInBytes: usize) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtMatmulPreferenceGetAttribute(pref, attr, buf, sizeInBytes, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Query cublasLt heuristic for algorithm appropriate for given use case.\n\n# Arguments\n\n* `lightHandle` [in]  -            Pointer to the allocated cuBLASLt handle for the cuBLASLt\ncontext. See cublasLtHandle_t.\n* `operationDesc` [in]  -          Handle to the matrix multiplication descriptor.\n* `Adesc` [in]  -                  Handle to the layout descriptors for matrix A.\n* `Bdesc` [in]  -                  Handle to the layout descriptors for matrix B.\n* `Cdesc` [in]  -                  Handle to the layout descriptors for matrix C.\n* `Ddesc` [in]  -                  Handle to the layout descriptors for matrix D.\n* `preference` [in]  -             Pointer to the structure holding the heuristic search\npreferences descriptor. See cublasLtMatrixLayout_t.\n* `requestedAlgoCount` [in]  -     Size of heuristicResultsArray (in elements) and requested\nmaximum number of algorithms to return.\n* `heuristicResultsArray` [in, out]  -  Output algorithms and associated runtime characteristics,\nordered in increasing estimated compute time.\n* `returnAlgoCount` [out]  -        The number of heuristicResultsArray elements written.\n\\retval CUBLAS_STATUS_INVALID_VALUE   if requestedAlgoCount is less or equal to zero\n\\retval CUBLAS_STATUS_NOT_SUPPORTED   if no heuristic function available for current configuration\n\\retval CUBLAS_STATUS_SUCCESS         if query was successful, inspect\nheuristicResultsArray[0 to (returnAlgoCount - 1)].state\nfor detail status of results"]
pub unsafe fn cublasLtMatmulAlgoGetHeuristic(
    lightHandle: cublasLtHandle_t,
    operationDesc: cublasLtMatmulDesc_t,
    Adesc: cublasLtMatrixLayout_t,
    Bdesc: cublasLtMatrixLayout_t,
    Cdesc: cublasLtMatrixLayout_t,
    Ddesc: cublasLtMatrixLayout_t,
    preference: cublasLtMatmulPreference_t,
    requestedAlgoCount: i32,
    heuristicResultsArray: *mut cublasLtMatmulHeuristicResult_t,
) -> Result<i32, crate::sys::cublasStatus_t> {
    let mut out_9: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtMatmulAlgoGetHeuristic(lightHandle, operationDesc, Adesc, Bdesc, Cdesc, Ddesc, preference, requestedAlgoCount as _, heuristicResultsArray, out_9.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_9.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Routine to get all algo IDs that can potentially run\n\n# Arguments\n\n* `int` [in]  -              requestedAlgoCount requested number of algos (must be less or equal to size of algoIdsA\n(in elements)) * `algoIdsA` [out]  -         array to write algoIds to * `returnAlgoCount` [out]  -  number of algoIds\nactually written\n\\retval CUBLAS_STATUS_INVALID_VALUE  if requestedAlgoCount is less or equal to zero\n\\retval CUBLAS_STATUS_SUCCESS        if query was successful, inspect returnAlgoCount to get actual number of IDs\navailable"]
pub unsafe fn cublasLtMatmulAlgoGetIds(
    lightHandle: cublasLtHandle_t,
    computeType: cublasComputeType_t,
    scaleType: cudaDataType_t,
    Atype: cudaDataType_t,
    Btype: cudaDataType_t,
    Ctype: cudaDataType_t,
    Dtype: cudaDataType_t,
    requestedAlgoCount: i32,
    algoIdsArray: *mut ::std::os::raw::c_int,
) -> Result<i32, crate::sys::cublasStatus_t> {
    let mut out_9: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtMatmulAlgoGetIds(lightHandle, computeType, scaleType, Atype, Btype, Ctype, Dtype, requestedAlgoCount as _, algoIdsArray, out_9.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_9.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Initialize algo structure\n\\retval CUBLAS_STATUS_INVALID_VALUE  if algo is NULL or algoId is outside of recognized range\n\\retval CUBLAS_STATUS_NOT_SUPPORTED  if algoId is not supported for given combination of data types\n\\retval CUBLAS_STATUS_SUCCESS        if the structure was successfully initialized"]
pub unsafe fn cublasLtMatmulAlgoInit<T0: types::CudaAsMutPtr>(
    lightHandle: cublasLtHandle_t,
    computeType: cublasComputeType_t,
    scaleType: cudaDataType_t,
    Atype: cudaDataType_t,
    Btype: cudaDataType_t,
    Ctype: cudaDataType_t,
    Dtype: cudaDataType_t,
    algoId: i32,
    mut algo: T0,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulAlgoInit(lightHandle, computeType, scaleType, Atype, Btype, Ctype, Dtype, algoId as _, algo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Check configured algo descriptor for correctness and support on current device.\nResult includes required workspace size and calculated wave count.\nCUBLAS_STATUS_SUCCESS doesn't fully guarantee algo will run (will fail if e.g. buffers are not correctly aligned);\nbut if cublasLtMatmulAlgoCheck fails, the algo will not run.\n\n# Arguments\n\n* `algo` [in]  -    algo configuration to check\n* `result` [out]  -  result structure to report algo runtime characteristics; algo field is never updated\n\\retval CUBLAS_STATUS_INVALID_VALUE  if matrix layout descriptors or operation descriptor don't match algo\ndescriptor\n\\retval CUBLAS_STATUS_NOT_SUPPORTED  if algo configuration or data type combination is not currently supported on\ngiven device\n\\retval CUBLAS_STATUS_ARCH_MISMATCH  if algo configuration cannot be run using the selected device\n\\retval CUBLAS_STATUS_SUCCESS        if check was successful"]
pub unsafe fn cublasLtMatmulAlgoCheck<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(
    lightHandle: cublasLtHandle_t,
    operationDesc: cublasLtMatmulDesc_t,
    Adesc: cublasLtMatrixLayout_t,
    Bdesc: cublasLtMatrixLayout_t,
    Cdesc: cublasLtMatrixLayout_t,
    Ddesc: cublasLtMatrixLayout_t,
    algo: T0,
    mut result: T1,
) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulAlgoCheck(lightHandle, operationDesc, Adesc, Bdesc, Cdesc, Ddesc, algo.as_const_ptr() as *const _, result.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get algo capability attribute.\nE.g. to get list of supported Tile IDs:\ncublasLtMatmulTile_t tiles[CUBLASLT_MATMUL_TILE_END];\nsize_t num_tiles, size_written;\nif (cublasLtMatmulAlgoCapGetAttribute(algo, CUBLASLT_ALGO_CAP_TILE_IDS, tiles, sizeof(tiles), size_written) ==\nCUBLAS_STATUS_SUCCESS) { num_tiles = size_written / sizeof(tiles[0]);\n}\n\n# Arguments\n\n* `algo` [in]  -         The algo descriptor\n* `attr` [in]  -         The attribute\n* `buf` [out]  -          memory address containing the new value\n* `sizeInBytes` [in]  -  size of buf buffer for verification (in bytes)\n* `sizeWritten` [out]  -  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\nbytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\\retval CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\nand buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatmulAlgoCapGetAttribute(algo: *const cublasLtMatmulAlgo_t, attr: cublasLtMatmulAlgoCapAttributes_t, buf: *mut ::std::os::raw::c_void, sizeInBytes: usize) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtMatmulAlgoCapGetAttribute(algo, attr, buf, sizeInBytes, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Set algo configuration attribute.\n\n# Arguments\n\n* `algo` [in]  -         The algo descriptor\n* `attr` [in]  -         The attribute\n* `buf` [in]  -          memory address containing the new value\n* `sizeInBytes` [in]  -  size of buf buffer for verification (in bytes)\n\\retval CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute was set successfully"]
pub unsafe fn cublasLtMatmulAlgoConfigSetAttribute<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr>(mut algo: T0, attr: cublasLtMatmulAlgoConfigAttributes_t, buf: T1, sizeInBytes: usize) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtMatmulAlgoConfigSetAttribute(algo.as_mut_ptr() as *mut _, attr, buf.as_const_ptr() as *const _, sizeInBytes) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get algo configuration attribute.\n\n# Arguments\n\n* `algo` [in]  -         The algo descriptor\n* `attr` [in]  -         The attribute\n* `buf` [out]  -          memory address containing the new value\n* `sizeInBytes` [in]  -  size of buf buffer for verification (in bytes)\n* `sizeWritten` [out]  -  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of\nbytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents\n\\retval CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero\nand buf is NULL or sizeInBytes doesn't match size of internal storage for\nselected attribute\n\\retval CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory"]
pub unsafe fn cublasLtMatmulAlgoConfigGetAttribute(algo: *const cublasLtMatmulAlgo_t, attr: cublasLtMatmulAlgoConfigAttributes_t, buf: *mut ::std::os::raw::c_void, sizeInBytes: usize) -> Result<usize, crate::sys::cublasStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cublasLtMatmulAlgoConfigGetAttribute(algo, attr, buf, sizeInBytes, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Experimental: Logger callback setter.\n\n# Arguments\n\n* `callback` [in]  -                     a user defined callback function to be called by the logger\n\\retval CUBLAS_STATUS_SUCCESS        if callback was set successfully"]
pub unsafe fn cublasLtLoggerSetCallback(callback: cublasLtLoggerCallback_t) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerSetCallback(callback) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Experimental: Log file setter.\n\n# Arguments\n\n* `file` [in]  -                         an open file with write permissions\n\\retval CUBLAS_STATUS_SUCCESS        if log file was set successfully"]
pub unsafe fn cublasLtLoggerSetFile<T0: types::CudaAsMutPtr>(mut file: T0) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerSetFile(file.as_mut_ptr() as *mut _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Experimental: Open log file.\n\n# Arguments\n\n* `logFile` [in]  -                      log file path. if the log file does not exist, it will be created\n\\retval CUBLAS_STATUS_SUCCESS        if log file was created successfully"]
pub unsafe fn cublasLtLoggerOpenFile<T0: types::CudaAsPtr>(logFile: T0) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerOpenFile(logFile.as_const_ptr() as *const _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Experimental: Log level setter.\n\n# Arguments\n\n* `level` [in]  -                        log level, should be one of the following:\n0. Off\n1. Errors\n2. Performance Trace\n3. Performance Hints\n4. Heuristics Trace\n5. API Trace\n\\retval CUBLAS_STATUS_INVALID_VALUE  if log level is not one of the above levels\n\\retval CUBLAS_STATUS_SUCCESS        if log level was set successfully"]
pub unsafe fn cublasLtLoggerSetLevel(level: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerSetLevel(level as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Experimental: Log mask setter.\n\n# Arguments\n\n* `mask` [in]  -                         log mask, should be a combination of the following masks:\n0.  Off\n1.  Errors\n2.  Performance Trace\n4.  Performance Hints\n8.  Heuristics Trace\n16. API Trace\n\\retval CUBLAS_STATUS_SUCCESS        if log mask was set successfully"]
pub unsafe fn cublasLtLoggerSetMask(mask: i32) -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerSetMask(mask as _) };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Experimental: Disable logging for the entire session.\n\\retval CUBLAS_STATUS_SUCCESS        if disabled logging"]
pub unsafe fn cublasLtLoggerForceDisable() -> Result<(), crate::sys::cublasStatus_t> {
    let status = unsafe { crate::sys::cublasLtLoggerForceDisable() };
    if status == crate::sys::cublasStatus_t::CUBLAS_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
